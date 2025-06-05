# 🧪 Manual End-to-End Testing (cr8s-fe)

This document describes how to run Playwright-based end-to-end tests against the `cr8s-fe` frontend and backend.

## ✅ Prerequisites

- **No local Node.js or Rust required**: All tooling runs in containers
- Docker and Docker Compose v2 must be installed and running
- Services running via `./scripts/quickstart.sh`
- Run it with `--help` to get latest usage message. 

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
> ./scripts/quickstart.sh ...
> ./scripts/quickstart.sh --shutdown
> ```

> ⚠️ This will erase your local database (including seeded data).

## 📧 Test Credentials

- **Username**: `admin@example.com`
- **Password**: `password123`
- **Roles**: admin, editor, viewer (comprehensive access for testing)

## 🚀 CI Integration

E2E tests now run by default in the CI pipeline with improved performance:

1. **Fast execution**: Complete CI pipeline runs in under 4 minutes
2. Runs `quickstart.sh --full-lint` for comprehensive code quality checks
3. Executes Playwright login tests across all browsers
4. Uploads test artifacts on failure for debugging
5. **Optimized permissions**: Automatic user permission handling across environments

To disable E2E tests in CI, use `workflow_dispatch` with `run_e2e=false`.

## Notes

- **Cross-platform compatibility**: The quickstart script automatically handles user permissions on different systems (local development vs CI runners)
- **Performance optimized**: Container operations streamlined for faster test execution
- The quickstart script now includes enhanced user role setup for comprehensive testing
- Default test user has all three roles (admin, editor, viewer) to test role-based features
- SQL schema files are automatically managed in `scripts/sql/` directory
- Container rebuilds ensure latest code changes are reflected in tests

## Bugs

> 📝 Note: See [Issue #10](https://github.com/JohnBasrai/cr8s-fe/issues/10) for details on why `npx playwright test` is currently avoided in favor of running each test file directly.
