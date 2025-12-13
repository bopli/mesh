use std::path::Path;

use rusqlite::Connection;

pub struct MeshDatabase {
    conn: Connection,
}

impl MeshDatabase {
    pub fn init<P: AsRef<Path>>(file_path: P) -> MeshDatabase {
        let conn = Connection::open(file_path)
            .map_err(|e| log::warn!("Failed to open database: {}", e))
            .unwrap();

        init_execute(&conn)
            .map_err(|e| log::warn!("Failed to init database: {}", e))
            .unwrap();

        MeshDatabase { conn }
    }

    pub fn sqlite_version(&self) -> rusqlite::Result<String> {
        self.conn.query_row(
            "SELECT sqlite_version()",
            [], // 参数为空
            |row| row.get(0),
        )
    }
}

fn init_execute(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS photos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            path TEXT NOT NULL UNIQUE,
            file_hash TEXT NOT NULL UNIQUE,
            filename TEXT NOT NULL,
            width INTEGER NOT NULL,
            height INTEGER NOT NULL,
            size INTEGER NOT NULL,
            created_at DATETIME NOT NULL,
            modified_at DATETIME NOT NULL
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            parent_id INTEGER,
            tag_color_hex  DEFAULT '#FFFFFF',
            FOREIGN KEY (parent_id) REFERENCES tags(id) ON DELETE SET NULL
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS photo_tags (
            photo_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            PRIMARY KEY (photo_id, tag_id),
            FOREIGN KEY (photo_id) REFERENCES photos(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tag_counts (
            tag_id INTEGER PRIMARY KEY,
            photo_count INTEGER NOT NULL,
            FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
        )",
        [],
    )?;
    conn.execute(
        "CREATE VIRTUAL TABLE IF NOT EXISTS photos_fts USING fts5(
            photo_id UNINDEXED,
            filename
        )",
        [],
    )?;
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS photos_fts_ai AFTER INSERT ON photos BEGIN
          INSERT INTO photos_fts(rowid, filename) VALUES (new.id, new.filename);
        END",
        [],
    )?;
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS photos_fts_au AFTER UPDATE OF filename ON photos BEGIN
          INSERT INTO photos_fts(photos_fts, rowid, filename) VALUES ('delete', old.id, old.filename);
          INSERT INTO photos_fts(rowid, filename) VALUES (new.id, new.filename);
        END",
        [],
    )?;
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS photos_fts_ad AFTER DELETE ON photos BEGIN
          INSERT INTO photos_fts(photos_fts, rowid, filename) VALUES ('delete', old.id, old.filename);
        END",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_photo_tags_tag_id ON photo_tags (tag_id)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_tags_parent_id ON tags (parent_id)",
        [],
    )?;
    conn.execute(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_photos_file_hash ON photos (file_hash)",
        [],
    )?;
    Ok(())
}
