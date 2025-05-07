// tests/playwright/utils/auth.ts
import { Page, expect } from '@playwright/test';

export async function loginAsAdmin(page: Page) {
  await page.goto('http://localhost:8080/login');
  await page.fill('input[name="username"]', 'admin@example.com');
  await page.fill('input[name="password"]', 'password123');
  await page.click('button[type="submit"]');
  await expect(page.getByRole('button', { name: 'Logout' })).toBeVisible();
}
