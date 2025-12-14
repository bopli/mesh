use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use clap::Parser;
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

    let extensions = [
        "jpg", "jpeg", "png", "gif", "bmp", "webp", // 图像
        "mp4", "mkv", "avi", "mov", "webm", // 视频
    ];

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

fn main() {
    let cli = Cli::parse();
    env_logger::init_from_env(env_logger::Env::new().filter("MESH_LOG"));

    let config = MeshConfig::init();

    let files = collect_files(config.album_dirs(), config.excluded_dirs(), cli.files);

    println!("{:?}", files);
}
