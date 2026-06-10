mod config;
mod database;
mod error;
mod export;
mod generator;
mod metadata;
mod scanner;

use config::PlexPaths;
use database::{BatchUpdateResult, DatabaseStats, MediaItem, PlexSection};
use export::ExportResult;
use generator::PluginConfig;
use metadata::{InjectionReport, MetadataPayload};
use std::sync::Mutex;
use tauri::State;

pub struct AppState {
    pub plex_paths: Mutex<Option<PlexPaths>>,
    pub plex_token: Mutex<Option<String>>,
}

// ─── Config ───────────────────────────────────────────────────

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
    *guard = if token.trim().is_empty() { None } else { Some(token.trim().to_string()) };
    Ok(())
}

// ─── Scanner ──────────────────────────────────────────────────

#[tauri::command]
fn list_plugins(state: State<AppState>) -> Result<Vec<scanner::Plugin>, String> {
    let guard = state.plex_paths.lock().unwrap();
    let paths = guard.as_ref().ok_or("Plex paths non initialisés")?;
    scanner::list_plugins(&paths.plugins_dir).map_err(|e| e.to_string())
}

#[tauri::command]
fn toggle_plugin(path: String, enable: bool) -> Result<String, String> {
    scanner::toggle_plugin(&std::path::PathBuf::from(path), enable).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_plugin(path: String) -> Result<(), String> {
    scanner::delete_plugin(&std::path::PathBuf::from(path)).map_err(|e| e.to_string())
}

// ─── Generator ────────────────────────────────────────────────

#[tauri::command]
fn create_plugin(name: String, state: State<AppState>) -> Result<String, String> {
    let guard = state.plex_paths.lock().unwrap();
    let paths = guard.as_ref().ok_or("Plex paths non initialisés")?;
    generator::create_plugin(&paths.plugins_dir, &name).map_err(|e| e.to_string())
}

#[tauri::command]
fn create_plugin_from_template(config: PluginConfig, state: State<AppState>) -> Result<String, String> {
    let guard = state.plex_paths.lock().unwrap();
    let paths = guard.as_ref().ok_or("Plex paths non initialisés")?;
    generator::create_plugin_from_config(&paths.plugins_dir, &config).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_plugin_templates() -> serde_json::Value {
    serde_json::json!([
        { "id": "blank",     "label": "Vierge",              "icon": "📄",
          "description": "Structure minimale. Tu codes tout toi-même.",
          "requires_tmdb": false, "requires_lastfm": false },
        { "id": "cinema",    "label": "Films",               "icon": "🎬",
          "description": "Agent complet films. Titres, synopsis, posters, casting depuis TMDB.",
          "requires_tmdb": true,  "requires_lastfm": false },
        { "id": "series",    "label": "Séries TV",           "icon": "📺",
          "description": "Agent séries avec épisodes, saisons, vignettes via TMDB.",
          "requires_tmdb": true,  "requires_lastfm": false },
        { "id": "musique",   "label": "Musique",             "icon": "🎵",
          "description": "Double agent Artiste + Album. Biographies, jaquettes via Last.fm.",
          "requires_tmdb": false, "requires_lastfm": true  },
        { "id": "anime",     "label": "Anime / Manga",       "icon": "⛩️",
          "description": "Spécialisé anime/manga via AniList. Titres JP/FR/EN, personnages, doubleurs. Sans clé.",
          "requires_tmdb": false, "requires_lastfm": false },
        { "id": "universal", "label": "Universel (All-in-one)", "icon": "🌐",
          "description": "Films + Séries + Anime + Musique. Détection automatique. TMDB + AniList + Last.fm.",
          "requires_tmdb": true,  "requires_lastfm": false }
    ])
}

#[tauri::command]
fn read_plugin_code(path: String) -> Result<String, String> {
    let init = std::path::PathBuf::from(&path).join("Contents").join("Code").join("__init__.py");
    std::fs::read_to_string(init).map_err(|e| e.to_string())
}

#[tauri::command]
fn write_plugin_code(path: String, content: String) -> Result<(), String> {
    let init = std::path::PathBuf::from(&path).join("Contents").join("Code").join("__init__.py");
    std::fs::write(init, content).map_err(|e| e.to_string())
}

// ─── Export ───────────────────────────────────────────────────

#[tauri::command]
fn export_plugin(path: String, dest_dir: Option<String>) -> Result<ExportResult, String> {
    let bundle = std::path::PathBuf::from(&path);
    let dest = dest_dir
        .map(std::path::PathBuf::from)
        .unwrap_or_else(export::default_export_dir);
    export::export_plugin_zip(&bundle, &dest).map_err(|e| e.to_string())
}

#[tauri::command]
fn export_all_plugins(state: State<AppState>, dest_dir: Option<String>) -> Result<ExportResult, String> {
    let guard = state.plex_paths.lock().unwrap();
    let paths = guard.as_ref().ok_or("Plex paths non initialisés")?;
    let dest = dest_dir
        .map(std::path::PathBuf::from)
        .unwrap_or_else(export::default_export_dir);
    export::export_all_plugins_zip(&paths.plugins_dir, &dest).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_export_dir() -> String {
    export::default_export_dir().to_string_lossy().to_string()
}

// ─── Database ─────────────────────────────────────────────────

#[tauri::command]
fn get_db_stats(state: State<AppState>) -> Result<DatabaseStats, String> {
    let guard = state.plex_paths.lock().unwrap();
    let paths = guard.as_ref().ok_or("Plex paths non initialisés")?;
    let db = database::PlexDatabase::open(&paths.database_path).map_err(|e| e.to_string())?;
    db.get_stats().map_err(|e| e.to_string())
}

#[tauri::command]
fn get_db_sections(state: State<AppState>) -> Result<Vec<PlexSection>, String> {
    let guard = state.plex_paths.lock().unwrap();
    let paths = guard.as_ref().ok_or("Plex paths non initialisés")?;
    let db = database::PlexDatabase::open(&paths.database_path).map_err(|e| e.to_string())?;
    db.get_sections().map_err(|e| e.to_string())
}

#[tauri::command]
fn search_plex_db(query: String, state: State<AppState>) -> Result<Vec<MediaItem>, String> {
    let guard = state.plex_paths.lock().unwrap();
    let paths = guard.as_ref().ok_or("Plex paths non initialisés")?;
    let db = database::PlexDatabase::open(&paths.database_path).map_err(|e| e.to_string())?;
    db.search_metadata_items(&query).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_items_by_section(section_id: i64, limit: Option<i64>, state: State<AppState>) -> Result<Vec<MediaItem>, String> {
    let guard = state.plex_paths.lock().unwrap();
    let paths = guard.as_ref().ok_or("Plex paths non initialisés")?;
    let db = database::PlexDatabase::open(&paths.database_path).map_err(|e| e.to_string())?;
    db.get_items_by_section(section_id, limit.unwrap_or(200)).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_incomplete_items(limit: Option<i64>, state: State<AppState>) -> Result<Vec<MediaItem>, String> {
    let guard = state.plex_paths.lock().unwrap();
    let paths = guard.as_ref().ok_or("Plex paths non initialisés")?;
    let db = database::PlexDatabase::open(&paths.database_path).map_err(|e| e.to_string())?;
    db.get_incomplete_items(limit.unwrap_or(100)).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_batch_clear_locks(state: State<AppState>) -> Result<BatchUpdateResult, String> {
    let guard = state.plex_paths.lock().unwrap();
    let paths = guard.as_ref().ok_or("Plex paths non initialisés")?;
    let db = database::PlexDatabase::open(&paths.database_path).map_err(|e| e.to_string())?;
    db.batch_clear_locks().map_err(|e| e.to_string())
}

// ─── Metadata Engine (Module C) ───────────────────────────────

#[tauri::command]
async fn inject_metadata(payload: MetadataPayload, state: State<'_, AppState>) -> Result<InjectionReport, String> {
    let plex_paths = state.plex_paths.lock().unwrap().clone();
    let plex_token = state.plex_token.lock().unwrap().clone();
    metadata::inject(payload, plex_paths, plex_token).await.map_err(|e| e.to_string())
}

// ─── Entry Point ──────────────────────────────────────────────

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
            // Config
            get_plex_paths, set_plex_token,
            // Scanner
            list_plugins, toggle_plugin, delete_plugin,
            // Generator
            create_plugin, create_plugin_from_template, get_plugin_templates,
            read_plugin_code, write_plugin_code,
            // Export
            export_plugin, export_all_plugins, get_export_dir,
            // Database
            get_db_stats, get_db_sections, search_plex_db,
            get_items_by_section, get_incomplete_items, db_batch_clear_locks,
            // Metadata
            inject_metadata,
        ])
        .run(tauri::generate_context!())
        .expect("error while running PlexMetaForge");
}
