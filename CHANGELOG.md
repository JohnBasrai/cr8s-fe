# Changelog
All notable changes to **cr8s-fe** will be documented in this file.
The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.1] – 2025-05-04
### Added
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

### CI/CD
- Added `e2e` GitHub Actions job to run Playwright tests in headless mode on every push and PR
- Node.js and Playwright setup added to `.github/workflows/ci.yml`
- Supports Chromium, Firefox, and WebKit
- E2E tests run in parallel alongside Rust formatting and build checks
- Includes inline job comments for clarity and future extension

### DevOps
- Committed `package-lock.json` for reproducible test environments
- Created initial `playwright.config.ts` with multi-browser support and 60s timeout

### Docs
  - Improved `cr8s-fe` README:
    - Simplified Docker setup instructions
    - Replaced outdated compose example with reference to cr8s repo
    - Added realistic smoke test walkthrough with UI interactions

### Changed
- Entire codebase reformatted via **`cargo fmt`**.
- Clippy clean-ups:
  - Replaced `len() > 0` with `is_empty()` checks.
  - Removed redundant `clone()` calls on ID fields.
  - Added missing trailing newlines.
- Updated `.gitignore` to exclude Emacs backup files, `node_modules/`, and Playwright test artifacts
- Added logging infrastructure (`log`, `wasm-logger`, `console_error_panic_hook`) for frontend debugging
- Updated `main.rs` to enable log level switching based on build mode

### Fixed
- API helper calls now pass `id` by value, avoiding unnecessary copies.

---

[0.1.0]: https://github.com/JohnBasrai/cr8s-fe/releases/tag/v0.1.0
