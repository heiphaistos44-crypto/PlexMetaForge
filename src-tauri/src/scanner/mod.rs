use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::error::Result;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Plugin {
    pub name: String,
    pub path: String,
    pub enabled: bool,
    pub has_code: bool,
}

pub fn list_plugins(plugins_dir: &PathBuf) -> Result<Vec<Plugin>> {
    let mut plugins = Vec::new();

    if !plugins_dir.exists() {
        return Ok(plugins);
    }

    for entry in std::fs::read_dir(plugins_dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name().to_string_lossy().to_string();

        if file_name.ends_with(".bundle") || file_name.ends_with(".bundle.disabled") {
            let enabled = !file_name.ends_with(".bundle.disabled");
            let has_code = path.join("Contents").join("Code").join("__init__.py").exists();
            let name = file_name
                .trim_end_matches(".disabled")
                .to_string();

            plugins.push(Plugin {
                name,
                path: path.to_string_lossy().to_string(),
                enabled,
                has_code,
            });
        }
    }

    plugins.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(plugins)
}

pub fn toggle_plugin(plugin_path: &PathBuf, enable: bool) -> Result<String> {
    let current = plugin_path.to_string_lossy().to_string();
    let is_disabled = current.ends_with(".disabled");

    // Already in the desired state — no-op
    if enable && !is_disabled {
        return Ok(current);
    }
    if !enable && is_disabled {
        return Ok(current);
    }

    backup_plugin(plugin_path)?;

    let new_path = if enable {
        PathBuf::from(current.trim_end_matches(".disabled"))
    } else {
        PathBuf::from(format!("{}.disabled", current))
    };

    std::fs::rename(plugin_path, &new_path)?;
    Ok(new_path.to_string_lossy().to_string())
}

pub fn delete_plugin(plugin_path: &PathBuf) -> Result<()> {
    backup_plugin(plugin_path)?;
    std::fs::remove_dir_all(plugin_path)?;
    Ok(())
}

pub fn backup_plugin(plugin_path: &PathBuf) -> Result<()> {
    let parent = plugin_path.parent().unwrap_or(plugin_path);
    let backup_dir = parent.join("_backups");
    std::fs::create_dir_all(&backup_dir)?;

    let file_name = plugin_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy();
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let backup_path = backup_dir.join(format!("{}_{}", file_name, timestamp));

    if plugin_path.is_dir() {
        copy_dir_recursive(plugin_path, &backup_path)?;
    } else {
        std::fs::copy(plugin_path, &backup_path)?;
    }

    Ok(())
}

fn copy_dir_recursive(src: &PathBuf, dst: &PathBuf) -> Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}
