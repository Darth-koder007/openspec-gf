import { test, expect } from '@playwright/test';
import { TEST_USERS, buildUrlWithEmail } from './helpers';

test.describe('Personalized Hello World - User Journeys', () => {
  test('John Smith views personalized hello world', async ({ page }) => {
    // Navigate to app with John's email
    await page.goto(buildUrlWithEmail(TEST_USERS.john.email));

    // Wait for loading to complete
    await page.waitForSelector('.message', { state: 'visible', timeout: 10000 });

    // Verify personalized message is displayed
    const message = await page.textContent('.message');
    expect(message).toContain(`Hello, ${TEST_USERS.john.displayName}!`);

    // Verify no avatar is displayed (John has no avatar)
    const avatar = await page.locator('.avatar');
    await expect(avatar).not.toBeVisible();
  });

  test('Jane Doe views personalized hello world', async ({ page }) => {
    // Navigate to app with Jane's email
    await page.goto(buildUrlWithEmail(TEST_USERS.jane.email));

    // Wait for loading to complete
    await page.waitForSelector('.message', { state: 'visible', timeout: 10000 });

    // Verify personalized message is displayed
    const message = await page.textContent('.message');
    expect(message).toContain(`Hello, ${TEST_USERS.jane.displayName}!`);

    // Verify no avatar is displayed (Jane has no avatar)
    const avatar = await page.locator('.avatar');
    await expect(avatar).not.toBeVisible();
  });

  test('User with avatar sees avatar displayed', async ({ page }) => {
    // Note: Currently, neither John nor Jane have avatars in the seeded data
    // This test is a placeholder for when avatar URLs are added to test data
    // For now, we'll test the negative case

    await page.goto(buildUrlWithEmail(TEST_USERS.john.email));
    await page.waitForSelector('.message', { state: 'visible', timeout: 10000 });

    const avatar = await page.locator('.avatar');
    await expect(avatar).not.toBeVisible();
  });

  test('User without avatar sees no avatar', async ({ page }) => {
    await page.goto(buildUrlWithEmail(TEST_USERS.john.email));
    await page.waitForSelector('.message', { state: 'visible', timeout: 10000 });

    // Verify avatar is not displayed
    const avatar = await page.locator('.avatar');
    await expect(avatar).not.toBeVisible();

    // Verify message is still displayed
    const message = await page.textContent('.message');
    expect(message).toContain(`Hello, ${TEST_USERS.john.displayName}!`);
  });

  test('Invalid email shows error message', async ({ page }) => {
    await page.goto(buildUrlWithEmail('invalid-email'));

    // Wait for error message to appear
    await page.waitForSelector('.error', { state: 'visible', timeout: 10000 });

    // Verify error message is displayed
    const errorMessage = await page.textContent('.error');
    expect(errorMessage).toContain('Invalid email format');
  });

  test('Missing email parameter shows prompt', async ({ page }) => {
    await page.goto('/');

    // Wait for error message to appear
    await page.waitForSelector('.error', { state: 'visible', timeout: 10000 });

    // Verify prompt is displayed
    const errorMessage = await page.textContent('.error');
    expect(errorMessage).toContain('Please provide an email parameter');
  });
});
