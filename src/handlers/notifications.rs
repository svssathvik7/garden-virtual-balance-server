use axum::http::header::AUTHORIZATION;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use std::{env, sync::Arc};

use crate::{appstate::AppState, models::notification::Notification};

#[derive(Debug)]
pub enum NotificationError {
    Unauthorized,
    MissingAuthToken,
    MissingEnvToken,
    DatabaseError(String),
}

impl IntoResponse for NotificationError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            NotificationError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "Invalid authentication token".to_string(),
            ),
            NotificationError::MissingAuthToken => (
                StatusCode::UNAUTHORIZED,
                "Missing authentication token".to_string(),
            ),
            NotificationError::MissingEnvToken => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server configuration error".to_string(),
            ),
            NotificationError::DatabaseError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create notification {:?}", e),
            ),
        };

        (status, error_message).into_response()
    }
}

pub async fn add_notification(
    headers: axum::http::HeaderMap,
    State(appstate): State<Arc<AppState>>,
    Json(notification): Json<Notification>,
) -> Result<impl IntoResponse, NotificationError> {
    // Check for auth token in headers
    let auth_token = headers
        .get("authToken")
        .and_then(|value| value.to_str().ok())
        .ok_or(NotificationError::MissingAuthToken)?;

    // Get expected token from environment
    let expected_token = env::var("AUTH_TOKEN").map_err(|_| NotificationError::MissingEnvToken)?;

    // Validate token
    if auth_token != expected_token {
        return Err(NotificationError::Unauthorized);
    }

    // Create notification in database
    match appstate
        .notification_repo
        .create_notification(notification)
        .await
    {
        Ok(_) => Ok((StatusCode::CREATED, "Notification created successfully").into_response()),
        Err(e) => {
            eprintln!("Database error: {}", e);
            Err(NotificationError::DatabaseError(e.to_string()))
        }
    }
}
