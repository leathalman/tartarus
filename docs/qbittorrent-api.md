# qBittorrent Web API Reference

**Default Port:** 8080
**Base URL:** `http://{host}:8080/api/v2/`
**Official Docs (4.1+):** https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-4.1)
**Official Docs (5.0):** https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)

## Authentication

qBittorrent uses cookie-based session authentication.

### Login Flow

1. POST to `/api/v2/auth/login` with form-encoded `username` and `password`
2. Server responds with `Set-Cookie: SID=<session_id>`
3. Include `Cookie: SID=<session_id>` on all subsequent requests

**Important:** You MUST include a `Referer` (or `Origin`) header matching the host/port on all requests.

```bash
# Login
curl -i --header 'Referer: http://localhost:8080' \
  --data 'username=admin&password=adminadmin' \
  http://localhost:8080/api/v2/auth/login

# Use the SID cookie from the response on subsequent requests
curl -b 'SID=<session_id>' \
  -H 'Referer: http://localhost:8080' \
  http://localhost:8080/api/v2/torrents/info
```

### Default Credentials

- **Pre-v5.0.0:** `admin` / `adminadmin`
- **v5.0.0+:** `admin` / randomly generated (printed to container logs on first boot)

### Disabling Auth for Localhost

Set `WebUI\LocalHostAuth=false` in qBittorrent config to skip auth for local connections.

## HTTP Method Rules

- **GET** for read-only operations
- **POST** for mutations
- v4.4.4+ returns `405 Method Not Allowed` if wrong method is used

## Adding Torrents

The primary integration point for Tartarus.

### `POST /api/v2/torrents/add`

**Content-Type:** `multipart/form-data`

| Parameter | Description |
|-----------|-------------|
| `urls` | Newline-separated magnet links or .torrent URLs |
| `torrents` | Raw .torrent file binary (multipart file upload) |
| `savepath` | Download destination path |
| `category` | Category to assign |
| `tags` | Tags to assign |
| `paused` | Add in paused state (`true`/`false`) |
| `skip_checking` | Skip hash verification |
| `autoTMM` | Automatic Torrent Management |
| `sequentialDownload` | Download pieces in order |
| `firstLastPiecePrio` | Prioritize first/last pieces (useful for previewing) |
| `dlLimit` | Download speed limit (bytes/sec) |
| `upLimit` | Upload speed limit (bytes/sec) |
| `ratioLimit` | Stop seeding at this ratio |
| `seedingTimeLimit` | Stop seeding after N minutes |
| `contentLayout` | `Original`, `Subfolder`, or `NoSubfolder` |
| `rename` | Rename the torrent |

**Response:** `Ok.` on success, `Fails.` on failure.

## Torrent Listing & Info

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v2/torrents/info` | List torrents |
| GET | `/api/v2/torrents/properties` | Detailed torrent properties |
| GET | `/api/v2/torrents/trackers` | Tracker list for a torrent |
| GET | `/api/v2/torrents/files` | Files within a torrent |
| GET | `/api/v2/torrents/pieceStates` | Per-piece download progress |

### `GET /api/v2/torrents/info` Parameters

| Parameter | Description |
|-----------|-------------|
| `filter` | `all`, `downloading`, `seeding`, `completed`, `paused`, `active`, `inactive`, `resumed`, `stalled`, `errored` |
| `category` | Filter by category |
| `tag` | Filter by tag |
| `sort` | Sort by field (e.g. `name`, `size`, `progress`, `added_on`) |
| `reverse` | Reverse sort order |
| `limit` | Max results |
| `offset` | Result offset |
| `hashes` | Filter by torrent hashes (pipe-separated) |

### Torrent Info Response Fields

| Field | Description |
|-------|-------------|
| `hash` | Torrent hash (used as ID in most endpoints) |
| `name` | Torrent name |
| `size` | Total size in bytes |
| `progress` | Download progress (0.0 to 1.0) |
| `dlspeed` | Current download speed (bytes/sec) |
| `upspeed` | Current upload speed (bytes/sec) |
| `state` | Current state (`downloading`, `stalledDL`, `uploading`, `pausedDL`, etc.) |
| `category` | Assigned category |
| `tags` | Assigned tags |
| `added_on` | Unix timestamp when added |
| `completion_on` | Unix timestamp when completed |
| `save_path` | Where files are saved |
| `content_path` | Full path to content |
| `ratio` | Current seed ratio |

## Torrent Control

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/v2/torrents/pause` | Pause torrent(s) |
| POST | `/api/v2/torrents/resume` | Resume torrent(s) |
| POST | `/api/v2/torrents/delete` | Delete torrent(s) |
| POST | `/api/v2/torrents/recheck` | Re-verify torrent data |
| POST | `/api/v2/torrents/reannounce` | Re-announce to trackers |
| POST | `/api/v2/torrents/setForceStart` | Force start (ignore queue) |

