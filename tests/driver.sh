#!/bin/sh
set -e

PROWLARR_URL="http://prowlarr:9696"
PROWLARR_API_KEY="tartarus-test-api-key"
QBIT_URL="http://qbittorrent:8080"
QBIT_USER="admin"
QBIT_PASS="adminadmin"

echo "=== Tartarus Integration Driver ==="
echo ""

# --- Prowlarr ---
echo "[Prowlarr] Pinging..."
curl -sf "$PROWLARR_URL/ping"
echo ""

echo "[Prowlarr] Fetching system status..."
STATUS=$(curl -s "$PROWLARR_URL/api/v1/system/status" \
  -H "X-Api-Key: $PROWLARR_API_KEY")
echo "[Prowlarr] System status: $STATUS"

echo "[Prowlarr] Listing indexers..."
INDEXERS=$(curl -s "$PROWLARR_URL/api/v1/indexer" \
  -H "X-Api-Key: $PROWLARR_API_KEY")
echo "[Prowlarr] Configured indexers: $INDEXERS"

echo ""

# --- qBittorrent ---
echo "[qBittorrent] Logging in..."
LOGIN_RESPONSE=$(curl -s -c /tmp/qbit_cookies \
  -H "Referer: $QBIT_URL" \
  --data "username=$QBIT_USER&password=$QBIT_PASS" \
  "$QBIT_URL/api/v2/auth/login")
echo "[qBittorrent] Login response: $LOGIN_RESPONSE"

if [ "$LOGIN_RESPONSE" = "Ok." ]; then
  echo "[qBittorrent] Authentication successful"

  VERSION=$(curl -s -b /tmp/qbit_cookies \
    -H "Referer: $QBIT_URL" \
    "$QBIT_URL/api/v2/app/version")
  echo "[qBittorrent] Version: $VERSION"

  API_VERSION=$(curl -s -b /tmp/qbit_cookies \
    -H "Referer: $QBIT_URL" \
    "$QBIT_URL/api/v2/app/webapiVersion")
  echo "[qBittorrent] Web API version: $API_VERSION"

  TORRENTS=$(curl -s -b /tmp/qbit_cookies \
    -H "Referer: $QBIT_URL" \
    "$QBIT_URL/api/v2/torrents/info")
  echo "[qBittorrent] Active torrents: $TORRENTS"
else
  echo "[qBittorrent] Login failed: $LOGIN_RESPONSE"
  exit 1
fi

echo ""
echo "=== Driver Complete ==="
echo "Both services are reachable and authenticated."
