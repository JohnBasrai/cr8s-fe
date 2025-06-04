#!/bin/bash
# quickstart.sh - Launches cr8s full-stack (backend + frontend) for development

set -euo pipefail

# cr8s-fe environment variables

# Version of cr8s backend server container no 'v' prefix
export CR8S_VERSION=0.4.6

# Rust toolchain version to use
export RUST_DEV_IMAGE_VERSION=1.83.0-rev5
export RUST_DEV_IMAGE="ghcr.io/johnbasrai/cr8s/rust-dev:${RUST_DEV_IMAGE_VERSION}"

if [[ "${CR8S_VERSION}" == 'latest' ]] ; then
    export CLI_IMAGE=cr8s-cli-dev:latest
    export BASE_IMAGE=cr8s-server-dev:latest
else
    export CLI_IMAGE=ghcr.io/johnbasrai/cr8s/cr8s-cli:${CR8S_VERSION}
    export BASE_IMAGE=ghcr.io/johnbasrai/cr8s/cr8s-server:${CR8S_VERSION}
fi

progname=$(basename $0)

# Parse command line arguments
LINT_MODE="basic"  # Default mode
BUILD_ARGS=""      # Docker build arguments
COMPOSE_ARGS=""    # Docker compose up arguments
FORCE_PULL_BASE=false  # Whether to force pull base images

USAGE_MSG="
$0 [--no-lint | --full-lint] [--no-cache | --force-pull | --force-rebuild | --fresh] [--verbose]
$0 --shutdown
"

function do_shutdown() {
    echo "${progname}: 🛑 Shutting down cr8s full-stack..."
    docker compose down -v
    echo "${progname}: ✅ All containers stopped and volumes removed."
    exit 0
}

function show_help() {
    cat <<__EOF
${USAGE_MSG}

Lifecycle:
  --shutdown      Stop all services and remove volumes

Lint options:
  --no-lint       Skip all lint checks for fast startup
  --full-lint     Run comprehensive lint checks (fmt + clippy + audit + outdated)

Build options:
  --no-cache      Force rebuild server without Docker cache (local images only)
  --force-pull    Force pull base images from registry before building
  --force-rebuild Force recreate all containers (keep cache)
  --fresh         Nuclear option: stop containers + no-cache + force-pull + force-recreate

Debug options:
  --verbose       Enable debug logging and verbose output
__EOF
}

while [[ "$#" -gt 0 ]]; do
    case $1 in
        --shutdown)
            do_shutdown
            ;;
        --no-lint)
            LINT_MODE="none"
            ;;
        --full-lint)
            LINT_MODE="full"
            ;;
        --no-cache)
            BUILD_ARGS="--no-cache"
            echo "${progname}: 🔥 Force rebuilding server without Docker cache..."
            ;;
        --force-pull)
            FORCE_PULL_BASE=true
            echo "${progname}: 🔄 Will force pull base images from registry..."
            ;;
        --force-rebuild)
            COMPOSE_ARGS="--force-recreate"
            echo "${progname}: 🔄 Force recreating all containers..."
            ;;
        --fresh)
            echo "${progname}: 🔥 Fresh build: stopping containers and rebuilding everything..."
            docker compose down
            BUILD_ARGS="--no-cache"
            COMPOSE_ARGS="--force-recreate"
            FORCE_PULL_BASE=true
            ;;
        --verbose)
            set -x
            export LOGLEVEL=debug
            export SERVER_DEBUG_ARGS="--dump-state-traits --check"
            export SERVER_DEBUG_ARGS
            export RUST_LOG=debug
            export DEBUG_MODE=true
            ;;
        -h|--help)
            show_help
            exit 0
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
            echo "${progname}: Unknown option: $1"
            echo "${USAGE_MSG}"
            echo "Run '$0 --help' for detailed options."
            exit 1
            ;;
    esac
    shift
done

if [ "${CI:-}" == true -o "${DEBUG_MODE:-}" == true ] ; then
    echo "${progname}: 🔍 DEBUG:
    CR8S_VERSION           : ${CR8S_VERSION}
    BASE_IMAGE             : ${BASE_IMAGE}
    CLI_IMAGE              : ${CLI_IMAGE}
    RUST_DEV_IMAGE_VERSION : ${RUST_DEV_IMAGE_VERSION}
    RUST_DEV_IMAGE         : ${RUST_DEV_IMAGE}
"
fi

echo "${progname}: 🚀 Starting cr8s full-stack development environment..."

: ${CR8S_VERSION:?is required, check .env}

# Display lint mode
case $LINT_MODE in
    none)
        echo "${progname}: ⚡ Skipping lint checks for fast startup..."
        ;;
    full)
        echo "${progname}: 🔍 Running FULL lint checks (fmt + clippy + audit + outdated)..."
        ;;
    basic)
        echo "${progname}: 🔍 Running basic lint checks (fmt + clippy)..."
        ;;
esac

# Force pull base images if requested
if [[ "$FORCE_PULL_BASE" == "true" ]]; then
    echo "${progname}: 🔄 Force pulling base image from registry..."
    docker pull "${BASE_IMAGE}"
fi

if [ $(id -u) == 1000 ] ; then
    USER=""
else
    USER="--user root"
