use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlexMetaForgeError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Plex API error: {0}")]
    PlexApi(String),

    #[error("Plugin not found: {0}")]
    PluginNotFound(String),

    #[error("Database locked after {0}ms timeout")]
    DatabaseLocked(u64),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Environment variable missing: {0}")]
    EnvVar(String),

    #[error("ZIP error: {0}")]
    Zip(#[from] zip::result::ZipError),
}

pub type Result<T> = std::result::Result<T, PlexMetaForgeError>;

impl serde::Serialize for PlexMetaForgeError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
