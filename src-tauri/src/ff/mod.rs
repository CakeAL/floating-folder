//! |--ff.exe                            The app file
//! |--ffs                               The float folders
//! |  |--folder1                        Example float folder instance
//! |  |  |--content                     The content folder to store the file in this folder
//! |  |  |  |-- folder_contents...
//! |  |  |--settings.toml               The settings file

use std::collections::HashSet;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};


#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone)]
pub enum FolderIcon {
    #[default]
    System,
    Image,
    /// 展开之后的悬浮窗渲染的缩略图
    MiniFolder,
}

/// 单个窗口的设置
#[derive(Default, Debug)]
pub struct FolderSettings {
    /// The order of contents.
    /// store filename.
    pub contents: Vec<OsString>,
    pub icon_scale: f32,
    pub content_scale: f32,
    pub icon: FolderIcon,
    pub open_by_click: bool,
    pub center_pos: (u32, u32),
}

pub struct FloatingFolder {
    pub content_path: PathBuf,
    pub settings: FolderSettings,
}

impl FloatingFolder {
    pub fn copy_in(&mut self, src: impl AsRef<Path>) -> std::io::Result<()> {
        let to = self.content_path.join(src.as_ref().file_name()
            .ok_or(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid src path"))?);
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
            Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid Index"))
        }
    }
}