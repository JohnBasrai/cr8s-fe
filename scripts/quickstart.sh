#!/bin/bash
# quickstart.sh - Launches cr8s full-stack (backend + frontend) for development

set -euo pipefail

echo "🚀 Starting cr8s full-stack development environment..."

# Source environment variables
source .env

# Parse command line arguments
LINT_MODE="basic"  # Default mode
BUILD_ARGS=""      # Docker build arguments
COMPOSE_ARGS=""    # Docker compose up arguments
FORCE_PULL_BASE=false  # Whether to force pull base images

USAGE_MSG="[--no-lint | --full-lint] [--no-cache | --force-pull | --force-rebuild | --fresh] [--verbose]"

while [[ "$#" -gt 0 ]]; do
    case $1 in
        --no-lint)
            LINT_MODE="none"
            ;;
        --full-lint)
            LINT_MODE="full"
            ;;
        --no-cache)
            BUILD_ARGS="--no-cache"
            echo "🔥 Force rebuilding server without Docker cache..."
            ;;
        --force-pull)
            FORCE_PULL_BASE=true
            echo "🔄 Will force pull base images from registry..."
            ;;
        --force-rebuild)
            COMPOSE_ARGS="--force-recreate"
            echo "🔄 Force recreating all containers..."
            ;;
        --fresh)
            echo "🔥 Fresh build: stopping containers and rebuilding everything..."
            docker compose down
            BUILD_ARGS="--no-cache"
            COMPOSE_ARGS="--force-recreate"
            FORCE_PULL_BASE=true
            ;;
        --verbose)
            set -x
            LOGLEVEL=debug
            SERVER_DEBUG_ARGS="--dump-state-traits --check"
            export SERVER_DEBUG_ARGS
            export RUST_LOG=debug
            typeset -x LOG=echo
            typeset -x RUST_LOG=debug
            typeset -x DEBUG_MODE=yes
            ;;
        -h|--help)
            echo "Usage: $0 ${USAGE_MSG}"
            echo ""
            echo "Lint options:"
            echo "  --no-lint       Skip all lint checks for fast startup"
            echo "  --full-lint     Run comprehensive lint checks (fmt + clippy + audit + outdated)"
            echo ""
            echo "Build options:"
            echo "  --no-cache      Force rebuild server without Docker cache (local images only)"
            echo "  --force-pull    Force pull base images from registry before building"
            echo "  --force-rebuild Force recreate all containers (keep cache)"
            echo "  --fresh         Nuclear option: stop containers + no-cache + force-pull + force-recreate"
            echo ""
            echo "Debug options:"
            echo "  --verbose       Enable debug logging and verbose output"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Usage: $0 ${USAGE_MSG}"
            echo "Run '$0 --help' for detailed options."
            exit 1
            ;;
    esac
    shift
done

: ${CR8S_VERSION:?is required, check .env}

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

# Force pull base images if requested
if [[ "$FORCE_PULL_BASE" == "true" ]]; then
    echo "🔄 Force pulling base image from registry..."
    docker pull ghcr.io/johnbasrai/cr8s/cr8s-server:${CR8S_VERSION}
fi

if [[ "${CR8S_VERSION}" == 'latest' ]] ; then
    echo "🔨 Building server with local dev image..."
    export CLI_IMAGE=cr8s-cli-dev:latest
    export BASE_IMAGE=cr8s-server-dev:latest
else
    export CLI_IMAGE=ghcr.io/johnbasrai/cr8s/cr8s-cli:${CR8S_VERSION}
    export BASE_IMAGE=ghcr.io/johnbasrai/cr8s/cr8s-server:${CR8S_VERSION}
    echo "🔨 Building server with latest code..."
fi

echo "🔍 DEBUG: CR8S_VERSION=${CR8S_VERSION}, BASE_IMAGE=${BASE_IMAGE}, CLI_IMAGE=${CLI_IMAGE}"

docker build $BUILD_ARGS \
    --build-arg BASE_IMAGE=${BASE_IMAGE} \
    --build-arg CR8S_VERSION=${CR8S_VERSION} \
    -f Dockerfile.server \
    -t cr8s-fe-server:latest \
    .


# Start all services
echo "📦 Starting backend and frontend services..."
docker compose up -d $COMPOSE_ARGS

# Wait for services to be healthy
echo "⏳ Waiting for services to be ready..."
docker compose up --wait

# Extract database schema
if [ ! -f scripts/sql/db-init.sql ] ; then
    if [ "${CR8S_VERSION}" == latest ] ; then
       echo "$0: Manually copy cr8s/scripts/sql/db-init.sql cr8s-fe/scripts/sql/db-init.sql"

    fi
    CR8S_URL=https://codeload.github.com/JohnBasrai/cr8s/tar.gz/v${CR8S_VERSION}
    curl --fail --silent --show-error --location --output - $CR8S_URL |
        tar xfvz - cr8s-${CR8S_VERSION}/scripts/sql/db-init.sql
    mkdir -p scripts/sql/
    mv cr8s-${CR8S_VERSION}/scripts/sql/db-init.sql scripts/sql/db-init.sql
    rm -rf cr8s-${CR8S_VERSION}
fi

# Load schema into postgres
echo "🗄️  Loading database schema..."
docker compose exec -T postgres psql -U postgres -d cr8s < scripts/sql/db-init.sql

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
