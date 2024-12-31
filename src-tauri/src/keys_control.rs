// src/keys_control.rs
use tauri::WebviewWindow; // Use WebviewWindow instead of Window

/// Injects JavaScript into the Tauri window to handle key controls.
pub fn setup_key_controls(window: &WebviewWindow) -> Result<(), tauri::Error> {
    window.eval(
        r#"
        document.addEventListener('keydown', (event) => {
            if (event.key === 'a' || event.key === 'A') {
                window.history.back();
            } else if (event.key === 's' || event.key === 'S') {
                window.history.forward();
            }
        });
        "#,
    )
}