### `POST /api/v2/torrents/delete`

| Parameter | Description |
|-----------|-------------|
| `hashes` | Pipe-separated torrent hashes, or `all` |
| `deleteFiles` | `true` to also delete downloaded files |

## Categories & Tags

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v2/torrents/categories` | List all categories |
| POST | `/api/v2/torrents/createCategory` | Create a category |
| POST | `/api/v2/torrents/editCategory` | Edit category |
| POST | `/api/v2/torrents/removeCategories` | Remove categories |
| POST | `/api/v2/torrents/setCategory` | Assign category to torrent(s) |
| GET | `/api/v2/torrents/tags` | List all tags |
| POST | `/api/v2/torrents/createTags` | Create tags |
| POST | `/api/v2/torrents/addTags` | Add tags to torrent(s) |
| POST | `/api/v2/torrents/removeTags` | Remove tags from torrent(s) |

## Transfer & Speed

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v2/transfer/info` | Global transfer stats |
| GET | `/api/v2/transfer/speedLimitsMode` | Alt speed limits active? |
| POST | `/api/v2/transfer/toggleSpeedLimitsMode` | Toggle alt speed limits |
| GET | `/api/v2/transfer/downloadLimit` | Get global download limit |
| POST | `/api/v2/transfer/setDownloadLimit` | Set global download limit |
| GET | `/api/v2/transfer/uploadLimit` | Get global upload limit |
| POST | `/api/v2/transfer/setUploadLimit` | Set global upload limit |

## Application

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v2/app/version` | App version string |
| GET | `/api/v2/app/webapiVersion` | Web API version |
| GET | `/api/v2/app/preferences` | All application settings |
| POST | `/api/v2/app/setPreferences` | Update settings (JSON body) |
| GET | `/api/v2/app/defaultSavePath` | Default download directory |
| POST | `/api/v2/app/shutdown` | Shut down qBittorrent |

## Sync (Real-time Updates)

For building responsive UIs that track download progress:

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v2/sync/maindata` | Delta sync of all torrent data |
| GET | `/api/v2/sync/torrentPeers` | Live peer data for a torrent |

The `maindata` endpoint accepts a `rid` (response ID) parameter. Pass `0` for a full snapshot, then use the returned `rid` on subsequent calls to get only changes since last request.

## Notes for Tartarus Integration

- qBittorrent is our **download engine**. We send it URLs/magnets from Prowlarr search results and monitor progress.
- For the v5.0.0+ password issue: on first boot, check container logs (`docker logs tartarus-qbittorrent`) for the generated password. For automated setup, we can pre-configure the password via the config file.
- The `Referer` header requirement is a common gotcha — every request needs it.
- Use categories to organize downloads by type (movies vs TV) and by user.
- The sync endpoint with `rid` is useful for building a real-time progress view without polling the full torrent list.
- Torrent hashes are the primary identifier — store these when adding torrents to track them later.
- `contentLayout: "Subfolder"` is useful for keeping downloads organized in the filesystem.
