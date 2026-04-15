import { describe, it, expect, vi, beforeEach } from 'vitest';
import { fetchUserProfile } from './services';

describe('fetchUserProfile', () => {
  beforeEach(() => {
    // Reset fetch mock before each test
    vi.resetAllMocks();
  });

  it('should fetch user profile successfully', async () => {
    const mockProfile = {
      displayName: 'John Smith',
      fullName: 'John Michael Smith',
      email: 'john@deliveryhero.com',
      avatarUrl: null,
    };

    global.fetch = vi.fn(() =>
      Promise.resolve({
        ok: true,
        json: () => Promise.resolve(mockProfile),
      } as Response)
    );

    const result = await fetchUserProfile('john@deliveryhero.com');
    expect(result).toEqual(mockProfile);
    expect(fetch).toHaveBeenCalledWith(expect.stringContaining('/user/john@deliveryhero.com'));
  });

  it('should throw error when user profile not found (404)', async () => {
    global.fetch = vi.fn(() =>
      Promise.resolve({
        ok: false,
        status: 404,
        statusText: 'Not Found',
      } as Response)
    );

    await expect(fetchUserProfile('nonexistent@example.com')).rejects.toThrow('Failed to fetch user profile: Not Found');
  });

  it('should throw error on network failure', async () => {
    global.fetch = vi.fn(() =>
      Promise.reject(new Error('Network error'))
    );

    await expect(fetchUserProfile('john@deliveryhero.com')).rejects.toThrow('Network error');
  });
});
