// Plugin Python templates — use __NAME__ placeholder replaced at call site

fn inject_name(template: &str, name: &str) -> String {
    template.replace("__NAME__", name)
}

// ─── Info.plist ───────────────────────────────────────────────

pub fn info_plist(id: &str, display_name: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleIdentifier</key>
    <string>com.plexapp.agents.{}</string>
    <key>CFBundleName</key>
    <string>{}</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
    <key>PlexClientPlatforms</key>
    <string>*</string>
    <key>PlexFrameworkVersion</key>
    <string>2</string>
    <key>PlexPluginClass</key>
    <string>Agent</string>
    <key>PlexPluginMode</key>
    <string>Resident</string>
</dict>
</plist>
"#,
        id.to_lowercase().replace(' ', "_"),
        display_name
    )
}

// ─── DefaultPrefs.json ────────────────────────────────────────

pub fn default_prefs_json(has_tmdb: bool, has_lastfm: bool) -> String {
    let mut prefs: Vec<&str> = Vec::new();
    let tmdb_pref = r##"    {
      "id": "tmdb_api_key",
      "type": "text",
      "label": "Clé API TMDB (themoviedb.org)",
      "default": "",
      "secure": false
    }"##;
    let lastfm_pref = r##"    {
      "id": "lastfm_api_key",
      "type": "text",
      "label": "Clé API Last.fm",
      "default": "",
      "secure": false
    }"##;
    if has_tmdb   { prefs.push(tmdb_pref); }
    if has_lastfm { prefs.push(lastfm_pref); }
    format!("{{\n  \"prefs\": [\n{}\n  ]\n}}", prefs.join(",\n"))
}

// ─── Blank ────────────────────────────────────────────────────

const BLANK_TEMPLATE: &str = r##"# ================================================================
# __NAME__ — Plugin Plex (Framework 2)
# Généré par PlexMetaForge — Version 1.0.0
# ================================================================

AGENT_VERSION = "1.0.0"

def Start():
    Log.Info("[__NAME__] v%s démarré" % AGENT_VERSION)

def ValidatePrefs():
    return MessageContainer("OK", "Prêt.")

class __NAME__(Agent.Movies):
    name             = "__NAME__"
    languages        = [Locale.Language.French, Locale.Language.English]
    primary_provider = True
    accepts_from     = ["com.plexapp.agents.localmedia"]

    def search(self, results, media, lang, manual):
        # TODO: implémenter la recherche
        results.Append(MetadataSearchResult(
            id    = media.name,
            name  = media.name,
            score = 80,
            lang  = lang
        ))

    def update(self, metadata, media, lang, force):
        # TODO: implémenter la mise à jour des métadonnées
        metadata.title = media.name
"##;

#[allow(dead_code)]
pub fn blank_init_py(name: &str) -> String { inject_name(BLANK_TEMPLATE, name) }

// ─── Cinema (Films — TMDB) ────────────────────────────────────

const CINEMA_TEMPLATE: &str = r##"# ================================================================
# __NAME__ — Agent Métadonnées Films pour Plex
# Source  : The Movie Database (TMDB)
# Version : 1.0.0  |  Généré par PlexMetaForge
# ================================================================
# CONFIGURATION :
#   Plex > Paramètres > Agents > Films > __NAME__
#   Clé API TMDB : https://www.themoviedb.org/settings/api
# ================================================================

AGENT_VERSION = "1.0.0"
TMDB_BASE     = "https://api.themoviedb.org/3"
TMDB_ORIG     = "https://image.tmdb.org/t/p/original"
TMDB_W500     = "https://image.tmdb.org/t/p/w500"

def Start():
    HTTP.CacheTime = CACHE_1HOUR
    Log.Info("[__NAME__] v%s démarré" % AGENT_VERSION)

def ValidatePrefs():
    if not Prefs["tmdb_api_key"]:
        return MessageContainer("Clé API manquante",
            "Configure ta clé TMDB dans Préférences > Agents > Films > __NAME__")
    return MessageContainer("OK", "Agent prêt.")

