use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::error::{PlexMetaForgeError, Result};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlexPaths {
    pub plugins_dir: PathBuf,
    pub database_path: PathBuf,
}

impl PlexPaths {
    pub fn detect() -> Result<Self> {
        let local_app_data = std::env::var("LOCALAPPDATA").map_err(|_| {
            PlexMetaForgeError::EnvVar("LOCALAPPDATA not set".to_string())
        })?;

        let plex_root = PathBuf::from(local_app_data).join("Plex Media Server");

        Ok(Self {
            plugins_dir: plex_root.join("Plug-ins"),
            database_path: plex_root
                .join("Plug-in Support")
                .join("Databases")
                .join("com.plexapp.plugins.library.db"),
        })
    }

    pub fn plugins_dir_exists(&self) -> bool {
        self.plugins_dir.exists()
    }

    pub fn database_exists(&self) -> bool {
        self.database_path.exists()
    }
}
