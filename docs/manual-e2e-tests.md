# 🧪 Manual End-to-End Testing (cr8s-fe)

This document describes how to run Playwright-based end-to-end tests against the `cr8s-fe` frontend and its backend (`cr8s`).

## ✅ Prerequisites

- Docker and Docker Compose v2 must be installed and running.
- No need to install Rust, Trunk, or Node.js — everything runs in containers.

## 🚀 Quick Setup for E2E Testing

From the root of the `cr8s-fe` repository:

> ```bash
> ./scripts/quickstart.sh
> ```

This script will:

- Clone and pin the correct `cr8s` backend version (see `docs/backend-version.txt`)
- Start Postgres, Redis, and Rocket backend
- Launch the frontend on http://localhost:8080
- Seed the test user `admin@example.com` with password `password123`

Once services are up, run the following commands with Playwright to run tests across Chromium, Firefox, and WebKit.

> ```bash
> npx playwright test tests/playwright/login.spec.ts
> npx playwright test tests/playwright/rustaceans.spec.ts
> npx playwright test tests/playwright/crates.spec.ts
> ```

## 🧪 Running Tests in Headed Mode

To visually observe test execution in a real browser window:

> ```bash
> npx playwright test tests/playwright/login.spec.ts --headed
> npx playwright test tests/playwright/rustaceans.spec.ts --headed
> npx playwright test tests/playwright/crates.spec.ts --headed
> ```

To run a specific test, just run one of the commands above.

## 🧼 Resetting Your Environment

To stop all services and remove containers and volumes:

> ```bash
> ./scripts/shutdown.sh
> ```

> ⚠️ This will erase your local Postgres volume (including seeded data).

## Notes

- E2E tests are not run automatically in CI by default.
- Full CI support for E2E is planned once container orchestration is stabilized.

## Bugs

> 📝 Note: See [Issue #10](https://github.com/JohnBasrai/cr8s-fe/issues/10) for details on why `npx playwright test` is currently avoided in favor of running each test file directly.
