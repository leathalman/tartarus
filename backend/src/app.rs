use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::error::AppError;
use crate::indexer::{Indexer, SearchParams, SearchResult};
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use crate::indexer::FakeIndexer;
    use crate::torrent::FakeTorrentClient;

    fn test_state() -> AppState {
        AppState {
            indexer: Arc::new(FakeIndexer),
            torrent: Arc::new(FakeTorrentClient),
        }
    }

    #[tokio::test]
    async fn test_search() {
        let app = build_router(test_state());

        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/api/search?q=test+movie")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);

        let body = resp.into_body().collect().await.unwrap().to_bytes();
        let results: Vec<SearchResult> = serde_json::from_slice(&body).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "test movie - Mock Result");
        assert_eq!(results[0].seeders, Some(42));
    }

    #[tokio::test]
    async fn test_search_missing_query() {
        let app = build_router(test_state());

        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/api/search")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_download() {
        let app = build_router(test_state());

        let resp = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/download")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"urls": "magnet:?xt=urn:btih:abc123"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_list_torrents() {
        let app = build_router(test_state());

        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/api/torrents")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);

        let body = resp.into_body().collect().await.unwrap().to_bytes();
        let torrents: Vec<TorrentInfo> = serde_json::from_slice(&body).unwrap();

        assert_eq!(torrents.len(), 1);
        assert_eq!(torrents[0].name, "Mock.Torrent.Name");
        assert_eq!(torrents[0].progress, 0.75);
    }

    #[tokio::test]
    async fn test_get_torrent() {
        let app = build_router(test_state());

        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/api/torrents/abcdef1234567890")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);

        let body = resp.into_body().collect().await.unwrap().to_bytes();
        let torrent: TorrentInfo = serde_json::from_slice(&body).unwrap();

        assert_eq!(torrent.hash, "abcdef1234567890");
        assert_eq!(torrent.state, "downloading");
    }
}
