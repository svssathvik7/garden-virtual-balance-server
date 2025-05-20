use anyhow::Result;
use mongodb::{
    bson::{self, doc},
    Client, Collection, Database,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    pub id: String,
    pub title: String,
    pub description: String,
    pub image: String,
    pub link: String,
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

    pub async fn create_notification(&self, notification: Notification) -> Result<()> {
        let id_exists = self
            .collection
            .count_documents(doc! {"id": notification.id.clone()}, None)
            .await
            .unwrap_or(0)
            > 0;
        if id_exists {
            return Err(anyhow::anyhow!(
                "Notification with id {} already exists",
                notification.id
            ));
        }
        self.collection.insert_one(notification, None).await?;
        Ok(())
    }

    pub async fn get_notification(&self, id: &str) -> Result<Option<Notification>> {
        let filter = doc! { "id": id };
        Ok(self.collection.find_one(filter, None).await?)
    }

    pub async fn get_all_notifications(&self) -> Result<Vec<Notification>> {
        let mut notifications = Vec::new();
        let mut cursor = self.collection.find(None, None).await?;
        while cursor.advance().await? {
            notifications.push(cursor.deserialize_current()?)
        }
        Ok(notifications)
    }

    pub async fn update_notification(&self, id: &str, notification: Notification) -> Result<bool> {
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
