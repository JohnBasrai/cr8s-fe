# Manual E2E Test Procedure for cr8s-fe

This document describes how to manually run end-to-end (E2E) tests for the `cr8s-fe` frontend project using Playwright. These tests are not part of the automated CI pipeline and must be run explicitly when needed.

---

## ✅ Prerequisites

Ensure the following are installed on your development machine:

* Docker and Docker Compose (v2+)
* Node.js (v20+) and npm
* Rust (1.81+) with `wasm32-unknown-unknown` target
* `trunk` and `wasm-bindgen-cli`

You will also need the backend (`cr8s`) running locally.

---

## ♻️ Initial Cleanup

We'll be using two terminal windows:

* **Terminal 1 (backend)** for the backend (`cr8s`)
* **Terminal 2 (frontend)** for the frontend (`cr8s-fe`)

Before beginning, stop any previously running containers in both terminals:

**Terminal 1**:

```bash
# in Terminal 1 (backend)
docker compose down -v
```

**Terminal 2**:

```bash
# in Terminal 2 (frontend)
docker compose down -v
```

---

## 🐳 One-Time Docker Setup (Terminal 1)

```bash
# in Terminal 1 (backend)
./scripts/quickstart.sh
```

You should see output similar to:

```text
Roles assigned [Role { id: 1, code: Admin, name: "admin", created_at: 2025-05-07T13:50:35.259070 }]
✅ Setup complete.
🧩 Starting backend app container...
[+] Running 3/3
 ✔ Container cr8s-redis-1     Healthy     0.5s
 ✔ Container cr8s-postgres-1  Healthy     0.5s
 ✔ Container cr8s-app-1       Started
```

This will:

