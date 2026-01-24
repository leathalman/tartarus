use serde::{Deserialize, Serialize};

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
