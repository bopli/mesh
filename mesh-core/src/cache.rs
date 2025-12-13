mod database;
mod thumbnail;

use std::path::Path;

use crate::MESH_DIR;
use crate::cache::database::MeshDatabase;
use crate::cache::thumbnail::MeshThumbnail;

pub struct MeshCache {
    database: MeshDatabase,
    thumbnail: MeshThumbnail,
}

impl MeshCache {
    pub(crate) fn dir_path() -> &'static Path {
        let path = MESH_DIR.cache_dir();
        if let Err(e) = std::fs::create_dir_all(&path) {
            log::error!("{:?}", e);
        }
        path
    }

    pub fn new() -> Self {
        let cache_dir_path = Self::dir_path();
        if let Err(e) = std::fs::create_dir_all(&cache_dir_path) {
            log::error!("{:?}", e);
        }

        let database = MeshDatabase::init(cache_dir_path.join("mesh.db"));
        let thumbnail = MeshThumbnail::new(cache_dir_path.join("thumbnail"));

        Self {
            database,
            thumbnail,
        }
    }

    pub fn database(&self) -> &MeshDatabase {
        &self.database
    }

    pub fn thumbnail(&self) -> &MeshThumbnail {
        &self.thumbnail
    }
}
