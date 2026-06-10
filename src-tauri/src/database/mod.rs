use rusqlite::{Connection, OpenFlags, params};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::error::{PlexMetaForgeError, Result};

// ─── Structs ──────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MediaItem {
    pub id: i64,
    pub title: String,
    pub year: Option<i32>,
    pub summary: Option<String>,
    pub thumb: Option<String>,
    pub media_type: Option<String>,
    pub section_id: Option<i64>,
    pub duration: Option<i64>,
    pub rating: Option<f64>,
    pub tagline: Option<String>,
    pub studio: Option<String>,
    pub originally_available_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlexSection {
    pub id: i64,
    pub name: String,
    pub section_type: String,
    pub location: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseStats {
    pub total_items: i64,
    pub movies: i64,
    pub shows: i64,
    pub episodes: i64,
    pub artists: i64,
    pub albums: i64,
    pub tracks: i64,
    pub sections: i64,
    pub items_with_thumb: i64,
    pub items_without_summary: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchUpdateResult {
    pub updated: usize,
    pub errors: Vec<String>,
}

// ─── Database ─────────────────────────────────────────────────

pub struct PlexDatabase {
    conn: Connection,
}

impl PlexDatabase {
    pub fn open(db_path: &PathBuf) -> Result<Self> {
        if !db_path.exists() {
            return Err(PlexMetaForgeError::PlexApi(format!(
                "Base de données introuvable : {}",
                db_path.display()
            )));
        }

        let conn = Connection::open_with_flags(
            db_path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )?;

        conn.execute_batch(
            "PRAGMA busy_timeout = 5000;
             PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;",
        )?;

        Ok(Self { conn })
    }

    // ─── Stats ──────────────────────────────────────────────

    pub fn get_stats(&self) -> Result<DatabaseStats> {
        let count = |q: &str| -> Result<i64> {
            self.conn.query_row(q, [], |r| r.get(0)).map_err(PlexMetaForgeError::Sqlite)
        };

        Ok(DatabaseStats {
            total_items:            count("SELECT COUNT(*) FROM metadata_items")?,
            movies:                 count("SELECT COUNT(*) FROM metadata_items WHERE metadata_type = 1")?,
            shows:                  count("SELECT COUNT(*) FROM metadata_items WHERE metadata_type = 2")?,
            episodes:               count("SELECT COUNT(*) FROM metadata_items WHERE metadata_type = 4")?,
            artists:                count("SELECT COUNT(*) FROM metadata_items WHERE metadata_type = 8")?,
            albums:                 count("SELECT COUNT(*) FROM metadata_items WHERE metadata_type = 9")?,
            tracks:                 count("SELECT COUNT(*) FROM metadata_items WHERE metadata_type = 10")?,
            sections:               count("SELECT COUNT(*) FROM library_sections")?,
            items_with_thumb:       count("SELECT COUNT(*) FROM metadata_items WHERE user_thumb_url IS NOT NULL AND user_thumb_url != ''")?,
            items_without_summary:  count("SELECT COUNT(*) FROM metadata_items WHERE (summary IS NULL OR summary = '') AND metadata_type IN (1,2,4)")?,
        })
    }

    // ─── Sections ───────────────────────────────────────────

    pub fn get_sections(&self) -> Result<Vec<PlexSection>> {
        // Try with location join first; fall back if table/column doesn't exist
        let sql_with_loc = "SELECT ls.id, ls.name, ls.section_type, lsp.root_path
             FROM library_sections ls
             LEFT JOIN library_section_locations lsp ON lsp.library_section_id = ls.id
             GROUP BY ls.id ORDER BY ls.name";
        let sql_no_loc = "SELECT id, name, section_type, NULL FROM library_sections ORDER BY name";

        let mut stmt = self.conn.prepare(sql_with_loc)
            .or_else(|_| self.conn.prepare(sql_no_loc))?;

        let rows = stmt.query_map([], |row| {
            Ok(PlexSection {
                id:           row.get(0)?,
                name:         row.get(1)?,
                section_type: row.get(2)?,
                location:     row.get(3)?,
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(rows)
    }

    // ─── Search ─────────────────────────────────────────────

    pub fn search_metadata_items(&self, query: &str) -> Result<Vec<MediaItem>> {
        let pattern = format!("%{}%", query);
        let mut stmt = self.conn.prepare(
            "SELECT mi.id, mi.title, mi.year, mi.summary, mi.user_thumb_url,
                    mi.metadata_type, mi.library_section_id, mi.duration,
                    mi.rating, mi.tagline, mi.studio, mi.originally_available_at
             FROM metadata_items mi
             WHERE mi.title LIKE ?1
               AND mi.metadata_type IN (1, 2, 4, 8, 9, 10)
             ORDER BY mi.title
             LIMIT 100"
        )?;

        let items = stmt.query_map([&pattern], |row| {
            Ok(MediaItem {
                id:                       row.get(0)?,
                title:                    row.get(1)?,
                year:                     row.get(2)?,
                summary:                  row.get(3)?,
                thumb:                    row.get(4)?,
                media_type:               row.get::<_, Option<i64>>(5)?.map(media_type_label),
                section_id:               row.get(6)?,
                duration:                 row.get(7)?,
                rating:                   row.get(8)?,
                tagline:                  row.get(9)?,
                studio:                   row.get(10)?,
                originally_available_at:  row.get(11)?,
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(items)
    }

    pub fn get_items_by_section(&self, section_id: i64, limit: i64) -> Result<Vec<MediaItem>> {
        let mut stmt = self.conn.prepare(
            "SELECT mi.id, mi.title, mi.year, mi.summary, mi.user_thumb_url,
                    mi.metadata_type, mi.library_section_id, mi.duration,
                    mi.rating, mi.tagline, mi.studio, mi.originally_available_at
             FROM metadata_items mi
             WHERE mi.library_section_id = ?1
               AND mi.metadata_type IN (1, 2, 8, 9)
             ORDER BY mi.title
             LIMIT ?2"
        )?;

        let items = stmt.query_map(params![section_id, limit], |row| {
            Ok(MediaItem {
                id:                       row.get(0)?,
                title:                    row.get(1)?,
                year:                     row.get(2)?,
                summary:                  row.get(3)?,
                thumb:                    row.get(4)?,
                media_type:               row.get::<_, Option<i64>>(5)?.map(media_type_label),
                section_id:               row.get(6)?,
                duration:                 row.get(7)?,
                rating:                   row.get(8)?,
                tagline:                  row.get(9)?,
                studio:                   row.get(10)?,
                originally_available_at:  row.get(11)?,
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(items)
    }

    pub fn get_media_item_by_title(&self, title: &str) -> Result<Option<MediaItem>> {
        let mut stmt = self.conn.prepare(
            "SELECT mi.id, mi.title, mi.year, mi.summary, mi.user_thumb_url,
                    mi.metadata_type, mi.library_section_id, mi.duration,
                    mi.rating, mi.tagline, mi.studio, mi.originally_available_at
             FROM metadata_items mi
             WHERE mi.title = ?1
             LIMIT 1"
        )?;

        let result = stmt.query_row([title], |row| {
            Ok(MediaItem {
                id:                       row.get(0)?,
                title:                    row.get(1)?,
                year:                     row.get(2)?,
                summary:                  row.get(3)?,
                thumb:                    row.get(4)?,
                media_type:               row.get::<_, Option<i64>>(5)?.map(media_type_label),
                section_id:               row.get(6)?,
                duration:                 row.get(7)?,
                rating:                   row.get(8)?,
                tagline:                  row.get(9)?,
                studio:                   row.get(10)?,
                originally_available_at:  row.get(11)?,
            })
        });

        match result {
            Ok(item) => Ok(Some(item)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(PlexMetaForgeError::Sqlite(e)),
        }
    }

    pub fn get_incomplete_items(&self, limit: i64) -> Result<Vec<MediaItem>> {
        let mut stmt = self.conn.prepare(
            "SELECT mi.id, mi.title, mi.year, mi.summary, mi.user_thumb_url,
                    mi.metadata_type, mi.library_section_id, mi.duration,
                    mi.rating, mi.tagline, mi.studio, mi.originally_available_at
             FROM metadata_items mi
             WHERE mi.metadata_type IN (1, 2)
               AND (mi.summary IS NULL OR mi.summary = ''
                    OR mi.user_thumb_url IS NULL OR mi.user_thumb_url = '')
             ORDER BY mi.title
             LIMIT ?1"
        )?;

        let items = stmt.query_map([limit], |row| {
            Ok(MediaItem {
                id:                       row.get(0)?,
                title:                    row.get(1)?,
                year:                     row.get(2)?,
                summary:                  row.get(3)?,
                thumb:                    row.get(4)?,
                media_type:               row.get::<_, Option<i64>>(5)?.map(media_type_label),
                section_id:               row.get(6)?,
                duration:                 row.get(7)?,
                rating:                   row.get(8)?,
                tagline:                  row.get(9)?,
                studio:                   row.get(10)?,
                originally_available_at:  row.get(11)?,
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(items)
    }

    // ─── Write ──────────────────────────────────────────────

    pub fn update_metadata(
        &self,
        id: i64,
        title: &str,
        year: Option<i32>,
        summary: &str,
    ) -> Result<usize> {
        let n = self.conn.execute(
            "UPDATE metadata_items SET title = ?1, year = ?2, summary = ?3 WHERE id = ?4",
            params![title, year, summary, id],
        )?;
        Ok(n)
    }

    #[allow(dead_code)]
    pub fn update_metadata_full(
        &self,
        id: i64,
        title: &str,
        year: Option<i32>,
        summary: &str,
        tagline: Option<&str>,
        studio: Option<&str>,
        rating: Option<f64>,
    ) -> Result<usize> {
        let n = self.conn.execute(
            "UPDATE metadata_items
             SET title = ?1, year = ?2, summary = ?3,
                 tagline = ?4, studio = ?5, rating = ?6
             WHERE id = ?7",
            params![title, year, summary, tagline, studio, rating, id],
        )?;
        Ok(n)
    }

    pub fn batch_clear_locks(&self) -> Result<BatchUpdateResult> {
        // Libère les metadata_items bloqués par un agent qui a planté
        let n = self.conn.execute(
            "UPDATE metadata_items SET refreshed_at = NULL WHERE refreshed_at IS NOT NULL",
            [],
        )?;
        Ok(BatchUpdateResult { updated: n, errors: vec![] })
    }

    #[allow(dead_code)]
    pub fn update_thumb(&self, id: i64, thumb_url: &str) -> Result<usize> {
        let n = self.conn.execute(
            "UPDATE metadata_items SET user_thumb_url = ?1 WHERE id = ?2",
            params![thumb_url, id],
        )?;
        Ok(n)
    }
}

fn media_type_label(t: i64) -> String {
    match t {
        1  => "Film".to_string(),
        2  => "Série".to_string(),
        3  => "Saison".to_string(),
        4  => "Épisode".to_string(),
        8  => "Artiste".to_string(),
        9  => "Album".to_string(),
        10 => "Piste".to_string(),
        13 => "Photo".to_string(),
        _  => format!("Type {}", t),
    }
}
