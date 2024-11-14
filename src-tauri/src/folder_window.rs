use tauri::{utils::config::WindowConfig, AppHandle, WebviewWindowBuilder};
// use winapi::{
//     shared::windef::HWND__,
//     um::winuser::{SetWindowPos, HWND_BOTTOM, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE},
// };

use crate::ff::FolderSettings;
use anyhow::Result;

pub fn new_folder_window(app: &AppHandle, folder_setting: &FolderSettings) -> Result<()> {
    let window = WebviewWindowBuilder::from_config(
        app,
        &WindowConfig {
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
    // 强制设置窗口大小为 64.0
    use crate::command::scale_folder;
    scale_folder(app.to_owned(), window.label(), 64.0);
    // 设置到最下面
    // set_window_below_desktop_icons(&_window)?;
    Ok(())
}

// fn set_window_below_desktop_icons(window: &tauri::WebviewWindow) -> Result<()> {
//     let hwnd = window.hwnd()?.0 as *mut HWND__;
//     unsafe {
//         SetWindowPos(
//             hwnd,
//             HWND_BOTTOM,
//             0,
//             0,
//             0,
//             0,
//             SWP_NOACTIVATE | SWP_NOMOVE | SWP_NOSIZE,
//         );
//     }
//     Ok(())
// }
