import { expect, describe, it, vi, beforeEach, afterEach } from 'vitest';
import { fixture, html } from '@open-wc/testing';
import './kudo-card';
import type { KudoCard } from './kudo-card';
import type { Kudo, UserProfile } from './types';

describe('KudoCard', () => {
  const mockKudo: Kudo = {
    id: 1,
    senderEmail: 'jane@deliveryhero.com',
    recipientEmail: 'john@deliveryhero.com',
    message: 'Great work on the presentation!',
    createdAt: '2026-04-13T12:00:00Z',
    isPublic: true,
  };

  const mockSenderProfile: UserProfile = {
    displayName: 'Jane Doe',
    fullName: 'Jane Doe',
    email: 'jane@deliveryhero.com',
    avatarUrl: 'https://example.com/avatar.jpg',
  };

  beforeEach(() => {
    // Mock the current time to a fixed point: 2026-04-15T12:00:00Z
    vi.useFakeTimers();
    vi.setSystemTime(new Date('2026-04-15T12:00:00Z'));
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it('renders the component', async () => {
    const el = await fixture<KudoCard>(html`<kudo-card .kudo=${mockKudo}></kudo-card>`);
    expect(el).to.exist;
    expect(el.shadowRoot).to.exist;
  });

  it('displays sender display name when profile is provided', async () => {
    const el = await fixture<KudoCard>(
      html`<kudo-card .kudo=${mockKudo} .senderProfile=${mockSenderProfile}></kudo-card>`
    );
    await el.updateComplete;

    const senderNameEl = el.shadowRoot!.querySelector('.sender-name');
    expect(senderNameEl).to.exist;
    expect(senderNameEl!.textContent).to.equal('Jane Doe');
  });

  it('displays sender email when profile is not provided', async () => {
    const el = await fixture<KudoCard>(html`<kudo-card .kudo=${mockKudo}></kudo-card>`);
    await el.updateComplete;

    const senderNameEl = el.shadowRoot!.querySelector('.sender-name');
    expect(senderNameEl).to.exist;
    expect(senderNameEl!.textContent).to.equal('jane@deliveryhero.com');
  });

  it('displays sender avatar when profile has avatarUrl', async () => {
    const el = await fixture<KudoCard>(
      html`<kudo-card .kudo=${mockKudo} .senderProfile=${mockSenderProfile}></kudo-card>`
    );
    await el.updateComplete;

    const avatarEl = el.shadowRoot!.querySelector('.avatar') as HTMLImageElement;
    expect(avatarEl).to.exist;
    expect(avatarEl.tagName).to.equal('IMG');
    expect(avatarEl.src).to.equal('https://example.com/avatar.jpg');
  });

  it('displays placeholder when profile has no avatarUrl', async () => {
    const profileWithoutAvatar: UserProfile = {
      ...mockSenderProfile,
      avatarUrl: null,
    };
    const el = await fixture<KudoCard>(
      html`<kudo-card .kudo=${mockKudo} .senderProfile=${profileWithoutAvatar}></kudo-card>`
    );
    await el.updateComplete;

    const avatarEl = el.shadowRoot!.querySelector('.avatar') as HTMLDivElement;
    expect(avatarEl).to.exist;
    expect(avatarEl.tagName).to.equal('DIV');
  });

  it('displays the kudo message', async () => {
    const el = await fixture<KudoCard>(html`<kudo-card .kudo=${mockKudo}></kudo-card>`);
    await el.updateComplete;

    const messageEl = el.shadowRoot!.querySelector('.message');
    expect(messageEl).to.exist;
    expect(messageEl!.textContent).to.equal('Great work on the presentation!');
  });

  it('displays relative timestamp', async () => {
    const el = await fixture<KudoCard>(html`<kudo-card .kudo=${mockKudo}></kudo-card>`);
    await el.updateComplete;

    const timestampEl = el.shadowRoot!.querySelector('.timestamp');
    expect(timestampEl).to.exist;
    expect(timestampEl!.textContent).to.equal('2 days ago');
  });
});
