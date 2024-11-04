use tauri::{utils::config::WindowConfig, AppHandle, WebviewWindowBuilder};

#[tauri::command]
pub fn new_folder_window(app: &AppHandle) -> Result<(), String> {
    WebviewWindowBuilder::from_config(
        app,
        &WindowConfig {
            width: 64.0,
            height: 64.0,
            fullscreen: false,
            decorations: false,
            always_on_bottom: true,
            resizable: false,
            skip_taskbar: true,
            minimizable: false,
            maximizable: false,
            label: "folder-0".into(),
            url: tauri::WebviewUrl::App("/".into()),
            ..Default::default()
        },
    )
    .map_err(|e| e.to_string())?
    .build()
    .map_err(|e| e.to_string())
    .map(|_| ())
}
