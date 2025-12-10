use std::{
    path::{Path, PathBuf},
    sync::{Arc, LazyLock},
};

use directories::ProjectDirs;

use crate::{cache::MeshCache, config::MeshConfig, database::MeshDatabase};

static MESH_DIR: LazyLock<ProjectDirs> =
    LazyLock::new(|| ProjectDirs::from_path(PathBuf::from("mesh")).unwrap());

pub struct MeshCore {
    config: MeshConfig,
    databese: Arc<MeshDatabase>,
    cache: Arc<MeshCache>,
}

impl MeshCore {
    pub fn init() -> Result<Self, anyhow::Error> {
        let config = MeshConfig::load()?;
        let databese = Arc::new(crate::database::init()?);
        let cache = Arc::new(crate::cache::init()?);

        Ok(Self {
            config,
            databese,
            cache,
        })
    }

    pub(crate) fn config_path<P: AsRef<Path>>(file_path: P) -> anyhow::Result<PathBuf> {
        let path = MESH_DIR.config_dir();
        std::fs::create_dir_all(&path)?;
        Ok(path.join(file_path))
    }

    pub(crate) fn cache_path<P: AsRef<Path>>(sub_dir_path: P) -> anyhow::Result<PathBuf> {
        let path = MESH_DIR.cache_dir().join(sub_dir_path);
        std::fs::create_dir_all(&path)?;
        Ok(path)
    }

    pub(crate) fn data_path<P: AsRef<Path>>(file_path: P) -> anyhow::Result<PathBuf> {
        let path = MESH_DIR.data_dir();
        std::fs::create_dir_all(&path)?;
        Ok(path.join(file_path))
    }
}
