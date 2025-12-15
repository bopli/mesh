use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use anyhow::Context;
use clap::Parser;
use image::{
    GenericImageView, ImageEncoder,
    codecs::{jpeg::JpegEncoder, png::PngEncoder},
};
use mesh_core::MeshConfig;
use walkdir::WalkDir;

#[derive(Debug, Parser)]
struct Cli {
    files: Vec<PathBuf>,
    #[arg(short)]
    tag: Option<String>,
}

fn collect_files(
    album_dirs: &[PathBuf],
    excluded_dirs: &[PathBuf],
    files: Vec<PathBuf>,
) -> Vec<PathBuf> {
    let ex: HashSet<_> = excluded_dirs.iter().collect();
    let al: HashSet<_> = album_dirs.iter().collect();

    let extensions = ["jpg", "jpeg", "png"];
    // "gif", "bmp", "webp", // 图像
    // "mp4", "mkv", "avi", "mov", "webm", // 视频

    fn under_any<'a>(p: &Path, roots: &HashSet<&'a PathBuf>) -> bool {
        roots.iter().any(|r| p.starts_with(r))
    }

    files
        .into_iter()
        .filter(|p| !ex.iter().any(|e| p.starts_with(e)))
        .filter(|p| p.is_file() || p.is_dir())
        .flat_map(|p| {
            WalkDir::new(&p)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
                .map(|e| e.path().to_path_buf())
                .filter(|f| {
                    under_any(f, &al)
                        && f.extension()
                            .and_then(|s| s.to_str())
                            .map(|e| extensions.contains(&e.to_lowercase().as_str()))
                            .unwrap_or(false)
                })
        })
        .collect()
}

fn generate_thumbnails(src_paths: &[PathBuf], dst_dir: &Path) -> anyhow::Result<()> {
    // 建立输出目录
    std::fs::create_dir_all(dst_dir).with_context(|| format!("创建输出目录 {:?} 失败", dst_dir))?;

    // 并行处理每一个文件
    src_paths
        .iter()
        // .par_iter()
        .try_for_each(|src_path| -> anyhow::Result<()> {
            // 读取原图
            let img =
                image::open(src_path).with_context(|| format!("无法读取图片 {:?}", src_path))?;

            // ① 计算目标尺寸（最大高度 300）
            const MAX_HEIGHT: u32 = 300;
            let (orig_w, orig_h) = img.dimensions();
            let (target_w, target_h) = if orig_h <= MAX_HEIGHT {
                (orig_w, orig_h)
            } else {
                let scale = MAX_HEIGHT as f32 / orig_h as f32;
                ((orig_w as f32 * scale).round() as u32, MAX_HEIGHT)
            };

            let resized =
                img.resize_exact(target_w, target_h, image::imageops::FilterType::Lanczos3);

            // ② 设定输出文件名 (与源文件同名，放到 dst_dir)
            let mut dst_file = dst_dir.join(
                src_path
                    .file_stem()
                    .ok_or_else(|| anyhow::anyhow!("无效文件名: {:?}", src_path))?
                    .to_os_string(),
            );

            // 用与源相同的后缀；若不支持则强制 jpg
            let ext = src_path
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("jpg")
                .to_ascii_lowercase();
            dst_file.set_extension(&ext);

            // ③ 写入图片（JPEG 质量40，PNG 级别9）
            let out_file = std::fs::File::create(&dst_file)
                .with_context(|| format!("创建文件 {:?} 失败", dst_file))?;
            let writer = std::io::BufWriter::new(out_file);

            match ext.as_str() {
                "png" => {
                    let encoder = PngEncoder::new_with_quality(
                        writer,
                        image::codecs::png::CompressionType::Level(7),
                        image::codecs::png::FilterType::NoFilter,
                    );
                    let rgba = resized.to_rgba8();
                    encoder
                        .write_image(&rgba, target_w, target_h, image::ExtendedColorType::Rgba8)
                        .context("PNG 编码失败")?;
                }
                _ => {
                    // JPEG
                    let mut encoder = JpegEncoder::new_with_quality(writer, 70);
                    encoder.encode_image(&resized).context("JPEG 编码失败")?;
                }
            }

            println!("✅ 生成缩略图: {:?}", dst_file);
            Ok(())
        })
}

fn main() {
    let cli = Cli::parse();
    env_logger::init_from_env(env_logger::Env::new().filter("MESH_LOG"));

    let config = MeshConfig::init();
    let dst_dir = PathBuf::from("tmp");

    let files = collect_files(config.album_dirs(), config.excluded_dirs(), cli.files);

    println!("file len is: {}", files.len());

    if let Err(e) = generate_thumbnails(&files, &dst_dir) {
        log::error!("{:?}", e);
    }
}
