use rusqlite::Connection;

use crate::core::MeshCore;

pub(crate) struct MeshDatabase {
    conn: Connection,
}

impl MeshDatabase {}

fn init_tables(conn: &Connection) -> anyhow::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS images (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            filename TEXT NOT NULL,
            path TEXT NOT NULL,
            mime_type TEXT,
            size INTEGER,
            modified_timestamp INTEGER NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            file_hash TEXT,
            UNIQUE (path, filename)
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS image_tags (
            image_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            PRIMARY KEY (image_id, tag_id),
            FOREIGN KEY (image_id) REFERENCES images(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
        )",
        [],
    )?;
    Ok(())
}

fn init_indexs(conn: &Connection) -> anyhow::Result<()> {
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_images_filename ON images(filename)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_image_tags_tag_id ON image_tags(tag_id)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_images_created_at ON images(created_at DESC)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_images_sync_ts ON images(modified_timestamp)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_images_file_hash ON images(file_hash)",
        [],
    )?;
    Ok(())
}

pub(crate) fn init() -> anyhow::Result<MeshDatabase> {
    let data_path = MeshCore::data_path("mesh.db")?;
    let conn = Connection::open(data_path)?;

    init_tables(&conn)?;
    init_indexs(&conn)?;

    Ok(MeshDatabase { conn })
}
