use commands::new_folder_window;
use state::AppState;
use tauri::{menu::{Menu, MenuEvent, MenuItem}, tray::TrayIconBuilder, App, AppHandle, Manager, WebviewWindowBuilder, Wry};
use crate::ff::FloatingFolder;

mod commands;
mod ff;
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
            // ..

            let app_state = app.state::<AppState>();

            let ffs = FloatingFolder::get_folders(app_state.settings.read().unwrap().get_ffs_dir(app.handle())?)?;
            for x in ffs {
                // todo(CakeAL): 是这样创么，我觉得要额外做点初始化？
                let _window = WebviewWindowBuilder::from_config(
                    app,
                    &tauri::utils::config::WindowConfig {
                        width: 192.0,
                        height: 192.0,
                        x: Some(x.settings.window_pos.0 as f64),
                        y: Some(x.settings.window_pos.1 as f64),
                        fullscreen: false,
                        decorations: false,
                        always_on_bottom: true,
                        resizable: false,
                        skip_taskbar: true,
                        minimizable: false,
                        maximizable: false,
                        transparent: true,
                        label: x.settings.label,
                        url: tauri::WebviewUrl::App("/".into()),
                        ..Default::default()
                    },
                )?.build()?;
            }


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
            let _ = new_folder_window(app, app.state());
        }
        _ => {}
    }
}
