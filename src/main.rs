use std::sync::Arc;

use crate::{
    adapters::InmemoryRepository, id_provider::NanoIdProvider, ports::rest::{Container, Server}
};
use dashmap::DashMap;

mod adapters;
mod app;
mod id_provider;
mod ports;

#[tokio::main]
async fn main() {
    let id_provider = NanoIdProvider;
    let storage = Arc::new(DashMap::new());
    let create_short_link_repo = InmemoryRepository::new(storage.clone());
    let query_full_url_repo = InmemoryRepository::new(storage.clone());

    // let mut cfg = Config::new();
    // cfg.host = Some("localhost".to_string());
    // cfg.user = Some("postgres".to_string());
    // cfg.dbname = Some("test_db".to_string());
    // let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

    // let create_short_link_repo = PgRepository::new(pool.clone());
    // let query_full_url_repo = PgRepository::new(pool.clone());

    let container = Arc::new(Container::new(
        id_provider,
        create_short_link_repo,
        query_full_url_repo,
    ));
    let server = Server::new(3000, container);
    server.run().await;
}
