use std::sync::Arc;

use axum::routing::{get, post};
use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::clients::{ProwlarrClient, QBitClient};
use crate::handlers;

#[derive(Clone)]
pub struct AppState {
    pub prowlarr: Arc<dyn ProwlarrClient>,
    pub qbit: Arc<dyn QBitClient>,
}

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/api/search", get(handlers::search::search))
        .route("/api/download", post(handlers::download::download))
        .route("/api/torrents", get(handlers::status::list_torrents))
        .route("/api/torrents/{hash}", get(handlers::status::get_torrent))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
