import { test, expect } from '@playwright/test';
import { loginAsAdmin } from './utils/auth';

test('can add a crate', async ({ page }) => {
  await loginAsAdmin(page);

  // Navigate to Crates section
  await page.click('text=Crates');
  await page.click('text=Add new crate');

  const timestamp = Date.now();
  const code = `${Math.floor(Math.random() * 1000000)}`;
  const name = `crate-${timestamp}`;
  const version = '1.0.0';
  const description = `Created by test run at ${new Date().toISOString()}`;

  await page.fill('input[name="code"]', code);
  await page.fill('input[name="name"]', name);
  await page.fill('input[name="version"]', version);

  // Select "Playwright Test" from the dropdown and capture its ID (value)
  const authorId = await page
    .locator('select[name="author"] option', { hasText: 'Playwright Test' })
    .first()
    .getAttribute('value');

  await page.selectOption('select[name="author"]', { label: 'Playwright Test' });

  await page.fill('textarea[name="description"]', description);
  await page.click('text=Save');

  // Locate the new row in the table
  const row = page.locator('tr', { hasText: name });

  // Verify it contains all expected info
  await expect(row).toContainText(code);
  await expect(row).toContainText(name);
  console.log('Row contents:', await row.innerText());
  // TODO: Uncomment once the version is displayed in the table
  //await expect(row).toContainText(version);
  await expect(row).toContainText(authorId!); // Rustacean ID

  // Bonus: Print row contents to terminal for CI/debug trace
  const text = await row.innerText();
  console.log('✅ Created crate row:', text);
});
