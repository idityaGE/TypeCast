pub mod tray;
use crate::setup::tray::setup_tray;

pub fn get_setup_handler() -> impl Fn(&mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    |app| {
        setup_tray(app)?;
        Ok(())
    }
}
