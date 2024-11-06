//! |--ff.exe                            The app file.
//! |--ffs                               The float folders.
//! |  |--folder1                        Example float folder instance.
//! |  |  |--content                     The content folder to store the file in this folder.
//! |  |  |  |-- folder_contents...
//! |  |  |--settings.toml               The settings file.

use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::ffi::{OsStr, OsString};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};


#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum FolderIcon {
    #[default]
    System,
    Image,
    /// 展开之后的悬浮窗渲染的缩略图
    MiniFolder,
}

/// 单个窗口的设置
#[derive(Debug, Serialize, Deserialize)]
pub struct FolderSettings {
    pub label: String,
    /// The order of contents.
    /// store filename.
    pub contents: Vec<OsString>,
    pub icon_scale: f32,
    pub content_scale: f32,
    pub icon: FolderIcon,
    pub open_by_click: bool,
    pub window_pos: (i32, i32),
}

impl Default for FolderSettings {
    fn default() -> Self {
        Self {
            label: String::new(),
            contents: vec![],
            icon_scale: 1.0,
            content_scale: 1.0,
            icon: Default::default(),
            open_by_click: false,
            window_pos: (200, 200),
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct FloatingFolder {
    pub content_path: PathBuf,
    pub settings: FolderSettings,
}

impl FloatingFolder {
    pub fn parse(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let content_path = path.as_ref().join("content");
        if !content_path.exists() {
            std::fs::create_dir(&content_path)?;
        }
        let settings_path = content_path.join("settings.json");

        let settings = if settings_path.exists() {
            serde_json::from_reader(std::fs::File::open(settings_path)?)?
        } else {
            Default::default()
        };

        Ok(FloatingFolder {
            content_path,
            settings,
        })
    }

    pub fn copy_in(&mut self, src: impl AsRef<Path>) -> std::io::Result<()> {
        let to: PathBuf = self.content_path.join(src.as_ref().file_name()
            .ok_or(std::io::Error::new(ErrorKind::InvalidInput, "Invalid src path"))?);
        std::fs::copy(src, to)
            .map(|_| ())
    }

    pub fn get_dir_name(&self) -> &OsStr {
        self.content_path.file_name().unwrap()
    }

    pub fn check_contents(&mut self) -> std::io::Result<()> {
        let mut valid_name = HashSet::new();
        for x in self.content_path.read_dir()? {
            let name = x?.file_name();
            valid_name.insert(name.clone());
            if !self.settings.contents.contains(&name) {
                self.settings.contents.push(name);
            }
        }
        self.settings.contents.retain(|x| valid_name.contains(x));
        Ok(())
    }

    pub fn get_contents(&self) -> Vec<PathBuf> {
        self.settings.contents.iter()
            .map(|x| self.content_path.join(x))
            .collect()
    }

    pub fn get_content(&self, idx: usize) -> std::io::Result<PathBuf> {
        if let Some(file) = self.settings.contents.get(idx) {
            let file_path = self.content_path.join(file);
            Ok(file_path)
        } else {
            Err(std::io::Error::new(ErrorKind::InvalidInput, "Invalid Index"))
        }
    }

    pub fn get_ffs_path() -> std::io::Result<PathBuf> {
        Ok(std::env::current_dir()?.join("ffs"))
    }

    pub fn get_folders(ffs: impl AsRef<Path>) -> std::io::Result<Vec<FloatingFolder>> {

        //|--ffs                               The float folders.
        //|  |--folder1                        Example float folder instance.
        //|  |  |--content                     The content folder to store the file in this folder.
        //|  |  |  |-- folder_contents...
        //|  |  |--settings.toml               The settings file.

        if !ffs.as_ref().exists() {
            std::fs::create_dir(ffs.as_ref())?;
        }

        let result = ffs.as_ref().read_dir()?
            .par_bridge()
            .map(|x| {
                if let Ok(entry) = x {
                    let entry_path = entry.path();
                    if !entry_path.join("content").exists() {
                        return None;
                    }
                    let settings_path = entry_path.join("settings.json");
                    let settings = match std::fs::File::open(settings_path)
                        .map(|x| serde_json::from_reader::<_, FolderSettings>(x)) {
                        Ok(o) => {
                            match o {
                                Ok(s) => s,
                                Err(e) => {
                                    log::warn!("Dir {:?} contains invalid settings file. {e:?}", &entry_path);
                                    return None;
                                }
                            }
                        }
                        Err(e) if e.kind() == ErrorKind::NotFound => Default::default(),
                        Err(e) => {
                            log::warn!("Dir {:?} contains invalid settings file. {e:?}", &entry_path);
                            return None;
                        }
                    };


                    let mut ff = FloatingFolder {
                        content_path: entry.path().join("content"),
                        settings,
                    };
                    if let Err(e) = ff.check_contents() {
                        log::warn!("Check {:?} content with error. {e:?}", &entry_path);
                    }
                    Some(ff)
                } else {
                    log::warn!("Cannot read dir! {x:?}");
                    None
                }
            })
            .filter(|x| x.is_some())
            .map(Option::unwrap)
            .collect();
        Ok(result)
    }

    pub fn create_folder(ffs: impl AsRef<Path>, name: &str) -> std::io::Result<FloatingFolder> {

        //|--ffs                               The float folders.
        //|  |--folder1                        Example float folder instance.
        //|  |  |--content                     The content folder to store the file in this folder.
        //|  |  |  |-- folder_contents...
        //|  |  |--settings.toml               The settings file.

        if !ffs.as_ref().exists() {
            std::fs::create_dir(ffs.as_ref())?;
        }

        let mut idx = None;
        loop {
            let dir_path = idx.map(|x| ffs.as_ref().join(format!("{name} ({x})")))
                .unwrap_or(ffs.as_ref().join(name));
            if dir_path.exists() {
                idx = Some(idx.unwrap_or(0) + 1);
                continue;
            }
            std::fs::create_dir_all(dir_path.join("content"))?;
            let settings = FolderSettings {
                label: name.to_string(),
                ..Default::default()
            };
            serde_json::to_writer(std::fs::File::create_new(dir_path.join("settings.json"))?, &settings)?;
            let ff = FloatingFolder {
                content_path: dir_path.join("content"),
                settings,
            };
            break Ok(ff);
        }
    }
}

