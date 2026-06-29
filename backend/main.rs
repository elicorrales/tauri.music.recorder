// Prevents the background console window from opening on Windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod melodies;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            database::save_recording,
            database::load_recordings,
            database::delete_recording,
            melodies::get_theme
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
