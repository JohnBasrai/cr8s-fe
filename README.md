
# cr8s-fe :art:

[![CI](https://github.com/JohnBasrai/cr8s-fe/actions/workflows/ci.yml/badge.svg)](https://github.com/JohnBasrai/cr8s-fe/actions/workflows/ci.yml)

Yew/WASM frontend companion to the **cr8s** Rust backend.
Built with ⚡ hot‑reload via Trunk, stateless components, and a clean Tailwind‑free CSS layer (see `style.scss`).

---

## Prerequisites

* **Rust 1.81&nbsp;+** with the `wasm32-unknown-unknown` target  
  `rustup target add wasm32-unknown-unknown`
* **Trunk** & **wasm-bindgen CLI** (one-time install)  
  `cargo install trunk wasm-bindgen-cli --locked`
* *(Optional)* **Docker ≥ 24** & Docker Compose

> **Why 1.81?**  
> Recent Trunk releases—and their transitive crates **`litemap`** and **`zerofrom`**—now require `rustc 1.81` or newer.

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

> **Heads-up:** When the container starts, Docker Compose may print  
> `Enable Watch →  watch is not yet configured.`  
> This is Compose’s optional *file-watch* feature. You don’t need it—  
> Trunk inside the container already hot-reloads on `src/**` changes.  
> Simply ignore the prompt (don’t type **w**) and keep coding.

<details>
<summary><strong>See hot-reload in action&nbsp;(30&nbsp;s)</strong></summary>

   1. Open `src/components/login_form.rs`.  
   2. Find the line that renders the username field:  

```rust
   <Input label="Username" ... />
```

   3. Change **`"Username"`** to **`"Enter your username"`** and **save**.
   4. Watch the Docker/Trunk terminal — a quick re-compile appears.
   5. Switch back to the browser (still on `/login`) — the placeholder now reads **Enter your username** without a manual refresh.

*Revert the text and save again to watch it snap back.*

</details>


<!-- 
### Need a tiny production image?  

```bash
docker build --target prod -t cr8s-fe:latest .
docker run -p 8080:80 cr8s-fe:latest
````

---
-->

## Running frontend + backend together

The frontend talks to the **[cr8s](https://github.com/JohnBasrai/cr8s)** Rocket/Postgres API on <http://localhost:8000>.
You can keep them in separate repos or side‑by‑side in a monorepo—either way the steps
are identical:

### Two‑terminal approach

```bash
# ⬅︎ Terminal 1 – backend
git clone https://github.com/JohnBasrai/cr8s.git
cd cr8s
cargo run              # 🚀 Rocket serves on :8000
```

```bash
# ➡︎ Terminal 2 – frontend
git clone https://github.com/JohnBasrai/cr8s-fe.git
cd cr8s-fe
trunk serve --proxy-backend=http://localhost:8000 --address 0.0.0.0 --port 8080
```

`trunk`’s `--proxy-backend` flag forwards any `/api/*` request from the browser to the
backend, so you don’t have to fiddle with CORS during local dev.

### Alternative: Run backend via Docker

If you're already using Docker for the backend, you can run:

```bash
# (in a second terminal window or tab)
cd ../cr8s
./scripts/quickstart.sh
```
> 💡 Note: If you started the frontend using `docker compose up`, that terminal will remain open for logs. You’ll need to run this in a separate shell.

### Docker Compose (all-in-one)

To run both frontend and backend via Docker, just use the existing Compose config in the `cr8s` repo:

👉 See: [cr8s/docker-compose.yml](https://github.com/JohnBasrai/cr8s/blob/main/docker-compose.yml)

```bash
cd ../cr8s
./scripts/quickstart.sh      # starts backend and database services
cd ../cr8s-fe

# Skip this if you've already started the frontend above.
docker compose up -d web     # launches frontend (http://localhost:8080)
```

Browse to <http://localhost:8080>—the frontend proxies API calls to the backend
container automatically.

---

## 🔒 Backend Login Test (Manual Smoke Test)

Once the backend is running (via Docker or `cargo run`):
> ⚠️ Make sure the backend is available at `http://localhost:8000`
before launching the frontend.

1. If you haven't already, open your browser to [http://127.0.0.1:8080](http://127.0.0.1:8080)
2. Enter the default credentials:

    ```
    Username: admin@example.com
    Password: password123
    ```

3. ✅ You should see the authenticated view (e.g. "Have a great day!")
4. Feel free to try out the APIs.<br>
   a) Click on Rustaceans<br>
   b) Click on "Add a new rustacean"<br>
   c) Enter some test values and press save<br>
   d) Click on Crates<br>
   e) Add a new crate<br>

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