class __NAME__(Agent.Movies):
    name             = "__NAME__"
    languages        = [Locale.Language.French, Locale.Language.English,
                        Locale.Language.Spanish, Locale.Language.German,
                        Locale.Language.Japanese]
    primary_provider = True
    accepts_from     = ["com.plexapp.agents.localmedia"]
    contributes_to   = ["com.plexapp.agents.imdb"]

    def search(self, results, media, lang, manual):
        key   = Prefs["tmdb_api_key"]
        title = media.name
        year  = getattr(media, "year", None)
        qs    = "api_key=%s&query=%s&language=%s" % (key, String.Quote(title), lang)
        if year:
            qs += "&primary_release_year=%s" % year
        try:
            data = JSON.ObjectFromURL("%s/search/movie?%s" % (TMDB_BASE, qs), sleep=0.5)
            for i, r in enumerate(data.get("results", [])[:5]):
                ttl   = r.get("title", "")
                yr    = r.get("release_date", "")[:4]
                score = 100 if ttl.lower() == title.lower() else max(60, 90 - i * 8)
                results.Append(MetadataSearchResult(
                    id=str(r["id"]), name=ttl,
                    year=int(yr) if yr.isdigit() else None,
                    score=score, lang=lang))
        except Exception as e:
            Log.Error("[__NAME__] search: %s" % e)

    def update(self, metadata, media, lang, force):
        key = Prefs["tmdb_api_key"]
        url = "%s/movie/%s?api_key=%s&language=%s&append_to_response=credits,images,keywords" % (
            TMDB_BASE, metadata.id, key, lang)
        try:
            d = JSON.ObjectFromURL(url, sleep=0.5)
            metadata.title          = d.get("title", "")
            metadata.original_title = d.get("original_title", "")
            metadata.summary        = d.get("overview", "")
            metadata.rating         = float(d.get("vote_average", 0))
            metadata.tagline        = d.get("tagline", "")
            rt = int(d.get("runtime") or 0)
            if rt:
                metadata.duration = rt * 60000
            companies = d.get("production_companies", [])
            if companies:
                metadata.studio = companies[0].get("name", "")
            if d.get("release_date"):
                try:
                    from datetime import datetime
                    dt = datetime.strptime(d["release_date"], "%Y-%m-%d")
                    metadata.originally_available_at = dt.date()
                    metadata.year = dt.year
                except Exception: pass
            metadata.genres.clear()
            for g in d.get("genres", []):
                metadata.genres.add(g["name"])
            metadata.tags.clear()
            for kw in d.get("keywords", {}).get("keywords", [])[:10]:
                metadata.tags.add(kw["name"])
            metadata.roles.clear()
            for actor in d.get("credits", {}).get("cast", [])[:15]:
                role       = metadata.roles.new()
                role.name  = actor["name"]
                role.role  = actor.get("character", "")
                if actor.get("profile_path"):
                    role.photo = TMDB_ORIG + actor["profile_path"]
            metadata.directors.clear()
            metadata.writers.clear()
            for crew in d.get("credits", {}).get("crew", []):
                if crew["job"] == "Director":
                    metadata.directors.new().name = crew["name"]
                elif crew["job"] in ("Screenplay", "Writer", "Story"):
                    metadata.writers.new().name = crew["name"]
            metadata.posters.validate_keys([])
            for p in d.get("images", {}).get("posters", [])[:4]:
                pu = TMDB_ORIG + p["file_path"]
                metadata.posters[pu] = Proxy.Preview(
                    HTTP.Request(TMDB_W500 + p["file_path"], sleep=0).content, sort_order=1)
            metadata.art.validate_keys([])
            for a in d.get("images", {}).get("backdrops", [])[:4]:
                au = TMDB_ORIG + a["file_path"]
                metadata.art[au] = Proxy.Preview(
                    HTTP.Request(TMDB_W500 + a["file_path"], sleep=0).content, sort_order=1)
            Log.Info("[__NAME__] Mis à jour: %s (%s)" % (metadata.title, metadata.year))
        except Exception as e:
            Log.Error("[__NAME__] update: %s" % e)
"##;

pub fn cinema_init_py(name: &str) -> String { inject_name(CINEMA_TEMPLATE, name) }

// ─── Séries TV (TMDB) ─────────────────────────────────────────

const SERIES_TEMPLATE: &str = r##"# ================================================================
# __NAME__ — Agent Métadonnées Séries TV pour Plex
# Source  : The Movie Database (TMDB)
# Version : 1.0.0  |  Généré par PlexMetaForge
# ================================================================
# CONFIGURATION :
#   Plex > Paramètres > Agents > Émissions TV > __NAME__
#   Clé API TMDB : https://www.themoviedb.org/settings/api
# ================================================================

AGENT_VERSION = "1.0.0"
TMDB_BASE     = "https://api.themoviedb.org/3"
TMDB_ORIG     = "https://image.tmdb.org/t/p/original"
TMDB_W500     = "https://image.tmdb.org/t/p/w500"

def Start():
    HTTP.CacheTime = CACHE_1HOUR
    Log.Info("[__NAME__] v%s démarré" % AGENT_VERSION)

def ValidatePrefs():
    if not Prefs["tmdb_api_key"]:
        return MessageContainer("Clé API manquante", "Configure ta clé TMDB.")
    return MessageContainer("OK", "Agent prêt.")

