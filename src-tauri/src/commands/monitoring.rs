use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use rdev::{listen, Event, EventType, Key};
use tauri::{Manager, State};

use crate::{EventSender, InputEvent, ModifierState, TaskData, TaskDataState};

#[tauri::command]
pub fn stop_monitoring(
    event_sender: State<'_, EventSender>,
    modifier_state: State<'_, ModifierState>,
    task_data: State<'_, TaskDataState>,
) -> Option<TaskData> {
    *event_sender.lock().unwrap() = None;
    modifier_state.lock().unwrap().clear();
    let mut task_data_guard = task_data.lock().unwrap();
    let result = task_data_guard.take();
    result
}

#[tauri::command(async)]
pub async fn start_monitoring(
    app: tauri::AppHandle,
    event_sender: State<'_, EventSender>,
    modifier_state: State<'_, ModifierState>,
    task_data: State<'_, TaskDataState>,
    task_name: String,
) -> Result<String, String> {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }

    let mut task_data_guard = task_data.lock().unwrap();
    *task_data_guard = Some(TaskData {
        name: task_name,
        data: Vec::new(),
    });
    drop(task_data_guard);

    let (tx, rx): (Sender<InputEvent>, Receiver<InputEvent>) = mpsc::channel();

    *event_sender.lock().unwrap() = Some(tx);

    let sender_clone = event_sender.inner().clone();
    let modifier_state_clone = modifier_state.inner().clone();
    let task_data_clone = task_data.inner().clone();

    // Spawn the event printer thread first
    thread::spawn(move || {
        for event in rx {
            println!("Event: {:?}", event);
            if let Some(ref mut td) = &mut *task_data_clone.lock().unwrap() {
                td.data.push(event);
            }
        }
    });

    // Spawn the input listener thread
    thread::spawn(move || {
        let callback = move |event: Event| {
            let modifiers: Vec<String> = {
                let modifier_set = modifier_state_clone.lock().unwrap();
                modifier_set.iter().cloned().collect()
            };

            let input_event = match event.event_type {
                EventType::MouseMove { x, y } => Some(InputEvent {
                    event_type: "mouse_move".to_string(),
                    x: Some(x),
                    y: Some(y),
                    key: None,
                    button: None,
                    modifiers: modifiers.clone(),
                    active_app: None,
                    active_window_title: None,
                    timestamp: event
                        .time
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis(),
                }),
                EventType::ButtonPress(button) => Some(InputEvent {
                    event_type: "mouse_press".to_string(),
                    x: None,
                    y: None,
                    key: None,
                    button: Some(format!("{:?}", button)),
                    modifiers: modifiers.clone(),
                    active_app: None,
                    active_window_title: None,
                    timestamp: event
                        .time
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis(),
                }),
                EventType::ButtonRelease(button) => Some(InputEvent {
                    event_type: "mouse_release".to_string(),
                    x: None,
                    y: None,
                    key: None,
                    button: Some(format!("{:?}", button)),
                    modifiers: modifiers.clone(),
                    active_app: None,
                    active_window_title: None,
                    timestamp: event
                        .time
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis(),
                }),
                EventType::KeyPress(key) => {
                    let key_str = format!("{:?}", key);

                    {
                        let mut modifier_set = modifier_state_clone.lock().unwrap();
                        match key {
                            Key::ControlLeft | Key::ControlRight => {
                                modifier_set.insert("Ctrl".to_string());
                            }
                            Key::Alt | Key::AltGr => {
                                modifier_set.insert("Alt".to_string());
                            }
                            Key::ShiftLeft | Key::ShiftRight => {
                                modifier_set.insert("Shift".to_string());
                            }
                            Key::MetaLeft | Key::MetaRight => {
                                modifier_set.insert("Meta".to_string());
                            }
                            Key::Function => {
                                modifier_set.insert("Fn".to_string());
                            }
                            _ => {}
                        }
                    }

                    Some(InputEvent {
                        event_type: "key_press".to_string(),
                        x: None,
                        y: None,
                        key: Some(key_str),
                        button: None,
                        modifiers: modifiers.clone(),
                        active_app: None,
                        active_window_title: None,
                        timestamp: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_millis(),
                    })
                }
                EventType::KeyRelease(key) => {
                    let key_str = format!("{:?}", key);

                    {
                        let mut modifier_set = modifier_state_clone.lock().unwrap();
                        match key {
                            Key::ControlLeft | Key::ControlRight => {
                                modifier_set.remove("Ctrl");
                            }
                            Key::Alt | Key::AltGr => {
                                modifier_set.remove("Alt");
                            }
                            Key::ShiftLeft | Key::ShiftRight => {
                                modifier_set.remove("Shift");
                            }
                            Key::MetaLeft | Key::MetaRight => {
                                modifier_set.remove("Meta");
                            }
                            Key::Function => {
                                modifier_set.remove("Fn");
                            }
                            _ => {}
                        }
                    }

                    Some(InputEvent {
                        event_type: "key_release".to_string(),
                        x: None,
                        y: None,
                        key: Some(key_str),
                        button: None,
                        modifiers: modifiers.clone(),
                        active_app: None,
                        active_window_title: None,
                        timestamp: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_millis(),
                    })
                }
                EventType::Wheel { delta_x, delta_y } => Some(InputEvent {
                    event_type: "wheel".to_string(),
                    x: Some(delta_x as f64),
                    y: Some(delta_y as f64),
                    key: None,
                    button: None,
                    modifiers: modifiers.clone(),
                    active_app: None,
                    active_window_title: None,
                    timestamp: event
                        .time
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis(),
                }),
            };

            if let Some(input_event) = input_event {
                if let Some(sender) = sender_clone.lock().unwrap().as_ref() {
                    let _ = sender.send(input_event);
                }
            }
        };

        if let Err(err) = listen(callback) {
            println!("Error: {:?}", err);
        }
    });

    Ok("Monitoring Started".to_string())
}
