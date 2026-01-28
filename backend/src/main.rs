use std::sync::Arc;

use backend::app::{AppState, build_router};
use backend::config::Config;
use backend::indexer::Prowlarr;
use backend::torrent::QBit;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug,tower_http=debug".into()),
        )
        .init();

    let config = Config::from_env().unwrap_or_else(|e| {
        eprintln!("Configuration error: {e}");
        std::process::exit(1);
    });

    let state = AppState {
        indexer: Arc::new(Prowlarr::new(
            &config.prowlarr_url,
            &config.prowlarr_api_key,
        )),
        torrent: Arc::new(
            QBit::new(
                &config.qbit_url,
                &config.qbit_username,
                &config.qbit_password,
            )
            .await,
        ),
    };

    let app = build_router(state);
    let addr = config.bind_addr();
    tracing::info!("Listening on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
