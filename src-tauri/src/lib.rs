mod commands;
mod setup;

use std::{
    collections::HashSet,
    sync::{atomic::AtomicBool, Arc, Mutex},
};

use serde::{Deserialize, Serialize};
use setup::get_setup_handler;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InputEvent {
    pub event_type: String,
    pub key: Option<String>,
    pub modifiers: Vec<String>,
    pub timestamp: u128,
}

pub type ModifierState = Arc<Mutex<HashSet<String>>>;
pub type IsListening = Arc<AtomicBool>;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default();

    let setup_handler = get_setup_handler();

    builder
        .manage(IsListening::new(AtomicBool::new(false)))
        .manage(ModifierState::new(Mutex::new(HashSet::new())))
        .setup(setup_handler)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::monitoring::start_monitoring,
            commands::monitoring::stop_monitoring
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
