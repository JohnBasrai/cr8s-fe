# cr8s-fe :art:

[![CI](https://github.com/JohnBasrai/cr8s-fe/actions/workflows/ci.yml/badge.svg)](https://github.com/JohnBasrai/cr8s-fe/actions/workflows/ci.yml)

Yew/WASM frontend companion to the **cr8s** Rust backend.
Built with ⚡ hot‑reload via Trunk, stateful components, and a clean Tailwind‑free CSS layer (see `style.scss`).

---

## Prerequisites

* **Docker ≥ 24** & Docker Compose v2

> **No local Rust installation required!** Everything runs in containers with the pre-built development image.

---

## Quick Start  

> ```bash
> ./scripts/quickstart.sh
> ```

This launches the full stack:
 - Starts PostgreSQL, Redis, and cr8s backend (v0.4.4)
 - Loads database schema and default roles
 - Creates test user: `admin@example.com` / `password123` (with admin, editor, viewer roles)
 - Launches the frontend with hot reload on http://localhost:8080

> 🐳 Requires Docker ≥ 24 and Docker Compose v2

Open <http://localhost:8080>; edits you make in `src/**` will hot‑reload in ~1 s.

### 🧼 Stopping Services

To stop all services and remove containers and volumes:

> ```
> ./scripts/shutdown.sh
> ```
> Note: this also removes database volumes — login data will be reset.

### Development Workflow

**Common development tasks:**

```bash
# Start development environment (with basic lint checks)
./scripts/quickstart.sh

# Fast startup (skip all lint checks)
./scripts/quickstart.sh --no-lint

# Comprehensive startup (includes security audit & outdated deps)
./scripts/quickstart.sh --full-lint

# Verbose debugging mode
./scripts/quickstart.sh --verbose

# Run cargo commands in frontend container
docker compose exec web cargo check
docker compose exec web cargo test

# Use backend CLI tools
docker compose run --rm cli list-users
docker compose run --rm cli create-user user@example.com password123 viewer

# View logs
docker compose logs web      # frontend logs
docker compose logs server   # backend logs

# Stop everything and clean up
./scripts/shutdown.sh
```

**Configuration:**
- Backend version controlled by `.env` file (currently v0.4.4)
- Frontend source code mounted for hot reload development
- Database persists between restarts (until `shutdown.sh` runs)

---

> **Heads-up:** When the container starts, Docker Compose may print  
>
> `Enable Watch →  watch is not yet configured.`  
>
> This is Compose's optional *file-watch* feature. You don't need it—  
> Trunk inside the container already hot-reloads on `src/**` changes.  
> Simply ignore the prompt (don't type **w**) and keep coding.

<details>
<summary><strong>See hot-reload in action&nbsp;</strong></summary>

   1. Open `src/components/login_form.rs`.  
   2. Find the line that renders the username field:  

```rust
   <Input label="Username" ... />
```

   3. Change **`"Username"`** to **`"Enter your username"`** and **save**.
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

## 🧪 End-to-End Testing

E2E tests run by default in CI. To run them manually see the full instructions in [docs/manual-e2e-tests.md](docs/manual-e2e-tests.md).

> Note: E2E tests can be disabled in CI via `workflow_dispatch` with `run_e2e=false`.

---

## Continuous Integration

Every push & PR runs **quickstart → lint checks → build → E2E tests** via
`.github/workflows/ci.yml`. The CI includes configurable lint levels and comprehensive testing across multiple browsers.

---

## Project Structure

```
cr8s-fe/
├── .env                        # Backend version configuration
├── Cargo.toml
├── README.md
├── CHANGELOG.md
├── docs/
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
└── docker-compose.yml          # Full-stack container definition
```

---

### Related Projects

| Project                                                                 | What it shows off                                           |
|-------------------------------------------------------------------------|-------------------------------------------------------------|
| **[cr8s](https://github.com/JohnBasrai/cr8s)**                          | Rocket + Postgres backend that powers this UI               |
| **[axum-quickstart](https://github.com/JohnBasrai/axum-quickstart)**   | Production-ready REST API using Axum, Redis, and Tokio      |
| **[rust-sqlx](https://github.com/JohnBasrai/rust-sqlx)**               | Async Postgres examples leveraging SQLx enum mapping        |

---

## License

MIT © John Basrai
