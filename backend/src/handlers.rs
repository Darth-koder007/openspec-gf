use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use std::sync::{Arc, Mutex};

use crate::db;
use crate::models::UserProfile;

pub type DbConn = Arc<Mutex<rusqlite::Connection>>;

fn is_valid_email(email: &str) -> bool {
    // Basic email validation: contains @ and has characters before and after it
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }
    !parts[0].is_empty() && !parts[1].is_empty() && parts[1].contains('.')
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
