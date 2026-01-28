use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::error::AppError;
use crate::indexers::{Indexer, SearchParams, SearchResult};
use crate::torrent::{AddTorrentParams, TorrentClient, TorrentInfo, TorrentListParams};

#[derive(Clone)]
pub struct AppState {
    pub indexer: Arc<dyn Indexer>,
    pub torrent: Arc<dyn TorrentClient>,
}

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/api/search", get(search))
        .route("/api/download", post(download))
        .route("/api/torrents", get(list_torrents))
        .route("/api/torrents/{hash}", get(get_torrent))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

async fn search(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Result<Json<Vec<SearchResult>>, AppError> {
    let results = state
        .indexer
        .search(&params.q, params.categories.as_deref())
        .await?;
    Ok(Json(results))
}

async fn download(
    State(state): State<AppState>,
    Json(params): Json<AddTorrentParams>,
) -> Result<StatusCode, AppError> {
    state
        .torrent
        .add_torrent(&params.urls, params.category.as_deref())
        .await?;
    Ok(StatusCode::OK)
}

async fn list_torrents(
    State(state): State<AppState>,
    Query(params): Query<TorrentListParams>,
) -> Result<Json<Vec<TorrentInfo>>, AppError> {
    let torrents = state
        .torrent
        .list_torrents(params.filter.as_deref(), params.category.as_deref())
        .await?;
    Ok(Json(torrents))
}

async fn get_torrent(
    State(state): State<AppState>,
    Path(hash): Path<String>,
) -> Result<Json<TorrentInfo>, AppError> {
    let torrent = state.torrent.get_torrent(&hash).await?;
    Ok(Json(torrent))
}
