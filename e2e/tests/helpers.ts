/**
 * Test helpers for E2E tests
 *
 * NOTE: The backend automatically seeds test users on startup:
 * - John Smith (john@deliveryhero.com)
 * - Jane Doe (jane@deliveryhero.com)
 *
 * No additional seeding is required for E2E tests.
 */

export const TEST_USERS = {
  john: {
    email: 'john@deliveryhero.com',
    displayName: 'John Smith',
    fullName: 'John Michael Smith',
    avatarUrl: null,
  },
  jane: {
    email: 'jane@deliveryhero.com',
    displayName: 'Jane Doe',
    fullName: 'Jane Elizabeth Doe',
    avatarUrl: null,
  },
};

/**
 * Build URL with email query parameter
 */
export function buildUrlWithEmail(email: string): string {
  return `/?email=${encodeURIComponent(email)}`;
}

/**
 * Wait for element to be visible
 */
export async function waitForElement(page: any, selector: string, timeout = 5000) {
  await page.waitForSelector(selector, { state: 'visible', timeout });
}
