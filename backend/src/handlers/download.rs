use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;

use crate::app::AppState;
use crate::error::AppError;
use crate::models::qbittorrent::AddTorrentParams;

pub async fn download(
    State(state): State<AppState>,
    Json(params): Json<AddTorrentParams>,
) -> Result<StatusCode, AppError> {
    state
        .qbit
        .add_torrent(&params.urls, params.category.as_deref())
        .await?;

    Ok(StatusCode::OK)
}
