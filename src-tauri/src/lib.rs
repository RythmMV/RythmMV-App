// src/lib.rs
use tauri::Manager;

mod keys_control; // Ensure this module is included

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Use get_webview_window instead of get_window
            if let Some(main_window) = app.handle().get_webview_window("main") {
                // Pass the correct type to the setup_key_controls function
                keys_control::setup_key_controls(&main_window)?;
            }
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
