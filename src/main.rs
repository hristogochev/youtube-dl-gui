use single_instance::SingleInstance;
#[cfg(target_os = "windows")]
use winapi::um::wincon::GetConsoleWindow;
#[cfg(target_os = "windows")]
use winapi::um::winuser::ShowWindow;
#[cfg(target_os = "windows")]
use winapi::um::winuser::SW_HIDE;

use utils::logger::init_logger;
use utils::query_cache;

mod gui;
mod resources;
mod utils;
mod views;

pub fn main() -> iced::Result {
    // If in release mode, hide the console window
    // On windows, we cannot use #![windows_subsystem = "windows"]
    // since it will create no console from which to read youtube-dl output
    hide_console_window();

    // Make sure there is only a single instance of the application running
    let instance = SingleInstance::new("youtube-dl-gui")
        .expect("There is another instance of the application running");

    if !instance.is_single() {
        panic!("There is another instance of the application running");
    }

    // Init the logger of the application
    if let Err(err) = init_logger() {
        panic!("{}", err);
    }

    // Init the download subscription inputs cache
    // We only want to clone the inputs of each subscription once, not everytime the state of the GUI updates.
    query_cache::init_global().expect("Error error");

    // Show the GUI
    gui::Gui::start()
}

#[cfg(target_os = "windows")]
pub fn hide_console_window() {
    if !cfg!(debug_assertions) {
        let console_window_handle = unsafe { GetConsoleWindow() };
        if console_window_handle.is_null() {
            panic!("Unable to get console window handle");
        }

        let result = unsafe { ShowWindow(console_window_handle, SW_HIDE) };
        if result == 0 {
            panic!("Unable to hide console window");
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub fn hide_console_window() {}
