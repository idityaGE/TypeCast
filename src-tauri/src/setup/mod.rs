pub mod tray;
use tauri::Manager;

use crate::setup::tray::setup_tray;

pub fn get_setup_handler() -> impl Fn(&mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    |app| {
        let window = app.get_webview_window("main").unwrap();
        window.set_ignore_cursor_events(true)?;

        setup_tray(app)?;
        Ok(())
    }
}
