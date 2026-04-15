import { expect, describe, it } from 'vitest';
import { fixture, html } from '@open-wc/testing';
import './hello-world';
import type { HelloWorld } from './hello-world';

describe('HelloWorld', () => {
  it('renders the component', async () => {
    const el = await fixture<HelloWorld>(html`<hello-world></hello-world>`);
    expect(el).to.exist;
    expect(el.shadowRoot).to.exist;
  });

  it('displays loading indicator while fetching profile', async () => {
    const el = await fixture<HelloWorld>(html`<hello-world></hello-world>`);
    const loadingEl = el.shadowRoot!.querySelector('.loading');
    expect(loadingEl).to.exist;
    expect(loadingEl!.textContent).to.contain('Loading');
  });

  it('displays personalized message with display name', async () => {
    const el = await fixture<HelloWorld>(html`<hello-world></hello-world>`);
    // Manually set profile data to test rendering
    el.profile = {
      displayName: 'John Smith',
      fullName: 'John Smith',
      email: 'john@deliveryhero.com',
      avatarUrl: null,
    };
    el.loading = false;
    await el.updateComplete;

    const messageEl = el.shadowRoot!.querySelector('.message');
    expect(messageEl).to.exist;
    expect(messageEl!.textContent).to.contain('Hello, John Smith!');
  });

  it('displays avatar when avatarUrl is present', async () => {
    const el = await fixture<HelloWorld>(html`<hello-world></hello-world>`);
    el.profile = {
      displayName: 'Jane Doe',
      fullName: 'Jane Doe',
      email: 'jane@deliveryhero.com',
      avatarUrl: 'https://example.com/avatar.jpg',
    };
    el.loading = false;
    await el.updateComplete;

    const avatarEl = el.shadowRoot!.querySelector('.avatar') as HTMLImageElement;
    expect(avatarEl).to.exist;
    expect(avatarEl.src).to.equal('https://example.com/avatar.jpg');
  });

  it('does not display avatar when avatarUrl is absent', async () => {
    const el = await fixture<HelloWorld>(html`<hello-world></hello-world>`);
    el.profile = {
      displayName: 'John Smith',
      fullName: 'John Smith',
      email: 'john@deliveryhero.com',
      avatarUrl: null,
    };
    el.loading = false;
    await el.updateComplete;

    const avatarEl = el.shadowRoot!.querySelector('.avatar');
    expect(avatarEl).to.not.exist;
  });

  it('displays error message on fetch failure', async () => {
    const el = await fixture<HelloWorld>(html`<hello-world></hello-world>`);
    el.error = 'Failed to fetch user profile';
    el.loading = false;
    await el.updateComplete;

    const errorEl = el.shadowRoot!.querySelector('.error');
    expect(errorEl).to.exist;
    expect(errorEl!.textContent).to.contain('Failed to fetch user profile');
  });
});
