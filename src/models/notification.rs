use anyhow::Result;
use mongodb::{
    bson::{self, doc, DateTime},
    options::{FindOneOptions, FindOptions},
    Client, Collection, Database,
};
use serde::{Deserialize, Serialize};
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
    pub updated_at: Option<DateTime>,
}

pub struct NotificationRepo {
    collection: Collection<Notification>,
}

impl NotificationRepo {
    pub async fn new() -> Result<Self> {
        let uri = std::env::var("MONGODB_URI").expect("MONGODB_URI must be set");

        let client = Client::with_uri_str(&uri).await?;
        let db: Database = client.database("info-v2");
        let collection = db.collection("notifications");

        Ok(Self { collection })
    }

    pub async fn create_notification(&self, mut notification: Notification) -> Result<()> {
        notification.id = Some(Uuid::new_v4().to_string());
        notification.updated_at = Some(DateTime::now());
        self.collection.insert_one(notification, None).await?;
        Ok(())
    }

    pub async fn get_notification(&self, id: Option<&str>) -> Result<Option<Notification>> {
        let response = match id {
            Some(id) => {
                let filter = doc! { "id": id };
                self.collection.find_one(filter, None).await
            }
            None => {
                let options = FindOneOptions::builder()
                    .sort(doc! {"updated_at": -1})
                    .build();
                self.collection.find_one(doc! {}, options).await
            }
        };
        Ok(response?)
    }

    pub async fn get_all_notifications(&self) -> Result<Vec<Notification>> {
        let mut notifications = Vec::new();

        let find_options = FindOptions::builder()
            .sort(doc! { "updated_at": -1 })
            .build();

        let mut cursor = self.collection.find(None, find_options).await?;

        while cursor.advance().await? {
            notifications.push(cursor.deserialize_current()?);
        }

        Ok(notifications)
    }

    pub async fn update_notification(&self, notification: Notification) -> Result<bool> {
        let id = notification
            .id
            .clone()
            .ok_or_else(|| anyhow::anyhow!("Notification id must be set for update operation"))?;
        let id_exists = self
            .collection
            .count_documents(doc! {"id": id.clone()}, None)
            .await
            .unwrap_or(0)
            > 0;
        if !id_exists {
            return Err(anyhow::anyhow!(
                "Notification with id {} does not exist",
                id
            ));
        }
        let filter = doc! { "id": id };
        let update = doc! { "$set": bson::to_document(&notification)? };
        let result = self.collection.update_one(filter, update, None).await?;
        Ok(result.modified_count > 0)
    }

    pub async fn delete_notification(&self, id: &str) -> Result<bool> {
        let filter = doc! { "id": id };
        let result = self.collection.delete_one(filter, None).await?;
        Ok(result.deleted_count > 0)
    }
}
