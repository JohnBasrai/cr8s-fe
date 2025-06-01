# Changelog
All notable changes to **cr8s-fe** will be documented in this file.
The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

## [0.2.1] - 2025-06-02

### Added
- Enhanced quickstart.sh with flexible lint checking options (`--no-lint`, `--full-lint`, `--verbose`)
- Full lint mode includes security audit and outdated dependency checks
- Comprehensive user role assignment (admin, editor, viewer) for default test user
- SQL directory management with automatic creation in quickstart script
- Enhanced CI workflow with unified test and build process
- Dockerfile.server now supports configurable CR8S_VERSION build argument
- User role verification query for debugging authentication setup

### Changed
- **BREAKING**: Updated to cr8s backend v0.4.3 from v0.4.2
- Streamlined CI workflow to single `test` job with conditional E2E execution
- E2E tests now run by default unless explicitly disabled via workflow_dispatch
- Quickstart script now rebuilds server container to ensure latest code
- Default test user now has all three roles (admin, editor, viewer) instead of just admin
- Improved error handling and logging throughout quickstart process
- CI timeout increased to 15 minutes for comprehensive testing

### Fixed
- Docker build context properly passes CR8S_VERSION argument to server build
- SQL file management prevents conflicts between quickstart runs
- User creation process more robust with better error messaging
- Container cleanup more reliable in CI with proper error handling

### Technical Details
- Backend: cr8s v0.4.3 (containerized)
- CI: Single job workflow with optional E2E testing
- Lint: Configurable cargo fmt, clippy, audit, and outdated checks
- Security: Default user has comprehensive role access for testing

---

## [0.2.0] - 2025-06-02

### Added
- Unified docker-compose.yml with full-stack development environment
- Automatic database schema and role initialization via quickstart script
- Support for cr8s backend v0.4.2 with containerized workflow
- Environment variable configuration (.env) for backend version management
- Custom Dockerfile.server with curl for health checks
- Enhanced volume mounts for complete hot reload support (Cargo.toml, Cargo.lock, yew-logo.svg)
- Container networking configuration for frontend-backend communication
- Simplified development workflow with single-command setup

### Changed
- **BREAKING**: Development workflow now requires Docker only (no local Rust installation)
- Updated quickstart.sh to use unified docker-compose instead of separate backend cloning
- Simplified prerequisites in README - removed local Rust toolchain requirements
- Updated shutdown.sh for unified container management
- Enhanced manual E2E testing documentation with simplified workflow
- Migrated from backend-version.txt to .env file for version configuration
- Updated project structure documentation to reflect new containerized approach

### Fixed
- Frontend-backend communication issues with proper container hostname resolution
- Environment variable handling for BACKEND_URL, ROCKET_HOST, and ROCKET_PORT
- Database initialization race conditions with health check dependencies
- Missing asset files (yew-logo.svg) causing build failures

### Removed
- Local Rust toolchain requirements from prerequisites
- backend-version.txt file (replaced with .env)
- Complex git clone/checkout workflow in quickstart script
- Separate backend repository management

### Technical Details
- Backend: cr8s v0.4.2 (containerized)
- Frontend: Yew with Trunk hot reload
- Database: Automatic PostgreSQL schema setup
- Authentication: Default admin user (admin@example.com / password123)
- Development: Single docker-compose command workflow


## [0.1.3] – 2025-05-14

### Added
- ✅ `scripts/quickstart.sh`: launches cr8s backend (cloned from version tag) and cr8s-fe frontend
- ✅ `scripts/shutdown.sh`: stops frontend/backend containers and removes volumes
- 📄 `docs/backend-version.txt`: tracks pinned backend version used during testing
- 🧪 Playwright login, crate, and rustacean tests run individually with known-good results
- 🔗 Issue #10 opened to track orchestration of full test suite
- ✅ Verified full manual login flow using `admin@example.com / password123`
- ✅ Enabled and passed Playwright login test across Chromium, Firefox, and WebKit
- 🎭 Playwright test runner supports `--headed` execution (see updated docs)

