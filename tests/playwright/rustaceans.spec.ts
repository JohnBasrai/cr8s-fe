import { test, expect } from '@playwright/test';
import { loginAsAdmin } from './utils/auth';

test('can add a rustacean', async ({ page }) => {
    await test.step('Login as admin', async () => {
        await loginAsAdmin(page);
    });

    await test.step('Navigate to Rustaceans list', async () => {
        await Promise.all([
            page.waitForNavigation({ waitUntil: 'load' }),
            page.click('text=Rustaceans'),
        ]);

        await expect(page).toHaveURL(/.*rustaceans/);
    });

    await test.step('Add a new rustacean', async () => {
        try {
            const name = 'Playwright Test';
            const email = `test-${Date.now()}@example.com`;

            await page.waitForSelector('text=Add new rustacean', { timeout: 10000 });
            await page.click('text=Add new rustacean');
            await page.fill('input[name="name"]', name);
            await page.fill('input[name="email"]', email);
            await page.click('text=Save');

            const row = page.locator('tr', { hasText: email });
            await expect(row).toContainText(name);
            await expect(row).toContainText(email);
        } catch (e) {
            await page.screenshot({ path: 'rustaceans-failure.png', fullPage: true });
            console.error(await page.content());
            throw e;
        }
    });
});
