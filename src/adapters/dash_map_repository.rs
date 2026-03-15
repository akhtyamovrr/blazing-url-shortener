use std::sync::Arc;

use async_trait::async_trait;
#[cfg(test)]
use dashmap::DashMap;

#[cfg(test)]
use crate::domain::repository::{CreateShortLinkRepository, QueryFullUrlRepository};
use crate::{error::Error};

#[derive(Clone)]
#[cfg(test)]
pub struct InmemoryRepository {
    storage: Arc<DashMap<String, String>>,
}

#[cfg(test)]
impl InmemoryRepository {
    pub fn new(storage: Arc<DashMap<String, String>>) -> Self {
        Self { storage }
    }
}

#[async_trait]
#[cfg(test)]
impl CreateShortLinkRepository for InmemoryRepository {
    async fn save(&self, id: String, full_url: String) -> Result<(), Error> {
        self.storage.insert(id, full_url);
        Ok(())
    }
}

#[async_trait]
#[cfg(test)]
impl QueryFullUrlRepository for InmemoryRepository {
    async fn get_full_url_by_id(&self, id: String) -> Result<String, Error> {
        match self.storage.get(&id) {
            Some(v) => Ok(v.clone()),
            None => Err(Error::NotFound(id)),
        }
    }
}