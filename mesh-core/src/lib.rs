mod cache;
mod config;

pub use cache::MeshCache;
pub use config::MeshConfig;

use directories::ProjectDirs;
use std::{path::PathBuf, sync::LazyLock};

static MESH_DIR: LazyLock<ProjectDirs> =
    LazyLock::new(|| ProjectDirs::from_path(PathBuf::from("mesh")).unwrap());