class __NAME__(Agent.TV_Shows):
    name             = "__NAME__"
    languages        = [Locale.Language.French, Locale.Language.English,
                        Locale.Language.Spanish, Locale.Language.German]
    primary_provider = True
    accepts_from     = ["com.plexapp.agents.localmedia"]

    def search(self, results, media, lang, manual):
        key   = Prefs["tmdb_api_key"]
        title = media.show
        try:
            data = JSON.ObjectFromURL(
                "%s/search/tv?api_key=%s&query=%s&language=%s" % (
                    TMDB_BASE, key, String.Quote(title), lang), sleep=0.5)
            for i, r in enumerate(data.get("results", [])[:5]):
                ttl   = r.get("name", "")
                yr    = r.get("first_air_date", "")[:4]
                score = 100 if ttl.lower() == title.lower() else max(60, 90 - i * 8)
                results.Append(MetadataSearchResult(
                    id=str(r["id"]), name=ttl,
                    year=int(yr) if yr.isdigit() else None,
                    score=score, lang=lang))
        except Exception as e:
            Log.Error("[__NAME__] search: %s" % e)

    def update(self, metadata, media, lang, force):
        key = Prefs["tmdb_api_key"]
        try:
            show = JSON.ObjectFromURL(
                "%s/tv/%s?api_key=%s&language=%s&append_to_response=credits,images" % (
                    TMDB_BASE, metadata.id, key, lang), sleep=0.5)
            metadata.title   = show.get("name", "")
            metadata.summary = show.get("overview", "")
            metadata.rating  = float(show.get("vote_average", 0))
            if show.get("first_air_date"):
                try:
                    from datetime import datetime
                    dt = datetime.strptime(show["first_air_date"], "%Y-%m-%d")
                    metadata.originally_available_at = dt.date()
                    metadata.year = dt.year
                except Exception: pass
            metadata.genres.clear()
            for g in show.get("genres", []):
                metadata.genres.add(g["name"])
            networks = show.get("networks", [])
            if networks:
                metadata.studio = networks[0].get("name", "")
            metadata.roles.clear()
            for actor in show.get("credits", {}).get("cast", [])[:15]:
                role       = metadata.roles.new()
                role.name  = actor["name"]
                role.role  = actor.get("character", "")
                if actor.get("profile_path"):
                    role.photo = TMDB_ORIG + actor["profile_path"]
            metadata.directors.clear()
            for creator in show.get("created_by", []):
                metadata.directors.new().name = creator["name"]
            metadata.posters.validate_keys([])
            for p in show.get("images", {}).get("posters", [])[:4]:
                pu = TMDB_ORIG + p["file_path"]
                metadata.posters[pu] = Proxy.Preview(
                    HTTP.Request(TMDB_W500 + p["file_path"], sleep=0).content, sort_order=1)
            metadata.art.validate_keys([])
            for a in show.get("images", {}).get("backdrops", [])[:4]:
                au = TMDB_ORIG + a["file_path"]
                metadata.art[au] = Proxy.Preview(
                    HTTP.Request(TMDB_W500 + a["file_path"], sleep=0).content, sort_order=1)
            # Saisons + épisodes
            TMDB_BASE_L = TMDB_BASE
            for season_num in media.seasons:
                try:
                    sd = JSON.ObjectFromURL(
                        "%s/tv/%s/season/%s?api_key=%s&language=%s" % (
                            TMDB_BASE_L, metadata.id, season_num, key, lang), sleep=0.5)
                    season = metadata.seasons[season_num]
                    season.summary = sd.get("overview", "")
                    if sd.get("poster_path"):
                        sp = TMDB_ORIG + sd["poster_path"]
                        season.posters[sp] = Proxy.Preview(
                            HTTP.Request(TMDB_W500 + sd["poster_path"], sleep=0).content)
                    for ep in sd.get("episodes", []):
                        en = ep.get("episode_number")
                        if en and en in season.episodes:
                            e_obj = season.episodes[en]
                            e_obj.title   = ep.get("name", "")
                            e_obj.summary = ep.get("overview", "")
                            e_obj.rating  = float(ep.get("vote_average", 0))
                            if ep.get("air_date"):
                                try:
                                    from datetime import datetime
                                    e_obj.originally_available_at = datetime.strptime(
                                        ep["air_date"], "%Y-%m-%d").date()
                                except Exception: pass
                            if ep.get("still_path"):
                                tu = TMDB_ORIG + ep["still_path"]
                                e_obj.thumbs[tu] = Proxy.Preview(
                                    HTTP.Request(TMDB_W500 + ep["still_path"], sleep=0).content)
                except Exception as se:
                    Log.Warn("[__NAME__] Saison %s: %s" % (season_num, se))
            Log.Info("[__NAME__] Série mise à jour: %s" % metadata.title)
        except Exception as e:
            Log.Error("[__NAME__] update: %s" % e)
"##;

pub fn series_init_py(name: &str) -> String { inject_name(SERIES_TEMPLATE, name) }

// ─── Musique (Last.fm + MusicBrainz) ─────────────────────────

const MUSIQUE_TEMPLATE: &str = r##"# ================================================================
# __NAME__ — Agent Métadonnées Musique pour Plex
# Source  : Last.fm + MusicBrainz
# Version : 1.0.0  |  Généré par PlexMetaForge
# ================================================================
# CONFIGURATION :
#   Plex > Paramètres > Agents > Artistes / Albums > __NAME__
#   Clé API Last.fm : https://www.last.fm/api/account/create
# ================================================================

AGENT_VERSION = "1.0.0"
LASTFM_BASE   = "https://ws.audioscrobbler.com/2.0"

def Start():
    HTTP.CacheTime = CACHE_1WEEK
    Log.Info("[__NAME__] v%s démarré" % AGENT_VERSION)

def ValidatePrefs():
    if not Prefs["lastfm_api_key"]:
        return MessageContainer("Clé API manquante", "Configure ta clé Last.fm.")
    return MessageContainer("OK", "Agent prêt.")

# ── Artiste ───────────────────────────────────────────────────

class __NAME__Artist(Agent.Artist):
    name             = "__NAME__ (Artiste)"
    languages        = [Locale.Language.French, Locale.Language.English]
    primary_provider = True
    accepts_from     = ["com.plexapp.agents.localmedia"]

    def search(self, results, media, lang, manual):
        key    = Prefs["lastfm_api_key"]
        artist = media.artist
        try:
            data = JSON.ObjectFromURL(
                "%s/?method=artist.search&artist=%s&api_key=%s&format=json" % (
                    LASTFM_BASE, String.Quote(artist), key), sleep=0.5)
            matches = data.get("results", {}).get("artistmatches", {}).get("artist", [])
            for i, a in enumerate(matches[:5]):
                results.Append(MetadataSearchResult(
                    id=a.get("mbid", a["name"]), name=a["name"],
                    score=max(60, 100 - i * 10), lang=lang))
        except Exception as e:
            Log.Error("[__NAME__Artist] search: %s" % e)

    def update(self, metadata, media, lang, force):
        key    = Prefs["lastfm_api_key"]
        artist = media.artist
        try:
            data = JSON.ObjectFromURL(
                "%s/?method=artist.getinfo&artist=%s&api_key=%s&lang=%s&format=json" % (
                    LASTFM_BASE, String.Quote(artist), key, lang[:2]), sleep=0.5)
            info = data.get("artist", {})
            metadata.title   = info.get("name", artist)
            bio = (info.get("bio", {}).get("summary", "") or "")
            metadata.summary = bio.split("<a href")[0].strip()
            metadata.genres.clear()
            for tag in info.get("tags", {}).get("tag", [])[:5]:
                metadata.genres.add(tag["name"])
            metadata.similar.clear()
            for sim in info.get("similar", {}).get("artist", [])[:8]:
                metadata.similar.add(sim["name"])
            for img in reversed(info.get("image", [])):
                url = img.get("#text", "")
                if url:
                    metadata.posters[url] = Proxy.Preview(
                        HTTP.Request(url, sleep=0).content, sort_order=1)
                    break
            Log.Info("[__NAME__Artist] Mis à jour: %s" % metadata.title)
        except Exception as e:
            Log.Error("[__NAME__Artist] update: %s" % e)

