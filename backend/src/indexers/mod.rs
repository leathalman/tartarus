mod fake;
mod prowlarr;

pub use fake::FakeIndexer;
pub use prowlarr::Prowlarr;

use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::error::AppError;

/// Query parameters for the search endpoint.
#[derive(Debug, Deserialize)]
pub struct SearchParams {
    pub q: String,
    #[serde(default)]
    pub categories: Option<String>,
}

/// A single result from Prowlarr's search API.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub title: String,
    pub size: i64,
    pub seeders: Option<i32>,
    pub leechers: Option<i32>,
    pub download_url: Option<String>,
    pub magnet_url: Option<String>,
    pub indexer: Option<String>,
    pub categories: Vec<Category>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: i32,
    pub name: Option<String>,
}

pub trait Indexer: Send + Sync {
    fn search(
        &self,
        query: &str,
        categories: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<SearchResult>, AppError>> + Send + '_>>;
}
