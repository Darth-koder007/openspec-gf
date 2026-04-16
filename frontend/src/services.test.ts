import { describe, it, expect, vi, beforeEach } from 'vitest';
import { fetchUserProfile, fetchKudos } from './services';

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

describe('fetchKudos', () => {
  beforeEach(() => {
    // Reset fetch mock before each test
    vi.resetAllMocks();
  });

  it('should fetch kudos successfully', async () => {
    const mockKudos = [
      {
        id: 1,
        senderEmail: 'jane@deliveryhero.com',
        recipientEmail: 'john@deliveryhero.com',
        message: 'Great work on the presentation!',
        createdAt: '2026-04-10T14:30:00Z',
        isPublic: true,
      },
      {
        id: 2,
        senderEmail: 'john@deliveryhero.com',
        recipientEmail: 'john@deliveryhero.com',
        message: 'Thanks for your help!',
        createdAt: '2026-04-09T10:15:00Z',
        isPublic: true,
      },
    ];

    global.fetch = vi.fn(() =>
      Promise.resolve({
        ok: true,
        json: () => Promise.resolve(mockKudos),
      } as Response)
    );

    const result = await fetchKudos('john@deliveryhero.com');
    expect(result).toEqual(mockKudos);
    expect(fetch).toHaveBeenCalledWith(expect.stringContaining('/kudos/john@deliveryhero.com'));
  });

  it('should return empty array when user has no kudos', async () => {
    global.fetch = vi.fn(() =>
      Promise.resolve({
        ok: true,
        json: () => Promise.resolve([]),
      } as Response)
    );

    const result = await fetchKudos('noone@example.com');
    expect(result).toEqual([]);
  });

  it('should throw error when request fails', async () => {
    global.fetch = vi.fn(() =>
      Promise.resolve({
        ok: false,
        status: 400,
        statusText: 'Bad Request',
      } as Response)
    );

    await expect(fetchKudos('invalid-email')).rejects.toThrow('Failed to fetch kudos: Bad Request');
  });

  it('should throw error on network failure', async () => {
    global.fetch = vi.fn(() =>
      Promise.reject(new Error('Network error'))
    );

    await expect(fetchKudos('john@deliveryhero.com')).rejects.toThrow('Network error');
  });
});
