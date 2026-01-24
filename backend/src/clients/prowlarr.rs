use std::future::Future;
use std::pin::Pin;

use reqwest::Client;

use crate::clients::ProwlarrClient;
use crate::error::AppError;
use crate::models::prowlarr::SearchResult;

pub struct RealProwlarrClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl RealProwlarrClient {
    pub fn new(base_url: &str, api_key: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key: api_key.to_string(),
        }
    }
}

impl ProwlarrClient for RealProwlarrClient {
    fn search(
        &self,
        query: &str,
        categories: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<SearchResult>, AppError>> + Send + '_>> {
        let query = query.to_string();
        let categories = categories.map(|s| s.to_string());

        Box::pin(async move {
            let mut req = self
                .client
                .get(format!("{}/api/v1/search", self.base_url))
                .header("X-Api-Key", &self.api_key)
                .query(&[("query", &query)]);

            if let Some(cats) = &categories {
                req = req.query(&[("categories", cats)]);
            }

            let results: Vec<SearchResult> = req.send().await?.json().await?;
            Ok(results)
        })
    }
}
