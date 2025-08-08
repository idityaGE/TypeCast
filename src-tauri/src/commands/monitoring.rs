use std::sync::atomic::Ordering;
use tauri::State;

use crate::{IsListening, ModifierState};

#[tauri::command]
pub fn stop_monitoring(
    is_listening: State<'_, IsListening>,
    modifier_state: State<'_, ModifierState>,
) -> Result<String, String> {
    is_listening.store(false, Ordering::SeqCst);
    modifier_state.lock().unwrap().clear();
    Ok(String::from("Stopped"))
}

#[tauri::command]
pub fn start_monitoring(
    is_listening: State<'_, IsListening>,
) -> Result<String, String> {
    is_listening.store(true, Ordering::SeqCst);
    Ok("Monitoring Started".to_string())
}