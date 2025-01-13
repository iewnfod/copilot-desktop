use preference::Preference;
use tauri::{image::Image, menu::{Menu, MenuItem}, tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent}, App, AppHandle, Manager, WebviewWindow, WebviewWindowBuilder, WindowEvent};

mod commands;
mod preference;
use commands::*;

pub const MAIN_WINDOW_LABEL: &str = "main";
const SETTINGS_WINDOW_LABEL: &str = "settings";
const ICON_DATA: &[u8] = include_bytes!("../icons/tray.png");

fn create_settings_window(app: &AppHandle) -> WebviewWindow {
    if let Some(settings_window) = app.get_webview_window(SETTINGS_WINDOW_LABEL) {
        settings_window
    } else {
        WebviewWindowBuilder::new(
            app,
            SETTINGS_WINDOW_LABEL,
            tauri::WebviewUrl::App("/settings".into())
        )
            .title("Settings - Copilot Desktop")
            .inner_size(800.0, 550.0)
            .build().unwrap()
    }
}

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
                    inner_show_main_window(tray.app_handle());
                },
                _ => {}
            }
        })
        .menu(&menu)
        .on_menu_event(|app_handle, event| {
            match event.id.as_ref() {
                "quit" => {
                    quit(app_handle);
                },
                "settings" => {
                    let window = create_settings_window(app_handle);
                    window.show().unwrap();
                    window.set_focus().unwrap();
                },
                _ => {}
            }
        });

    tray_builder.build(app).unwrap();
}

fn is_settings_window_exist(app_handle: &AppHandle) -> bool {
    if let Some(_win) = app_handle.get_webview_window(SETTINGS_WINDOW_LABEL) {
        true
    } else {
        false
    }
}

fn main_loop(app_handle: &AppHandle, event: tauri::RunEvent, pre: &Preference) {
    match event {
        tauri::RunEvent::WindowEvent { label, event, .. } => {
            if label == MAIN_WINDOW_LABEL {
                match event {
                    WindowEvent::Focused(false) => {
                        if pre.get_hide_window_when_not_focus() {
                            if !is_settings_window_exist(app_handle) {
                                app_handle.get_webview_window(&label).unwrap().hide().unwrap();
                            }
                        }
                    },
                    _ => {}
                }
            }
        },
        _ => {}
    }
}

fn quit(app_handle: &AppHandle) {
    let mut pre = Preference::default();
    if let Some(main_window) = app_handle.get_webview_window(MAIN_WINDOW_LABEL) {
        let size = main_window.inner_size().unwrap();
        let scale = main_window.scale_factor().unwrap();
        pre.set_width((size.width as f64 / scale) as u32);
    }
    pre.save();
    check_pre(app_handle, Some(pre.clone()));
    app_handle.exit(0);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            init_tray(app);
            app.handle().plugin(
                tauri_plugin_autostart::init(tauri_plugin_autostart::MacosLauncher::LaunchAgent, None)
            ).unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            set_window,
            get_ai_url,
            get_preference,
            set_preference,
            show_main_window
        ]);

    if let Ok(app) = builder.build(tauri::generate_context!()) {
        let pre = Preference::default();
        app.run(move |app_handle, event| {
            main_loop(app_handle, event, &pre);
        });
    } else {
        println!("Failed to build tauri app!");
    }
}
