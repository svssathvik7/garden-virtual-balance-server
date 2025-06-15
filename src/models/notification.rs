use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, PgPool};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub title: String,
    pub description: String,
    pub image: String,
    pub link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

pub struct NotificationRepo {
    pool: PgPool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedNotifications {
    pub data: Vec<Notification>,
    pub page: i64,
    pub per_page: i64,
    pub total: u64,
}

impl NotificationRepo {
    pub async fn new() -> Result<Self> {
        let uri = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPool::connect(&uri).await?;

        // Create table if not exists
        sqlx::query(
            "
            CREATE TABLE IF NOT EXISTS notifications (
                id UUID PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT NOT NULL,
                image TEXT NOT NULL,
                link TEXT NOT NULL,
                updated_at TIMESTAMPTZ NOT NULL
            )
        ",
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }

    pub async fn create_notification(&self, mut notification: Notification) -> Result<()> {
        notification.id = Some(Uuid::new_v4().to_string());
        notification.updated_at = Some(Utc::now());

        sqlx::query(
            "
            INSERT INTO notifications (id, title, description, image, link, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
        ",
        )
        .bind(Uuid::parse_str(&notification.id.unwrap())?)
        .bind(&notification.title)
        .bind(&notification.description)
        .bind(&notification.image)
        .bind(&notification.link)
        .bind(notification.updated_at.unwrap())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_notification(&self, id: Option<&str>) -> Result<Option<Notification>> {
        match id {
            Some(id) => {
                let notification = sqlx::query_as!(
                    Notification,
                    r#"
                    SELECT 
                        id::TEXT as "id?",
                        title,
                        description,
                        image,
                        link,
                        updated_at as "updated_at?"
                    FROM notifications
                    WHERE id = $1
                    "#,
                    Uuid::parse_str(id)?
                )
                .fetch_optional(&self.pool)
                .await?;

                Ok(notification)
            }
            None => {
                let notification = sqlx::query_as!(
                    Notification,
                    r#"
                    SELECT 
                        id::TEXT as "id?",
                        title,
                        description,
                        image,
                        link,
                        updated_at as "updated_at?"
                    FROM notifications
                    ORDER BY updated_at DESC
                    LIMIT 1
                    "#
                )
                .fetch_optional(&self.pool)
                .await?;

                Ok(notification)
            }
        }
    }

    pub async fn get_all_notifications(
        &self,
        page: i64,
        per_page: i64,
    ) -> Result<PaginatedNotifications> {
        let notifications = sqlx::query_as!(
            Notification,
            r#"
            SELECT 
                id::TEXT as "id?",
                title,
                description,
                image,
                link,
                updated_at as "updated_at?"
            FROM notifications
            ORDER BY updated_at DESC
            LIMIT $2 OFFSET $1
            "#,
            (page - 1) * per_page,
            per_page,
        )
        .fetch_all(&self.pool)
        .await?;

        let total = notifications.len() as u64;

        Ok(PaginatedNotifications {
            data: notifications,
            page,
            per_page,
            total,
        })
    }

    pub async fn update_notification(&self, notification: Notification) -> Result<bool> {
        let id = notification
            .id
            .clone()
            .ok_or_else(|| anyhow::anyhow!("Notification id must be set for update operation"))?;

        let result: PgQueryResult = sqlx::query(
            "
            UPDATE notifications
            SET title = $1, description = $2, image = $3, link = $4, updated_at = $5
            WHERE id = $6
        ",
        )
        .bind(&notification.title)
        .bind(&notification.description)
        .bind(&notification.image)
        .bind(&notification.link)
        .bind(Utc::now())
        .bind(Uuid::parse_str(&id)?)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn set_latest_notification(&self, id: &str) -> Result<bool> {
        let result = sqlx::query!(
            r#"
            UPDATE notifications
            SET updated_at = $1
            WHERE id = $2
            "#,
            Utc::now(),
            Uuid::parse_str(id)?
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete_notification(&self, id: &str) -> Result<bool> {
        let result = sqlx::query(
            "
            DELETE FROM notifications
            WHERE id = $1
        ",
        )
        .bind(Uuid::parse_str(id)?)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
