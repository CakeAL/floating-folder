use tauri::{utils::config::WindowConfig, AppHandle, State, WebviewWindowBuilder};

use crate::{ff::FloatingFolder, state::AppState};

#[tauri::command]
pub fn new_folder_window(app: &AppHandle, app_state: State<AppState>) -> Result<(), String> {
    let label = format!("folder-{}", app_state.folders.read().unwrap().len());
    let _window = WebviewWindowBuilder::from_config(
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


    let settings = app_state.settings.read().unwrap();
    let ffs_dir = settings.get_ffs_dir(app).map_err(|e| e.to_string())?;
    let mut ff = FloatingFolder::create_folder(ffs_dir, &label).map_err(|e| e.to_string())?;
    // todo(CakeAL): 这个窗口和这个ff绑定？ 当然，这个 ff 对象好像用完就不需要存着了
    app_state.folders.write().unwrap().push(ff);
    Ok(())
}
