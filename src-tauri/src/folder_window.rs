use tauri::{utils::config::WindowConfig, AppHandle, WebviewWindowBuilder};

use crate::ff::FolderSettings;
use anyhow::Result;

pub fn new_folder_window(app: &AppHandle, folder_setting: &FolderSettings) -> Result<()> {
    let window = WebviewWindowBuilder::from_config(
        app,
        &WindowConfig {
            width: 192.0,
            height: 192.0,
            x: Some(folder_setting.window_pos.0 as f64),
            y: Some(folder_setting.window_pos.1 as f64),
            fullscreen: false,
            decorations: false,
            always_on_bottom: true,
            resizable: false,
            skip_taskbar: true,
            minimizable: false,
            maximizable: false,
            transparent: true,
            shadow: false,
            label: folder_setting.label.to_owned(),
            url: tauri::WebviewUrl::App("/".into()),
            ..Default::default()
        },
    )?
    .build()?;
    // 发送标识符到该窗口，让窗口知道自己是谁
    // app.emit_to(&folder_setting.label, "set-label", &folder_setting.label)?;
    window.eval(&format!("window.label = '{}';", &folder_setting.label))?;
    Ok(())
}
