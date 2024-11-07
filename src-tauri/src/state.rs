use std::{collections::HashMap, fs::OpenOptions, path::PathBuf, sync::RwLock};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

use crate::{ff::FloatingFolder, folder_window::new_folder_window};
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AppSettings {
    pub folder_root: Option<PathBuf>,
}

impl AppSettings {
    pub fn get_ffs_dir(&self, app: &AppHandle) -> Result<PathBuf> {
        Ok(self
            .folder_root
            .clone()
            .unwrap_or(app.path().app_data_dir()?.join("ffs")))
    }
}

pub type FolderMap = HashMap<String, FloatingFolder>;
#[derive(Debug)]
pub struct AppState {
    pub settings: RwLock<AppSettings>,
    pub folders: RwLock<FolderMap>,
}

impl AppState {
    pub fn init(app: &AppHandle) -> Result<Self> {
        let path = get_data_path(app)?;
        let settings = if path.exists() {
            let file = OpenOptions::new().read(true).open(path)?;
            serde_json::from_reader(file)?
        } else {
            let file = OpenOptions::new().write(true).create(true).open(path)?;
            let settings = AppSettings::default();
            serde_json::to_writer(&file, &settings)?;
            settings
        };

        let folders = FloatingFolder::get_folders(settings.get_ffs_dir(app)?)?;

        let settings = RwLock::new(settings);
        let folders = RwLock::new(folders);
        Ok(Self { settings, folders })
    }

    pub fn create_saved_floating_folders(&self, app: &AppHandle) {
        let floating_folders = &*self.folders.read().unwrap();
        floating_folders.iter().for_each(|ff| {
            if let Err(e) = new_folder_window(app, &ff.1.settings) {
                log::error!("Cannot create new floating folders: {e:?}");
            };
        });
    }
}

fn get_data_path(app: &AppHandle) -> Result<PathBuf> {
    let data_dir = app.path().app_data_dir()?;
    dbg!(&data_dir);
    if !data_dir.exists() {
        std::fs::create_dir(&data_dir)?;
    }
    Ok(data_dir.join("data.json"))
}
