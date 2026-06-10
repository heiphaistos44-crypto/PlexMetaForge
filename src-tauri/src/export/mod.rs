use std::io::Write;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;
use crate::error::{PlexMetaForgeError, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportResult {
    pub zip_path: String,
    pub file_count: usize,
    pub size_bytes: u64,
}

/// Exporte un .bundle Plex en archive ZIP
pub fn export_plugin_zip(bundle_path: &PathBuf, dest_dir: &PathBuf) -> Result<ExportResult> {
    if !bundle_path.exists() {
        return Err(PlexMetaForgeError::PluginNotFound(
            bundle_path.to_string_lossy().to_string(),
        ));
    }

    std::fs::create_dir_all(dest_dir)?;

    let bundle_name = bundle_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy();
    let zip_name = format!(
        "{}_{}.zip",
        bundle_name.trim_end_matches(".disabled"),
        chrono::Local::now().format("%Y%m%d_%H%M%S")
    );
    let zip_path = dest_dir.join(&zip_name);

    let file = std::fs::File::create(&zip_path)?;
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o644);

    let mut file_count = 0usize;

    for entry in WalkDir::new(bundle_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let relative = path
            .strip_prefix(bundle_path.parent().unwrap_or(bundle_path))
            .unwrap_or(path);
        let zip_entry = relative.to_string_lossy().replace('\\', "/");

        if path.is_dir() {
            zip.add_directory(&zip_entry, options)?;
        } else {
            zip.start_file(&zip_entry, options)?;
            let data = std::fs::read(path)?;
            zip.write_all(&data)?;
            file_count += 1;
        }
    }

    zip.finish()?;
    let size_bytes = std::fs::metadata(&zip_path)?.len();

    Ok(ExportResult {
        zip_path: zip_path.to_string_lossy().to_string(),
        file_count,
        size_bytes,
    })
}

/// Exporte tous les bundles du dossier Plug-ins
pub fn export_all_plugins_zip(plugins_dir: &PathBuf, dest_dir: &PathBuf) -> Result<ExportResult> {
    std::fs::create_dir_all(dest_dir)?;

    let zip_name = format!(
        "PlexPlugins_all_{}.zip",
        chrono::Local::now().format("%Y%m%d_%H%M%S")
    );
    let zip_path = dest_dir.join(&zip_name);

    let file = std::fs::File::create(&zip_path)?;
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o644);

    let mut file_count = 0usize;

    for entry in std::fs::read_dir(plugins_dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if !name.ends_with(".bundle") && !name.ends_with(".bundle.disabled") {
            continue;
        }

        for inner in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
            let ipath = inner.path();
            let relative = ipath
                .strip_prefix(plugins_dir)
                .unwrap_or(ipath);
            let zip_entry = relative.to_string_lossy().replace('\\', "/");

            if ipath.is_dir() {
                zip.add_directory(&zip_entry, options)?;
            } else {
                zip.start_file(&zip_entry, options)?;
                zip.write_all(&std::fs::read(ipath)?)?;
                file_count += 1;
            }
        }
    }

    zip.finish()?;
    let size_bytes = std::fs::metadata(&zip_path)?.len();

    Ok(ExportResult {
        zip_path: zip_path.to_string_lossy().to_string(),
        file_count,
        size_bytes,
    })
}

pub fn default_export_dir() -> PathBuf {
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join("Downloads").join("PlexMetaForge_Exports")
}
