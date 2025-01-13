use tauri::{AppHandle, Manager, PhysicalPosition, PhysicalSize};
use tauri_plugin_autostart::ManagerExt;

use crate::{preference::Preference, MAIN_WINDOW_LABEL};

#[tauri::command]
pub fn set_window(app_handle: AppHandle) {
    let pre = Preference::default();
    inner_set_window(&app_handle, &pre, true);
}

pub fn inner_set_window(app_handle: &AppHandle, pre: &Preference, check_max_height: bool) {
    if let Some(main_window) = app_handle.get_webview_window(MAIN_WINDOW_LABEL) {
        let mut max_size = main_window.inner_size().unwrap();

        if check_max_height {
            main_window.maximize().unwrap();
            max_size = main_window.inner_size().unwrap();
            main_window.unmaximize().unwrap();
        }

        let expect_width = (pre.get_width() as f64 * main_window.scale_factor().unwrap()) as u32;

        let new_size = PhysicalSize::new(expect_width, max_size.height);
        let new_position = PhysicalPosition::new(max_size.width - expect_width, 0);
        let min_size = PhysicalSize::new(0, max_size.height);

        main_window.set_min_size(Some(min_size)).unwrap();

        main_window.set_size(new_size).unwrap();
        main_window.set_position(new_position).unwrap();

        main_window.set_always_on_top(pre.get_always_on_top()).unwrap();
    }
}

#[tauri::command]
pub fn get_ai_url() -> String {
    Preference::default().get_ai_url()
}

#[tauri::command]
pub fn get_preference() -> Preference {
    Preference::default()
}

#[tauri::command]
pub fn set_preference(app_handle: AppHandle, pre: Preference) {
    check_pre(&app_handle, Some(pre.clone()));
    pre.save();
}

pub fn check_pre(app_handle: &AppHandle, preference: Option<Preference>) {
    let pre: Preference;

    if let Some(stored_pre) = preference {
        pre = stored_pre;
    } else {
        pre = Preference::default();
    }

    // auto launch
    let autostart_manager = app_handle.autolaunch();
    if pre.get_start_at_login() {
        if !autostart_manager.is_enabled().unwrap() {
            autostart_manager.enable().unwrap();
        }
    } else {
        if autostart_manager.is_enabled().unwrap() {
            autostart_manager.disable().unwrap();
        }
    }
}

#[tauri::command]
pub fn show_main_window(app_handle: AppHandle) {
    inner_show_main_window(&app_handle);
}

pub fn inner_show_main_window(app_handle: &AppHandle) {
    if let Some(window) = app_handle.get_webview_window(MAIN_WINDOW_LABEL) {
        window.hide_menu().unwrap();
        window.show().unwrap();
        window.set_focus().unwrap();
    }
}
