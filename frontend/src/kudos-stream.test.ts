import { expect, describe, it, vi, beforeEach } from 'vitest';
import { fixture, html } from '@open-wc/testing';
import './kudos-stream';
import type { KudosStream } from './kudos-stream';
import type { Kudo, UserProfile } from './types';
import * as services from './services';

describe('KudosStream', () => {
  const mockKudos: Kudo[] = [
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
      senderEmail: 'alice@deliveryhero.com',
      recipientEmail: 'john@deliveryhero.com',
      message: 'Thanks for your help!',
      createdAt: '2026-04-09T10:15:00Z',
      isPublic: true,
    },
  ];

  const mockProfile: UserProfile = {
    displayName: 'Jane Doe',
    fullName: 'Jane Doe',
    email: 'jane@deliveryhero.com',
    avatarUrl: 'https://example.com/avatar.jpg',
  };

  beforeEach(() => {
    vi.resetAllMocks();
    // Mock window.location.search
    delete (window as any).location;
    (window as any).location = { search: '?email=john@deliveryhero.com' };
  });

  it('renders the component', async () => {
    vi.spyOn(services, 'fetchKudos').mockResolvedValue([]);
    const el = await fixture<KudosStream>(html`<kudos-stream></kudos-stream>`);
    expect(el).to.exist;
    expect(el.shadowRoot).to.exist;
  });

  it('displays loading indicator while fetching kudos', async () => {
    // Create a never-resolving promise to keep loading state
    vi.spyOn(services, 'fetchKudos').mockImplementation(
      () => new Promise(() => {})
    );

    const el = await fixture<KudosStream>(html`<kudos-stream></kudos-stream>`);
    await el.updateComplete;

    const loadingEl = el.shadowRoot!.querySelector('.loading');
    expect(loadingEl).to.exist;
    expect(loadingEl!.textContent).to.contain('Loading');
  });

  it('displays empty state when no kudos are found', async () => {
    vi.spyOn(services, 'fetchKudos').mockResolvedValue([]);

    const el = await fixture<KudosStream>(html`<kudos-stream></kudos-stream>`);
    await new Promise(resolve => setTimeout(resolve, 10)); // Wait for async operations
    await el.updateComplete;

    const emptyEl = el.shadowRoot!.querySelector('.empty');
    expect(emptyEl).to.exist;
    expect(emptyEl!.textContent).to.contain("You haven't received any kudos yet");
  });

  it('displays error message when kudos fetch fails', async () => {
    vi.spyOn(services, 'fetchKudos').mockRejectedValue(new Error('Network error'));

    const el = await fixture<KudosStream>(html`<kudos-stream></kudos-stream>`);
    await new Promise(resolve => setTimeout(resolve, 10)); // Wait for async operations
    await el.updateComplete;

    const errorEl = el.shadowRoot!.querySelector('.error');
    expect(errorEl).to.exist;
    expect(errorEl!.textContent).to.contain('Network error');
  });

  it('displays kudos when successfully fetched', async () => {
    vi.spyOn(services, 'fetchKudos').mockResolvedValue(mockKudos);
    vi.spyOn(services, 'fetchUserProfile').mockResolvedValue(mockProfile);

    const el = await fixture<KudosStream>(html`<kudos-stream></kudos-stream>`);
    await new Promise(resolve => setTimeout(resolve, 10)); // Wait for async operations
    await el.updateComplete;

    const kudoCards = el.shadowRoot!.querySelectorAll('kudo-card');
    expect(kudoCards.length).to.equal(2);
  });

  it('fetches sender profiles for unique senders', async () => {
    const fetchKudosSpy = vi.spyOn(services, 'fetchKudos').mockResolvedValue(mockKudos);
    const fetchProfileSpy = vi.spyOn(services, 'fetchUserProfile').mockResolvedValue(mockProfile);

    const el = await fixture<KudosStream>(html`<kudos-stream></kudos-stream>`);
    await new Promise(resolve => setTimeout(resolve, 10)); // Wait for async operations
    await el.updateComplete;

    expect(fetchKudosSpy).toHaveBeenCalledWith('john@deliveryhero.com');
    expect(fetchProfileSpy).toHaveBeenCalledWith('jane@deliveryhero.com');
    expect(fetchProfileSpy).toHaveBeenCalledWith('alice@deliveryhero.com');
    expect(fetchProfileSpy).toHaveBeenCalledTimes(2); // Two unique senders
  });

  it('handles sender profile fetch failure gracefully', async () => {
    vi.spyOn(services, 'fetchKudos').mockResolvedValue(mockKudos);
    vi.spyOn(services, 'fetchUserProfile').mockRejectedValue(new Error('Profile not found'));

    const el = await fixture<KudosStream>(html`<kudos-stream></kudos-stream>`);
    await new Promise(resolve => setTimeout(resolve, 10)); // Wait for async operations
    await el.updateComplete;

    // Should still render kudos even if profile fetch fails
    const kudoCards = el.shadowRoot!.querySelectorAll('kudo-card');
    expect(kudoCards.length).to.equal(2);
  });
});
