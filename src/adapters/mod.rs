use std::sync::Arc;

use async_trait::async_trait;

use dashmap::DashMap;
use deadpool_postgres::Pool;

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

pub struct PgRepository {
    pool: Pool,
}

impl PgRepository {
    pub fn new(pool: Pool) -> Self {
        PgRepository { pool }
    }
}

#[async_trait]
impl CreateShortLinkRepository for PgRepository {
    async fn save(&self, id: String, full_url: String) -> Result<(), String> {
        let client = self.pool.get().await.unwrap();
        client
            .execute(
                "INSERT INTO links (id, full_url) VALUES ($1, $2)",
                &[&id, &full_url],
            )
            .await
            .unwrap();
        Ok(())
    }
}

#[async_trait]
impl QueryFullUrlRepository for PgRepository {
    async fn get_full_url_by_id(&self, id: String) -> Result<String, String> {
        let client = self.pool.get().await.unwrap();
        let stmt = client
            .prepare("SELECT full_url FROM links where id = $1")
            .await
            .unwrap();
        let row = client.query_one(&stmt, &[&id]).await.unwrap();
        let value = row.get(0);
        return Ok(value);
    }
}
