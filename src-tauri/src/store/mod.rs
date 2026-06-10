use serde::{Deserialize, Serialize};
use std::io::Read;
use std::path::PathBuf;
use crate::error::{PlexMetaForgeError, Result};

// ─── Catalogue ────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StorePlugin {
    pub id: &'static str,
    pub name: &'static str,
    pub author: &'static str,
    pub description: &'static str,
    pub category: &'static str,
    pub subcategory: &'static str,
    pub tags: Vec<&'static str>,
    pub license: &'static str,
    pub stars: &'static str,
    pub github_url: &'static str,
    pub zip_url: &'static str,
    pub bundle_name: &'static str,
    pub verified: bool,
}

pub fn categories() -> Vec<(&'static str, Vec<&'static str>)> {
    vec![
        ("Métadonnées", vec!["Films", "Séries TV", "Anime / Manga", "Musique", "Audiobooks", "Comics"]),
        ("Sous-titres", vec!["Multi-langues", "Français"]),
        ("Outils", vec!["Utilitaires", "Scanners", "IPTV", "Podcast", "Sync"]),
    ]
}

pub fn catalog() -> Vec<StorePlugin> {
    vec![
        // ══ MÉTADONNÉES — Films ══════════════════════════════
        StorePlugin {
            id: "tmdb-agent",
            name: "The Movie Database (TMDB)",
            author: "Plex Inc.",
            description: "Agent officiel TMDB pour films. Titres, synopsis, posters HD, casting, réalisateurs, backdrops. Maintenu par Plex. Recommandé pour tous les films.",
            category: "Métadonnées",
            subcategory: "Films",
            tags: vec!["films", "TMDB", "officiel", "posters"],
            license: "MIT",
            stars: "310",
            github_url: "https://github.com/plexinc-plugins/TheMovieDB.bundle",
            zip_url: "https://github.com/plexinc-plugins/TheMovieDB.bundle/archive/refs/heads/master.zip",
            bundle_name: "TheMovieDB.bundle",
            verified: true,
        },
        StorePlugin {
            id: "localmedia",
            name: "Local Media Assets",
            author: "Plex Inc.",
            description: "Agent assets locaux officiel. Lit les poster.jpg, fanart.jpg et fichiers .nfo présents dans les dossiers médias. Essentiel pour utiliser PlexMetaForge.",
            category: "Métadonnées",
            subcategory: "Films",
            tags: vec!["local", "nfo", "poster", "officiel", "essentiel"],
            license: "MIT",
            stars: "420",
            github_url: "https://github.com/plexinc-plugins/LocalMedia.bundle",
            zip_url: "https://github.com/plexinc-plugins/LocalMedia.bundle/archive/refs/heads/master.zip",
            bundle_name: "LocalMedia.bundle",
            verified: true,
        },
        StorePlugin {
            id: "fanart-tv",
            name: "Fanart.tv",
            author: "saltydk",
            description: "Posters, fonds d'écran et banners en très haute qualité depuis Fanart.tv. Complète TMDB avec des assets artistiques uniques et personnalisés.",
            category: "Métadonnées",
            subcategory: "Films",
            tags: vec!["posters", "fanart", "haute qualité", "artwork"],
            license: "MIT",
            stars: "420",
            github_url: "https://github.com/saltydk/Fanart.tv.bundle",
            zip_url: "https://github.com/saltydk/Fanart.tv.bundle/archive/refs/heads/master.zip",
            bundle_name: "Fanart.tv.bundle",
            verified: true,
        },

        // ══ MÉTADONNÉES — Séries TV ══════════════════════════
        StorePlugin {
            id: "tvdb-agent",
            name: "TheTVDB",
            author: "Plex Inc.",
            description: "Agent officiel TheTVDB pour séries TV. Épisodes, saisons, posters, casting depuis The TV Database. Le standard pour les séries. Maintenu par Plex.",
            category: "Métadonnées",
            subcategory: "Séries TV",
            tags: vec!["séries", "TVDB", "officiel", "épisodes"],
            license: "MIT",
            stars: "290",
            github_url: "https://github.com/plexinc-plugins/TheTVDB.bundle",
            zip_url: "https://github.com/plexinc-plugins/TheTVDB.bundle/archive/refs/heads/master.zip",
            bundle_name: "TheTVDB.bundle",
            verified: true,
        },
        StorePlugin {
            id: "xbmc-nfo-movies",
            name: "XBMC NFO Movies Importer",
            author: "gboudreau",
            description: "Importe les métadonnées films depuis les fichiers .nfo au format XBMC/Kodi. Idéal si tu migres depuis Kodi ou utilises MediaElch pour générer des NFO.",
            category: "Métadonnées",
            subcategory: "Films",
            tags: vec!["kodi", "xbmc", "nfo", "import", "migration"],
            license: "MIT",
            stars: "320",
            github_url: "https://github.com/gboudreau/XBMCnfoMoviesImporter.bundle",
            zip_url: "https://github.com/gboudreau/XBMCnfoMoviesImporter.bundle/archive/refs/heads/master.zip",
            bundle_name: "XBMCnfoMoviesImporter.bundle",
            verified: true,
        },
        StorePlugin {
            id: "xbmc-nfo-tv",
            name: "XBMC NFO TV Importer",
            author: "gboudreau",
            description: "Version séries TV de l'agent NFO XBMC/Kodi. Importe métadonnées épisodes depuis les .nfo Kodi, MediaElch ou Tiny Media Manager.",
            category: "Métadonnées",
            subcategory: "Séries TV",
            tags: vec!["kodi", "xbmc", "nfo", "séries", "migration"],
            license: "MIT",
            stars: "280",
            github_url: "https://github.com/gboudreau/XBMCnfoTVImporter.bundle",
            zip_url: "https://github.com/gboudreau/XBMCnfoTVImporter.bundle/archive/refs/heads/master.zip",
            bundle_name: "XBMCnfoTVImporter.bundle",
            verified: true,
        },

        // ══ MÉTADONNÉES — Anime / Manga ══════════════════════
        StorePlugin {
            id: "hama",
            name: "Hama",
            author: "ZeroQI",
            description: "Agent anime ultra-complet. Supporte AniDB, TVDB, TMDB, MAL, AniList. Le standard absolu pour l'anime sur Plex. Gère OVA, films, specials.",
            category: "Métadonnées",
            subcategory: "Anime / Manga",
            tags: vec!["anime", "AniDB", "TVDB", "populaire", "référence"],
            license: "MIT",
            stars: "2.1k",
            github_url: "https://github.com/ZeroQI/Hama.bundle",
            zip_url: "https://github.com/ZeroQI/Hama.bundle/archive/refs/heads/master.zip",
            bundle_name: "Hama.bundle",
            verified: true,
        },
        StorePlugin {
            id: "anidb-agent",
            name: "AniDB Agent",
            author: "Fribb",
            description: "Agent AniDB alternatif. Métadonnées anime depuis AniDB avec support des fichiers hash pour identification précise. Léger et précis.",
            category: "Métadonnées",
            subcategory: "Anime / Manga",
            tags: vec!["anime", "AniDB", "hash", "précis"],
            license: "GPL-3.0",
            stars: "340",
            github_url: "https://github.com/Fribb/AniDB.bundle",
            zip_url: "https://github.com/Fribb/AniDB.bundle/archive/refs/heads/master.zip",
            bundle_name: "AniDB.bundle",
            verified: true,
        },

        // ══ MÉTADONNÉES — Musique ════════════════════════════
        StorePlugin {
            id: "lastfm",
            name: "Last.fm",
            author: "Plex Inc.",
            description: "Agent officiel Last.fm. Biographies artistes, jaquettes albums, genres, artistes similaires. Scrobble automatique de ta musique Plex.",
            category: "Métadonnées",
            subcategory: "Musique",
            tags: vec!["musique", "lastfm", "scrobble", "officiel"],
            license: "MIT",
            stars: "210",
            github_url: "https://github.com/plexinc-plugins/LastFM.bundle",
            zip_url: "https://github.com/plexinc-plugins/LastFM.bundle/archive/refs/heads/master.zip",
            bundle_name: "LastFM.bundle",
            verified: true,
        },

        // ══ MÉTADONNÉES — Audiobooks ════════════════════════
        StorePlugin {
            id: "audnexus",
            name: "Audnexus — Audiobooks",
            author: "djdembeck",
            description: "Agent métadonnées pour livres audio. Auteurs, narrateurs, résumés, jaquettes depuis Audible/Amazon. Parfait pour organiser ta bibliothèque audio.",
            category: "Métadonnées",
            subcategory: "Audiobooks",
            tags: vec!["audiobooks", "livres audio", "audible", "narrateurs"],
            license: "MIT",
            stars: "380",
            github_url: "https://github.com/djdembeck/Audnexus.bundle",
            zip_url: "https://github.com/djdembeck/Audnexus.bundle/archive/refs/heads/main.zip",
            bundle_name: "Audnexus.bundle",
            verified: true,
        },

        // ══ MÉTADONNÉES — Comics ═════════════════════════════
        StorePlugin {
            id: "personal-media",
            name: "Personal Media (Movies)",
            author: "Plex Inc.",
            description: "Agent officiel pour vidéos personnelles et home vidéo. Organise sans recherche en ligne. Utilise les noms de fichiers. Parfait pour films maison.",
            category: "Métadonnées",
            subcategory: "Comics",
            tags: vec!["personnel", "home vidéo", "local", "officiel"],
            license: "MIT",
            stars: "190",
            github_url: "https://github.com/plexinc-plugins/PlexMovie.bundle",
            zip_url: "https://github.com/plexinc-plugins/PlexMovie.bundle/archive/refs/heads/master.zip",
            bundle_name: "PlexMovie.bundle",
            verified: true,
        },

        // ══ SOUS-TITRES — Multi-langues ══════════════════════
        StorePlugin {
            id: "subzero",
            name: "Sub-Zero",
            author: "pannal",
            description: "Gestionnaire de sous-titres tout-en-un. Télécharge depuis OpenSubtitles, Subscene, Addic7ed, etc. Synchronisation et correction automatiques. Incontournable.",
            category: "Sous-titres",
            subcategory: "Multi-langues",
            tags: vec!["sous-titres", "opensubtitles", "automatique", "populaire"],
            license: "MIT",
            stars: "1.4k",
            github_url: "https://github.com/nicedoc/Sub-Zero.bundle",
            zip_url: "https://github.com/nicedoc/Sub-Zero.bundle/archive/refs/heads/master.zip",
            bundle_name: "Sub-Zero.bundle",
            verified: true,
        },
        StorePlugin {
            id: "opensubtitles",
            name: "OpenSubtitles",
            author: "nicedoc",
            description: "Agent sous-titres OpenSubtitles.org. Télécharge les sous-titres les mieux notés pour vos films et séries. 30+ langues supportées.",
            category: "Sous-titres",
            subcategory: "Multi-langues",
            tags: vec!["sous-titres", "opensubtitles", "multi-langue"],
            license: "MIT",
            stars: "320",
            github_url: "https://github.com/nicedoc/OpenSubtitles.bundle",
            zip_url: "https://github.com/nicedoc/OpenSubtitles.bundle/archive/refs/heads/master.zip",
            bundle_name: "OpenSubtitles.bundle",
            verified: true,
        },

        // ══ SOUS-TITRES — Français ═══════════════════════════
        StorePlugin {
            id: "betaseries",
            name: "BetaSeries Subtitles",
            author: "Kazamajinz",
            description: "Sous-titres et métadonnées depuis BetaSeries.com. Plateforme française de suivi de séries. Sous-titres francophones de qualité.",
            category: "Sous-titres",
            subcategory: "Français",
            tags: vec!["sous-titres", "français", "betaseries", "france"],
            license: "MIT",
            stars: "210",
            github_url: "https://github.com/Kazamajinz/BetaSeries.bundle",
            zip_url: "https://github.com/Kazamajinz/BetaSeries.bundle/archive/refs/heads/master.zip",
            bundle_name: "BetaSeries.bundle",
            verified: true,
        },

        // ══ OUTILS — Utilitaires ══════════════════════════════
        StorePlugin {
            id: "webtools",
            name: "WebTools",
            author: "ukdtom",
            description: "Suite d'outils Plex : gestionnaire de plugins graphique, nettoyage de bundles, synchronisation, statistiques. Interface web intégrée à Plex.",
            category: "Outils",
            subcategory: "Utilitaires",
            tags: vec!["outils", "gestion", "interface", "populaire"],
            license: "MIT",
            stars: "950",
            github_url: "https://github.com/ukdtom/WebTools.bundle",
            zip_url: "https://github.com/ukdtom/WebTools.bundle/archive/refs/heads/master.zip",
            bundle_name: "WebTools.bundle",
            verified: true,
        },
        StorePlugin {
            id: "cinemavision",
            name: "CinemaVision",
            author: "Veritas83",
            description: "Crée des expériences cinéma dans Plex. Ajoute intros, trailers, bumpers et countdowns avant tes films. Ambiance salle de cinéma chez toi.",
            category: "Outils",
            subcategory: "Utilitaires",
            tags: vec!["cinéma", "intro", "trailer", "expérience"],
            license: "MIT",
            stars: "490",
            github_url: "https://github.com/Veritas83/CinemaVision",
            zip_url: "https://github.com/Veritas83/CinemaVision/archive/refs/heads/master.zip",
            bundle_name: "CinemaVision.bundle",
            verified: true,
        },

        // ══ OUTILS — Scanners ════════════════════════════════
        StorePlugin {
            id: "absolute-series-scanner",
            name: "Absolute Series Scanner",
            author: "ZeroQI",
            description: "Scanner de séries TV avancé. Indispensable avec Hama pour l'anime. Gère les numérotations absolues et les arcs d'épisodes complexes.",
            category: "Outils",
            subcategory: "Scanners",
            tags: vec!["anime", "scanner", "séries", "populaire"],
            license: "MIT",
            stars: "850",
            github_url: "https://github.com/ZeroQI/Absolute-Series-Scanner",
            zip_url: "https://github.com/ZeroQI/Absolute-Series-Scanner/archive/refs/heads/master.zip",
            bundle_name: "Absolute-Series-Scanner",
            verified: true,
        },
        StorePlugin {
            id: "extended-personal-media",
            name: "Extended Personal Media",
            author: "bnewbold",
            description: "Agent pour enregistrements personnels et home vidéo. Organise et affiche les vidéos personnelles comme des émissions TV avec structure avancée.",
            category: "Outils",
            subcategory: "Scanners",
            tags: vec!["personnel", "home vidéo", "enregistrements"],
            license: "Apache-2.0",
            stars: "180",
            github_url: "https://github.com/bnewbold/ExtendedPersonalMedia-Agent.bundle",
            zip_url: "https://github.com/bnewbold/ExtendedPersonalMedia-Agent.bundle/archive/refs/heads/master.zip",
            bundle_name: "ExtendedPersonalMedia-Agent.bundle",
            verified: true,
        },

        // ══ OUTILS — IPTV ════════════════════════════════════
        StorePlugin {
            id: "iptv",
            name: "IPTV",
            author: "Cigaras",
            description: "Lecteur IPTV pour Plex. Lit des listes M3U de chaînes TV en direct. Supporte les EPG (guides des programmes électroniques). Compatible avec la plupart des fournisseurs.",
            category: "Outils",
            subcategory: "IPTV",
            tags: vec!["iptv", "tv live", "m3u", "epg", "streaming"],
            license: "LGPL-3.0",
            stars: "680",
            github_url: "https://github.com/Cigaras/IPTV.bundle",
            zip_url: "https://github.com/Cigaras/IPTV.bundle/archive/refs/heads/master.zip",
            bundle_name: "IPTV.bundle",
            verified: true,
        },

        // ══ OUTILS — Sync ════════════════════════════════════
        StorePlugin {
            id: "trakt-scrobbler",
            name: "Trakt Scrobbler",
            author: "trakt",
            description: "Synchronise automatiquement tes visionnages Plex avec Trakt.tv. Marque les films/épisodes vus, suit ta progression, génère des statistiques de visionnage.",
            category: "Outils",
            subcategory: "Sync",
            tags: vec!["trakt", "sync", "scrobble", "statistiques"],
            license: "MIT",
            stars: "520",
            github_url: "https://github.com/trakt/Plex-Trakt-Sync",
            zip_url: "https://github.com/trakt/Plex-Trakt-Sync/archive/refs/heads/master.zip",
            bundle_name: "Plex-Trakt-Sync.bundle",
            verified: true,
        },
    ]
}

