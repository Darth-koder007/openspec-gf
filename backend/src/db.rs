use rusqlite::{Connection, Result};
use std::env;
use std::path::Path;

const SCHEMA_VERSION: i32 = 2;

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

    // Migration 2: Create kudos table
    if current_version < 2 {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS kudos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                sender_email TEXT NOT NULL,
                recipient_email TEXT NOT NULL,
                message TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                is_public INTEGER NOT NULL DEFAULT 1
            )",
            [],
        )?;
        set_schema_version(conn, 2)?;
    }

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

pub fn seed_test_kudos(conn: &Connection) -> Result<()> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    // Kudo from Jane to John
    conn.execute(
        "INSERT INTO kudos (sender_email, recipient_email, message, created_at, is_public) VALUES (?1, ?2, ?3, ?4, ?5)",
        [
            "jane@deliveryhero.com",
            "john@deliveryhero.com",
            "Great work on the presentation!",
            &(now - 86400).to_string(), // 1 day ago
            "1",
        ],
    )?;

    // Kudo from John to Jane
    conn.execute(
        "INSERT INTO kudos (sender_email, recipient_email, message, created_at, is_public) VALUES (?1, ?2, ?3, ?4, ?5)",
        [
            "john@deliveryhero.com",
            "jane@deliveryhero.com",
            "Thanks for your help with the project!",
            &(now - 172800).to_string(), // 2 days ago
            "1",
        ],
    )?;

    // Another kudo from Jane to John
    conn.execute(
        "INSERT INTO kudos (sender_email, recipient_email, message, created_at, is_public) VALUES (?1, ?2, ?3, ?4, ?5)",
        [
            "jane@deliveryhero.com",
            "john@deliveryhero.com",
            "Excellent debugging skills!",
            &(now - 259200).to_string(), // 3 days ago
            "1",
        ],
    )?;

    Ok(())
}

pub fn get_kudos_by_recipient(conn: &Connection, email: &str) -> Result<Vec<(i64, String, String, String, i64, i32)>> {
    let mut stmt = conn.prepare(
        "SELECT id, sender_email, recipient_email, message, created_at, is_public
         FROM kudos
         WHERE recipient_email = ?1
         ORDER BY created_at DESC"
    )?;

    let kudos_iter = stmt.query_map([email], |row| {
        Ok((
            row.get(0)?,  // id
            row.get(1)?,  // sender_email
            row.get(2)?,  // recipient_email
            row.get(3)?,  // message
            row.get(4)?,  // created_at
            row.get(5)?,  // is_public
        ))
    })?;

    let mut kudos = Vec::new();
    for kudo in kudos_iter {
        kudos.push(kudo?);
    }

    Ok(kudos)
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
