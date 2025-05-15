
# cr8s-fe :art:

[![CI](https://github.com/JohnBasrai/cr8s-fe/actions/workflows/ci.yml/badge.svg)](https://github.com/JohnBasrai/cr8s-fe/actions/workflows/ci.yml)

Yew/WASM frontend companion to the **cr8s** Rust backend.
Built with ⚡ hot‑reload via Trunk, stateless components, and a clean Tailwind‑free CSS layer (see `style.scss`).

---

## Prerequisites

* **Rust 1.83&nbsp;+** with the `wasm32-unknown-unknown` target  
  `rustup target add wasm32-unknown-unknown`
* **Trunk** & **wasm-bindgen CLI** (one-time install)  
  `cargo install trunk wasm-bindgen-cli --locked`
* *(Optional)* **Docker ≥ 24** & Docker Compose

> **Why 1.83?**  
> Recent Trunk releases—and their transitive crates **`litemap`** and **`zerofrom`**—now require `rustc 1.83` or newer.

---

## Quick Start  

> ```bash
> ./scripts/quickstart.sh
> ```

This launches the full stack:
 - Clones and checks out the correct backend version (see docs/backend-version.txt)
 - Starts Postgres, Redis, and Rocket backend
 - Launches the frontend on http://localhost:8080

> 🐳 Requires Docker ≥ 24 and Docker Compose v2

Open <http://localhost:8080>; edits you make in `src/**` will hot‑reload in ~1 s.

### 🧼 Stopping Services

To stop both the backend and frontend containers and remove volumes:

> ```
> scripts/shutdown.sh
> ```
> Note: this also removes database volumes — login data will be reset.

---

> **Heads-up:** When the container starts, Docker Compose may print  
>
> `Enable Watch →  watch is not yet configured.`  
>
> This is Compose’s optional *file-watch* feature. You don’t need it—  
> Trunk inside the container already hot-reloads on `src/**` changes.  
> Simply ignore the prompt (don’t type **w**) and keep coding.

<details>
<summary><strong>See hot-reload in action&nbsp;</strong></summary>

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


---

## 🔒 Backend Login Test (Manual Smoke Test)

Once the frontend & backend are running

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

## 🧪 End-to-End Testing (Manual)

E2E tests are not run in CI by default. To run them manually see the full instructions in [manual-e2e-tests.md](docs/manual-e2e-tests.md).

> Note: E2E tests in CI are gated via `workflow_dispatch` and require `run_e2e=true`.

---

## Continuous Integration

Every push & PR runs **fmt → clippy → build (native + wasm)** via
`.github/workflows/ci.yml`.

---

## Project Structure

```
cr8s-fe/
├── cr8s/                       # Cloned cr8s backend (github) via quickstart.sh
│   ├── Cargo.toml
│   ├── scripts/
│   └── ...
├── Cargo.toml
├── README.md
├── CHANGELOG.md
├── docs/
│   ├── backend-version.txt     # Pinned cr8s backend version
│   └── manual-e2e-tests.md     # E2E instructions for local dev
├── public/
│   └── index.html              # App entrypoint
├── scripts/
│   ├── quickstart.sh           # One-command dev startup (backend + frontend)
│   └── shutdown.sh             # Stops all containers and removes volumes
├── src/
│   ├── api/                    # REST/GraphQL helpers
│   ├── components/             # Reusable Yew components
│   ├── pages/                  # Top-level routes
│   ├── contexts.rs             # Global state providers
│   ├── hooks.rs                # Custom hooks
│   └── main.rs                 # Yew entrypoint & router
├── style.scss
├── tests/
│   └── playwright/             # E2E browser tests (Playwright)
└── docker-compose.yml          # Frontend container definition```

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
