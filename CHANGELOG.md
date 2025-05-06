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

### Fixed
- API helper calls now pass `id` by value, avoiding unnecessary copies.

---

[0.1.0]: https://github.com/JohnBasrai/cr8s-fe/releases/tag/v0.1.0
