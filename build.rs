#[cfg(target_os = "windows")]
use std::env::current_dir;
// build.rs

#[cfg(target_os = "windows")]
fn main() {
    let mut res = winres::WindowsResource::new();
    let current_dir = current_dir().unwrap();
    let icon_path = current_dir.join("assets/images/icon.ico");
    let icon_path_str = icon_path.to_str().unwrap();
    res.set_icon(icon_path_str);
    res.compile().unwrap();
}

#[cfg(not(target_os = "windows"))]
fn main() {}
