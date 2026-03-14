use std::sync::Arc;

use async_trait::async_trait;

use dashmap::DashMap;


#[async_trait]
pub trait CreateShortLinkRepository: Send + Sync {
    async fn save(&self, id: String, full_url: String) -> Result<(), String>;
}

#[async_trait] 
pub trait QueryFullUrlRepository: Send + Sync {
    async fn get_full_url_by_id(&self, id: String) -> Result<String, String>;
}

#[derive(Clone)]
pub struct InmemoryRepository {
    storage: Arc<DashMap<String, String>>,
}

impl InmemoryRepository {
    pub fn new(storage: Arc<DashMap<String, String>>) -> Self {
        Self { storage }
    }
}

#[async_trait] 
impl CreateShortLinkRepository for InmemoryRepository {
    async fn save(&self, id: String, full_url: String) -> Result<(), String> {
        self.storage.insert(id, full_url);
        Ok(())
    }
}

#[async_trait]
impl QueryFullUrlRepository for InmemoryRepository {
    async fn get_full_url_by_id(&self, id: String) -> Result<String, String> {
        match self.storage.get(&id) {
            Some(v) => Ok(v.clone()),
            None => Err("no such key".to_owned()),
        }
    }
}
