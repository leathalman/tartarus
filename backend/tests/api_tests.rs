use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

use backend::app::{AppState, build_router};
use backend::clients::mock_prowlarr::MockProwlarrClient;
use backend::clients::mock_qbittorrent::MockQBitClient;
use backend::models::prowlarr::SearchResult;
use backend::models::qbittorrent::TorrentInfo;

fn test_state() -> AppState {
    AppState {
        prowlarr: Arc::new(MockProwlarrClient),
        qbit: Arc::new(MockQBitClient),
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

    // Axum returns 400 when required query params are missing
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
                .body(Body::from(
                    r#"{"urls": "magnet:?xt=urn:btih:abc123"}"#,
                ))
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