// ─── Install ──────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct InstallResult {
    pub bundle_path: String,
    pub bundle_name: String,
    pub already_existed: bool,
}

pub async fn download_and_install(
    zip_url: &str,
    bundle_name: &str,
    plugins_dir: &PathBuf,
) -> Result<InstallResult> {
    // Cible finale : plugins_dir / bundle_name
    let target = plugins_dir.join(bundle_name);
    let already_existed = target.exists();

    // Backup si déjà installé
    if already_existed {
        crate::scanner::backup_plugin(&target).ok();
        std::fs::remove_dir_all(&target)?;
    }

    std::fs::create_dir_all(plugins_dir)?;

    // Télécharger le ZIP — fallback master→main si 404
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .user_agent("PlexMetaForge/1.0")
        .build()
        .map_err(PlexMetaForgeError::Http)?;

    let resp = client.get(zip_url).send().await.map_err(PlexMetaForgeError::Http)?;

    let (final_url, resp) = if resp.status() == reqwest::StatusCode::NOT_FOUND {
        // Essaie l'autre branche (master↔main)
        let alt = if zip_url.contains("/master.zip") {
            zip_url.replace("/master.zip", "/main.zip")
        } else if zip_url.contains("/main.zip") {
            zip_url.replace("/main.zip", "/master.zip")
        } else {
            zip_url.to_string()
        };

        if alt != zip_url {
            let r2 = client.get(&alt).send().await.map_err(PlexMetaForgeError::Http)?;
            (alt, r2)
        } else {
            (zip_url.to_string(), resp)
        }
    } else {
        (zip_url.to_string(), resp)
    };

    if !resp.status().is_success() {
        return Err(PlexMetaForgeError::PlexApi(format!(
            "Téléchargement échoué HTTP {} — {}\nEssayé aussi : {}",
            resp.status(),
            zip_url,
            final_url
        )));
    }

    let bytes = resp.bytes().await.map_err(PlexMetaForgeError::Http)?.to_vec();

    // Extraire le ZIP
    let cursor = std::io::Cursor::new(bytes);
    let mut archive = zip::ZipArchive::new(cursor)?;

    // Trouve le dossier racine dans le ZIP (ex: "Hama.bundle-master/")
    let zip_root = find_zip_root(&mut archive, bundle_name)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let raw_path = file.name().to_string();

        // Retire le préfixe racine du ZIP
        let relative = match raw_path.strip_prefix(&zip_root) {
            Some(r) => r,
            None => continue,
        };

        if relative.is_empty() {
            continue;
        }

        let dest = target.join(relative.replace('/', std::path::MAIN_SEPARATOR_STR));

        if raw_path.ends_with('/') {
            std::fs::create_dir_all(&dest)?;
        } else {
            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut data = Vec::new();
            file.read_to_end(&mut data)?;
            std::fs::write(&dest, data)?;
        }
    }

    Ok(InstallResult {
        bundle_path: target.to_string_lossy().to_string(),
        bundle_name: bundle_name.to_string(),
        already_existed,
    })
}

