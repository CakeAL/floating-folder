use crate::ff::FloatingFolder;
use folder_window::new_folder_window;
use state::AppState;
use tauri::{
    menu::{Menu, MenuEvent, MenuItem},
    tray::TrayIconBuilder,
    App, AppHandle, Manager, Wry,
};

mod ff;
mod folder_window;
mod state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            app.manage(AppState::init(app.handle())?);
            // 系统托盘
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu(app)?)
                .on_menu_event(handle_menu_event)
                .build(app)?;

            // 初始化已存储的文件夹
            app.state::<AppState>()
                .create_saved_floating_folders(app.handle());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn menu(app: &mut App) -> tauri::Result<Menu<Wry>> {
    let new_folder = MenuItem::with_id(app, "new_folder", "新建文件夹", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    Menu::with_items(app, &[&new_folder, &quit_i])
}

fn handle_menu_event(app: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
        "quit" => {
            app.exit(0);
        }
        "new_folder" => {
            let app_state = app.state::<AppState>();
            let settings = app_state.settings.read().unwrap();
            let ffs_dir = settings.get_ffs_dir(app).unwrap_or_else(|e| {
                log::error!("Cannot get ffs dir: {e:?}");
                panic!("😅😅");
            });
            let label = format!("folder-{}", app_state.folders.read().unwrap().len());
            let ff = FloatingFolder::create_folder(ffs_dir, &label).unwrap_or_else(|e| {
                log::error!("Cannot create folder: {e:?}");
                panic!("😅😅");
            });
            new_folder_window(app, &ff.settings).unwrap_or_else(|e| {
                log::error!("Cannot create floating folder window: {e:?}");
            });
            // push 一下更新 len，以便于生成下一个 label
            app_state.folders.write().unwrap().push(ff);
        }
        _ => {}
    }
}
