use std::{fs::OpenOptions, path::PathBuf, sync::RwLock};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

use crate::ff::FloatingFolder;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppState {
    pub folders: RwLock<Vec<FloatingFolder>>,
}

impl AppState {
    pub fn init(app: &AppHandle) -> Result<Self> {
        let path = get_data_path(app)?;
        if path.exists() {
            let file = OpenOptions::new().read(true).open(path)?;
            let appstate: Self = serde_json::from_reader(file)?;
            Ok(appstate)
        } else {
            let file = OpenOptions::new().write(true).create(true).open(path)?;
            let new_appstate = Self {
                folders: RwLock::new(Vec::new()),
            };
            serde_json::to_writer(&file, &new_appstate)?;
            Ok(new_appstate)
        }
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
