use std::sync::Arc;

use crate::cache::{assets_cache::AssetsCache, blocknumbers_cache::BlockNumbers};
use crate::models::notification::NotificationRepo;

pub struct AppState {
    pub cached_assets: Arc<AssetsCache>,
    pub block_numbers: Arc<BlockNumbers>,
    pub notification_repo: Arc<NotificationRepo>,
}
