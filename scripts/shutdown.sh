#!/bin/bash
set -euo pipefail

echo "🛑 Shutting down cr8s full-stack (frontend + backend)..."
docker compose down -v

echo "✅ All containers stopped and volumes removed."
