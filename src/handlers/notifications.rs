use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use std::{env, sync::Arc};

use crate::{appstate::AppState, models::notification::Notification, utils::ApiResponse};

pub async fn add_notification(
    headers: axum::http::HeaderMap,
    State(appstate): State<Arc<AppState>>,
    Json(notification): Json<Notification>,
) -> impl IntoResponse {
    // Check for auth token in headers
    let auth_token = match headers
        .get("authToken")
        .and_then(|value| value.to_str().ok())
    {
        Some(token) => token,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::error("Unauthorized")),
            )
        }
    };

    let expected_token = env::var("AUTH_TOKEN").expect("Missing AUTH_TOKEN in .env");

    if auth_token != expected_token {
        return (
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::error("Unauthorized")),
        );
    }

    match appstate
        .notifications
        .create_notification(notification)
        .await
    {
        Ok(_) => {
            return (
                StatusCode::CREATED,
                Json(ApiResponse::ok("Notification created successfully")),
            )
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error("Failed to create notification")),
            )
        }
    }
}

pub async fn get_notification_by_id(
    State(appstate): State<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> impl IntoResponse {
    match appstate.notifications.get_notification(Some(&id)).await {
        Ok(Some(notification)) => {
            (StatusCode::OK, Json(ApiResponse::ok(notification))).into_response()
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::error("No notification found")),
        )
            .into_response(),
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                StatusCode::NOT_FOUND,
                Json(ApiResponse::error("Failed to fetch notification")),
            )
                .into_response()
        }
    }
}

pub async fn get_latest_notification(State(appstate): State<Arc<AppState>>) -> impl IntoResponse {
    match appstate.notifications.get_notification(None).await {
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
                StatusCode::NOT_FOUND,
                Json(ApiResponse::error("Failed to fetch notification")),
            )
                .into_response()
        }
    }
}

pub async fn get_all_notifications(State(appstate): State<Arc<AppState>>) -> impl IntoResponse {
    match appstate.notifications.get_all_notifications().await {
        Ok(notifications) => {
            return (StatusCode::ACCEPTED, Json(ApiResponse::ok(notifications))).into_response();
        }
        Err(e) => {
            eprintln!("Error getting all notifications {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error("Error fetching all notifications")),
            )
                .into_response();
        }
    };
}

pub async fn update_notifications(
    headers: axum::http::HeaderMap,
    State(appstate): State<Arc<AppState>>,
    Json(notification): Json<Notification>,
) -> impl IntoResponse {
    let auth_token = match headers
        .get("authToken")
        .and_then(|value| value.to_str().ok())
    {
        Some(token) => token,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::error("Unauthorized")),
            )
        }
    };

    let expected_token = env::var("AUTH_TOKEN").expect("Missing AUTH_TOKEN in .env");

    if auth_token != expected_token {
        return (
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::error("Unauthorized")),
        );
    }

    match appstate
        .notifications
        .update_notification(notification)
        .await
    {
        Ok(_) => {
            return (
                StatusCode::OK,
                Json(ApiResponse::ok("Notification updated successfully")),
            )
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error("Failed to update notification")),
            )
        }
    }
}
