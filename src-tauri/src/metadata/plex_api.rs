use serde::Deserialize;
use crate::error::{PlexMetaForgeError, Result};

const PLEX_BASE: &str = "http://localhost:32400";

#[derive(Deserialize)]
struct SectionsEnvelope {
    #[serde(rename = "MediaContainer")]
    media_container: SectionsContainer,
}

#[derive(Deserialize)]
struct SectionsContainer {
    #[serde(rename = "Directory", default)]
    directories: Vec<PlexSection>,
}

#[derive(Deserialize)]
struct PlexSection {
    key: String,
    #[serde(rename = "Location", default)]
    locations: Vec<PlexLocation>,
}

#[derive(Deserialize)]
struct PlexLocation {
    path: String,
}

pub async fn refresh_section(media_path: &str, token: &str) -> Result<()> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(PlexMetaForgeError::Http)?;

    // List sections
    let envelope: SectionsEnvelope = client
        .get(format!("{}/library/sections", PLEX_BASE))
        .header("X-Plex-Token", token)
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(PlexMetaForgeError::Http)?
        .json()
        .await
        .map_err(|e| PlexMetaForgeError::PlexApi(format!("Parse sections: {}", e)))?;

    // Find section matching media_path prefix
    let key = envelope
        .media_container
        .directories
        .iter()
        .find(|sec| {
            sec.locations
                .iter()
                .any(|loc| media_path.starts_with(&loc.path))
        })
        .map(|sec| sec.key.clone())
        .ok_or_else(|| {
            PlexMetaForgeError::PlexApi(format!(
                "Aucune section ne correspond au chemin: {}",
                media_path
            ))
        })?;

    // Targeted refresh on section + path
    let resp = client
        .get(format!("{}/library/sections/{}/refresh", PLEX_BASE, key))
        .header("X-Plex-Token", token)
        .query(&[("path", media_path)])
        .send()
        .await
        .map_err(PlexMetaForgeError::Http)?;

    if !resp.status().is_success() {
        return Err(PlexMetaForgeError::PlexApi(format!(
            "Refresh HTTP {}",
            resp.status()
        )));
    }

    Ok(())
}
