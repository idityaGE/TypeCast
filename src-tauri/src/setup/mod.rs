pub mod tray;
use tauri::Manager;

use crate::setup::tray::setup_tray;

pub fn get_setup_handler() -> impl Fn(&mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    |app| {
        let window = app.get_webview_window("main").unwrap();
        window.set_always_on_top(true)?;
        window.set_ignore_cursor_events(true)?;
        window.set_fullscreen(true)?;
        window.set_skip_taskbar(true)?;
        window.set_visible_on_all_workspaces(true)?;

        setup_tray(app)?;
        Ok(())
    }
}
