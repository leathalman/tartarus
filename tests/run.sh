#!/bin/sh
set -e

DIR=$(cd "$(dirname "$0")" && pwd)

docker compose -f "$DIR/docker-compose.yml" up --abort-on-container-exit --exit-code-from driver
docker compose -f "$DIR/docker-compose.yml" down -v
