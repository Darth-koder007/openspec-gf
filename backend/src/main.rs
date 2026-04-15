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
}
