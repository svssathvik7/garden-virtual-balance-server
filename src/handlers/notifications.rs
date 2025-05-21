use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use std::{env, sync::Arc};

use crate::{
    appstate::AppState,
    models::notification::Notification,
    utils::{ApiResponse, NotificationError},
};

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

    let expected_token = env::var("AUTH_TOKEN").map_err(|_| NotificationError::MissingEnvToken)?;

    if auth_token != expected_token {
        return Err(NotificationError::Unauthorized);
    }

    match appstate
        .notification_repo
        .create_notification(notification)
        .await
    {
        Ok(_) => Ok((
            StatusCode::CREATED,
            Json(ApiResponse::ok("Notification created successfully")),
        )
            .into_response()),
        Err(e) => {
            eprintln!("Database error: {}", e);
            Err(NotificationError::DatabaseError(e.to_string()))
        }
    }
}

pub async fn get_notification_by_id(
    State(appstate): State<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> impl IntoResponse {
    match appstate.notification_repo.get_notification(Some(&id)).await {
        Ok(Some(notification)) => {
            (StatusCode::OK, Json(ApiResponse::ok(notification))).into_response()
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::error("No notifications found")),
        )
            .into_response(),
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch notifications",
            )
                .into_response()
        }
    }
}

pub async fn get_latest_notification(State(appstate): State<Arc<AppState>>) -> impl IntoResponse {
    match appstate.notification_repo.get_notification(None).await {
        Ok(Some(notification)) => {
            (StatusCode::OK, Json(ApiResponse::ok(notification))).into_response()
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::error("No notifications found")),
        )
            .into_response(),
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch notifications",
            )
                .into_response()
        }
    }
}