# ── Album ─────────────────────────────────────────────────────

class __NAME__Album(Agent.Album):
    name             = "__NAME__ (Album)"
    languages        = [Locale.Language.French, Locale.Language.English]
    primary_provider = True
    accepts_from     = ["com.plexapp.agents.localmedia"]

    def search(self, results, media, lang, manual):
        key   = Prefs["lastfm_api_key"]
        album = media.album
        try:
            data = JSON.ObjectFromURL(
                "%s/?method=album.search&album=%s&api_key=%s&format=json" % (
                    LASTFM_BASE, String.Quote(album), key), sleep=0.5)
            matches = data.get("results", {}).get("albummatches", {}).get("album", [])
            for i, a in enumerate(matches[:5]):
                score = 100 if a["name"].lower() == album.lower() else max(60, 90 - i * 10)
                results.Append(MetadataSearchResult(
                    id=a.get("mbid", a["name"]), name=a["name"], score=score, lang=lang))
        except Exception as e:
            Log.Error("[__NAME__Album] search: %s" % e)

    def update(self, metadata, media, lang, force):
        key    = Prefs["lastfm_api_key"]
        album  = media.album
        artist = media.artist
        try:
            data = JSON.ObjectFromURL(
                "%s/?method=album.getinfo&album=%s&artist=%s&api_key=%s&lang=%s&format=json" % (
                    LASTFM_BASE, String.Quote(album), String.Quote(artist), key, lang[:2]),
                sleep=0.5)
            info = data.get("album", {})
            metadata.title   = info.get("name", album)
            wiki = (info.get("wiki", {}).get("summary", "") or "")
            metadata.summary = wiki.split("<a href")[0].strip()
            metadata.genres.clear()
            for tag in info.get("tags", {}).get("tag", [])[:5]:
                metadata.genres.add(tag["name"])
            for img in reversed(info.get("image", [])):
                url = img.get("#text", "")
                if url:
                    metadata.posters[url] = Proxy.Preview(
                        HTTP.Request(url, sleep=0).content, sort_order=1)
                    break
            for track in info.get("tracks", {}).get("track", []):
                t_num = track.get("@attr", {}).get("rank")
                if t_num and int(t_num) in media.tracks:
                    t = metadata.tracks[int(t_num)]
                    t.title    = track.get("name", "")
                    t.duration = int(float(track.get("duration", 0))) * 1000
            Log.Info("[__NAME__Album] Mis à jour: %s — %s" % (artist, metadata.title))
        except Exception as e:
            Log.Error("[__NAME__Album] update: %s" % e)
"##;

pub fn musique_init_py(name: &str) -> String { inject_name(MUSIQUE_TEMPLATE, name) }

// ─── Anime / Manga (AniList — sans clé API) ───────────────────

const ANIME_TEMPLATE: &str = r##"# ================================================================
# __NAME__ — Agent Anime, Manga & Dessin Animé pour Plex
# Source  : AniList GraphQL API (gratuit, sans clé API)
# Version : 1.0.0  |  Généré par PlexMetaForge
# ================================================================
# Compatible : Anime, Manga, Manhwa, Manhua, Dessin animé
# Titres supportés : Français, Anglais, Romaji, Japonais
# AUCUNE CLÉ API REQUISE
# ================================================================

import json as _json

AGENT_VERSION = "1.0.0"
ANILIST_URL   = "https://graphql.anilist.co"

SEARCH_QUERY = (
    "query ($search: String, $type: MediaType) {"
    "  Page(perPage: 5) {"
    "    media(search: $search, type: $type, sort: SEARCH_MATCH) {"
    "      id title { romaji english native french }"
    "      startDate { year }"
    "      coverImage { extraLarge large }"
    "      episodes chapters averageScore genres"
    "    }"
    "  }"
    "}"
)

DETAIL_QUERY = (
    "query ($id: Int) {"
    "  Media(id: $id) {"
    "    id title { romaji english native french }"
    "    description(asHtml: false)"
    "    startDate { year month day }"
    "    episodes chapters averageScore"
    "    genres tags { name }"
    "    studios { nodes { name isAnimationStudio } }"
    "    coverImage { extraLarge large }"
    "    bannerImage"
    "    characters(sort: ROLE, perPage: 20) {"
    "      edges {"
    "        role"
    "        node { name { full native } image { large } }"
    "        voiceActors(language: JAPANESE) { name { full } image { large } }"
    "      }"
    "    }"
    "    staff(sort: RELEVANCE, perPage: 10) {"
    "      edges { role node { name { full } } }"
    "    }"
    "  }"
    "}"
)

def Start():
    HTTP.CacheTime = CACHE_1DAY
    Log.Info("[__NAME__] v%s démarré" % AGENT_VERSION)

def _anilist(query, variables):
    body = _json.dumps({"query": query, "variables": variables})
    resp = HTTP.Request(ANILIST_URL, method="POST", data=body,
                        headers={"Content-Type": "application/json"}, sleep=0.5)
    return _json.loads(resp.content)

def _title(t, lang):
    l = lang[:2] if lang else "en"
    if l == "fr" and t.get("french"):  return t["french"]
    if l == "en" and t.get("english"): return t["english"]
    return t.get("english") or t.get("romaji") or t.get("native", "")

