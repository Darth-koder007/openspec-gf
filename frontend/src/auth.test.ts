import { describe, it, expect } from 'vitest';
import { getEmailFromQuery, isValidEmail } from './auth';

describe('getEmailFromQuery', () => {
  it('should extract email from query string', () => {
    const result = getEmailFromQuery('?email=john@deliveryhero.com');
    expect(result).toBe('john@deliveryhero.com');
  });

  it('should return null when email parameter is missing', () => {
    const result = getEmailFromQuery('?other=value');
    expect(result).toBeNull();
  });
});

describe('isValidEmail', () => {
  it('should return false for invalid email format', () => {
    expect(isValidEmail('invalid-email')).toBe(false);
    expect(isValidEmail('no-at-sign')).toBe(false);
    expect(isValidEmail('@no-local-part.com')).toBe(false);
    expect(isValidEmail('no-domain@')).toBe(false);
  });

  it('should return true for valid email format', () => {
    expect(isValidEmail('john@deliveryhero.com')).toBe(true);
    expect(isValidEmail('jane@example.org')).toBe(true);
  });
});
