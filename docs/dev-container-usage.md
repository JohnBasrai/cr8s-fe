# Development Environment: Container-Based Workflow

This document describes how the `cr8s-fe` project uses a containerized development environment to provide consistent linting, builds, and local test automation across machines.

---

## Quickstart CLI

The `quickstart` binary orchestrates the entire dev environment. It runs services via Docker Compose, performs lint checks, builds images, and coordinates frontend and backend startup.

```bash
cargo run --bin quickstart -- start --lint full
```

👉 Run `quickstart --help` and `quickstart start --help` for the most up-to-date usage instructions.

Flags:

* `--lint [none|basic|full]`: Controls lint checks (e.g., `cargo fmt`, `clippy`, `audit`, `outdated`)
* `--dev`: _(currently a work-in-progress)_ Intended to use locally built dev images (e.g., `cr8s-server-dev`) instead of pulling from GHCR. This will eventually support testing local backend changes before pushing to the registry.
* `--dry-run`: Prints commands without executing them

---

## Lint Checks in Container

To avoid host mismatches (e.g., toolchain version, UID), all lint checks are performed inside the dev container.

### Script Generation

`quickstart` generates a `run-checks.sh` file based on the `LintMode`.

```bash
#!/bin/bash
set -euo pipefail
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
...
```

This file is mounted into a temporary container and executed using the `cr8s` ecosystem's custom Rust dev image:

```bash
docker run --rm -u root \
  -v "$PWD:$PWD" \
  -v /var/tmp/dev-target:/app/target \
  -v /var/tmp/dev-cargo:/usr/local/cargo/registry \
  -w $PWD ghcr.io/johnbasrai/cr8s/rust-dev:1.83.0-rev5 \
  ./run-checks.sh
```

**Note:** If `/var/tmp` is not usable on your system or CI runner, override via:

```bash
# For example
export CR8S_SCRATCH_DIR="$HOME/tmp"
```
CI workflow can use: 
```
    CR8S_SCRATCH_DIR: ${{ runner.temp }}/cr8s-scratch
```
---

## Scratch Volume Usage

To speed up builds and avoid permission errors:

- `/app/target` → `${CR8S_SCRATCH_DIR:-/tmp/tmp}/dev-target`
- `/usr/local/cargo/registry` → `${CR8S_SCRATCH_DIR:-/tmp/tmp}/dev-cargo`

These volumes are reused across container runs and cleaned up via:

```bash
quickstart shutdown
# which runs:
docker compose down -v
sudo rm -rf "$CR8S_SCRATCH_DIR/dev-target" "$CR8S_SCRATCH_DIR/dev-cargo"
```

✅ As of **v0.3.0**, both `quickstart` and `docker-compose.yml` now default to the same scratch path fallback: `/tmp/tmp`. When `CR8S_SCRATCH_DIR` is set, all tooling — including Compose volume mounts — will respect this value for consistent cross-environment behavior.


## Frontend Compilation and Playwright

Frontend is served via `trunk` inside the `web` container.

```yaml
volumes:
  - ./src:/app/src
  - ./index.html:/app/index.html
```

Manual Playwright tests can be run after startup:

```bash
npx playwright test tests/playwright/login.spec.ts
```

---

## Version Tagging

Images built by `quickstart` are tagged with `CR8S_VERSION`, e.g., `cr8s-fe-server:latest`. By default, this version is read from the top-level `Cargo.toml`, or overridden via the `CR8S_VERSION` environment variable.
