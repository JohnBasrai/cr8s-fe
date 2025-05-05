
# cr8s-fe :art:

[![CI](https://github.com/JohnBasrai/cr8s-fe/actions/workflows/ci.yml/badge.svg)](https://github.com/JohnBasrai/cr8s-fe/actions/workflows/ci.yml)

Yew/WASM frontend companion to the **cr8s** Rust backend.
Built with ⚡ hot‑reload via Trunk, stateless components, and a clean Tailwind‑free CSS layer (see `style.scss`).

---

## Prerequisites

* **Rust 1.78 +** with the `wasm32-unknown-unknown` target
  `rustup target add wasm32-unknown-unknown`
* **Trunk** & **wasm‑bindgen CLI** (one‑time install)
  `cargo install trunk wasm-bindgen-cli --locked`
* *(Optional)* **Docker ≥ 24** & Docker Compose

---

## Quick Start  
Choose **one** of the two paths below: **Native** or **Docker**

### Native (fastest feedback)

```bash
git clone https://github.com/JohnBasrai/cr8s-fe.git
cd cr8s-fe

rustup target add wasm32-unknown-unknown        # once per machine
cargo install trunk wasm-bindgen-cli --locked   # once per machine

trunk serve --address 0.0.0.0 --port 8080
```
### Docker (tool‑chain free)

```bash
docker compose up --build       # or: docker compose up -d web
```

For both choices above, open <http://localhost:8080>; edits you make in `src/**` will hot‑reload in ~1 s.

To shutdown, for **Native** stop with **Ctrl‑C** or for **Docker** stop with `docker compose down -v`.

---

## Running frontend + backend together

The frontend talks to the **cr8s** Rocket/Postgres API on <http://localhost:8000>.
You can keep them in separate repos or side‑by‑side in a monorepo—either way the steps
are identical:

### Two‑terminal approach

```bash
# ⬅︎ Terminal 1 – backend
git clone https://github.com/JohnBasrai/cr8s.git
cd cr8s
cargo run --release              # 🚀 Rocket serves on :8000
```

```bash
# ➡︎ Terminal 2 – frontend
git clone https://github.com/JohnBasrai/cr8s-fe.git
cd cr8s-fe
trunk serve --proxy-backend=http://localhost:8000 --address 0.0.0.0 --port 8080
```

`trunk`’s `--proxy-backend` flag forwards any `/api/*` request from the browser to the
backend, so you don’t have to fiddle with CORS during local dev.

### Docker Compose (single command)

If you prefer containers, create a `docker-compose.dev.yml` in *either* repo:

```yaml
services:
  backend:
    image: rust:1.78
    working_dir: /app
    volumes: ["../cr8s:/app"]
    command: ["cargo","run","--release"]
    ports: ["8000:8000"]

  frontend:
    build: ../cr8s-fe          # uses the Dockerfile already in the repo
    ports: ["8080:80"]
    volumes:
      - ../cr8s-fe/src:/app/src
      - ../cr8s-fe/index.html:/app/index.html
      - ../cr8s-fe/style.scss:/app/style.scss
    environment:
      - TRUNK_PROXY_BACKEND=http://backend:8000
```

Then:

```bash
docker compose -f docker-compose.dev.yml up --build
```

Browse to <http://localhost:8080>—the frontend proxies API calls to the backend
container automatically.

---

## Continuous Integration

Every push & PR runs **fmt → clippy → build (native + wasm)** via
`.github/workflows/ci.yml`.

1. `cargo fmt --all -- --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo build --release` (host)
4. `cargo build --release --target wasm32-unknown-unknown`

---

## Project Structure

```
src/
├── api/           # REST/GraphQL helpers
├── components/    # Re‑usable UI pieces
├── pages/         # Top‑level routes
├── contexts.rs    # Global state providers
├── hooks.rs       # Custom hooks
└── main.rs        # Yew entry‑point & router
```

---

### Related Projects

| Project | What it shows off |
|---------|------------------|
| **[cr8s](https://github.com/JohnBasrai/cr8s)** | Rocket + Postgres backend that powers this UI |
| **[axum-quickstart](https://github.com/JohnBasrai/axum-quickstart)** | Production-ready REST API using Axum, Redis, and Tokio |
| **[rust-sqlx](https://github.com/JohnBasrai/rust-sqlx)** | Async Postgres examples leveraging SQLx enum mapping |

---

## License

MIT © John Basrai
