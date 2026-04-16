import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { formatRelativeTime } from './utils';

describe('formatRelativeTime', () => {
  beforeEach(() => {
    // Mock the current time to a fixed point: 2026-04-15T12:00:00Z
    vi.useFakeTimers();
    vi.setSystemTime(new Date('2026-04-15T12:00:00Z'));
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it('should return "just now" for timestamps within the last minute', () => {
    const timestamp = new Date('2026-04-15T11:59:30Z').toISOString();
    expect(formatRelativeTime(timestamp)).toBe('just now');
  });

  it('should return "1 minute ago" for timestamps exactly 1 minute ago', () => {
    const timestamp = new Date('2026-04-15T11:59:00Z').toISOString();
    expect(formatRelativeTime(timestamp)).toBe('1 minute ago');
  });

  it('should return "N minutes ago" for timestamps within the last hour', () => {
    const timestamp = new Date('2026-04-15T11:30:00Z').toISOString();
    expect(formatRelativeTime(timestamp)).toBe('30 minutes ago');
  });

  it('should return "1 hour ago" for timestamps exactly 1 hour ago', () => {
    const timestamp = new Date('2026-04-15T11:00:00Z').toISOString();
    expect(formatRelativeTime(timestamp)).toBe('1 hour ago');
  });

  it('should return "N hours ago" for timestamps within the last day', () => {
    const timestamp = new Date('2026-04-15T09:00:00Z').toISOString();
    expect(formatRelativeTime(timestamp)).toBe('3 hours ago');
  });

  it('should return "1 day ago" for timestamps exactly 1 day ago', () => {
    const timestamp = new Date('2026-04-14T12:00:00Z').toISOString();
    expect(formatRelativeTime(timestamp)).toBe('1 day ago');
  });

  it('should return "N days ago" for timestamps within the last week', () => {
    const timestamp = new Date('2026-04-13T12:00:00Z').toISOString();
    expect(formatRelativeTime(timestamp)).toBe('2 days ago');
  });

  it('should return "1 week ago" for timestamps exactly 1 week ago', () => {
    const timestamp = new Date('2026-04-08T12:00:00Z').toISOString();
    expect(formatRelativeTime(timestamp)).toBe('1 week ago');
  });

  it('should return "N weeks ago" for timestamps within the last month', () => {
    const timestamp = new Date('2026-04-01T12:00:00Z').toISOString();
    expect(formatRelativeTime(timestamp)).toBe('2 weeks ago');
  });

  it('should return "1 month ago" for timestamps around 1 month ago', () => {
    const timestamp = new Date('2026-03-15T12:00:00Z').toISOString();
    expect(formatRelativeTime(timestamp)).toBe('1 month ago');
  });

  it('should return "N months ago" for timestamps older than 1 month', () => {
    const timestamp = new Date('2026-02-01T12:00:00Z').toISOString();
    expect(formatRelativeTime(timestamp)).toBe('2 months ago');
  });
});
