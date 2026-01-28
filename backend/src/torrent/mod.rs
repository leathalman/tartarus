mod fake;
mod qbittorrent;

pub use fake::FakeTorrentClient;
pub use qbittorrent::QBit;

use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::error::AppError;

/// What we POST to our `/api/download` endpoint.
#[derive(Debug, Deserialize)]
pub struct AddTorrentParams {
    pub urls: String,
    #[serde(default)]
    pub category: Option<String>,
}

/// Torrent info returned by qBittorrent's API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorrentInfo {
    pub hash: String,
    pub name: String,
    pub size: i64,
    pub progress: f64,
    pub state: String,
    pub num_seeds: i64,
    pub num_leechs: i64,
    #[serde(default)]
    pub category: String,
    pub added_on: i64,
    pub eta: i64,
    pub dlspeed: i64,
}

/// Query parameters for listing torrents.
#[derive(Debug, Deserialize)]
pub struct TorrentListParams {
    #[serde(default)]
    pub filter: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
}

pub trait TorrentClient: Send + Sync {
    fn add_torrent(
        &self,
        urls: &str,
        category: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<(), AppError>> + Send + '_>>;

    fn list_torrents(
        &self,
        filter: Option<&str>,
        category: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<TorrentInfo>, AppError>> + Send + '_>>;

    fn get_torrent(
        &self,
        hash: &str,
    ) -> Pin<Box<dyn Future<Output = Result<TorrentInfo, AppError>> + Send + '_>>;
}
