# 🧪 Manual End-to-End Testing (cr8s-fe)

This document describes how to run Playwright-based end-to-end tests against the `cr8s-fe` frontend and backend.

## ✅ Prerequisites

- Docker and Docker Compose v2 must be installed and running
- Services running via `./scripts/quickstart.sh`

## 🚀 Running E2E Tests

Ensure services are running first. Choose your preferred startup mode:

> ```bash
> # Standard startup with basic lint checks (recommended for testing)
> ./scripts/quickstart.sh
> 
> # Fast startup for quick test iterations
> ./scripts/quickstart.sh --no-lint
> 
> # Comprehensive startup with full lint suite
> ./scripts/quickstart.sh --full-lint
> ```

Then run Playwright tests across Chromium, Firefox, and WebKit:

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

## 🧼 Test Environment Reset

To reset the test environment between test runs:

> ```bash
> ./scripts/shutdown.sh
> ./scripts/quickstart.sh
> ```

> ⚠️ This will erase your local database (including seeded data).

## 📧 Test Credentials

- **Username**: `admin@example.com`
- **Password**: `password123`
- **Roles**: admin, editor, viewer (comprehensive access for testing)

## 🚀 CI Integration

E2E tests now run by default in the CI pipeline. The CI workflow:

1. Runs `quickstart.sh --full-lint` for comprehensive code quality checks
2. Executes Playwright login tests across all browsers
3. Uploads test artifacts on failure for debugging

To disable E2E tests in CI, use `workflow_dispatch` with `run_e2e=false`.

## Notes

- The quickstart script now includes enhanced user role setup for comprehensive testing
- Default test user has all three roles (admin, editor, viewer) to test role-based features
- SQL schema files are automatically managed in `scripts/sql/` directory
- Container rebuilds ensure latest code changes are reflected in tests

## Bugs

> 📝 Note: See [Issue #10](https://github.com/JohnBasrai/cr8s-fe/issues/10) for details on why `npx playwright test` is currently avoided in favor of running each test file directly.