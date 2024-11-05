use std::{fs::OpenOptions, path::PathBuf, sync::RwLock};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

use crate::ff::FloatingFolder;
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AppSettings {
    pub folder_root: Option<PathBuf>
}

impl AppSettings {
    pub fn get_ffs_dir(&self, app: &AppHandle) -> Result<PathBuf> {
        Ok(self.folder_root.clone().unwrap_or(app.path().app_data_dir()?.join("ffs")))
    }
}
#[derive(Debug)]
pub struct AppState {
    pub settings: RwLock<AppSettings>,
    pub folders: RwLock<Vec<FloatingFolder>>,
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
        Ok(Self {
            settings,
            folders
        })
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
