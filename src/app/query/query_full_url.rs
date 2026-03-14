use crate::adapters::QueryFullUrlRepository;

pub struct QueryFullUrl<Q>
where
    Q: QueryFullUrlRepository,
{
    repo: Q,
}

impl<Q> QueryFullUrl<Q>
where
    Q: QueryFullUrlRepository,
{
    pub fn new(repo: Q) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: &str) -> Result<String, String> {
        self.repo.get_full_url_by_id(id)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use dashmap::DashMap;

    use crate::adapters::{InmemoryRepository, QueryFullUrlRepository};

    #[tokio::test]
    async fn should_return_value() {
        let storage = Arc::new(DashMap::new());
        storage.insert("123".to_owned(), "https://yandex.ru".to_owned());
        let repo = InmemoryRepository::new(storage);

        let result = repo.get_full_url_by_id("123").unwrap();
        assert_eq!("https://yandex.ru", result);
    }

    #[tokio::test]
    async fn should_not_return_value() {
        let storage = Arc::new(DashMap::new());
        let repo = InmemoryRepository::new(storage);

        let result = repo.get_full_url_by_id("1234");
        assert!(result.is_err());
    }
}
