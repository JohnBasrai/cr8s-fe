#!/bin/bash
set -euo pipefail

echo "🛑 Shutting down frontend (cr8s-fe)..."
docker compose down -v

echo "🛑 Shutting down backend (cr8s)..."
if [[ -d ../cr8s ]]; then
  cd ../cr8s
  ./scripts/stop.sh
else
  echo "⚠️ ../cr8s not found — skipping backend shutdown."
fi
wait

echo "✅ All containers stopped and volumes removed."
