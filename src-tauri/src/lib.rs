use tauri::{image::Image, menu::{Menu, MenuItem}, tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent}, App, AppHandle, Manager, WindowEvent};

mod commands;
use commands::*;

pub const MAIN_WINDOW_LABEL: &str = "main";
const SETTINGS_WINDOW_LABEL: &str = "settings";
const ICON_DATA: &[u8] = include_bytes!("../icons/tray.png");

fn init_tray(app: &mut App) {
    let settings_item = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>).unwrap();
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>).unwrap();
    let menu = Menu::with_items(app, &[&settings_item, &quit_item]).unwrap();

    let tray_builder = TrayIconBuilder::new()
        .icon(Image::from_bytes(ICON_DATA).unwrap())
        .icon_as_template(true)
        .on_tray_icon_event(|tray, event| {
            match event {
                TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } => {
                    if let Some(main_window) = tray.app_handle().get_webview_window(MAIN_WINDOW_LABEL) {
                        main_window.show().unwrap();
                        main_window.set_focus().unwrap();
                    }
                },
                _ => {}
            }
        })
        .menu(&menu)
        .on_menu_event(|app_handle, event| {
            match event.id.as_ref() {
                "quit" => {
                    app_handle.exit(0);
                },
                "settings" => {
                    if let Some(settings_window) = app_handle.get_webview_window(SETTINGS_WINDOW_LABEL) {
                        settings_window.show().unwrap();
                        settings_window.set_focus().unwrap();
                    }
                },
                _ => {}
            }
        });

    tray_builder.build(app).unwrap();
}

fn main_loop(app_handle: &AppHandle, event: tauri::RunEvent) {
    match event {
        tauri::RunEvent::WindowEvent { label, event, .. } => {
            match event {
                WindowEvent::Focused(false) => {
                    if label == MAIN_WINDOW_LABEL {
                        app_handle.get_webview_window(&label).unwrap().hide().unwrap();
                    }
                },
                _ => {}
            }
        },
        _ => {}
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            init_tray(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            set_window,
            get_ai_url
        ]);

    if let Ok(app) = builder.build(tauri::generate_context!()) {
        app.run(main_loop);
    } else {
        println!("Failed to build tauri app!");
    }
}
