use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::core::MeshCore;

pub(crate) struct MeshCache {}

impl MeshCache {}

pub(crate) fn init() -> anyhow::Result<MeshCache> {
    let _cache_dir = MeshCore::cache_path("thumbnail")?;

    Ok(MeshCache {})
}

// 假设我们使用原图路径的哈希值来定位缩略图
pub fn generate_file_hash(data: &[u8]) -> u64 {
    let hasher = blake3::hash(data);
    let bytes = hasher.as_bytes();
    u64::from_le_bytes(bytes[0..8].try_into().unwrap())
}

/// 计算缩略图的完整缓存路径
fn get_cache_path(cache_root: &Path, file_hash: u64) -> PathBuf {
    // 2. 使用 u64 值计算目录分桶。
    // 提取 u64 的最高两位十六进制数字作为第一层目录（即最高字节）。
    let dir_segment_1_val = (file_hash >> 56) as u8;
    let dir_segment_1 = format!("{:02x}", dir_segment_1_val);

    // 提取 u64 的接下来两位十六进制数字作为第二层目录（即次高字节）。
    let dir_segment_2_val = ((file_hash >> 48) & 0xFF) as u8;
    let dir_segment_2 = format!("{:02x}", dir_segment_2_val);

    // 3. 构建目录路径: {cache_root}/{dir_segment_1}/{dir_segment_2}
    let bucket_dir = cache_root.join(dir_segment_1).join(dir_segment_2);

    // 4. 构建最终文件路径。文件名依然是完整的十六进制字符串。
    let filename = format!("{}.jpg", file_hash);
    bucket_dir.join(filename)
}

pub struct ThumbnailCacheManager {
    root_path: PathBuf,
}

impl ThumbnailCacheManager {
    // pub fn new<P: AsRef<Path>>(root: P) -> Self {
    //     let root_path = root.as_ref().to_path_buf();
    //     // 确保根目录存在
    //     fs::create_dir_all(&root_path).expect("Failed to create cache root directory");
    //     ThumbnailCacheManager { root_path }
    // }

    /// 尝试从缓存中读取缩略图数据
    pub fn read_thumbnail(&self, file_hash: u64) -> io::Result<Vec<u8>> {
        let full_path = get_cache_path(&self.root_path, file_hash);

        // 如果文件存在，读取并返回
        if full_path.exists() {
            fs::read(&full_path)
        } else {
            // 如果不存在，返回特定的错误（例如 NotFound）
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Thumbnail not found in cache",
            ))
        }
    }

    /// 将新生成的缩略图数据写入缓存
    pub fn write_thumbnail(&self, file_hash: u64, data: &[u8]) -> io::Result<()> {
        let full_path = get_cache_path(&self.root_path, file_hash);

        // 确保分桶目录存在
        let parent_dir = full_path.parent().unwrap();
        fs::create_dir_all(parent_dir)?;

        // 写入文件
        let mut file = fs::File::create(full_path)?;
        file.write_all(data)
    }
}
