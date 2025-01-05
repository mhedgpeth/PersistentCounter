use rusqlite::{Connection, Result};

pub struct Counter {
    pub id: i64,
    pub name: String,
    pub count: i64,
}

use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbPathError {
    #[error("Could not determine data directory")]
    NoDataDir,
    #[error("Failed to create directories: {0}")]
    CreateDirFailed(#[from] std::io::Error),
}

pub fn get_db_path() -> Result<PathBuf, DbPathError> {
    // Get the appropriate data directory for the platform
    let mut path = dirs::data_local_dir().ok_or(DbPathError::NoDataDir)?;

    // On macOS this will be ~/Library/Application Support/
    // On iOS this will be in the app's sandbox data directory
    path.push("persistent-counter");

    // Create the directory if it doesn't exist
    std::fs::create_dir_all(&path)?;

    // Add the database file name
    path.push("counter.db");

    Ok(path)
}

pub fn initialize_db(db_path: &str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;

    // Create table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS counters (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            count INTEGER NOT NULL DEFAULT 0
        )",
        [],
    )?;

    // Check if the 'app' counter exists
    let count: isize = conn.query_row(
        "SELECT COUNT(*) FROM counters WHERE name = ?1",
        ["app"],
        |row| row.get(0),
    )?;

    // Insert the 'app' counter if it doesn't exist
    if count == 0 {
        conn.execute("INSERT INTO counters (name, count) VALUES (?1, 0)", ["app"])?;
    }

    Ok(conn)
}

pub fn update_counter(conn: &Connection, id: isize, new_count: isize) -> Result<()> {
    conn.execute(
        "UPDATE counters SET count = ?1 WHERE id = ?2",
        [new_count, id],
    )?;
    Ok(())
}

pub fn fetch_counter(conn: &Connection, id: isize) -> Result<isize> {
    let count: isize = conn.query_row("SELECT count FROM counters WHERE id = ?1", [id], |row| {
        row.get(0)
    })?;
    Ok(count)
}

// Example usage:
// fn main() -> Result<()> {
//     let conn = initialize_db("my_database.db")?;
//
//     // Get the ID of the 'app' counter
//     let app_id: i64 = conn.query_row(
//         "SELECT id FROM counters WHERE name = ?1",
//         ["app"],
//         |row| row.get(0),
//     )?;
//
//     // Update the counter
//     update_counter(&conn, app_id, 42)?;
//
//     // Fetch and print the counter
//     let count = fetch_counter(&conn, app_id)?;
//     println!("Current count: {}", count);
//
//     Ok(())
// }
