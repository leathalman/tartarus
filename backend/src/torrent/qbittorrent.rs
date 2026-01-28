use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use reqwest::Client;

use super::{TorrentClient, TorrentInfo};
use crate::error::AppError;

pub struct QBit {
    client: Client,
    base_url: String,
}

impl QBit {
    pub async fn new(base_url: &str, username: &str, password: &str) -> Self {
        let jar = Arc::new(reqwest::cookie::Jar::default());
        let base_url = base_url.trim_end_matches('/').to_string();

        let client = Client::builder()
            .cookie_provider(jar)
            .build()
            .expect("Failed to build reqwest client");

        // Authenticate immediately — the cookie jar stores the SID
        let resp = client
            .post(format!("{base_url}/api/v2/auth/login"))
            .header("Referer", &base_url)
            .form(&[("username", username), ("password", password)])
            .send()
            .await
            .expect("Failed to connect to qBittorrent");

        if !resp.status().is_success() {
            panic!("qBittorrent auth failed with status {}", resp.status());
        }

        Self { client, base_url }
    }
}

impl TorrentClient for QBit {
    fn add_torrent(
        &self,
        urls: &str,
        category: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<(), AppError>> + Send + '_>> {
        let urls = urls.to_string();
        let category = category.map(|s| s.to_string());

        Box::pin(async move {
            let mut form = vec![("urls", urls.as_str())];
            let cat;
            if let Some(c) = &category {
                cat = c.clone();
                form.push(("category", &cat));
            }

            let resp = self
                .client
                .post(format!("{}/api/v2/torrents/add", self.base_url))
                .header("Referer", &self.base_url)
                .form(&form)
                .send()
                .await?;

            if !resp.status().is_success() {
                return Err(AppError::ExternalService(format!(
                    "qBittorrent add failed: {}",
                    resp.status()
                )));
            }

            Ok(())
        })
    }

    fn list_torrents(
        &self,
        filter: Option<&str>,
        category: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<TorrentInfo>, AppError>> + Send + '_>> {
        let filter = filter.map(|s| s.to_string());
        let category = category.map(|s| s.to_string());

        Box::pin(async move {
            let mut req = self
                .client
                .get(format!("{}/api/v2/torrents/info", self.base_url))
                .header("Referer", &self.base_url);

            if let Some(f) = &filter {
                req = req.query(&[("filter", f)]);
            }
            if let Some(c) = &category {
                req = req.query(&[("category", c)]);
            }

            let torrents: Vec<TorrentInfo> = req.send().await?.json().await?;
            Ok(torrents)
        })
    }

    fn get_torrent(
        &self,
        hash: &str,
    ) -> Pin<Box<dyn Future<Output = Result<TorrentInfo, AppError>> + Send + '_>> {
        let hash = hash.to_string();

        Box::pin(async move {
            let torrents: Vec<TorrentInfo> = self
                .client
                .get(format!("{}/api/v2/torrents/info", self.base_url))
                .header("Referer", &self.base_url)
                .query(&[("hashes", &hash)])
                .send()
                .await?
                .json()
                .await?;

            torrents
                .into_iter()
                .next()
                .ok_or_else(|| AppError::NotFound(format!("No torrent with hash {hash}")))
        })
    }
}
