use std::path::PathBuf;

use tauri::State;

use crate::state::AppState;

#[tauri::command(async)]
pub fn moved_folder(
    app_state: State<'_, AppState>,
    label: &str,
    x: i32,
    y: i32,
) -> Result<(), String> {
    if let Some(folder) = app_state.folders.write().unwrap().get_mut(label) {
        folder.settings.window_pos = (x, y);
        // TODO: 保存坐标到对应 label 的文件夹的 setting.json
        dbg!(label, x, y);
    }
    Ok(())
}

#[tauri::command(async)]
pub fn send_path_to_folder(
    app_state: State<'_, AppState>,
    label: &str,
    path: &str,
) -> Result<(), String> {
    if let Some(folder) = app_state.folders.write().unwrap().get_mut(label) {
        // 判断文件 or 文件夹，后缀名 .lnk or others
        // 如果是文件夹，创建一个快捷方式
        // 如果是 .lnk 移动该快捷方式到 data
        let file_path = PathBuf::from(path);
        if file_path.ends_with(".lnk") {
            if let Err(e) = folder.copy_in(file_path) {
                log::error!("Folder cannot copy in the file, with path: {path}, err: {e:?}");
            };
        } 
        // 其他情况不做任何事
    }
    Ok(())
}