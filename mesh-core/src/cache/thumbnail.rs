use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub struct MeshThumbnail {
    thumbnail_dir_path: PathBuf,
}

impl MeshThumbnail {
    pub fn new(thumbnail_dir_path: PathBuf) -> Self {
        fs::create_dir_all(&thumbnail_dir_path)
            .map_err(|e| log::warn!("Failed to create path: {}", e))
            .unwrap();

        Self { thumbnail_dir_path }
    }

    /// 尝试从缓存中读取缩略图数据
    pub fn read_thumbnail(&self, file_hash: u128) -> io::Result<Vec<u8>> {
        let full_path = get_file_path(&self.thumbnail_dir_path, &file_hash);

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
    pub fn write_thumbnail(&self, file_hash: u128, data: &[u8]) -> io::Result<()> {
        let full_path = get_file_path(&self.thumbnail_dir_path, &file_hash);

        // 确保分桶目录存在
        let parent_dir = full_path.parent().unwrap();
        fs::create_dir_all(parent_dir)?;

        // 写入文件
        let mut file = fs::File::create(full_path)?;
        file.write_all(data)
    }

    pub fn generate_file_hash(file_path: &PathBuf) -> u128 {
        let hasher = blake3::hash(file_path.as_os_str().as_encoded_bytes());
        let bytes = hasher.as_bytes();
        u128::from_le_bytes(bytes[0..16].try_into().unwrap())
    }
}

// 假设我们使用原图路径的哈希值来定位缩略图

/// 计算缩略图的完整缓存路径
fn get_file_path(root: &Path, file_hash: &u128) -> PathBuf {
    // 2. 使用 u128 值计算目录分桶。
    // 提取 u128 的最高两位十六进制数字作为第一层目录（即最高字节）。
    let dir_segment_1_val = (file_hash >> 120) as u8;
    let dir_segment_1 = format!("{:02x}", dir_segment_1_val);

    // 提取 u128 的接下来两位十六进制数字作为第二层目录（即次高字节）。
    let dir_segment_2_val = ((file_hash >> 112) & 0xFF) as u8;
    let dir_segment_2 = format!("{:02x}", dir_segment_2_val);

    // 3. 构建目录路径: {thumbnail_dir}/{dir_segment_1}/{dir_segment_2}
    let bucket_dir = root.join(dir_segment_1).join(dir_segment_2);

    // 4. 构建最终文件路径。文件名依然是完整的十六进制字符串。
    bucket_dir.join(format!("{}", file_hash))
}
