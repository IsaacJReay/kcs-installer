#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod handler;
mod db;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![handler::get_install_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
