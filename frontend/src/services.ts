import type { UserProfile } from './types';

// Get API base URL from environment variable or use default
const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:3000';

export async function fetchUserProfile(email: string): Promise<UserProfile> {
  const response = await fetch(`${API_BASE_URL}/user/${email}`);

  if (!response.ok) {
    throw new Error(`Failed to fetch user profile: ${response.statusText}`);
  }

  return response.json();
}
