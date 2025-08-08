pub mod tray;
use rdev::{listen as rdev_listen, Event, EventType, Key};
use std::{sync::mpsc, thread};
use tauri::{Emitter, Manager};

use crate::{setup::tray::setup_tray, InputEvent, IsListening, ModifierState};

pub fn get_setup_handler() -> impl Fn(&mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    |app| {
        let window = app.get_webview_window("main").unwrap();
        window.set_ignore_cursor_events(true)?;

        setup_tray(app)?;

        let is_listening = app.state::<IsListening>().inner().clone();
        let modifier_state = app.state::<ModifierState>().inner().clone();

        let (tx, rx) = mpsc::channel::<InputEvent>();

        {
            let window = window.clone();
            thread::spawn(move || {
                for event in rx {
                    let _ = window.emit("key-logger", event);
                }
            });
        }

        {
            let is_listening = is_listening.clone();
            let modifier_state = modifier_state.clone();
            let tx = tx.clone();

            thread::spawn(move || {
                let callback = move |event: Event| {
                    let modifiers: Vec<String> = {
                        let modifier_set = modifier_state.lock().unwrap();
                        modifier_set.iter().cloned().collect()
                    };

                    let ts_ms = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis();

                    let maybe_event = match event.event_type {
                        EventType::KeyPress(key) => {
                            {
                                let mut modifier_set = modifier_state.lock().unwrap();
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
                                key: Some(format!("{:?}", key)),
                                modifiers: modifiers.clone(),
                                timestamp: ts_ms,
                            })
                        }
                        EventType::KeyRelease(key) => {
                            {
                                let mut modifier_set = modifier_state.lock().unwrap();
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
                                key: Some(format!("{:?}", key)),
                                modifiers: modifiers.clone(),
                                timestamp: ts_ms,
                            })
                        }
                        _ => None,
                    };

                    if is_listening.load(std::sync::atomic::Ordering::SeqCst) {
                        if let Some(ev) = maybe_event {
                            let _ = tx.send(ev);
                        }
                    }
                };

                if let Err(err) = rdev_listen(callback) {
                    println!("Global input listener error: {:?}", err);
                }
            });
        }

        Ok(())
    }
}
