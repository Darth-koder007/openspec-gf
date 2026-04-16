use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use std::sync::{Arc, Mutex};

use crate::db;
use crate::models::{Kudo, UserProfile};

pub type DbConn = Arc<Mutex<rusqlite::Connection>>;

fn is_valid_email(email: &str) -> bool {
    // Basic email validation: contains @ and has characters before and after it
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }
    !parts[0].is_empty() && !parts[1].is_empty() && parts[1].contains('.')
}

pub async fn get_kudos(
    State(db): State<DbConn>,
    Path(email): Path<String>,
) -> impl IntoResponse {
    // Validate email format
    if !is_valid_email(&email) {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "Invalid email format"
        }))).into_response();
    }

    let conn = db.lock().unwrap();

    match db::get_kudos_by_recipient(&conn, &email) {
        Ok(kudos_data) => {
            // Convert Unix timestamps to ISO 8601 strings
            let kudos: Vec<Kudo> = kudos_data
                .into_iter()
                .map(|(id, sender_email, recipient_email, message, created_at, is_public)| {
                    use std::time::{Duration, UNIX_EPOCH};
                    let timestamp = UNIX_EPOCH + Duration::from_secs(created_at as u64);
                    let datetime: chrono::DateTime<chrono::Utc> = timestamp.into();
                    let created_at_str = datetime.to_rfc3339();

                    Kudo {
                        id,
                        sender_email,
                        recipient_email,
                        message,
                        created_at: created_at_str,
                        is_public: is_public == 1,
                    }
                })
                .collect();

            (StatusCode::OK, Json(kudos)).into_response()
        }
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Internal server error"
            }))).into_response()
        }
    }
}

pub async fn get_user_profile(
    State(db): State<DbConn>,
    Path(email): Path<String>,
) -> impl IntoResponse {
    // Validate email format
    if !is_valid_email(&email) {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "Invalid email format"
        }))).into_response();
    }

    let conn = db.lock().unwrap();

    match db::get_user_by_email(&conn, &email) {
        Ok(Some((display_name, full_name, email, avatar_url))) => {
            let profile = UserProfile {
                display_name,
                full_name,
                email,
                avatar_url,
            };
            (StatusCode::OK, Json(profile)).into_response()
        }
        Ok(None) => {
            (StatusCode::NOT_FOUND, Json(serde_json::json!({
                "error": "User not found"
            }))).into_response()
        }
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Internal server error"
            }))).into_response()
        }
    }
}
