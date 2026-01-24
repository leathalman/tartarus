# Tartarus Features

*A unified media management application.*

## New Features

- **Sensible defaults**
  - High resolution: 4K at x265, or x264
  - Streaming: 1080p at x265, or x264
- **Multiple users**
  - Permissions allowing users to grab any movie they want (and delete only the movies they added)
  - Any number of admin users
- **Automatically grab the top 5 movies from the month** — gives the library a current, new release feel
- **UI modernization**
  - The primary action when logging on is to grab a new movie or show, so that should be the default flow
  - Search should default to new content rather than the existing library
- **Combining TV shows and movies in the same application** — no need for separate instances for movies and TV
- **Demo mode** — doesn't delete any existing files, only adds new ones
  - Makes the setup experience quick and easy
- **Subtitle integration** — Bazarr-like functionality for consistent and correct subtitles across releases
- **File extension whitelist** — only download certain extensions

## Existing Features

- Get new releases as they come in (weekly episode tracking)
- Scan existing library and import it (part of onboarding)

## Nice to Haves

- Multiple resolutions for a single item
- Compatibility with different torrent clients and trackers
- Metadata service integration (skyhook, servarr)
- OIDC support
- LLM-powered recommendation engine
- Prevent download until X days before airing date
- Rundarr integration
- Restrict max file size
- Torznab/Newznab support (removes the need for Jackett; VPN considerations apply)
- Exportable config that can be ported between servers or used as a backup
- API compatibility with Sonarr/Radarr so existing mobile apps continue to work
- Config file created or imported via web GUI
- Demo on the landing page
- Profilarr basic profiles

## What Users Actually Want

| Feature | Why |
|---------|-----|
| One app for movies + TV | Eliminate duplicate instances, unified interface |
| Sensible defaults | "Just give me 1080p x265" without 20 custom formats |
| Multiple qualities per item | 720p for streaming, 4K for home theater — without running 4 instances |
| Don't download before air date | Prevents fake/malware torrents that flood trackers pre-release |
| TMDB support | Better metadata, especially for non-English content |
| Local show aliases | When indexers use different titles than TVDB |
| Multi-user with requests | Built-in Jellyseerr functionality |
| OAuth/SSO | Modern auth, integrate with existing identity providers |
| Config as code | YAML/TOML that can be version controlled, not SQLite blobs |
| Subtitle integration | Built-in Bazarr-like functionality |
| Season pack support | Improve on existing limitations in this area |
| Filter by file extension | Block .iso, .lnk, .scr malware automatically |
| Language preferences | "German dubbed, fallback to English" without running multiple instances |