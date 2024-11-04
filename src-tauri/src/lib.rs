use commands::new_folder_window;
use tauri::{
    menu::{Menu, MenuEvent, MenuItem},
    tray::TrayIconBuilder,
    App, AppHandle, Wry,
};

mod commands;
mod ff;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            // 系统托盘
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu(app)?)
                .on_menu_event(handle_menu_event)
                .build(app)?;
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
            let _ = new_folder_window(app);
        }
        _ => {}
    }
}
