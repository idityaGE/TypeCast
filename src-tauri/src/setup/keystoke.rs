use crate::{InputEvent, IsListening, ModifierState};
use rdev::{listen as rdev_listen, Event, EventType, Key};
use std::{sync::mpsc, thread};
use tauri::{Emitter, Manager};

pub fn start_keystoke_listener(app: &tauri::App) {
    let window = app.get_webview_window("main").unwrap();
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
                            key: get_key_string(key),
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
                            key: get_key_string(key),
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
}

fn get_key_string(key: Key) -> Option<String> {
    match key {
        Key::Alt => Some("Alt".to_string()),
        Key::AltGr => Some("AltGr".to_string()),
        Key::Backspace => Some("⌫".to_string()),
        Key::CapsLock => Some("⇬".to_string()),
        Key::ControlLeft => Some("Ctrl".to_string()),
        Key::ControlRight => Some("Ctrl".to_string()),
        Key::Delete => Some("⌦".to_string()),
        Key::DownArrow => Some("↓".to_string()),
        Key::End => Some("End".to_string()),
        Key::Escape => Some("esc".to_string()),
        Key::F1 => Some("F1".to_string()),
        Key::F2 => Some("F2".to_string()),
        Key::F3 => Some("F3".to_string()),
        Key::F4 => Some("F4".to_string()),
        Key::F5 => Some("F5".to_string()),
        Key::F6 => Some("F6".to_string()),
        Key::F7 => Some("F7".to_string()),
        Key::F8 => Some("F8".to_string()),
        Key::F9 => Some("F9".to_string()),
        Key::F10 => Some("F10".to_string()),
        Key::F11 => Some("F11".to_string()),
        Key::F12 => Some("F12".to_string()),
        Key::Home => Some("Home".to_string()),
        Key::LeftArrow => Some("←".to_string()),
        Key::MetaLeft => Some("⌘".to_string()),
        Key::MetaRight => Some("⌘".to_string()),
        Key::PageDown => Some("PgDn".to_string()),
        Key::PageUp => Some("PgUp".to_string()),
        Key::Return => Some("⏎".to_string()),
        Key::RightArrow => Some("→".to_string()),
        Key::ShiftLeft => Some("⇧".to_string()),
        Key::ShiftRight => Some("⇧".to_string()),
        Key::Space => Some("␣".to_string()),
        Key::Tab => Some("↹".to_string()),
        Key::UpArrow => Some("↑".to_string()),
        Key::PrintScreen => Some("PrtSc".to_string()),
        Key::ScrollLock => Some("ScrollLock".to_string()),
        Key::Pause => Some("⎉".to_string()),
        Key::NumLock => Some("NumLock".to_string()),
        Key::BackQuote => Some("`".to_string()),
        Key::Num1 => Some("1".to_string()),
        Key::Num2 => Some("2".to_string()),
        Key::Num3 => Some("3".to_string()),
        Key::Num4 => Some("4".to_string()),
        Key::Num5 => Some("5".to_string()),
        Key::Num6 => Some("6".to_string()),
        Key::Num7 => Some("7".to_string()),
        Key::Num8 => Some("8".to_string()),
        Key::Num9 => Some("9".to_string()),
        Key::Num0 => Some("0".to_string()),
        Key::Minus => Some("-".to_string()),
        Key::Equal => Some("=".to_string()),
        Key::KeyQ => Some("q".to_string()),
        Key::KeyW => Some("w".to_string()),
        Key::KeyE => Some("e".to_string()),
        Key::KeyR => Some("r".to_string()),
        Key::KeyT => Some("t".to_string()),
        Key::KeyY => Some("y".to_string()),
        Key::KeyU => Some("u".to_string()),
        Key::KeyI => Some("i".to_string()),
        Key::KeyO => Some("o".to_string()),
        Key::KeyP => Some("p".to_string()),
        Key::LeftBracket => Some("[".to_string()),
        Key::RightBracket => Some("]".to_string()),
        Key::KeyA => Some("a".to_string()),
        Key::KeyS => Some("s".to_string()),
        Key::KeyD => Some("d".to_string()),
        Key::KeyF => Some("f".to_string()),
        Key::KeyG => Some("g".to_string()),
        Key::KeyH => Some("h".to_string()),
        Key::KeyJ => Some("j".to_string()),
        Key::KeyK => Some("k".to_string()),
        Key::KeyL => Some("l".to_string()),
        Key::SemiColon => Some(";".to_string()),
        Key::Quote => Some("'".to_string()),
        Key::KeyZ => Some("z".to_string()),
        Key::KeyX => Some("x".to_string()),
        Key::KeyC => Some("c".to_string()),
        Key::KeyV => Some("v".to_string()),
        Key::KeyB => Some("b".to_string()),
        Key::KeyN => Some("n".to_string()),
        Key::KeyM => Some("m".to_string()),
        Key::Comma => Some(",".to_string()),
        Key::Dot => Some(".".to_string()),
        Key::Slash => Some("/".to_string()),
        Key::Insert => Some("ins".to_string()),
        Key::KpReturn => Some("⏎".to_string()),
        Key::KpMinus => Some("−".to_string()),
        Key::KpPlus => Some("+".to_string()),
        Key::KpMultiply => Some("×".to_string()),
        Key::KpDivide => Some("÷".to_string()),
        Key::Kp0 => Some("0".to_string()),
        Key::Kp1 => Some("1".to_string()),
        Key::Kp2 => Some("2".to_string()),
        Key::Kp3 => Some("3".to_string()),
        Key::Kp4 => Some("4".to_string()),
        Key::Kp5 => Some("5".to_string()),
        Key::Kp6 => Some("6".to_string()),
        Key::Kp7 => Some("7".to_string()),
        Key::Kp8 => Some("8".to_string()),
        Key::Kp9 => Some("9".to_string()),
        Key::Function => Some("Fn".to_string()),
        Key::BackSlash => Some("'\'".to_string()),
        Key::Unknown(_) => None,
        _ => None,
    }
}