fn find_zip_root(archive: &mut zip::ZipArchive<std::io::Cursor<Vec<u8>>>, bundle_name: &str) -> Result<String> {
    let stem = bundle_name
        .trim_end_matches(".bundle")
        .to_lowercase()
        .replace(['-', '_'], "");

    // Collect all top-level directory entries (no '/' in name except trailing)
    let mut top_dirs: Vec<String> = Vec::new();
    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        let name = file.name().to_string();
        if name.ends_with('/') {
            // Top-level: no slash before the trailing slash
            let trimmed = name.trim_end_matches('/');
            if !trimmed.contains('/') {
                top_dirs.push(name.clone());
            }
        }
    }

    // Priority 1: exact stem match
    for dir in &top_dirs {
        let folder_norm = dir.trim_end_matches('/').to_lowercase().replace(['-', '_'], "");
        if folder_norm.contains(&stem) {
            return Ok(dir.clone());
        }
    }

    // Priority 2: GitHub archive suffix patterns (e.g. Hama.bundle-master/, repo-main/)
    for dir in &top_dirs {
        let lower = dir.to_lowercase();
        if lower.contains("-master/") || lower.contains("-main/") || lower.ends_with("-master/") {
            return Ok(dir.clone());
        }
    }

    // Fallback: first top-level dir
    if let Some(first) = top_dirs.into_iter().next() {
        return Ok(first);
    }

    Err(PlexMetaForgeError::PlexApi(
        "Impossible de trouver le dossier racine dans le ZIP".to_string()
    ))
}
