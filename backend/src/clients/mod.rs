pub mod mock_prowlarr;
pub mod mock_qbittorrent;
pub mod prowlarr;
pub mod qbittorrent;

use std::future::Future;
use std::pin::Pin;

use crate::error::AppError;
use crate::models::prowlarr::SearchResult;
use crate::models::qbittorrent::TorrentInfo;

pub trait ProwlarrClient: Send + Sync {
    fn search(
        &self,
        query: &str,
        categories: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<SearchResult>, AppError>> + Send + '_>>;
}

pub trait QBitClient: Send + Sync {
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
