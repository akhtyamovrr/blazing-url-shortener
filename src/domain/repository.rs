use async_trait::async_trait;

use crate::error::Error;

#[async_trait]
pub trait CreateShortLinkRepository: Send + Sync {
    async fn save(&self, id: String, full_url: String) -> Result<(), Error>;
}

#[async_trait]
pub trait QueryFullUrlRepository: Send + Sync {
    async fn get_full_url_by_id(&self, id: String) -> Result<String, Error>;
}
