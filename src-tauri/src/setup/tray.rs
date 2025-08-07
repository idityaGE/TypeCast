use tauri::{
    menu::{Menu, MenuBuilder, MenuEvent, MenuItem},
    tray::{TrayIcon, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager,
};

pub fn setup_tray(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let icon = app.default_window_icon().cloned().unwrap();
    let menu = get_menu(&app)?;
    let tray_icon_handler = get_tray_icon_handler();
    let menu_event_handler = get_menu_event_handler();

    let _tray = TrayIconBuilder::new()
        .icon(icon)
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(menu_event_handler)
        .on_tray_icon_event(tray_icon_handler)
        .build(app)?;
    Ok(())
}

fn get_menu(app: &tauri::App) -> Result<Menu<tauri::Wry>, Box<dyn std::error::Error>> {
    let start = MenuItem::with_id(
        app,
        "start_monitoring",
        "Start Monitoring",
        true,
        None::<&str>,
    )?;
    let stop = MenuItem::with_id(
        app,
        "stop_monitoring",
        "Stop Monitoring",
        true,
        None::<&str>,
    )?;
    let quit = MenuItem::with_id(app, "quit", "Quit App", true, None::<&str>)?;

    let menu = MenuBuilder::new(app)
        .item(&start)
        .item(&stop)
        .separator()
        .item(&quit)
        .build()?;

    Ok(menu)
}

fn get_menu_event_handler() -> impl Fn(&AppHandle, MenuEvent) {
    |app: &AppHandle, event: MenuEvent| match event.id.as_ref() {
        "start_monitoring" => {
            println!("Start is Clicked");
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.emit("start_monitoring", ());
            }
        }
        "stop_monitoring" => {
            println!("Stop is clicked");
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.emit("stop_monitoring", ());
            }
        }
        "quit" => {
            println!("quit menu item was clicked");
            app.exit(0);
        }
        _ => {
            println!("menu item {:?} not handled", event.id);
        }
    }
}

fn get_tray_icon_handler() -> impl Fn(&TrayIcon, TrayIconEvent) {
    |_tray: &TrayIcon, event: TrayIconEvent| match event {
        TrayIconEvent::Click {
            id,
            position,
            rect,
            button,
            button_state,
        } => {
            println!("Click event - id: {id:?}, position: {position:?}, rect: {rect:?}, button: {button:?}, button_state: {button_state:?}");
        }
        _ => {
            println!("unhandled event {event:?}");
        }
    }
}