### Changed
- 🔧 CI now checks out `cr8s@v0.3.0` and runs backend setup manually via `start.sh` and `bootstrap.sh`
- 📘 `manual-e2e-tests.md` rewritten to match Docker-first workflow and one-command test setup
- 📘 `README.md` simplified: removed native/dual-terminal paths and made `quickstart.sh` canonical
- 🧼 Project tree updated to reflect backend nested under `cr8s-fe/cr8s/` after cloning
- 🔧 Replaced deprecated `quickstart.sh` with `start.sh` + `bootstrap.sh` in README
- 🧪 Added optional CLI user creation command for manual testing workflows
- 📄 Expanded Docker usage guidance in `README.md` (log viewing, volume behavior)
- 📘 Clarified manual vs CI E2E execution in `docs/manual-e2e-tests.md`

### Fixed
- 🧪 Removed flaky `npx playwright test` entry; now running specs individually to avoid discovery issues
- 🧪 Aligned frontend test assumptions with actual backend login behavior
- Verified against cr8s backend at tag: `v0.3.0`

---

[0.1.3]: https://github.com/JohnBasrai/cr8s-fe/releases/tag/v0.1.3

## [0.1.2] – 2025-05-06

### Added
- Enabled manual GitHub Actions trigger (`workflow_dispatch`) with optional input to run E2E tests
- Added conditional check to skip E2E tests unless explicitly requested
- Added `docs/manual-e2e-tests.md` with detailed manual E2E instructions for cr8s-fe using Playwright
- feat(dev-docker): hot-reload container + Rust 1.81 toolchain
- **CI pipeline** (`.github/workflows/ci.yml`)
  Runs `cargo fmt`, `cargo clippy`, and dual `cargo build`
  (native + `wasm32-unknown-unknown`) with caching.
- **README overhaul**
  - Native & Docker quick-start
  - Frontend ↔ backend proxy instructions (two-terminal & Compose)
  - CI status badge and project structure map
  - Related-projects table (cr8s, axum-quickstart, rust-sqlx)
- 🎭 Playwright test framework with cross-browser E2E support (Chromium, Firefox, WebKit)
- ✅ End-to-end login test via `login.spec.ts`
- ✅ Rustacean creation flow via `rustaceans.spec.ts`
- ✅ Crate creation flow via `crates.spec.ts`, including author dropdown handling
- 🔐 Auth helper extracted to `utils/auth.ts` for reuse
- 🧪 `test.step()` used to group high-level actions for trace clarity

### Changed
- Mark optional fields for Yew 0.21 compatibility (#3)
- Entire codebase reformatted via **`cargo fmt`**.
- Clippy clean-ups:
  - Replaced `len() > 0` with `is_empty()` checks.
  - Removed redundant `clone()` calls on ID fields.
  - Added missing trailing newlines.
- Updated `.gitignore` to exclude Emacs backup files, `node_modules/`, and Playwright test artifacts
- Added logging infrastructure (`log`, `wasm-logger`, `console_error_panic_hook`) for frontend debugging
- Updated `main.rs` to enable log level switching based on build mode

### CI/CD
- E2E tests in CI are gated via `workflow_dispatch` and require `run_e2e=true`.
- Node.js and Playwright setup added to `.github/workflows/ci.yml`
- Supports Chromium, Firefox, and WebKit
- Includes inline job comments for clarity and future extension

### DevOps
- Committed `package-lock.json` for reproducible test environments
- Created initial `playwright.config.ts` with multi-browser support and 60s timeout

### Docs
- Clarify Quick Start – choose Native *or* Docker
- Improved `cr8s-fe` README:
  - Simplified Docker setup instructions
  - Replaced outdated compose example with reference to cr8s repo
  - Added realistic smoke test walkthrough with UI interactions

### Fixed
- API helper calls now pass `id` by value, avoiding unnecessary copies.

---

[0.1.0]: https://github.com/JohnBasrai/cr8s-fe/releases/tag/v0.1.0
