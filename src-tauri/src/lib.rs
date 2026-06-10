mod config;
mod database;
mod error;
mod generator;
mod metadata;
mod scanner;

use config::PlexPaths;
use generator::PluginConfig;
use metadata::{InjectionReport, MetadataPayload};
use std::sync::Mutex;
use tauri::State;

pub struct AppState {
    pub plex_paths: Mutex<Option<PlexPaths>>,
    pub plex_token: Mutex<Option<String>>,
}

// ─── Commands — Config ────────────────────────────────────────────────────────

#[tauri::command]
fn get_plex_paths(state: State<AppState>) -> Result<serde_json::Value, String> {
    let guard = state.plex_paths.lock().unwrap();
    match &*guard {
        Some(p) => Ok(serde_json::json!({
            "plugins_dir": p.plugins_dir.to_string_lossy(),
            "database_path": p.database_path.to_string_lossy(),
            "plugins_dir_exists": p.plugins_dir_exists(),
            "database_exists": p.database_exists(),
        })),
        None => Err("Plex non détecté (LOCALAPPDATA manquant)".to_string()),
    }
}

#[tauri::command]
fn set_plex_token(token: String, state: State<AppState>) -> Result<(), String> {
    let mut guard = state.plex_token.lock().unwrap();
    *guard = if token.trim().is_empty() {
        None
    } else {
        Some(token.trim().to_string())
    };
    Ok(())
}

// ─── Commands — Scanner ───────────────────────────────────────────────────────

#[tauri::command]
fn list_plugins(state: State<AppState>) -> Result<Vec<scanner::Plugin>, String> {
    let guard = state.plex_paths.lock().unwrap();
    let paths = guard
        .as_ref()
        .ok_or_else(|| "Plex paths non initialisés".to_string())?;
    scanner::list_plugins(&paths.plugins_dir).map_err(|e| e.to_string())
}

#[tauri::command]
fn toggle_plugin(path: String, enable: bool) -> Result<String, String> {
    let plugin_path = std::path::PathBuf::from(path);
    scanner::toggle_plugin(&plugin_path, enable).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_plugin(path: String) -> Result<(), String> {
    let plugin_path = std::path::PathBuf::from(path);
    scanner::delete_plugin(&plugin_path).map_err(|e| e.to_string())
}

// ─── Commands — Generator ────────────────────────────────────────────────────

#[tauri::command]
fn create_plugin(name: String, state: State<AppState>) -> Result<String, String> {
    let guard = state.plex_paths.lock().unwrap();
    let paths = guard
        .as_ref()
        .ok_or_else(|| "Plex paths non initialisés".to_string())?;
    generator::create_plugin(&paths.plugins_dir, &name).map_err(|e| e.to_string())
}

#[tauri::command]
fn create_plugin_from_template(
    config: PluginConfig,
    state: State<AppState>,
) -> Result<String, String> {
    let guard = state.plex_paths.lock().unwrap();
    let paths = guard
        .as_ref()
        .ok_or_else(|| "Plex paths non initialisés".to_string())?;
    generator::create_plugin_from_config(&paths.plugins_dir, &config)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_plugin_templates() -> serde_json::Value {
    serde_json::json!([
        {
            "id": "blank",
            "label": "Vierge",
            "icon": "📄",
            "description": "Structure minimale. Tu codes tout toi-même.",
            "requires_tmdb": false,
            "requires_lastfm": false
        },
        {
            "id": "cinema",
            "label": "Films",
            "icon": "🎬",
            "description": "Agent complet pour les films. Récupère titres, synopsis, posters, casting, réalisateurs depuis TMDB.",
            "requires_tmdb": true,
            "requires_lastfm": false
        },
        {
            "id": "series",
            "label": "Séries TV",
            "icon": "📺",
            "description": "Agent séries avec épisodes, saisons, vignettes et casting complet depuis TMDB.",
            "requires_tmdb": true,
            "requires_lastfm": false
        },
        {
            "id": "musique",
            "label": "Musique",
            "icon": "🎵",
            "description": "Double agent Artiste + Album. Biographies, jaquettes, pistes via Last.fm.",
            "requires_tmdb": false,
            "requires_lastfm": true
        },
        {
            "id": "anime",
            "label": "Anime / Manga",
            "icon": "⛩️",
            "description": "Spécialisé anime et manga via AniList. Titres JP/FR/EN, personnages, doubleurs. Sans clé API.",
            "requires_tmdb": false,
            "requires_lastfm": false
        },
        {
            "id": "universal",
            "label": "Universel (All-in-one)",
            "icon": "🌐",
            "description": "Couvre films, séries, anime et musique. Détection automatique du type. TMDB + AniList + Last.fm.",
            "requires_tmdb": true,
            "requires_lastfm": false
        }
    ])
}

#[tauri::command]
fn read_plugin_code(path: String) -> Result<String, String> {
    let init_path = std::path::PathBuf::from(&path)
        .join("Contents")
        .join("Code")
        .join("__init__.py");
    std::fs::read_to_string(init_path).map_err(|e| e.to_string())
}

#[tauri::command]
fn write_plugin_code(path: String, content: String) -> Result<(), String> {
    let init_path = std::path::PathBuf::from(&path)
        .join("Contents")
        .join("Code")
        .join("__init__.py");
    std::fs::write(init_path, content).map_err(|e| e.to_string())
}

// ─── Commands — Database ─────────────────────────────────────────────────────

#[tauri::command]
fn search_plex_db(query: String, state: State<AppState>) -> Result<Vec<database::MediaItem>, String> {
    let guard = state.plex_paths.lock().unwrap();
    let paths = guard
        .as_ref()
        .ok_or_else(|| "Plex paths non initialisés".to_string())?;
    let db = database::PlexDatabase::open(&paths.database_path).map_err(|e| e.to_string())?;
    db.search_metadata_items(&query).map_err(|e| e.to_string())
}

// ─── Commands — Metadata Engine (Module C) ───────────────────────────────────

#[tauri::command]
async fn inject_metadata(
    payload: MetadataPayload,
    state: State<'_, AppState>,
) -> Result<InjectionReport, String> {
    let plex_paths = state.plex_paths.lock().unwrap().clone();
    let plex_token = state.plex_token.lock().unwrap().clone();

    metadata::inject(payload, plex_paths, plex_token)
        .await
        .map_err(|e| e.to_string())
}

// ─── Entry Point ─────────────────────────────────────────────────────────────

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    let plex_paths = PlexPaths::detect().ok();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState {
            plex_paths: Mutex::new(plex_paths),
            plex_token: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            get_plex_paths,
            set_plex_token,
            list_plugins,
            toggle_plugin,
            delete_plugin,
            create_plugin,
            create_plugin_from_template,
            get_plugin_templates,
            read_plugin_code,
            write_plugin_code,
            search_plex_db,
            inject_metadata,
        ])
        .run(tauri::generate_context!())
        .expect("error while running PlexMetaForge");
}