1. Tear down previous containers
2. Launch Postgres and Redis
3. Set up the database
4. Seed a default user: `admin@example.com / password123`
5. Start the backend on [http://localhost:8000](http://localhost:8000)

---

## 🚀 Start the Frontend (Terminal 2)

In **Terminal 2 (frontend)**:

```bash
npm install           # first time only

# Start frontend via Docker Compose
docker compose up -d web
```

You should see output similar to:

```text
[+] Running 2/2
 ✔ Network cr8s-fe_default  Created     0.0s
 ✔ Container cr8s-fe-web-1  Started
```

Wait a few seconds for the app to boot, or tail the logs to watch for successful compilation:

```bash
docker compose logs -f web
```

Look for the message indicating that Trunk has finished building and the app is serving on port 8080. You will need to press **Ctrl‑C** to stop tailing the logs once you’ve confirmed it’s ready.

You should see output similar to:

```text
web-1  |    Compiling cr8s-fe v0.1.1 (/app)
web-1  |     Finished `dev` profile [unoptimized + debuginfo] target(s) in 23.13s
web-1  | 2025-05-07T13:55:20.126436Z  INFO applying new distribution
web-1  | 2025-05-07T13:55:20.127164Z  INFO success
web-1  | 2025-05-07T13:55:20.127203Z  INFO serving static assets at -> /
web-1  | 2025-05-07T13:55:20.127333Z  INFO server listening at:
web-1  | 2025-05-07T13:55:20.127338Z  INFO     http://127.0.0.1:80/
web-1  | 2025-05-07T13:55:20.127340Z  INFO     http://172.20.0.2:80/
web-1  | 2025-05-07T13:55:20.127456Z  INFO     http://localhost.:80/
web-1  | 2025-05-07T13:55:20.127461Z  INFO     http://a57ffedd508c.:80/
```

---

## 🧪 Run E2E Tests (Terminal 2 – frontend)

In **Terminal 2 (frontend)**:

```bash
npx playwright install  # first time only
```
This installs browser binaries (Chromium, Firefox, WebKit).  If this fails due to missing
system dependencies, try:

```bash
npx playwright install --with-deps
```
This may require elevated privileges depending on your OS. We avoid recommending `sudo`
unless you understand and accept the risks of modifying system libraries.

If `package-lock.json` is modified by the install step, review and commit the changes to
ensure consistent test environments.

> ⚠️ Test Dependency Order: The tests are designed to be run in this order: **login,
> rustaceans, crates**
> 
> Since Playwright does not guarantee test file order, you should run them one at a time
> in this sequence.  Running them all at once or out of order may lead to flaky or failing
> tests.


**Test Steps:**
> ```bash
> npx wait-on http://localhost:8080/login # Wait for app to be ready.
> npx playwright test tests/playwright/login.spec.ts
> npx playwright test tests/playwright/rustaceans.spec.ts
> npx playwright test tests/playwright/crates.spec.ts
> ```

For each **_test_** this will:
 - Launch Playwright
 - Run **_test_** in headless mode for each browser (Chromium/Firefox/WebKit)
 - Target [http://localhost:8080](http://localhost:8080) and validate output for **_test_**

For for the full test run, you should see output similar to:

```
 $ npx playwright test tests/playwright/login.spec.ts

Running 3 tests using 3 workers

  ✓  1 [firefox] › tests/playwright/login.spec.ts:4:5 › can log in as admin (2.0s)
  ✓  2 [webkit] › tests/playwright/login.spec.ts:4:5 › can log in as admin (1.5s)
  ✓  3 [chromium] › tests/playwright/login.spec.ts:4:5 › can log in as admin (2.2s)

  3 passed (3.4s)
 $ npx playwright test tests/playwright/rustaceans.spec.ts 

Running 3 tests using 3 workers

  ✓  1 [chromium] › tests/playwright/rustaceans.spec.ts:4:5 › can add a rustacean (1.4s)
  ✓  2 [webkit] › tests/playwright/rustaceans.spec.ts:4:5 › can add a rustacean (2.1s)
  ✓  3 [firefox] › tests/playwright/rustaceans.spec.ts:4:5 › can add a rustacean (2.3s)

  3 passed (3.7s)
 $ npx playwright test tests/playwright/crates.spec.ts 

Running 3 tests using 3 workers

  ✓  1 [firefox] › tests/playwright/crates.spec.ts:4:5 › can add a crate (2.4s)
  ✓  2 [chromium] › tests/playwright/crates.spec.ts:4:5 › can add a crate (1.5s)
  ✓  3 [webkit] › tests/playwright/crates.spec.ts:4:5 › can add a crate (2.2s)
Row contents: 3	49340	crate-1746628693400	1		Created by test run at 2025-05-07T14:38:13.400Z	edit/delete
✅ Created crate row: 3	49340	crate-1746628693400	1		Created by test run at 2025-05-07T14:38:13.400Z	edit/delete
Row contents: 4	201823	crate-1746628694088	1		Created by test run at 2025-05-07T14:38:14.088Z	edit/delete
✅ Created crate row: 4	201823	crate-1746628694088	1		Created by test run at 2025-05-07T14:38:14.088Z	edit/delete
Row contents: 5	508678	crate-1746628694628	1		Created by test run at 2025-05-07T14:38:14.628Z	edit/delete
✅ Created crate row: 5	508678	crate-1746628694628	1		Created by test run at 2025-05-07T14:38:14.628Z	edit/delete

  3 passed (3.8s)
```

---

## 📁 Test Files

| File                 | Description                         |
| -------------------- | ----------------------------------- |
| `login.spec.ts`      | Basic login with seeded credentials |
| `crates.spec.ts`     | Create crate, validate dropdown     |
| `rustaceans.spec.ts` | Add a new rustacean profile         |

All tests live in `cr8s-fe/tests/` and share helpers from `utils/auth.ts`.

---

## 🔁 Cleanup

When you're done, stop and clean up both environments:

**Terminal 1 (backend):**

```bash
# in Terminal 1 (backend)
docker compose down -v
```

**Terminal 2 (frontend):**

```bash
# in Terminal 2 (frontend)
docker compose down -v
```

You may drop the `-v` if you want to preseve container cache to speed start up times.
Also be sure you remove it from the _Initial Cleanup_ when you re-run.

---

## 📝 Final Notes

* E2E tests are **not run automatically in CI**. They must be invoked manually.
* Although a manual trigger (`workflow_dispatch`) is wired up in GitHub Actions, the current setup **does not yet support full execution in CI**, due to environment limitations (e.g., missing dev container, frontend build timeouts).
* We plan to address this by creating a custom development container and testing the workflow locally.
* In the meantime, always run E2E tests manually from your development host.
* Make sure the backend is accessible at `http://localhost:8000`.
* Playwright tests run in **headless** mode by default. To run tests in _headed_ mode:

```bash
npx playwright test tests/playwright/login.spec.ts  --headed
npx playwright test tests/playwright/rustaceans.spec.ts  --headed
npx playwright test tests/playwright/crates.spec.ts  --headed
```
