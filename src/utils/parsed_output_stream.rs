use iced::futures;
use iced::futures::Stream;
use log::{error, info, warn};
// use std::env::current_dir;
use youtube_dl_parser::expressions::{mp3_download_expression, mp4_download_expression};
use youtube_dl_parser::reader::{OutputReader, ParsedOutputReader};
use youtube_dl_parser::state::parsed_state::{
    DeletingState, DownloadState, ErrorState, FFmpegState, ParsedState, YoutubeState,
};
use youtube_dl_parser::state::ParsedOutputState;

use crate::gui::Message;
use crate::query_cache;
use crate::utils::format::Format;

pub fn parsed_output_stream() -> impl Stream<Item = Message> + Send {
    futures::stream::unfold(InternalParsedOutputState::Setup, |state| async move {
        match state {
            InternalParsedOutputState::Setup => {
                // When a subscription is ran, get the url, download directory and format from the cache
                let (url, destination, format) = match query_cache::replace_global(None) {
                    Ok(cache) => match cache {
                        Some(cache) => cache,
                        None => {
                            return Some((
                                Message::Failed("Cache was empty".to_owned()),
                                InternalParsedOutputState::Failed,
                            ));
                        }
                    },
                    Err(err) => {
                        return Some((
                            Message::Failed(err.to_string()),
                            InternalParsedOutputState::Failed,
                        ));
                    }
                };

                let youtube_dl_executable_name = if cfg!(target_os = "windows") {
                    "youtube-dl.exe"
                } else {
                    "youtube-dl"
                };

                // Expect the youtube-dl executable to be in the current directory
                // let current_dir = match current_dir()
                //     .map(|dir| dir.join(youtube_dl_executable_name))
                //     .map_err(|err| err.to_string())
                //     .and_then(|path| match path.to_str() {
                //         None => Err("youtube-dl executable path is not utf-8".to_owned()),
                //         Some(str) => str,
                //     })
                //     .map_err(|err| format!("Unable to obtain current dir: {err}"))
                // {
                //     Ok(current_dir) => current_dir,
                //     Err(err) => {
                //         return Some((Message::Failed(err), InternalParsedOutputState::Failed));
                //     }
                // };

                let big_cmd = match format {
                    Format::Mp3 => {
                        mp3_download_expression(youtube_dl_executable_name, &url, &destination)
                    }
                    Format::Mp4 => {
                        mp4_download_expression(youtube_dl_executable_name, &url, &destination)
                    }
                };

                // Get the youtube-dl stdout
                let stdout = match big_cmd.stderr_to_stdout().reader() {
                    Ok(stdout) => stdout,
                    Err(err) => {
                        error!("Unable to get stdout: {err}");
                        return Some((
                            Message::Failed("No stdout output".to_owned()),
                            InternalParsedOutputState::Finish,
                        ));
                    }
                };

                // Put the stdout into an output reader
                let output_reader = OutputReader::new(stdout);

                // Get the parsed output
                let parsed_output_reader = ParsedOutputReader::new(output_reader);

                // Start reading the parsing output
                Some((
                    Message::Empty,
                    InternalParsedOutputState::Read(parsed_output_reader),
                ))
            }
            InternalParsedOutputState::Read(mut parsed_output_reader) => {
                let parsed_output_state = match parsed_output_reader.next() {
                    None => return Some((Message::Saved, InternalParsedOutputState::Finish)),
                    Some(parsed_output_state) => parsed_output_state,
                };
                match parsed_output_state {
                    ParsedOutputState::Parsed(parsed_state) => match parsed_state {
                        ParsedState::Youtube(youtube_state) => match youtube_state {
                            YoutubeState::Initiating => {
                                info!("Starting download");
                                Some((
                                    Message::Empty,
                                    InternalParsedOutputState::Read(parsed_output_reader),
                                ))
                            }
                            YoutubeState::ParseError(err) => {
                                warn!("Download parse error: {err}");
                                Some((
                                    Message::Empty,
                                    InternalParsedOutputState::Read(parsed_output_reader),
                                ))
                            }
                        },
                        ParsedState::Download(download_state) => match download_state {
                            DownloadState::Destination(destination) => {
                                info!("Saving temp file to: {destination}");
                                Some((
                                    Message::Empty,
                                    InternalParsedOutputState::Read(parsed_output_reader),
                                ))
                            }
                            DownloadState::Resuming(byte) => {
                                info!("Resuming from byte: {byte}");
                                Some((
                                    Message::Empty,
                                    InternalParsedOutputState::Read(parsed_output_reader),
                                ))
                            }
                            DownloadState::Downloading(
                                progress,
                                total_size,
                                download_speed,
                                eta,
                            ) => {
                                info!("Progress: {progress}%, Total size: {total_size} bytes, Download speed: {download_speed} bytes per second, ETA: {eta} seconds");
                                Some((
                                    Message::Downloading(progress, total_size, download_speed, eta),
                                    InternalParsedOutputState::Read(parsed_output_reader),
                                ))
                            }
                            DownloadState::Downloaded(progress, total_size, completion_time) => {
                                info!("Download finished at {progress}% of {total_size} bytes in {completion_time} seconds");
                                Some((
                                    Message::Downloaded(progress, total_size, completion_time),
                                    InternalParsedOutputState::Read(parsed_output_reader),
                                ))
                            }
                            DownloadState::ParseError(err) => {
                                warn!("Download parse error: {err}");
                                Some((
                                    Message::Empty,
                                    InternalParsedOutputState::Read(parsed_output_reader),
                                ))
                            }
                        },
                        ParsedState::FFMpeg(ffmpeg_state) => match ffmpeg_state {
                            FFmpegState::Destination(destination) => {
                                info!("Saving mp3 file to: {destination}");
                                Some((
                                    Message::Converting(destination),
                                    InternalParsedOutputState::Read(parsed_output_reader),
                                ))
                            }
                            FFmpegState::ParseError(err) => {
                                warn!("FFmpeg parse error: {err}");
                                Some((
                                    Message::Empty,
                                    InternalParsedOutputState::Read(parsed_output_reader),
                                ))
                            }
                        },
                        ParsedState::None(output) => {
                            warn!("Unrecognized output: {output}");
                            Some((
                                Message::Empty,
                                InternalParsedOutputState::Read(parsed_output_reader),
                            ))
                        }
                        ParsedState::Unknown(output) => {
                            warn!("Unknown state: {output}");
                            Some((
                                Message::Empty,
                                InternalParsedOutputState::Read(parsed_output_reader),
                            ))
                        }
                        ParsedState::ParseError(err) => {
                            warn!("Output parse error: {err}");
                            Some((
                                Message::Empty,
                                InternalParsedOutputState::Read(parsed_output_reader),
                            ))
                        }
                        ParsedState::Error(err) => match err {
                            ErrorState::Error(err) => {
                                error!("Error occurred: {err}");
                                Some((Message::Failed(err), InternalParsedOutputState::Failed))
                            }
                        },
                        ParsedState::Deleting(deleting_state) => match deleting_state {
                            DeletingState::DeletingTemporaryFile(destination) => {
                                info!("Deleting temporary file at: {destination}");
                                Some((
                                    Message::Empty,
                                    InternalParsedOutputState::Read(parsed_output_reader),
                                ))
                            }
                            DeletingState::ParseError(err) => {
                                warn!("Deleting parse error: {err}");
                                Some((
                                    Message::Empty,
                                    InternalParsedOutputState::Read(parsed_output_reader),
                                ))
                            }
                        },
                    },
                    ParsedOutputState::Finished => {
                        info!("Finished output");
                        Some((Message::Saved, InternalParsedOutputState::Finish))
                    }
                    ParsedOutputState::Error(err) => {
                        error!("Exit code: {:?}, Error: {}", err.exit_code, err.error);
                        Some((
                            Message::Failed(err.error.to_string()),
                            InternalParsedOutputState::Failed,
                        ))
                    }
                }
            }
            InternalParsedOutputState::Finish => None,
            InternalParsedOutputState::Failed => None,
        }
    })
}

pub enum InternalParsedOutputState {
    Setup,
    Read(ParsedOutputReader),
    Finish,
    Failed,
}
