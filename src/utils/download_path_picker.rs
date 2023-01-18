/// Function for selecting download dir
pub async fn select_download_path() -> Option<String> {
    let task = rfd::AsyncFileDialog::new().pick_folder();
    let handle = task.await;
    handle.and_then(|handle| handle.path().to_str().map(String::from))
}
