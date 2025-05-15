#!/bin/bash
# quickstart.sh - Launches cr8s backend (checked out from version tag) and cr8s-fe
# frontend for local development or E2E testing

set -euo pipefail

# Constants
CR8S_DIR="../cr8s"
BACKEND_VERSION_FILE="docs/backend-version.txt"
DEV_CONTAINER="cr8s-dev-${USER}"

# Read the tag from backend-version.txt
if [[ ! -f "$BACKEND_VERSION_FILE" ]]; then
  echo "❌ $BACKEND_VERSION_FILE not found. Please create it with a line like:"
  echo "cr8s version: v0.3.0"
  exit 1
fi

CR8S_TAG=$(grep -oE 'v[0-9]+\.[0-9]+\.[0-9]+' "$BACKEND_VERSION_FILE")

if [[ -z "$CR8S_TAG" ]]; then
  echo "❌ Failed to parse cr8s version from $BACKEND_VERSION_FILE"
  exit 1
fi

echo "🚀 Setting up backend from cr8s@$CR8S_TAG..."

# Clone if missing
if [ ! -d "$CR8S_DIR" ]; then
  cd ..
  git clone https://github.com/JohnBasrai/cr8s.git "$CR8S_DIR"
  cd "$CR8S_DIR"
  git fetch --tags
  echo "🔁 Detached HEAD — switching to tag $CR8S_TAG"
  git checkout "$CR8S_TAG"
else
  cd "$CR8S_DIR"
  echo "🔎 Working on branch: $(git rev-parse --abbrev-ref HEAD) — leaving as-is"
fi

./scripts/start.sh
./scripts/bootstrap.sh

echo "👤 Seeding default test user (admin@example.com)..."
docker exec -it "$DEV_CONTAINER" \
  cargo run --bin cli -- users create admin@example.com password123 admin || true

echo "🧠 Starting backend server inside $DEV_CONTAINER..."
docker exec -d "$DEV_CONTAINER" \
       bash -c 'cargo run --bin server > /tmp/server.log 2>&1'

cd - > /dev/null

echo "🎨 Starting frontend..."
docker compose up -d web

echo "✅ Quickstart complete! Open http://localhost:8080"
