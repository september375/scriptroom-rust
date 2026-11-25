// Avoid a separate console window in release builds on Windows.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Native Rust application host. The frontend is embedded by Tauri.
    // No filesystem/shell plugins or custom IPC commands are exposed.
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("Scriptroom could not start");
}
