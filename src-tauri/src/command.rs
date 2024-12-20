use std::path::PathBuf;

use serde::Serialize;
use tauri::{LogicalPosition, LogicalSize, Manager, State};
use tauri_plugin_shell::ShellExt;

use crate::{state::AppState, util::get_icon_base64};

#[tauri::command(async)]
pub fn moved_folder(
    app: tauri::AppHandle,
    app_state: State<'_, AppState>,
    label: &str,
) -> Result<(), String> {
    if let Some(folder) = app_state.folders.write().unwrap().get_mut(label) {
        // 保存坐标到对应 label 的文件夹的 setting.json
        // dbg!(label, x, y);
        let window = app.get_webview_window(label).unwrap();
        let position = window
            .outer_position()
            .unwrap()
            .to_logical(window.scale_factor().unwrap());
        if let Err(e) = folder.save_position(position.x, position.y) {
            log::error!(
                "Folder {label} cannot save position ({},{}): {e:?}",
                position.x,
                position.y
            );
        }
    }
    Ok(())
}

#[tauri::command(async)]
pub fn send_path_to_folder(
    app_state: State<'_, AppState>,
    label: &str,
    path: Vec<&str>,
) -> Result<(), String> {
    if let Some(folder) = app_state.folders.write().unwrap().get_mut(label) {
        // 判断文件 or 文件夹，后缀名 .lnk or others
        // 如果是文件夹，创建一个快捷方式
        // 如果是 .lnk 移动该快捷方式到 data
        if folder.settings.contents.len() >= 9 {
            return Err("这个文件夹已经满了".into());
        }
        path.iter().for_each(|path| {
            let file_path = PathBuf::from(path);
            if file_path.extension().map_or(false, |ext| ext == "lnk") {
                dbg!(path);
                if let Err(e) = folder.copy_in(file_path) {
                    log::error!("Folder cannot copy in the file, with path: {path}, err: {e:?}");
                };
            }
            // 其他情况目前不做任何事
        });
    }
    Ok(())
}

#[derive(Debug, Serialize)]
struct Icon {
    base64: String,
    name: String,
    path: String,
}

#[tauri::command(async)]
pub fn get_icons(app_state: State<'_, AppState>, label: &str) -> Result<String, String> {
    if let Some(folder) = app_state.folders.read().unwrap().get(label) {
        let icons = folder
            .get_contents()
            .iter()
            .map(|path| Icon {
                base64: get_icon_base64(path).unwrap_or_default(),
                name: path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or_default()
                    .to_string(),
                path: path.to_str().unwrap_or_default().to_string(),
            })
            .collect::<Vec<Icon>>();
        Ok(serde_json::json!(icons).to_string())
    } else {
        log::error!("No Such Folder, label: {label}");
        Err("No Such Folder".into())
    }
}

#[tauri::command(async)]
pub fn del_folder(
    app_state: State<'_, AppState>,
    app: tauri::AppHandle,
    label: &str,
) -> Result<(), String> {
    dbg!(label);
    app_state
        .folders
        .write()
        .unwrap()
        .remove(label)
        .and_then(|folder| Some(folder.del_folder()));
    app.get_webview_window(label)
        .unwrap()
        .close()
        .map_err(|e| e.to_string())
}

#[tauri::command(async)]
pub fn open_folder(
    app_state: State<'_, AppState>,
    app: tauri::AppHandle,
    label: &str,
) -> Result<(), String> {
    let folder_path = app_state
        .folders
        .read()
        .unwrap()
        .get(label)
        .map(|ff| ff.path.clone())
        .unwrap_or_default();
    app.shell()
        .open(folder_path.to_str().unwrap_or_default(), None)
        .map_err(|e| e.to_string())
}

#[tauri::command(async)]
pub fn scale_folder(app: tauri::AppHandle, label: &str, len: f64) {
    // 获取窗口对象
    let window = app.get_webview_window(label).unwrap();

    // 获取当前窗口的位置和大小
    let scale_factor = window.scale_factor().unwrap();
    let current_position: LogicalPosition<f64> =
        window.outer_position().unwrap().to_logical(scale_factor);
    let current_size: LogicalSize<f64> = window.inner_size().unwrap().to_logical(scale_factor);

    // 计算新的左上角位置，确保窗口中心不变
    let new_x = current_position.x + (current_size.width / 2.0) - len / 2.0;
    let new_y = current_position.y + (current_size.height / 2.0) - len / 2.0;

    // 设置新的窗口大小
    let _ = window.set_size(LogicalSize {
        width: len,
        height: len,
    });

    // 设置新的窗口位置
    // let _ = window.set_position(LogicalPosition { x: new_x, y: new_y });
}
