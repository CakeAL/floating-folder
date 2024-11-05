use tauri::{utils::config::WindowConfig, AppHandle, State, WebviewWindowBuilder};

use crate::{ff::FloatingFolder, state::AppState};

#[tauri::command]
pub fn new_folder_window(app: &AppHandle, app_state: State<AppState>) -> Result<(), String> {
    let label = format!("folder-{}", app_state.folders.read().unwrap().len());
    WebviewWindowBuilder::from_config(
        app,
        &WindowConfig {
            width: 192.0,
            height: 192.0,
            fullscreen: false,
            decorations: false,
            always_on_bottom: true,
            resizable: false,
            skip_taskbar: true,
            minimizable: false,
            maximizable: false,
            transparent: true,
            label: label.clone(),
            url: tauri::WebviewUrl::App("/".into()),
            ..Default::default()
        },
    )
    .map_err(|e| e.to_string())?
    .build()
    .map_err(|e| e.to_string())?;
    app_state.folders.write().unwrap().push(FloatingFolder {
        label,
        ..Default::default()
    });
    Ok(())
}
