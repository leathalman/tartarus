# Prowlarr API Reference

**Default Port:** 9696
**Base URL:** `http://{host}:9696/api/v1/`
**Official Docs:** https://prowlarr.com/docs/api/
**OpenAPI Spec:** https://raw.githubusercontent.com/Prowlarr/Prowlarr/develop/src/Prowlarr.Api.V1/openapi.json

## Authentication

Prowlarr uses API key authentication. The key is auto-generated on first boot and can be found in Settings > General.

Pass the key via header (preferred) or query parameter:

```
X-Api-Key: <your-api-key>
```

```
GET /api/v1/search?apikey=<your-api-key>&query=...
```

## Search

The primary integration point. Searches across all configured indexers and returns results with download URLs.

### `GET /api/v1/search`

| Parameter | Description |
|-----------|-------------|
| `query` | Search term (URL-encoded) |
| `indexerIds` | Indexer IDs to search. Use `-1` for all. Repeat param for multiple: `&indexerIds=1&indexerIds=2` |
| `categories` | Category IDs. Repeat param for multiple: `&categories=2000&categories=5000` |
| `type` | `search`, `tvsearch`, `moviesearch`, `musicsearch`, `booksearch` |

**Do NOT comma-separate `categories` or `indexerIds`.** Repeat the parameter instead.

### Search Response Fields

| Field | Description |
|-------|-------------|
| `guid` | Unique result identifier |
| `indexerId` | Which indexer returned this result |
| `indexer` | Indexer name |
| `title` | Release title |
| `size` | File size in bytes |
| `publishDate` | When the release was published |
| `downloadUrl` | Magnet link or .torrent URL — pass this to qBittorrent |
| `infoUrl` | Link to the release page |
| `categories` | Category list |
| `protocol` | `torrent` or `usenet` |
| `seeders` | Number of seeders |
| `leechers` | Number of leechers |
| `imdbId` | IMDB ID (if available) |
| `tmdbId` | TMDB ID (if available) |
| `tvdbId` | TVDB ID (if available) |

## Indexer Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/indexer` | List all configured indexers |
| POST | `/api/v1/indexer` | Add a new indexer |
| GET | `/api/v1/indexer/{id}` | Get specific indexer |
| PUT | `/api/v1/indexer/{id}` | Update indexer |
| DELETE | `/api/v1/indexer/{id}` | Remove indexer |
| GET | `/api/v1/indexer/schema` | Available indexer type templates |
| POST | `/api/v1/indexer/test` | Test an indexer connection |
| POST | `/api/v1/indexer/testall` | Test all indexers |

## Download Client Configuration

Prowlarr can be configured to know about download clients (for direct grabs).

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/downloadclient` | List download clients |
| POST | `/api/v1/downloadclient` | Add download client |
| PUT | `/api/v1/downloadclient/{id}` | Update download client |
| DELETE | `/api/v1/downloadclient/{id}` | Remove download client |
| POST | `/api/v1/downloadclient/test` | Test client connectivity |
| GET | `/api/v1/downloadclient/schema` | Available client type templates |

## System & Health

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/ping` | Simple availability check (no auth required) |
| GET | `/api/v1/system/status` | Version, OS, runtime info |
| GET | `/api/v1/health` | Health check |
| GET | `/api/v1/indexerstats` | Indexer performance stats |
| GET | `/api/v1/indexerstatus` | Current indexer statuses |

## History

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/history` | Paginated search/grab history |
| GET | `/api/v1/history/indexer` | History filtered by indexer |

## Categories

Common category IDs:

| ID | Category |
|----|----------|
| 2000 | Movies |
| 2010 | Movies/Foreign |
| 2020 | Movies/Other |
| 2030 | Movies/SD |
| 2040 | Movies/HD |
| 2045 | Movies/UHD |
| 2050 | Movies/BluRay |
| 2060 | Movies/3D |
| 5000 | TV |
| 5010 | TV/Foreign |
| 5020 | TV/SD |
| 5030 | TV/HD |
| 5040 | TV/UHD |
| 5045 | TV/Other |
| 5050 | TV/Sport |
| 5060 | TV/Anime |
| 5070 | TV/Documentary |

## Notes for Tartarus Integration

- Prowlarr provides the **search layer**. Our app queries Prowlarr, gets results with `downloadUrl`, then hands those URLs to qBittorrent.
- The API key is generated on first boot. For automated setup, we'll need to either read it from `/config/config.xml` inside the container, or set it via the config file before boot.
- Prowlarr itself can be configured with download clients, but for our use case we'll handle the Prowlarr→qBittorrent handoff ourselves for more control.
- The Torznab/Newznab endpoints (`/api/v1/indexer/{id}/newznab`) provide compatibility with existing tools if needed.