class __NAME__(Agent.TV_Shows):
    name             = "__NAME__"
    languages        = [Locale.Language.French, Locale.Language.English,
                        Locale.Language.Japanese, Locale.Language.NoLanguage]
    primary_provider = True
    accepts_from     = ["com.plexapp.agents.localmedia"]

    def search(self, results, media, lang, manual):
        title = media.show
        for mtype in ("ANIME", "MANGA"):
            try:
                resp  = _anilist(SEARCH_QUERY, {"search": title, "type": mtype})
                items = resp.get("data", {}).get("Page", {}).get("media", [])
                for i, item in enumerate(items):
                    ttl = _title(item.get("title", {}), lang)
                    yr  = (item.get("startDate") or {}).get("year")
                    results.Append(MetadataSearchResult(
                        id="ani_%s_%s" % (mtype.lower(), item["id"]),
                        name=ttl, year=yr, score=max(60, 95 - i * 8), lang=lang))
            except Exception as e:
                Log.Warn("[__NAME__] search (%s): %s" % (mtype, e))

    def update(self, metadata, media, lang, force):
        raw = metadata.id.split("_")
        if len(raw) < 3 or not raw[-1].isdigit():
            Log.Error("[__NAME__] ID invalide: %s" % metadata.id)
            return
        ani_id = int(raw[-1])
        try:
            resp = _anilist(DETAIL_QUERY, {"id": ani_id})
            d    = resp.get("data", {}).get("Media", {})
            if not d: return

            t_obj = d.get("title", {})
            metadata.title   = _title(t_obj, lang)
            if t_obj.get("native"):
                metadata.original_title = t_obj["native"]
            desc = d.get("description", "") or ""
            metadata.summary = desc.replace("<br>", "\n").replace("<i>","").replace("</i>","")
            sc = d.get("averageScore")
            if sc:
                metadata.rating = float(sc) / 10.0
            sd = d.get("startDate") or {}
            if sd.get("year"):
                try:
                    from datetime import date as _date
                    metadata.originally_available_at = _date(
                        sd["year"], sd.get("month") or 1, sd.get("day") or 1)
                    metadata.year = sd["year"]
                except Exception: metadata.year = sd["year"]

            metadata.genres.clear()
            for g in d.get("genres", []): metadata.genres.add(g)
            metadata.tags.clear()
            for tag in d.get("tags", [])[:8]: metadata.tags.add(tag["name"])

            for studio in d.get("studios", {}).get("nodes", []):
                if studio.get("isAnimationStudio"):
                    metadata.studio = studio["name"]; break

            metadata.roles.clear()
            for edge in d.get("characters", {}).get("edges", [])[:15]:
                node = edge.get("node", {})
                char = metadata.roles.new()
                char.name = node.get("name", {}).get("full", "")
                vas = edge.get("voiceActors", [])
                if vas: char.role = "VA: " + (vas[0].get("name", {}).get("full", ""))
                img = node.get("image", {}).get("large", "")
                if img: char.photo = img

            metadata.directors.clear()
            for edge in d.get("staff", {}).get("edges", []):
                if edge.get("role") in ("Director", "Series Director"):
                    metadata.directors.new().name = edge["node"]["name"]["full"]

            metadata.posters.validate_keys([])
            cover = d.get("coverImage", {})
            for k in ("extraLarge", "large"):
                url = cover.get(k, "")
                if url:
                    metadata.posters[url] = Proxy.Preview(
                        HTTP.Request(url, sleep=0).content, sort_order=1)
                    break

            metadata.art.validate_keys([])
            banner = d.get("bannerImage", "")
            if banner:
                metadata.art[banner] = Proxy.Preview(
                    HTTP.Request(banner, sleep=0).content, sort_order=1)

            Log.Info("[__NAME__] Mis à jour: %s (%s)" % (metadata.title, metadata.year))
        except Exception as e:
            Log.Error("[__NAME__] update: %s" % e)
"##;

pub fn anime_init_py(name: &str) -> String { inject_name(ANIME_TEMPLATE, name) }

// ─── Universel (Films + Séries + Anime + Musique) ─────────────

const UNIVERSAL_TEMPLATE: &str = r##"# ================================================================
# __NAME__ — Agent Universel Tous Médias pour Plex
# Sources : TMDB (films/séries) + AniList (anime/manga) + Last.fm (musique)
# Version : 1.0.0  |  Généré par PlexMetaForge
# ================================================================
# Détection automatique : Films → TMDB | Séries → TMDB
#                         Anime/Manga → AniList (sans clé)
#                         Musique → Last.fm (clé optionnelle)
# ================================================================

import json as _json

AGENT_VERSION = "1.0.0"
TMDB_BASE     = "https://api.themoviedb.org/3"
TMDB_ORIG     = "https://image.tmdb.org/t/p/original"
TMDB_W500     = "https://image.tmdb.org/t/p/w500"
ANILIST_URL   = "https://graphql.anilist.co"
LASTFM_BASE   = "https://ws.audioscrobbler.com/2.0"

ANIME_KEYWORDS = [
    "anime","manga","ova","ona","manhwa","manhua","light novel",
    "sakura","dragon ball","naruto","one piece","bleach","attack on titan",
    "demon slayer","sword art online","spirited away","ghibli","pokemon",
    "digimon","saint seiya","evangelion","death note","fullmetal","my hero"
]

