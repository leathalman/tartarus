use axum::extract::{Path, Query, State};
use axum::Json;

use crate::app::AppState;
use crate::error::AppError;
use crate::models::qbittorrent::{TorrentInfo, TorrentListParams};

pub async fn list_torrents(
    State(state): State<AppState>,
    Query(params): Query<TorrentListParams>,
) -> Result<Json<Vec<TorrentInfo>>, AppError> {
    let torrents = state
        .qbit
        .list_torrents(params.filter.as_deref(), params.category.as_deref())
        .await?;

    Ok(Json(torrents))
}

pub async fn get_torrent(
    State(state): State<AppState>,
    Path(hash): Path<String>,
) -> Result<Json<TorrentInfo>, AppError> {
    let torrent = state.qbit.get_torrent(&hash).await?;
    Ok(Json(torrent))
}
