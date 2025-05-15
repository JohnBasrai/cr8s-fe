# Changelog
All notable changes to **cr8s-fe** will be documented in this file.
The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

## [0.1.3] – 2025-05-14

### Added
- ✅ Verified full manual login flow using `admin@example.com / password123`
- ✅ Enabled and passed Playwright login test across Chromium, Firefox, and WebKit
- 🎭 Playwright test runner supports `--headed` execution (see updated docs)

### Changed
- 🔧 Replaced deprecated `quickstart.sh` with `start.sh` + `bootstrap.sh` in README
- 🧪 Added optional CLI user creation command for manual testing workflows
- 📄 Expanded Docker usage guidance in `README.md` (log viewing, volume behavior)
- 📘 Clarified manual vs CI E2E execution in `docs/manual-e2e-tests.md`

### Fixed
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
