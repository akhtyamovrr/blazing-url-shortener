use crate::{adapters::CreateShortLinkRepository, id_provider::IDProvider};

pub struct CreateShortLinkCommand<I, R>
where
    I: IDProvider,
    R: CreateShortLinkRepository,
{
    id_provider: I,
    repo: R,
}

impl<I, R> CreateShortLinkCommand<I, R>
where
    I: IDProvider,
    R: CreateShortLinkRepository,
{
    pub fn new(id_provider: I, repo: R) -> Self {
        Self { id_provider, repo }
    }

    pub async fn execute(&self, url: String) -> Result<String, String> {
        let id = self.id_provider.provide_id();
        self.repo.save(id.clone(), url)?;
        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use dashmap::DashMap;

    use crate::adapters::InmemoryRepository;

    use super::*;

    #[tokio::test]
    async fn get_not_empty_short_link() {
        let id_provider = crate::id_provider::NanoIdProvider;
        let storage = Arc::new(DashMap::new());
        let repo = InmemoryRepository::new(storage);
        let command = CreateShortLinkCommand::new(id_provider, repo);

        let result = command.execute("https://google.com".to_owned()).await;

        assert_ne!(result, Ok("".to_owned()));
    }

    #[tokio::test]
    async fn get_different_links() {
        let id_provider = crate::id_provider::NanoIdProvider;
        let storage = Arc::new(DashMap::new());
        let repo = InmemoryRepository::new(storage);
        let command = CreateShortLinkCommand::new(id_provider, repo);
        let result_google = command.execute("https://google.com".to_owned()).await;
        let result_yandex = command.execute("https://ya.ru".to_owned()).await;
        assert_ne!(result_google, result_yandex);
    }

    #[tokio::test]
    async fn storage_has_item_after_command_execution() {
        let id_provider = crate::id_provider::NanoIdProvider;
        let storage = Arc::new(DashMap::new());
        let repo = InmemoryRepository::new(storage.clone());
        let command = CreateShortLinkCommand::new(id_provider, repo);
        let id = command
            .execute("https://google.com".to_owned())
            .await
            .unwrap();
        assert_eq!(storage.len(), 1);
        let full_url_value = storage.get(&id).unwrap();
        assert_eq!(full_url_value.value(), "https://google.com");
    }
}
