import { test, expect } from '@playwright/test';
import { loginAsAdmin } from './utils/auth';

test('can add a rustacean', async ({ page }) => {
  await test.step('Login as admin', async () => {
    await loginAsAdmin(page);
  });

  await test.step('Navigate to Rustaceans list', async () => {
    await page.click('text=Rustaceans');
  });

  await test.step('Add a new rustacean', async () => {
    const name = 'Playwright Test';
    const email = `test-${Date.now()}@example.com`;

    await page.click('text=Add new rustacean');
    await page.fill('input[name="name"]', name);
    await page.fill('input[name="email"]', email);
    await page.click('text=Save');

    const row = page.locator('tr', { hasText: email });
    await expect(row).toContainText(name);
    await expect(row).toContainText(email);
  });
});
