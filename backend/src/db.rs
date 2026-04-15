use rusqlite::{Connection, Result};
use std::env;
use std::path::Path;

const SCHEMA_VERSION: i32 = 1;

pub fn get_database_path() -> String {
    env::var("DATABASE_PATH").unwrap_or_else(|_| "data/kudos.db".to_string())
}

fn get_schema_version(conn: &Connection) -> Result<i32> {
    // Create schema_version table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER PRIMARY KEY
        )",
        [],
    )?;

    // Get current version, default to 0 if no version exists
    let version: i32 = conn
        .query_row("SELECT version FROM schema_version LIMIT 1", [], |row| {
            row.get(0)
        })
        .unwrap_or(0);

    Ok(version)
}

fn set_schema_version(conn: &Connection, version: i32) -> Result<()> {
    conn.execute("DELETE FROM schema_version", [])?;
    conn.execute("INSERT INTO schema_version (version) VALUES (?1)", [version])?;
    Ok(())
}

fn run_migrations(conn: &Connection) -> Result<()> {
    let current_version = get_schema_version(conn)?;

    // Migration 1: Create users table
    if current_version < 1 {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY,
                display_name TEXT NOT NULL,
                full_name TEXT NOT NULL,
                email TEXT NOT NULL UNIQUE,
                avatar_url TEXT
            )",
            [],
        )?;
        set_schema_version(conn, 1)?;
    }

    // Future migrations can be added here:
    // if current_version < 2 {
    //     conn.execute("ALTER TABLE users ADD COLUMN ...", [])?;
    //     set_schema_version(conn, 2)?;
    // }

    Ok(())
}

pub fn init_database() -> Result<Connection> {
    init_database_with_path(&get_database_path())
}

pub fn init_database_with_path(path: &str) -> Result<Connection> {
    let conn = if path == ":memory:" {
        Connection::open_in_memory()?
    } else {
        // Create parent directory if it doesn't exist
        if let Some(parent) = Path::new(path).parent() {
            std::fs::create_dir_all(parent).ok();
        }
        Connection::open(path)?
    };

    // Run migrations to ensure schema is up to date
    run_migrations(&conn)?;

    Ok(conn)
}

pub fn seed_test_users(conn: &Connection) -> Result<()> {
    conn.execute(
        "INSERT INTO users (display_name, full_name, email, avatar_url) VALUES (?1, ?2, ?3, ?4)",
        [
            "John Smith",
            "John Michael Smith",
            "john@deliveryhero.com",
            "",
        ],
    )?;

    conn.execute(
        "INSERT INTO users (display_name, full_name, email, avatar_url) VALUES (?1, ?2, ?3, ?4)",
        [
            "Jane Doe",
            "Jane Elizabeth Doe",
            "jane@deliveryhero.com",
            "",
        ],
    )?;

    Ok(())
}

pub fn get_user_by_email(conn: &Connection, email: &str) -> Result<Option<(String, String, String, Option<String>)>> {
    let mut stmt = conn.prepare(
        "SELECT display_name, full_name, email, avatar_url FROM users WHERE email = ?1"
    )?;

    let result = stmt.query_row([email], |row| {
        let avatar: String = row.get(3)?;
        let avatar_url = if avatar.is_empty() { None } else { Some(avatar) };
        Ok((row.get(0)?, row.get(1)?, row.get(2)?, avatar_url))
    });

    match result {
        Ok(user) => Ok(Some(user)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}
