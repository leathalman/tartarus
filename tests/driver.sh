#!/bin/sh
set -e

TARTARUS_URL="http://tartarus:3001"

echo "=== Tartarus Integration Tests ==="
echo ""

# --- Test: List Torrents ---
echo "[TEST] GET /api/torrents - List all torrents"
TORRENTS=$(curl -sf "$TARTARUS_URL/api/torrents")
echo "Response: $TORRENTS"

# Validate response is a JSON array
if ! echo "$TORRENTS" | jq -e 'type == "array"' > /dev/null 2>&1; then
  echo "FAIL: Expected JSON array"
  exit 1
fi
echo "PASS: Returns valid JSON array"
echo ""

# --- Test: Search ---
echo "[TEST] GET /api/search?q=test - Search for torrents"
SEARCH=$(curl -sf "$TARTARUS_URL/api/search?q=test")
echo "Response: $SEARCH"

# Validate response is a JSON array
if ! echo "$SEARCH" | jq -e 'type == "array"' > /dev/null 2>&1; then
  echo "FAIL: Expected JSON array"
  exit 1
fi
echo "PASS: Returns valid JSON array"
echo ""

# --- Test: Download (POST) ---
echo "[TEST] POST /api/download - Submit a torrent for download"

# Use a well-known public domain torrent (Big Buck Bunny)
DOWNLOAD_RESPONSE=$(curl -sf -X POST "$TARTARUS_URL/api/download" \
  -H "Content-Type: application/json" \
  -d '{"url": "magnet:?xt=urn:btih:dd8255ecdc7ca55fb0bbf81323d87062db1f6d1c&dn=Big+Buck+Bunny"}')
echo "Response: $DOWNLOAD_RESPONSE"

# Validate response contains expected fields
if ! echo "$DOWNLOAD_RESPONSE" | jq -e '.hash' > /dev/null 2>&1; then
  echo "FAIL: Expected response with 'hash' field"
  exit 1
fi
TORRENT_HASH=$(echo "$DOWNLOAD_RESPONSE" | jq -r '.hash')
echo "PASS: Torrent submitted with hash: $TORRENT_HASH"
echo ""

# --- Test: Get Single Torrent ---
echo "[TEST] GET /api/torrents/{hash} - Get single torrent"
SINGLE_TORRENT=$(curl -sf "$TARTARUS_URL/api/torrents/$TORRENT_HASH")
echo "Response: $SINGLE_TORRENT"

# Validate response contains the hash
if ! echo "$SINGLE_TORRENT" | jq -e '.hash' > /dev/null 2>&1; then
  echo "FAIL: Expected torrent object with 'hash' field"
  exit 1
fi
echo "PASS: Returns valid torrent object"
echo ""

# --- Test: Verify Torrent in List ---
echo "[TEST] GET /api/torrents - Verify torrent appears in list"
TORRENTS_AFTER=$(curl -sf "$TARTARUS_URL/api/torrents")
echo "Response: $TORRENTS_AFTER"

# Validate the list now contains at least one torrent
TORRENT_COUNT=$(echo "$TORRENTS_AFTER" | jq 'length')
if [ "$TORRENT_COUNT" -lt 1 ]; then
  echo "FAIL: Expected at least 1 torrent in list"
  exit 1
fi
echo "PASS: Torrent list contains $TORRENT_COUNT torrent(s)"
echo ""

echo "=== All Integration Tests Passed ==="