fi
RUST_DEV_COMMAND="docker run --rm -i -w$PWD -v$PWD:$PWD $USER ${RUST_DEV_IMAGE}"

if false ; then
set -x
    docker pull "${RUST_DEV_IMAGE}"
    ${RUST_DEV_COMMAND} pwd ; ls -la ; id
    echo "${progname}: 🔍 Testing write permissions..."
    ${RUST_DEV_COMMAND} touch test-write-permission
    ${RUST_DEV_COMMAND} ls -la test-write-permission
    ${RUST_DEV_COMMAND} rm -f test-write-permission
    
    echo "${progname}: 🔍 Checking target directory..."
    ${RUST_DEV_COMMAND} mkdir -p target
    ${RUST_DEV_COMMAND} ls -la target/

    echo "${progname}: 🔍 Disk space check..."
    df -h
    ${RUST_DEV_COMMAND} mkdir -p target
    ${RUST_DEV_COMMAND} chmod 755 target
set +x
fi

# Run lint checks based on mode
if [[ "$LINT_MODE" != "none" ]]; then
    echo "${progname}: ✍️  Checking code formatting..."
    ${RUST_DEV_COMMAND} cargo fmt --all -- --check

    echo "${progname}: 🔎 Running clippy lints..."
    ${RUST_DEV_COMMAND} cargo clippy --workspace --all-targets -- -D warnings

    # Additional checks only with --full-lint
    if [[ "$LINT_MODE" == "full" ]]; then
        echo "${progname}: 🔒 Running security audit..."
        ${RUST_DEV_COMMAND} web cargo audit --ignore RUSTSEC-2023-0071 || true
        
        echo "${progname}: 📦 Checking for outdated dependencies..."
        ${RUST_DEV_COMMAND} cargo outdated || true
    fi

    echo "${progname}: ✅ Lint checks passed!"
fi

echo "${progname}: 🔨 Building server ..."

docker build $BUILD_ARGS \
    --build-arg BASE_IMAGE=${BASE_IMAGE} \
    --build-arg CR8S_VERSION=${CR8S_VERSION} \
    -f Dockerfile.server \
    -t $BASE_IMAGE \
    .

# Start all services
echo "${progname}: 📦 Starting backend and frontend services..."
docker compose up -d $COMPOSE_ARGS
echo "${progname}: 🔍 Manual server connectivity test..."
for i in {1..12}; do
    if curl -sf http://localhost:8000/cr8s/health; then
        echo "${progname}: ✅ Server is responding on attempt $i"
        break
    else
        echo "${progname}: ⏳ Server not ready on attempt $i, waiting 10s..."
        sleep 10
    fi
done

# Wait for services to be healthy
echo "${progname}: ⏳ Waiting for services to be ready..."
docker compose up --wait

# Extract database schema
if [ ! -f scripts/sql/db-init.sql ] ; then
    if [ "${CR8S_VERSION}" == latest ] ; then
       echo "${progname}: $0: Manually copy cr8s/scripts/sql/db-init.sql cr8s-fe/scripts/sql/db-init.sql"
    fi
    CR8S_URL=https://codeload.github.com/JohnBasrai/cr8s/tar.gz/v${CR8S_VERSION}
    curl --fail --silent --show-error --location --output - $CR8S_URL |
        tar xfvz - cr8s-${CR8S_VERSION}/scripts/sql/db-init.sql
    mkdir -p scripts/sql/
    mv cr8s-${CR8S_VERSION}/scripts/sql/db-init.sql scripts/sql/db-init.sql
    rm -rf cr8s-${CR8S_VERSION}
fi

# Load schema into postgres
echo "${progname}: 🗄️  Loading database schema..."
docker compose exec -T postgres psql -U postgres -d cr8s < scripts/sql/db-init.sql

# Insert default roles (Admin, Editor, Viewer)
echo "${progname}: 👥 Adding default roles..."
docker compose exec -T postgres psql -U postgres -d cr8s << 'EOF'
INSERT INTO role (code, name) VALUES 
  ('Admin', 'Admin'),
  ('Editor', 'Editor'), 
  ('Viewer', 'Viewer')
ON CONFLICT (code) DO NOTHING;
EOF

# Seed default test user
# CLI user creation should fail if it doesn't work
echo "${progname}: 👤 Creating default test user (admin@example.com)..."
if ! docker compose run --rm cli create-user \
       --username admin@example.com \
       --password password123 \
       --roles admin,editor,viewer; then
    echo "${progname}: ❌ FATAL: Failed to create test user"
    exit 1
fi

# Verify user was actually created
echo "${progname}: 🔍 Verifying test user creation..."
USER_COUNT=$(docker compose exec postgres psql -U postgres -d cr8s -t -c \
    "SELECT COUNT(*) FROM app_user WHERE username = 'admin@example.com';")

if [ "${USER_COUNT// /}" != "1" ]; then
    echo "${progname}: ❌ FATAL: Test user was not created successfully"
    echo "${progname}: Expected 1 user, found: ${USER_COUNT// /}"
    exit 1
fi

echo "${progname}: ✅ Quickstart complete! Open http://localhost:8080"
echo "${progname}: 📧 Test login: admin@example.com / password123"
