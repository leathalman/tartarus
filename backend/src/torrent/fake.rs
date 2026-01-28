use std::future::Future;
use std::pin::Pin;

use super::{TorrentClient, TorrentInfo};
use crate::error::AppError;

pub struct FakeTorrentClient;

impl FakeTorrentClient {
    fn mock_torrent() -> TorrentInfo {
        TorrentInfo {
            hash: "abcdef1234567890".to_string(),
            name: "Mock.Torrent.Name".to_string(),
            size: 1_500_000_000,
            progress: 0.75,
            state: "downloading".to_string(),
            num_seeds: 10,
            num_leechs: 3,
            category: "movies".to_string(),
            added_on: 1700000000,
            eta: 3600,
            dlspeed: 5_000_000,
        }
    }
}

impl TorrentClient for FakeTorrentClient {
    fn add_torrent(
        &self,
        _urls: &str,
        _category: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<(), AppError>> + Send + '_>> {
        Box::pin(async { Ok(()) })
    }

    fn list_torrents(
        &self,
        _filter: Option<&str>,
        _category: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<TorrentInfo>, AppError>> + Send + '_>> {
        Box::pin(async { Ok(vec![Self::mock_torrent()]) })
    }

    fn get_torrent(
        &self,
        _hash: &str,
    ) -> Pin<Box<dyn Future<Output = Result<TorrentInfo, AppError>> + Send + '_>> {
        Box::pin(async { Ok(Self::mock_torrent()) })
    }
}
