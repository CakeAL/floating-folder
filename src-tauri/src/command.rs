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
