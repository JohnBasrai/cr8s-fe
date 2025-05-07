import { loginAsAdmin } from './utils/auth';
import { test, expect } from '@playwright/test';

test('can log in as admin', async ({ page }) => {
  await loginAsAdmin(page);
});
