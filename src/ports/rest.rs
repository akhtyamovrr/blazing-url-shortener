use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

use crate::{
    adapters::{CreateShortLinkRepository, QueryFullUrlRepository},
    app::{
        command::create_short_link::CreateShortLinkCommand, query::query_full_url::QueryFullUrl,
    },
    error::Error,
    id_provider::IDProvider,
};

pub struct Container<I, R, Q>
where
    I: IDProvider,
    R: CreateShortLinkRepository,
    Q: QueryFullUrlRepository,
{
    pub create_short_command: CreateShortLinkCommand<I, R>,
    pub query_full_url: QueryFullUrl<Q>,
}

pub struct Server<I, R, Q>
where
    I: IDProvider + Send + Sync + 'static,
    R: CreateShortLinkRepository + Send + Sync + 'static,
    Q: QueryFullUrlRepository + Send + Sync + 'static,
{
    port: u16,
    container: Arc<Container<I, R, Q>>,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            Error::NotFound(id) => (
                http::StatusCode::NOT_FOUND,
                format!("URL not found for {id}"),
            ),
            Error::UrlSyntax => (http::StatusCode::BAD_REQUEST, "Invalid URL".to_owned()),
            Error::DbFailure => (
                http::StatusCode::INTERNAL_SERVER_ERROR,
                "Server error, please try again later".to_owned(),
            ),
        };
        (status, Json(ErrorResponse { message })).into_response()
    }
}

impl<I, R, Q> Container<I, R, Q>
where
    I: IDProvider,
    R: CreateShortLinkRepository,
    Q: QueryFullUrlRepository,
{
    pub fn new(id_provider: I, create_short_link_repo: R, query_full_url_repo: Q) -> Self {
        let create_short_command = CreateShortLinkCommand::new(id_provider, create_short_link_repo);
        let query_full_url = QueryFullUrl::new(query_full_url_repo);
        Container {
            create_short_command,
            query_full_url,
        }
    }
}

impl<I, R, Q> Server<I, R, Q>
where
    I: IDProvider + Send + Sync + 'static,
    R: CreateShortLinkRepository + Send + Sync + 'static,
    Q: QueryFullUrlRepository + Send + Sync + 'static,
{
    pub fn new(port: u16, container: Arc<Container<I, R, Q>>) -> Self {
        Server { port, container }
    }

    pub async fn run(self) {
        let addr = format!("0.0.0.0:{}", self.port);
        let router = get_router(self.container);
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        axum::serve(listener, router).await.unwrap();
    }
}

fn get_router<I, R, Q>(container: Arc<Container<I, R, Q>>) -> Router
where
    I: IDProvider + Send + Sync + 'static,
    R: CreateShortLinkRepository + Send + Sync + 'static,
    Q: QueryFullUrlRepository + Send + Sync + 'static,
{
    Router::new()
        .route("/{id}", get(get_full_url))
        .route("/", post(shorten_url))
        .with_state(container)
}

#[derive(Deserialize, Serialize)]
struct CreateShortUrlRequest {
    url: String,
}

#[derive(Deserialize, Serialize)]
struct ShortUrlResponse {
    id: String,
}

async fn shorten_url<I, R, Q>(
    State(container): State<Arc<Container<I, R, Q>>>,
    Json(input): Json<CreateShortUrlRequest>,
) -> Result<Json<ShortUrlResponse>, Error>
where
    I: IDProvider + Send + Sync + 'static,
    R: CreateShortLinkRepository + Send + Sync + 'static,
    Q: QueryFullUrlRepository + Send + Sync + 'static,
{
    container
        .create_short_command
        .execute(input.url)
        .await
        .map(|id| Json(ShortUrlResponse { id }))
}

#[derive(serde::Deserialize, serde::Serialize)]
struct QueryFullUrlResponse {
    url: String,
}

impl From<String> for QueryFullUrlResponse {
    fn from(url: String) -> Self {
        QueryFullUrlResponse { url }
    }
}

async fn get_full_url<I, R, Q>(
    Path(id): Path<String>,
    State(container): State<Arc<Container<I, R, Q>>>,
) -> Result<Json<QueryFullUrlResponse>, Error>
where
    I: IDProvider + Send + Sync + 'static,
    R: CreateShortLinkRepository + Send + Sync + 'static,
    Q: QueryFullUrlRepository + Send + Sync + 'static,
{
    container
        .query_full_url
        .execute(&id)
        .await
        .map(|url| Json(QueryFullUrlResponse::from(url)))
}
