use serde::{Deserialize, Serialize};

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
