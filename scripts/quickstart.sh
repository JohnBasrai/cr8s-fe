#!/bin/bash
# quickstart.sh - Launches cr8s full-stack (backend + frontend) for development

set -euo pipefail

echo "🚀 Starting cr8s full-stack development environment..."

# Source environment variables
source .env

# Parse command line arguments
LINT_MODE="basic"  # Default mode

while [[ "$#" -gt 0 ]]; do
    case $1 in
        --no-lint)
            LINT_MODE="none"
            ;;
        --full-lint)
            LINT_MODE="full"
            ;;
        --verbose)
            set -x
            LOGLEVEL=debug
            typeset -x LOG=echo
            typeset -x RUST_LOG=debug
            typeset -x DEBUG_MODE=yes
            ;;
        *)
            echo "Unknown option: $1"
            echo "Usage: $0 [--no-lint|--full-lint] [--verbose]"
            exit 1
            ;;
    esac
    shift
done

# Display lint mode
case $LINT_MODE in
    none)
        echo "⚡ Skipping lint checks for fast startup..."
        ;;
    full)
        echo "🔍 Running FULL lint checks (fmt + clippy + audit + outdated)..."
        ;;
    basic)
        echo "🔍 Running basic lint checks (fmt + clippy)..."
        ;;
esac

# Run lint checks based on mode
if [[ "$LINT_MODE" != "none" ]]; then
    echo "  ✍️  Checking code formatting..."
    docker compose run --rm web cargo fmt --all -- --check

    echo "  🔎 Running clippy lints..."
    docker compose run --rm web cargo clippy --workspace --all-targets -- -D warnings

    # Additional checks only with --full-lint
    if [[ "$LINT_MODE" == "full" ]]; then
        echo "  🔒 Running security audit..."
        docker compose run --rm web cargo audit --ignore RUSTSEC-2023-0071 || true
        
        echo "  📦 Checking for outdated dependencies..."
        docker compose run --rm web cargo outdated || true
    fi

    echo "✅ Lint checks passed!"
fi

echo "🔨 Building server with latest code..."
docker compose build server

# Start all services
echo "📦 Starting backend and frontend services..."
docker compose up -d

# Wait for services to be healthy
echo "⏳ Waiting for services to be ready..."
docker compose up --wait

# Extract database schema
echo "🗄️  Loading database schema..."
CR8S_URL=https://codeload.github.com/JohnBasrai/cr8s/tar.gz/v${CR8S_VERSION}
curl --fail --silent --show-error --location --output - $CR8S_URL |
  tar xfvz - cr8s-${CR8S_VERSION}/scripts/sql/db-init.sql

# Load schema into postgres
docker compose exec -T postgres psql -U postgres -d cr8s < cr8s-${CR8S_VERSION}/scripts/sql/db-init.sql
mkdir -p scripts/sql/ ; mv cr8s-${CR8S_VERSION}/scripts/sql/db-init.sql scripts/sql/db-init.sql

# Insert default roles (Admin, Editor, Viewer)
echo "👥 Adding default roles..."
docker compose exec -T postgres psql -U postgres -d cr8s << 'EOF'
INSERT INTO role (code, name) VALUES 
  ('Admin', 'Admin'),
  ('Editor', 'Editor'), 
  ('Viewer', 'Viewer')
ON CONFLICT (code) DO NOTHING;
EOF

# Seed default test user
# CLI user creation should fail if it doesn't work
echo "👤 Creating default test user (admin@example.com)..."
if ! docker compose run --rm cli create-user \
       --username admin@example.com \
       --password password123 \
       --roles admin,editor,viewer; then
    echo "❌ FATAL: Failed to create test user"
    exit 1
fi

# Verify user was actually created
echo "🔍 Verifying test user creation..."
USER_COUNT=$(docker compose exec postgres psql -U postgres -d cr8s -t -c \
    "SELECT COUNT(*) FROM app_user WHERE username = 'admin@example.com';")

if [ "${USER_COUNT// /}" != "1" ]; then
    echo "❌ FATAL: Test user was not created successfully"
    echo "Expected 1 user, found: ${USER_COUNT// /}"
    exit 1
fi

echo "✅ Quickstart complete! Open http://localhost:8080"
echo "📧 Test login: admin@example.com / password123"
