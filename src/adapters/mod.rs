use std::sync::Arc;

use dashmap::DashMap;


pub trait CreateShortLinkRepository {
    fn save(&self, id: String, full_url: String) -> Result<(), String>;
}

pub trait QueryFullUrlRepository {
    fn get_full_url_by_id(&self, id: &str) -> Result<String, String>;
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

impl CreateShortLinkRepository for InmemoryRepository {
    fn save(&self, id: String, full_url: String) -> Result<(), String> {
        self.storage.insert(id, full_url);
        Ok(())
    }
}

impl QueryFullUrlRepository for InmemoryRepository {
    fn get_full_url_by_id(&self, id: &str) -> Result<String, String> {
        match self.storage.get(id) {
            Some(v) => Ok(v.clone()),
            None => Err("no such key".to_owned()),
        }
    }
}
