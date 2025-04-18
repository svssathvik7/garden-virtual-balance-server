use std::sync::Arc;

use tokio::sync::Mutex;

use crate::cache::{assets_cache::AssetsCache, blocknumbers_cache::BlockNumbers};

pub struct AppState {
    pub cached_assets: Arc<AssetsCache>,
    pub block_numbers: Arc<BlockNumbers>,
}
