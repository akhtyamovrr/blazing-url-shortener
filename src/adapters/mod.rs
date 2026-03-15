use std::sync::Arc;

use async_trait::async_trait;

use dashmap::DashMap;
use deadpool_postgres::Pool;

use crate::error::Error;

#[async_trait]
pub trait CreateShortLinkRepository: Send + Sync {
    async fn save(&self, id: String, full_url: String) -> Result<(), Error>;
}

#[async_trait]
pub trait QueryFullUrlRepository: Send + Sync {
    async fn get_full_url_by_id(&self, id: String) -> Result<String, Error>;
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
    async fn save(&self, id: String, full_url: String) -> Result<(), Error> {
        self.storage.insert(id, full_url);
        Ok(())
    }
}

#[async_trait]
impl QueryFullUrlRepository for InmemoryRepository {
    async fn get_full_url_by_id(&self, id: String) -> Result<String, Error> {
        match self.storage.get(&id) {
            Some(v) => Ok(v.clone()),
            None => Err(Error::NotFound(id)),
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
    async fn save(&self, id: String, full_url: String) -> Result<(), Error> {
        let client = self.pool.get().await.unwrap();
        client
            .execute(
                "INSERT INTO links (id, full_url) VALUES ($1, $2)",
                &[&id, &full_url],
            )
            .await
            .map_err(|_| Error::DbFailure)?;
        Ok(())
    }
}

#[async_trait]
impl QueryFullUrlRepository for PgRepository {
    async fn get_full_url_by_id(&self, id: String) -> Result<String, Error> {
        let client = self.pool.get().await.map_err(|_| Error::DbFailure)?;
        let stmt = client
            .prepare("SELECT full_url FROM links where id = $1")
            .await
            .map_err(|_| Error::DbFailure)?;
        let row = client
            .query_opt(&stmt, &[&id])
            .await
            .map_err(|_| Error::DbFailure)?;

        return match row {
            Some(r) => Ok(r.get(0)),
            None => Err(Error::NotFound(id)),
        };
    }
}
