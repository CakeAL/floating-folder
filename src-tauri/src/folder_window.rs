use tauri::{utils::config::WindowConfig, AppHandle, LogicalSize, WebviewWindowBuilder};
// use winapi::{
//     shared::{ntdef::NULL, windef::HWND__},
//     um::winuser::{
//         FindWindowExA, FindWindowW, GetDesktopWindow, SetParent, SetWindowPos, HWND_BOTTOM, HWND_TOP, HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW
//     },
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
    let _ = window.set_size(LogicalSize {
        width: 64.0,
        height: 64.0,
    });
    // // 设置到最下面
    // set_window_below_desktop_icons(&window)?;
    Ok(())
}

// fn set_window_below_desktop_icons(window: &tauri::WebviewWindow) -> Result<()> {
//     let hwnd = window.hwnd()?.0 as *mut HWND__;
//     unsafe {
//         let workerw = FindWindowW("WorkerW\0".encode_utf16().collect::<Vec<u16>>().as_ptr(), null_mut());
//         if workerw.is_null() {
//             dbg!("Could not find workerw Window");
//             return Err(anyhow::anyhow!("Could not find workerw Window"));
//         }
//         SetParent(hwnd, workerw);
//         SetWindowPos(
//             hwnd,
//             HWND_BOTTOM,
//             0,
//             0,
//             0,
//             0,
//             SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
//         );
//     }
//     Ok(())
// }
