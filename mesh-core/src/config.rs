use std::path::PathBuf;

use directories::UserDirs;
use serde::{Deserialize, Serialize};

use crate::core::MeshCore;

const CONFIG_FILE_NAME: &str = "config.toml";

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct MeshConfig {
    album_paths: Vec<PathBuf>,
    excluded_dirs: Vec<PathBuf>,
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
        }
    }
}

impl MeshConfig {
    pub(crate) fn load() -> anyhow::Result<Self> {
        let config_path = MeshCore::config_path(CONFIG_FILE_NAME)?;
        if !config_path.exists() {
            let default_config = Self::default();
            default_config.save()?;
            return Ok(default_config);
        }

        let config_content = std::fs::read_to_string(&config_path)?;
        let config: Self = toml::from_str(&config_content)?;

        Ok(config)
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let config_path = MeshCore::config_path(CONFIG_FILE_NAME)?;

        let toml_string = toml::to_string_pretty(self)?;
        std::fs::write(&config_path, toml_string)?;

        Ok(())
    }
}
