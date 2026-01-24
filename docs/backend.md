# Backend Architecture

## Overview

The backend is an Axum web server that sits between the frontend and two external services:

- **Prowlarr** — indexer aggregator, searched for torrents
- **qBittorrent** — torrent client, manages downloads

The frontend never talks to Prowlarr or qBit directly. Tartarus proxies all requests, handling authentication and normalizing responses.

## Stack

| Crate | Purpose |
|-------|---------|
| axum 0.8 | HTTP framework (routing, extractors, responses) |
| tokio | Async runtime |
| reqwest | HTTP client (with cookie store for qBit sessions) |
| serde / serde_json | JSON serialization/deserialization |
| thiserror | Error type derivation |
| tracing / tracing-subscriber | Structured logging |
| tower-http | CORS and request trace middleware |

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/search?q=...&categories=...` | Search Prowlarr for torrents |
| POST | `/api/download` | Send a magnet/torrent URL to qBittorrent |
| GET | `/api/torrents?filter=...&category=...` | List all torrents from qBittorrent |
| GET | `/api/torrents/{hash}` | Get status of a single torrent |

## Repository Structure

```
backend/
├── Cargo.toml
├── src/
│   ├── main.rs                  # Tokio entrypoint, loads config, boots server
│   ├── lib.rs                   # Module declarations (for integration test access)
│   ├── config.rs                # Config struct loaded from env vars
│   ├── error.rs                 # AppError enum implementing IntoResponse
│   ├── app.rs                   # AppState (Arc<dyn Trait>), build_router()
│   ├── models/
│   │   ├── mod.rs
│   │   ├── prowlarr.rs          # SearchResult, SearchParams, Category
│   │   └── qbittorrent.rs       # TorrentInfo, AddTorrentParams, TorrentListParams
│   ├── clients/
│   │   ├── mod.rs               # ProwlarrClient + QBitClient trait definitions
│   │   ├── prowlarr.rs          # Real Prowlarr client (X-Api-Key auth)
│   │   ├── qbittorrent.rs       # Real qBit client (cookie/session auth)
│   │   ├── mock_prowlarr.rs     # Mock for tests
│   │   └── mock_qbittorrent.rs  # Mock for tests
│   └── handlers/
│       ├── mod.rs
│       ├── search.rs            # GET /api/search handler
│       ├── download.rs          # POST /api/download handler
│       └── status.rs            # GET /api/torrents handlers
└── tests/
    └── api_tests.rs             # Integration tests using mocks + tower::oneshot
```

## Design Decisions

### Trait-based clients

`ProwlarrClient` and `QBitClient` are traits stored as `Arc<dyn Trait>` in `AppState`. This allows swapping in mock implementations for testing without conditional compilation or feature flags.

### Boxed futures in traits

Trait methods return `Pin<Box<dyn Future<...> + Send + '_>>` to be dyn-compatible. The heap allocation is negligible for a web server making network calls.

### Cookie-based qBit auth

reqwest's cookie jar handles qBittorrent's SID session automatically. The client authenticates once at startup, and the cookie is sent with every subsequent request.

### Config from env vars

All external service URLs and credentials come from environment variables. Required vars cause a fast failure at startup with a clear message. `HOST` and `PORT` have defaults (`0.0.0.0:3001`).

## Running

```bash
export PROWLARR_URL=http://localhost:9696
export PROWLARR_API_KEY=your-api-key
export QBIT_URL=http://localhost:8080
export QBIT_USERNAME=admin
export QBIT_PASSWORD=adminadmin

cargo run
```

## Testing

```bash
cargo test
```

Tests use mock clients and `tower::ServiceExt::oneshot` to exercise the full HTTP layer (routing, extraction, serialization) without starting a TCP server or requiring real services.
