use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use rdev::{listen, Event, EventType, Key};
use tauri::{Emitter, State};

use crate::{EventSender, InputEvent, ModifierState};

#[tauri::command]
pub fn stop_monitoring(
    event_sender: State<'_, EventSender>,
    modifier_state: State<'_, ModifierState>,
) -> Result<String, String> {
    *event_sender.lock().unwrap() = None;
    modifier_state.lock().unwrap().clear();
    Ok(String::from("Stoped"))
}

#[tauri::command(async)]
pub async fn start_monitoring(
    window: tauri::Window,
    event_sender: State<'_, EventSender>,
    modifier_state: State<'_, ModifierState>,
) -> Result<String, String> {
    let (tx, rx): (Sender<InputEvent>, Receiver<InputEvent>) = mpsc::channel();

    *event_sender.lock().unwrap() = Some(tx);

    let sender_clone = event_sender.inner().clone();
    let modifier_state_clone = modifier_state.inner().clone();

    thread::spawn(move || {
        for event in rx {
            println!("Event: {:?}", event);
            let _ = window.emit("key-logger", event);
        }
    });

    thread::spawn(move || {
        let callback = move |event: Event| {
            let modifiers: Vec<String> = {
                let modifier_set = modifier_state_clone.lock().unwrap();
                modifier_set.iter().cloned().collect()
            };

            let input_event = match event.event_type {
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
                        key: Some(key_str),
                        modifiers: modifiers.clone(),
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
                        key: Some(key_str),
                        modifiers: modifiers.clone(),
                        timestamp: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_millis(),
                    })
                }
                _ => None,
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
