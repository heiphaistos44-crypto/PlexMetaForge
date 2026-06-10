use rusqlite::{Connection, OpenFlags};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::error::{PlexMetaForgeError, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaItem {
    pub id: i64,
    pub title: String,
    pub year: Option<i32>,
    pub summary: Option<String>,
    pub thumb: Option<String>,
}

pub struct PlexDatabase {
    conn: Connection,
}

impl PlexDatabase {
    pub fn open(db_path: &PathBuf) -> Result<Self> {
        if !db_path.exists() {
            return Err(PlexMetaForgeError::PlexApi(format!(
                "Database not found at: {}",
                db_path.display()
            )));
        }

        let conn = Connection::open_with_flags(
            db_path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )?;

        // WAL mode + 5s busy timeout for locked DB resistance
        conn.execute_batch(
            "PRAGMA busy_timeout = 5000;
             PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;",
        )?;

        Ok(Self { conn })
    }

    pub fn get_media_item_by_title(&self, title: &str) -> Result<Option<MediaItem>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, year, summary, user_thumb_url
             FROM metadata_items
             WHERE title = ?1
             LIMIT 1",
        )?;

        let result = stmt.query_row([title], |row| {
            Ok(MediaItem {
                id: row.get(0)?,
                title: row.get(1)?,
                year: row.get(2)?,
                summary: row.get(3)?,
                thumb: row.get(4)?,
            })
        });

        match result {
            Ok(item) => Ok(Some(item)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(PlexMetaForgeError::Sqlite(e)),
        }
    }

    pub fn update_metadata(
        &self,
        id: i64,
        title: &str,
        year: Option<i32>,
        summary: &str,
    ) -> Result<usize> {
        let rows = self.conn.execute(
            "UPDATE metadata_items SET title = ?1, year = ?2, summary = ?3 WHERE id = ?4",
            rusqlite::params![title, year, summary, id],
        )?;
        Ok(rows)
    }

    pub fn search_metadata_items(&self, query: &str) -> Result<Vec<MediaItem>> {
        let pattern = format!("%{}%", query);
        let mut stmt = self.conn.prepare(
            "SELECT id, title, year, summary, user_thumb_url
             FROM metadata_items
             WHERE title LIKE ?1
             ORDER BY title
             LIMIT 50",
        )?;

        let items = stmt.query_map([&pattern], |row| {
            Ok(MediaItem {
                id: row.get(0)?,
                title: row.get(1)?,
                year: row.get(2)?,
                summary: row.get(3)?,
                thumb: row.get(4)?,
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(items)
    }
}