ANILIST_QUERY = (
    "query ($search: String, $type: MediaType) {"
    "  Page(perPage: 5) {"
    "    media(search: $search, type: $type, sort: SEARCH_MATCH) {"
    "      id title { romaji english french native }"
    "      startDate { year } coverImage { extraLarge large }"
    "      episodes averageScore genres description(asHtml: false)"
    "      studios { nodes { name isAnimationStudio } }"
    "      characters(sort: ROLE, perPage: 12) {"
    "        edges { node { name { full } image { large } } role }"
    "      }"
    "      staff(sort: RELEVANCE, perPage: 5) {"
    "        edges { role node { name { full } } }"
    "      }"
    "    }"
    "  }"
    "}"
)

def Start():
    HTTP.CacheTime = CACHE_1HOUR
    Log.Info("[__NAME__] v%s démarré — Agent Universel" % AGENT_VERSION)

def ValidatePrefs():
    if not Prefs["tmdb_api_key"]:
        return MessageContainer("Clé TMDB manquante", "Configure ta clé TMDB.")
    return MessageContainer("OK", "Agent Universel prêt.")

def _is_anime(title):
    tl = title.lower()
    return any(kw in tl for kw in ANIME_KEYWORDS)

def _anilist(query, variables):
    body = _json.dumps({"query": query, "variables": variables})
    resp = HTTP.Request(ANILIST_URL, method="POST", data=body,
                        headers={"Content-Type": "application/json"}, sleep=0.5)
    return _json.loads(resp.content)

def _ani_title(t, lang):
    l = lang[:2] if lang else "en"
    if l == "fr" and t.get("french"):  return t["french"]
    if l == "en" and t.get("english"): return t["english"]
    return t.get("english") or t.get("romaji") or t.get("native", "")

def _apply_tmdb_movie(metadata, d, lang):
    TMDB_BASE_L = TMDB_BASE
    metadata.title          = d.get("title", "")
    metadata.original_title = d.get("original_title", "")
    metadata.summary        = d.get("overview", "")
    metadata.rating         = float(d.get("vote_average", 0))
    metadata.tagline        = d.get("tagline", "")
    rt = int(d.get("runtime") or 0)
    if rt: metadata.duration = rt * 60000
    if d.get("production_companies"):
        metadata.studio = d["production_companies"][0].get("name", "")
    if d.get("release_date"):
        try:
            from datetime import datetime
            dt = datetime.strptime(d["release_date"], "%Y-%m-%d")
            metadata.originally_available_at = dt.date()
            metadata.year = dt.year
        except Exception: pass
    metadata.genres.clear()
    for g in d.get("genres", []): metadata.genres.add(g["name"])
    metadata.roles.clear()
    for actor in d.get("credits", {}).get("cast", [])[:12]:
        r = metadata.roles.new(); r.name = actor["name"]
        r.role = actor.get("character", "")
        if actor.get("profile_path"): r.photo = TMDB_ORIG + actor["profile_path"]
    metadata.directors.clear()
    for c in d.get("credits", {}).get("crew", []):
        if c["job"] == "Director": metadata.directors.new().name = c["name"]
    metadata.posters.validate_keys([])
    for p in d.get("images", {}).get("posters", [])[:3]:
        pu = TMDB_ORIG + p["file_path"]
        metadata.posters[pu] = Proxy.Preview(
            HTTP.Request(TMDB_W500 + p["file_path"], sleep=0).content, sort_order=1)
    metadata.art.validate_keys([])
    for a in d.get("images", {}).get("backdrops", [])[:3]:
        au = TMDB_ORIG + a["file_path"]
        metadata.art[au] = Proxy.Preview(
            HTTP.Request(TMDB_W500 + a["file_path"], sleep=0).content, sort_order=1)

def _apply_anilist(metadata, item, lang):
    t_obj = item.get("title", {})
    metadata.title   = _ani_title(t_obj, lang)
    desc = item.get("description", "") or ""
    metadata.summary = desc.replace("<br>","").replace("<i>","").replace("</i>","")
    sc = item.get("averageScore")
    if sc: metadata.rating = float(sc) / 10.0
    sd = item.get("startDate") or {}
    if sd.get("year"): metadata.year = sd["year"]
    metadata.genres.clear()
    for g in item.get("genres", []): metadata.genres.add(g)
    for studio in item.get("studios", {}).get("nodes", []):
        if studio.get("isAnimationStudio"):
            metadata.studio = studio["name"]; break
    metadata.roles.clear()
    for edge in item.get("characters", {}).get("edges", [])[:12]:
        node = edge.get("node", {})
        r = metadata.roles.new()
        r.name = node.get("name", {}).get("full", "")
        r.role = edge.get("role", "")
        img = node.get("image", {}).get("large", "")
        if img: r.photo = img
    metadata.directors.clear()
    for edge in item.get("staff", {}).get("edges", []):
        if edge.get("role") in ("Director", "Series Director"):
            metadata.directors.new().name = edge["node"]["name"]["full"]
    metadata.posters.validate_keys([])
    cover = item.get("coverImage", {})
    for k in ("extraLarge", "large"):
        url = cover.get(k, "")
        if url:
            metadata.posters[url] = Proxy.Preview(
                HTTP.Request(url, sleep=0).content, sort_order=1)
            break

# ── Films ─────────────────────────────────────────────────────

