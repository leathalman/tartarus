use std::future::Future;
use std::pin::Pin;

use super::{Category, Indexer, SearchResult};
use crate::error::AppError;

pub struct FakeIndexer;

impl Indexer for FakeIndexer {
    fn search(
        &self,
        query: &str,
        _categories: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<SearchResult>, AppError>> + Send + '_>> {
        let query = query.to_string();
        Box::pin(async move {
            Ok(vec![SearchResult {
                title: format!("{query} - Mock Result"),
                size: 1_500_000_000,
                seeders: Some(42),
                leechers: Some(5),
                download_url: Some("https://example.com/download/mock.torrent".to_string()),
                magnet_url: Some("magnet:?xt=urn:btih:mock_hash".to_string()),
                indexer: Some("MockIndexer".to_string()),
                categories: vec![Category {
                    id: 2000,
                    name: Some("Movies".to_string()),
                }],
            }])
        })
    }
}
