# 🧪 Manual End-to-End Testing (cr8s-fe)

This document describes how to run Playwright-based end-to-end tests against the `cr8s-fe` frontend and backend.

## ✅ Prerequisites

- **No local Node.js or Rust required**: All tooling runs in containers
- Docker and Docker Compose v2 must be installed and running
- Services running via `target/debug/quickstart start`
👉 Run `quickstart --help` and `quickstart start --help` for the most up-to-date usage instructions.

## 🚀 Running E2E Tests

Ensure services are running first. Choose your preferred startup mode:

💡 **Note:** If you're running `quickstart` via the container-based workflow, the CLI binary is built inside the dev container under:

```bash
${CR8S_SCRATCH_DIR:-/tmp/tmp}/dev-target/debug/quickstart
```

This is a temporary path and will be deleted when you run:

```bash
quickstart shutdown
```

To avoid this and run `quickstart` consistently from your host:

```bash
cargo install --path cli --bin quickstart
```

This installs it to `$CARGO_HOME/bin` (usually `~/.cargo/bin`), so you can run it from anywhere like:

> ```bash
> # Recommended for testing
> quickstart start --fresh --lint basic
> 
> # Fast startup for quick test iterations
> quickstart start --fresh --lint none
> 
> # Comprehensive startup with full lint suite (used by CI workflow)
> quickstart start --fresh --lint full
> ```

After you start the service the first time, you should wait for the frontend to finish initializing. This gives **Trunk**, the Rust/WASM bundler, time to compile the frontend code in `cr8s-fe/src` and serve it on port `8080`.

Use the following command:
>```
> quickstart wait
>```

Once `quickstart wait` returns, run Playwright tests across Chromium, Firefox, and WebKit:
> ```bash
> npx playwright test tests/playwright/login.spec.ts
> npx playwright test tests/playwright/rustaceans.spec.ts
> npx playwright test tests/playwright/crates.spec.ts # Currently not working*
> ```
* See [Issue #35](https://github.com/JohnBasrai/cr8s/issues/35)

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
> quickstart shutdown
> ```

> ⚠️ This will erase your local database (including seeded data) and also deletes scratch volumes under `${CR8S_SCRATCH_DIR}` (e.g., `/var/tmp/dev-target`, `/var/tmp/dev-cargo`).

## 📧 Test Credentials

- **Username**: `admin@example.com`
- **Password**: `password123`
- **Roles**: admin, editor, viewer (comprehensive access for testing)

## 🚀 CI Integration

E2E tests now run by default in the CI pipeline with improved performance:

1. **Fast execution**: Complete CI pipeline runs in under 4 minutes
2. Runs `quickstart start --lint full` for comprehensive code quality checks
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
> 📝 See [Issue #10](https://github.com/JohnBasrai/cr8s-fe/issues/10) for details on why `npx playwright test` is currently avoided in favor of running each test file directly.
> 📝 See [Issue #35](https://github.com/JohnBasrai/cr8s/issues/35) for details on why `crates.spec.ts` playwright test is disabled.