class __NAME__Movies(Agent.Movies):
    name             = "__NAME__ — Films & Anime Films"
    languages        = [Locale.Language.French, Locale.Language.English,
                        Locale.Language.Japanese, Locale.Language.NoLanguage]
    primary_provider = True
    accepts_from     = ["com.plexapp.agents.localmedia"]

    def search(self, results, media, lang, manual):
        title = media.name
        year  = getattr(media, "year", None)
        if _is_anime(title):
            try:
                resp  = _anilist(ANILIST_QUERY, {"search": title, "type": "ANIME"})
                items = resp.get("data", {}).get("Page", {}).get("media", [])
                for i, item in enumerate(items):
                    ttl = _ani_title(item.get("title", {}), lang)
                    yr  = (item.get("startDate") or {}).get("year")
                    results.Append(MetadataSearchResult(
                        id="ani_%s" % item["id"], name=ttl, year=yr,
                        score=max(60, 95 - i * 8), lang=lang))
                if results: return
            except Exception as e:
                Log.Warn("[__NAME__Movies] AniList: %s" % e)
        key = Prefs["tmdb_api_key"]
        qs  = "api_key=%s&query=%s&language=%s" % (key, String.Quote(title), lang)
        if year: qs += "&primary_release_year=%s" % year
        try:
            data = JSON.ObjectFromURL("%s/search/movie?%s" % (TMDB_BASE, qs), sleep=0.5)
            for i, r in enumerate(data.get("results", [])[:5]):
                ttl   = r.get("title", "")
                yr    = r.get("release_date", "")[:4]
                score = 100 if ttl.lower() == title.lower() else max(60, 90 - i * 8)
                results.Append(MetadataSearchResult(
                    id=str(r["id"]), name=ttl,
                    year=int(yr) if yr.isdigit() else None,
                    score=score, lang=lang))
        except Exception as e:
            Log.Error("[__NAME__Movies] TMDB: %s" % e)

    def update(self, metadata, media, lang, force):
        mid = metadata.id
        if mid.startswith("ani_"):
            ani_id = int(mid.replace("ani_", ""))
            try:
                resp  = _anilist(ANILIST_QUERY, {"search": metadata.title, "type": "ANIME"})
                items = resp.get("data", {}).get("Page", {}).get("media", [])
                target = next((x for x in items if x["id"] == ani_id), None)
                if not target and items: target = items[0]
                if target: _apply_anilist(metadata, target, lang)
            except Exception as e:
                Log.Error("[__NAME__Movies] AniList update: %s" % e)
            return
        key = Prefs["tmdb_api_key"]
        try:
            d = JSON.ObjectFromURL(
                "%s/movie/%s?api_key=%s&language=%s&append_to_response=credits,images" % (
                    TMDB_BASE, mid, key, lang), sleep=0.5)
            _apply_tmdb_movie(metadata, d, lang)
            Log.Info("[__NAME__Movies] Mis à jour: %s" % metadata.title)
        except Exception as e:
            Log.Error("[__NAME__Movies] TMDB update: %s" % e)

# ── Séries & Anime ────────────────────────────────────────────

class __NAME__Shows(Agent.TV_Shows):
    name             = "__NAME__ — Séries, Anime & Manga"
    languages        = [Locale.Language.French, Locale.Language.English,
                        Locale.Language.Japanese, Locale.Language.NoLanguage]
    primary_provider = True
    accepts_from     = ["com.plexapp.agents.localmedia"]

    def search(self, results, media, lang, manual):
        title = media.show
        if _is_anime(title):
            for mtype in ("ANIME", "MANGA"):
                try:
                    resp  = _anilist(ANILIST_QUERY, {"search": title, "type": mtype})
                    items = resp.get("data", {}).get("Page", {}).get("media", [])
                    for i, item in enumerate(items):
                        ttl = _ani_title(item.get("title", {}), lang)
                        yr  = (item.get("startDate") or {}).get("year")
                        results.Append(MetadataSearchResult(
                            id="ani_%s_%s" % (mtype.lower(), item["id"]),
                            name=ttl, year=yr, score=max(60, 95 - i * 8), lang=lang))
                except Exception as e:
                    Log.Warn("[__NAME__Shows] AniList (%s): %s" % (mtype, e))
            return
        key = Prefs["tmdb_api_key"]
        try:
            data = JSON.ObjectFromURL(
                "%s/search/tv?api_key=%s&query=%s&language=%s" % (
                    TMDB_BASE, key, String.Quote(title), lang), sleep=0.5)
            for i, r in enumerate(data.get("results", [])[:5]):
                ttl   = r.get("name", "")
                yr    = r.get("first_air_date", "")[:4]
                score = 100 if ttl.lower() == title.lower() else max(60, 90 - i * 8)
                results.Append(MetadataSearchResult(
                    id=str(r["id"]), name=ttl,
                    year=int(yr) if yr.isdigit() else None,
                    score=score, lang=lang))
        except Exception as e:
            Log.Error("[__NAME__Shows] TMDB: %s" % e)

    def update(self, metadata, media, lang, force):
        mid = metadata.id
        if mid.startswith("ani_"):
            ani_id = int(mid.split("_")[-1])
            try:
                resp  = _anilist(ANILIST_QUERY, {"search": metadata.title, "type": "ANIME"})
                items = resp.get("data", {}).get("Page", {}).get("media", [])
                target = next((x for x in items if x["id"] == ani_id), None)
                if not target and items: target = items[0]
                if target: _apply_anilist(metadata, target, lang)
                Log.Info("[__NAME__Shows] Anime: %s" % metadata.title)
            except Exception as e:
                Log.Error("[__NAME__Shows] AniList update: %s" % e)
            return
        key = Prefs["tmdb_api_key"]
        try:
            d = JSON.ObjectFromURL(
                "%s/tv/%s?api_key=%s&language=%s&append_to_response=credits,images" % (
                    TMDB_BASE, mid, key, lang), sleep=0.5)
            metadata.title   = d.get("name", "")
            metadata.summary = d.get("overview", "")
            metadata.rating  = float(d.get("vote_average", 0))
            if d.get("first_air_date"):
                try:
                    from datetime import datetime
                    dt = datetime.strptime(d["first_air_date"], "%Y-%m-%d")
                    metadata.originally_available_at = dt.date()
                    metadata.year = dt.year
                except Exception: pass
            metadata.genres.clear()
            for g in d.get("genres", []): metadata.genres.add(g["name"])
            if d.get("networks"): metadata.studio = d["networks"][0].get("name","")
            metadata.roles.clear()
            for actor in d.get("credits", {}).get("cast", [])[:12]:
                r = metadata.roles.new(); r.name = actor["name"]
                r.role = actor.get("character","")
                if actor.get("profile_path"): r.photo = TMDB_ORIG + actor["profile_path"]
            metadata.directors.clear()
            for c in d.get("created_by", []):
                metadata.directors.new().name = c["name"]
            metadata.posters.validate_keys([])
            for p in d.get("images", {}).get("posters", [])[:3]:
                pu = TMDB_ORIG + p["file_path"]
                metadata.posters[pu] = Proxy.Preview(
                    HTTP.Request(TMDB_W500 + p["file_path"], sleep=0).content, sort_order=1)
            metadata.art.validate_keys([])
            for a in d.get("images", {}).get("backdrops", [])[:3]:
                au = TMDB_ORIG + a["file_path"]
                metadata.art[au] = Proxy.Preview(
                    HTTP.Request(TMDB_W500 + a["file_path"], sleep=0).content, sort_order=1)
            Log.Info("[__NAME__Shows] Série: %s" % metadata.title)
        except Exception as e:
            Log.Error("[__NAME__Shows] TMDB update: %s" % e)

