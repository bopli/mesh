use std::{
    cell::RefCell,
    path::{Path, PathBuf},
};

use directories::UserDirs;
use serde::{Deserialize, Serialize};

use crate::MESH_DIR;

const CONFIG_FILE_NAME: &str = "config.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct MeshConfig {
    album_paths: Vec<PathBuf>,
    excluded_dirs: Vec<PathBuf>,
    theme: RefCell<String>,
}

impl Default for MeshConfig {
    fn default() -> Self {
        let mut album_paths = Vec::new();

        if let Some(user_dirs) = UserDirs::new() {
            if let Some(picture_dir) = user_dirs.picture_dir() {
                album_paths.push(picture_dir.to_path_buf());
            }
        }

        Self {
            album_paths,
            excluded_dirs: Vec::new(),
            theme: RefCell::new("Default Light".to_owned()),
        }
    }
}

impl MeshConfig {
    pub fn dir_path() -> &'static Path {
        let path = MESH_DIR.config_dir();
        if let Err(e) = std::fs::create_dir_all(&path) {
            log::error!("{:?}", e);
        }
        path
    }

    pub fn init() -> Self {
        let config_path = Self::dir_path().join(CONFIG_FILE_NAME);
        if !config_path.exists() {
            let default_config = Self::default();
            default_config.save();
            return default_config;
        }

        let config_content = std::fs::read_to_string(&config_path)
            .map_err(|e| log::warn!("Failed to read config: {}", e))
            .unwrap_or_default();

        toml::from_str::<Self>(&config_content)
            .map_err(|e| log::warn!("Failed to parse config: {}", e))
            .unwrap_or_default()
    }

    pub fn save(&self) {
        let config_path = Self::dir_path().join(CONFIG_FILE_NAME);
        if let Ok(config_content) = toml::to_string_pretty(self) {
            if let Err(e) = std::fs::write(&config_path, config_content) {
                log::warn!("{:?}", e);
            }
        };
    }

    pub fn change_theme(&self, theme: String) {
        *self.theme.borrow_mut() = theme;
    }

    pub fn current_theme(&self) -> String {
        self.theme.borrow().clone()
    }

    pub fn themes_dir_path() -> PathBuf {
        Self::dir_path().join("themes")
    }
}
