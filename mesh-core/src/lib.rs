// use std::path::PathBuf;

// use clap::Parser;
// use walkdir::WalkDir;

// use crate::core::MeshApp;

mod cache;
mod config;
mod core;
mod database;

pub use core::MeshCore;
// #[derive(Debug, Parser)]
// struct Cli {
//     files: Vec<PathBuf>,
//     #[arg(short)]
//     tag: Option<String>,
// }

// fn collect_files(paths: &Vec<PathBuf>) -> Vec<PathBuf> {
//     let extensions = [
//         "jpg", "jpeg", "png", "gif", "bmp", "webp", // 图像
//         "mp4", "mkv", "avi", "mov", "webm", // 视频
//     ];

//     paths
//         .into_iter()
//         .flat_map(|path| {
//             WalkDir::new(path)
//                 .into_iter()
//                 .filter_map(|e| e.ok())
//                 .filter(|entry| entry.file_type().is_file())
//                 .filter_map(|entry| {
//                     entry
//                         .path()
//                         .extension()
//                         .and_then(|s| s.to_str())
//                         .map(|ext| ext.to_lowercase())
//                         // 检查扩展名是否在允许的列表中
//                         .filter(|ext_lower| extensions.contains(&ext_lower.as_str()))
//                         // 如果通过，返回完整的路径
//                         .map(|_| entry.into_path())
//                 })
//         })
//         .collect()
// }

// fn main() {
//     let cli = Cli::parse();
//     env_logger::init();

//     let files = collect_files(&cli.files);

//     let app = match MeshApp::init() {
//         Ok(app) => app,
//         Err(e) => {
//             log::error!("Error: {}", e);
//             return;
//         }
//     };
// }