# ── Musique ───────────────────────────────────────────────────

class __NAME__Artist(Agent.Artist):
    name             = "__NAME__ — Artiste"
    languages        = [Locale.Language.French, Locale.Language.English]
    primary_provider = True
    accepts_from     = ["com.plexapp.agents.localmedia"]

    def search(self, results, media, lang, manual):
        key = Prefs.get("lastfm_api_key", "")
        if not key:
            results.Append(MetadataSearchResult(
                id=media.artist, name=media.artist, score=80, lang=lang)); return
        try:
            data = JSON.ObjectFromURL(
                "%s/?method=artist.search&artist=%s&api_key=%s&format=json" % (
                    LASTFM_BASE, String.Quote(media.artist), key), sleep=0.5)
            for i, a in enumerate(data.get("results",{}).get("artistmatches",{}).get("artist",[])[:5]):
                results.Append(MetadataSearchResult(
                    id=a.get("mbid",a["name"]), name=a["name"],
                    score=max(60,100-i*10), lang=lang))
        except Exception as e:
            Log.Error("[__NAME__Artist] search: %s" % e)

    def update(self, metadata, media, lang, force):
        key = Prefs.get("lastfm_api_key", "")
        if not key: return
        try:
            data = JSON.ObjectFromURL(
                "%s/?method=artist.getinfo&artist=%s&api_key=%s&lang=%s&format=json" % (
                    LASTFM_BASE, String.Quote(media.artist), key, lang[:2]), sleep=0.5)
            info = data.get("artist", {})
            metadata.title = info.get("name", media.artist)
            bio = (info.get("bio", {}).get("summary","") or "")
            metadata.summary = bio.split("<a href")[0].strip()
            metadata.genres.clear()
            for tag in info.get("tags",{}).get("tag",[])[:5]:
                metadata.genres.add(tag["name"])
            for img in reversed(info.get("image",[])):
                url = img.get("#text","")
                if url:
                    metadata.posters[url] = Proxy.Preview(
                        HTTP.Request(url,sleep=0).content, sort_order=1); break
        except Exception as e:
            Log.Error("[__NAME__Artist] update: %s" % e)

class __NAME__Album(Agent.Album):
    name             = "__NAME__ — Album"
    languages        = [Locale.Language.French, Locale.Language.English]
    primary_provider = True
    accepts_from     = ["com.plexapp.agents.localmedia"]

    def search(self, results, media, lang, manual):
        key = Prefs.get("lastfm_api_key", "")
        if not key:
            results.Append(MetadataSearchResult(
                id=media.album, name=media.album, score=80, lang=lang)); return
        try:
            data = JSON.ObjectFromURL(
                "%s/?method=album.search&album=%s&api_key=%s&format=json" % (
                    LASTFM_BASE, String.Quote(media.album), key), sleep=0.5)
            for i, a in enumerate(data.get("results",{}).get("albummatches",{}).get("album",[])[:5]):
                score = 100 if a["name"].lower() == media.album.lower() else max(60,90-i*10)
                results.Append(MetadataSearchResult(
                    id=a.get("mbid",a["name"]), name=a["name"], score=score, lang=lang))
        except Exception as e:
            Log.Error("[__NAME__Album] search: %s" % e)

    def update(self, metadata, media, lang, force):
        key = Prefs.get("lastfm_api_key", "")
        if not key: return
        try:
            data = JSON.ObjectFromURL(
                "%s/?method=album.getinfo&album=%s&artist=%s&api_key=%s&lang=%s&format=json" % (
                    LASTFM_BASE, String.Quote(media.album), String.Quote(media.artist),
                    key, lang[:2]), sleep=0.5)
            info = data.get("album", {})
            metadata.title = info.get("name", media.album)
            wiki = (info.get("wiki",{}).get("summary","") or "")
            metadata.summary = wiki.split("<a href")[0].strip()
            metadata.genres.clear()
            for tag in info.get("tags",{}).get("tag",[])[:5]:
                metadata.genres.add(tag["name"])
            for img in reversed(info.get("image",[])):
                url = img.get("#text","")
                if url:
                    metadata.posters[url] = Proxy.Preview(
                        HTTP.Request(url,sleep=0).content, sort_order=1); break
        except Exception as e:
            Log.Error("[__NAME__Album] update: %s" % e)
"##;

pub fn universal_init_py(name: &str) -> String { inject_name(UNIVERSAL_TEMPLATE, name) }
