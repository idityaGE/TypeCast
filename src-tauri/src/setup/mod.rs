pub mod keystoke;
pub mod tray;

use crate::setup::{keystoke::start_keystoke_listener, tray::setup_tray};
use tauri::Manager;

pub fn get_setup_handler() -> impl Fn(&mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    |app| {
        let window = app.get_webview_window("main").unwrap();
        window.set_ignore_cursor_events(true)?;

        setup_tray(app)?;
        start_keystoke_listener(app);

        Ok(())
    }
}
