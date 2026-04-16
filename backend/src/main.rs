use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};

mod db;
mod handlers;
mod models;

use handlers::DbConn;

fn create_app(db: DbConn) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/", get(|| async { "Kudos Backend" }))
        .route("/user/:email", get(handlers::get_user_profile))
        .route("/kudos/:email", get(handlers::get_kudos))
        .layer(cors)
        .with_state(db)
}

#[tokio::main]
async fn main() {
    eprintln!("Starting Kudos Backend...");

    // Initialize database
    eprintln!("Initializing database...");
    let conn = db::init_database().expect("Failed to initialize database");
    eprintln!("Database initialized successfully");

    eprintln!("Seeding test users...");
    db::seed_test_users(&conn).expect("Failed to seed test users");
    eprintln!("Test users seeded successfully");

    eprintln!("Seeding test kudos...");
    db::seed_test_kudos(&conn).expect("Failed to seed test kudos");
    eprintln!("Test kudos seeded successfully");

    let db = Arc::new(Mutex::new(conn));

    let app = create_app(db);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    eprintln!("Binding to {}...", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    eprintln!("Server listening on {}", addr);
    eprintln!("Backend ready to accept connections");

    axum::serve(listener, app).await.unwrap();

    eprintln!("Server shutdown");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::UserProfile;

    #[test]
    fn test_user_profile_creation() {
        let profile = UserProfile {
            display_name: "John Smith".to_string(),
            full_name: "John Michael Smith".to_string(),
            email: "john@deliveryhero.com".to_string(),
            avatar_url: Some("https://example.com/avatar.jpg".to_string()),
        };

        assert_eq!(profile.display_name, "John Smith");
        assert_eq!(profile.full_name, "John Michael Smith");
        assert_eq!(profile.email, "john@deliveryhero.com");
        assert_eq!(profile.avatar_url, Some("https://example.com/avatar.jpg".to_string()));
    }

    #[test]
    fn test_user_profile_without_avatar() {
        let profile = UserProfile {
            display_name: "Jane Doe".to_string(),
            full_name: "Jane Elizabeth Doe".to_string(),
            email: "jane@deliveryhero.com".to_string(),
            avatar_url: None,
        };

        assert_eq!(profile.display_name, "Jane Doe");
        assert!(profile.avatar_url.is_none());
    }

    #[test]
    fn test_database_initialization() {
        let conn = db::init_database_with_path(":memory:").unwrap();

        // Verify users table exists
        let table_exists: bool = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='users'",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert!(table_exists);
    }

    #[test]
    fn test_kudos_table_migration() {
        let conn = db::init_database_with_path(":memory:").unwrap();

        // Verify kudos table exists
        let table_exists: bool = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='kudos'",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert!(table_exists);

        // Verify schema has correct columns
        let mut stmt = conn
            .prepare("PRAGMA table_info(kudos)")
            .unwrap();

        let columns: Vec<String> = stmt
            .query_map([], |row| row.get::<_, String>(1))
            .unwrap()
            .filter_map(Result::ok)
            .collect();

        assert!(columns.contains(&"id".to_string()));
        assert!(columns.contains(&"sender_email".to_string()));
        assert!(columns.contains(&"recipient_email".to_string()));
        assert!(columns.contains(&"message".to_string()));
        assert!(columns.contains(&"created_at".to_string()));
        assert!(columns.contains(&"is_public".to_string()));
    }

    #[test]
    fn test_seed_test_users() {
        let conn = db::init_database_with_path(":memory:").unwrap();
        db::seed_test_users(&conn).unwrap();

        // Verify John Smith exists
        let john: (String, String, String) = conn
            .query_row(
                "SELECT display_name, full_name, email FROM users WHERE email = ?1",
                ["john@deliveryhero.com"],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .unwrap();

        assert_eq!(john.0, "John Smith");
        assert_eq!(john.1, "John Michael Smith");
        assert_eq!(john.2, "john@deliveryhero.com");

        // Verify Jane Doe exists
        let jane: (String, String, String) = conn
            .query_row(
                "SELECT display_name, full_name, email FROM users WHERE email = ?1",
                ["jane@deliveryhero.com"],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .unwrap();

        assert_eq!(jane.0, "Jane Doe");
        assert_eq!(jane.1, "Jane Elizabeth Doe");
        assert_eq!(jane.2, "jane@deliveryhero.com");

        // Verify exactly 2 users
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM users", [], |row| row.get(0))
            .unwrap();

        assert_eq!(count, 2);
    }

    #[test]
    fn test_seed_test_kudos() {
        let conn = db::init_database_with_path(":memory:").unwrap();
        db::seed_test_users(&conn).unwrap();
        db::seed_test_kudos(&conn).unwrap();

        // Verify kudos exist
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM kudos", [], |row| row.get(0))
            .unwrap();

        assert_eq!(count, 3);

        // Verify John has received kudos
        let john_kudos_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM kudos WHERE recipient_email = ?1",
                ["john@deliveryhero.com"],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(john_kudos_count, 2);

        // Verify Jane has received kudos
        let jane_kudos_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM kudos WHERE recipient_email = ?1",
                ["jane@deliveryhero.com"],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(jane_kudos_count, 1);

        // Verify kudos have required fields
        let kudo: (String, String, String) = conn
            .query_row(
                "SELECT sender_email, recipient_email, message FROM kudos WHERE recipient_email = ?1 LIMIT 1",
                ["john@deliveryhero.com"],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .unwrap();

        assert_eq!(kudo.0, "jane@deliveryhero.com");
        assert_eq!(kudo.1, "john@deliveryhero.com");
        assert!(!kudo.2.is_empty());
    }

    #[test]
    fn test_get_kudos_by_recipient_success() {
        let conn = db::init_database_with_path(":memory:").unwrap();
        db::seed_test_users(&conn).unwrap();
        db::seed_test_kudos(&conn).unwrap();

        let kudos = db::get_kudos_by_recipient(&conn, "john@deliveryhero.com").unwrap();

        assert_eq!(kudos.len(), 2);
        // Verify ordering (newest first)
        assert!(kudos[0].4 > kudos[1].4); // created_at comparison
    }

    #[test]
    fn test_get_kudos_by_recipient_empty() {
        let conn = db::init_database_with_path(":memory:").unwrap();
        db::seed_test_users(&conn).unwrap();
        db::seed_test_kudos(&conn).unwrap();

        // User with no kudos
        let kudos = db::get_kudos_by_recipient(&conn, "nobody@example.com").unwrap();

        assert_eq!(kudos.len(), 0);
    }

    #[test]
    fn test_get_kudos_by_recipient_fields() {
        let conn = db::init_database_with_path(":memory:").unwrap();
        db::seed_test_users(&conn).unwrap();
        db::seed_test_kudos(&conn).unwrap();

        let kudos = db::get_kudos_by_recipient(&conn, "john@deliveryhero.com").unwrap();

        assert!(kudos.len() > 0);
        let kudo = &kudos[0];
        assert!(kudo.0 > 0); // id
        assert_eq!(kudo.1, "jane@deliveryhero.com"); // sender_email
        assert_eq!(kudo.2, "john@deliveryhero.com"); // recipient_email
        assert!(!kudo.3.is_empty()); // message
        assert!(kudo.4 > 0); // created_at
        assert_eq!(kudo.5, 1); // is_public
    }

    #[tokio::test]
    async fn test_get_user_profile_success() {
        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use tower::ServiceExt;

        let conn = db::init_database_with_path(":memory:").unwrap();
        db::seed_test_users(&conn).unwrap();
        let db = Arc::new(Mutex::new(conn));

        let app = create_app(db);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/user/john@deliveryhero.com")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let profile: UserProfile = serde_json::from_slice(&body).unwrap();

        assert_eq!(profile.display_name, "John Smith");
        assert_eq!(profile.email, "john@deliveryhero.com");
    }

    #[tokio::test]
    async fn test_get_user_profile_not_found() {
        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use tower::ServiceExt;

        let conn = db::init_database_with_path(":memory:").unwrap();
        db::seed_test_users(&conn).unwrap();
        let db = Arc::new(Mutex::new(conn));

        let app = create_app(db);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/user/nonexistent@example.com")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_user_profile_invalid_email() {
        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use tower::ServiceExt;

        let conn = db::init_database_with_path(":memory:").unwrap();
        db::seed_test_users(&conn).unwrap();
        let db = Arc::new(Mutex::new(conn));

        let app = create_app(db);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/user/invalid-email-format")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_cors_preflight() {
        use axum::body::Body;
        use axum::http::{header, Method, Request, StatusCode};
        use tower::ServiceExt;

        let conn = db::init_database_with_path(":memory:").unwrap();
        db::seed_test_users(&conn).unwrap();
        let db = Arc::new(Mutex::new(conn));

        let app = create_app(db);

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::OPTIONS)
                    .uri("/user/john@deliveryhero.com")
                    .header(header::ORIGIN, "http://localhost:5173")
                    .header(header::ACCESS_CONTROL_REQUEST_METHOD, "GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_cors_headers_on_get() {
        use axum::body::Body;
        use axum::http::{header, Request, StatusCode};
        use tower::ServiceExt;

        let conn = db::init_database_with_path(":memory:").unwrap();
        db::seed_test_users(&conn).unwrap();
        let db = Arc::new(Mutex::new(conn));

        let app = create_app(db);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/user/john@deliveryhero.com")
                    .header(header::ORIGIN, "http://localhost:5173")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // Verify CORS header is present
        let cors_header = response.headers().get(header::ACCESS_CONTROL_ALLOW_ORIGIN);
        assert!(cors_header.is_some());
    }

    #[tokio::test]
    async fn test_get_kudos_success() {
        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use tower::ServiceExt;

        let conn = db::init_database_with_path(":memory:").unwrap();
        db::seed_test_users(&conn).unwrap();
        db::seed_test_kudos(&conn).unwrap();
        let db = Arc::new(Mutex::new(conn));

        let app = create_app(db);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/kudos/john@deliveryhero.com")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let kudos: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();

        assert_eq!(kudos.len(), 2);
        assert_eq!(kudos[0]["recipientEmail"], "john@deliveryhero.com");
        assert_eq!(kudos[0]["senderEmail"], "jane@deliveryhero.com");
    }

    #[tokio::test]
    async fn test_get_kudos_empty_array() {
        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use tower::ServiceExt;

        let conn = db::init_database_with_path(":memory:").unwrap();
        db::seed_test_users(&conn).unwrap();
        db::seed_test_kudos(&conn).unwrap();
        let db = Arc::new(Mutex::new(conn));

        let app = create_app(db);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/kudos/nobody@example.com")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let kudos: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();

        assert_eq!(kudos.len(), 0);
    }

    #[tokio::test]
    async fn test_get_kudos_invalid_email() {
        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use tower::ServiceExt;

        let conn = db::init_database_with_path(":memory:").unwrap();
        db::seed_test_users(&conn).unwrap();
        let db = Arc::new(Mutex::new(conn));

        let app = create_app(db);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/kudos/invalid-email-format")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
