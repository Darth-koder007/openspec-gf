import { describe, it, expect, vi } from 'vitest';
import { initApp } from './main';
import * as auth from './auth';
import * as services from './services';
import type { UserProfile } from './types';

// Mock the HelloWorld component
vi.mock('./hello-world', () => ({
  HelloWorld: class {
    loading = true;
    profile = null;
    error = null;
  },
}));

describe('App Initialization', () => {
  it('initializes app with email from query string', async () => {
    const getEmailSpy = vi.spyOn(auth, 'getEmailFromQuery');
    const fetchProfileSpy = vi.spyOn(services, 'fetchUserProfile');

    getEmailSpy.mockReturnValue('john@deliveryhero.com');
    const mockProfile: UserProfile = {
      displayName: 'John Smith',
      fullName: 'John Smith',
      email: 'john@deliveryhero.com',
      avatarUrl: null,
    };
    fetchProfileSpy.mockResolvedValue(mockProfile);

    const component = await initApp('?email=john@deliveryhero.com');

    expect(getEmailSpy).toHaveBeenCalledWith('?email=john@deliveryhero.com');
    expect(fetchProfileSpy).toHaveBeenCalledWith('john@deliveryhero.com');
    expect(component).toBeDefined();
  });
});
