use tauri::{AppHandle, Manager, PhysicalPosition, PhysicalSize};

use crate::MAIN_WINDOW_LABEL;

#[tauri::command]
pub fn set_window(app_handle: AppHandle) {
    inner_set_window(&app_handle);
}

pub fn inner_set_window(app_handle: &AppHandle) {
    if let Some(main_window) = app_handle.get_webview_window(MAIN_WINDOW_LABEL) {
        main_window.maximize().unwrap();
        let max_size = main_window.inner_size().unwrap();
        main_window.unmaximize().unwrap();

        let current_window_size = main_window.inner_size().unwrap();

        let new_size = PhysicalSize::new(current_window_size.width, max_size.height);
        let new_position = PhysicalPosition::new(max_size.width - current_window_size.width, 0);
        let min_size = PhysicalSize::new(0, max_size.height);

        main_window.set_min_size(Some(min_size)).unwrap();

        main_window.set_size(new_size).unwrap();
        main_window.set_position(new_position).unwrap();
        main_window.set_decorations(false).unwrap();
        main_window.set_always_on_top(true).unwrap();
    }
}

#[tauri::command]
pub fn get_ai_url() -> String {
	"https://copilot.microsoft.com/".to_string()
}
