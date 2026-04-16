import { LitElement, html, css } from 'lit';
import { customElement, state } from 'lit/decorators.js';
import type { Kudo, UserProfile } from './types';
import { fetchKudos, fetchUserProfile } from './services';
import './kudo-card';

@customElement('kudos-stream')
export class KudosStream extends LitElement {
  static styles = css`
    :host {
      display: block;
      font-family: system-ui, -apple-system, sans-serif;
      padding: 2rem;
    }

    .loading {
      color: #666;
      font-style: italic;
    }

    .error {
      color: #d32f2f;
      background-color: #ffebee;
      padding: 1rem;
      border-radius: 4px;
      border-left: 4px solid #d32f2f;
    }

    .empty {
      color: #666;
      text-align: center;
      padding: 2rem;
    }

    .empty-title {
      font-size: 1.25rem;
      margin-bottom: 0.5rem;
    }

    .empty-subtitle {
      font-size: 0.875rem;
    }

    .kudos-list {
      display: flex;
      flex-direction: column;
      gap: 1rem;
    }
  `;

  @state()
  loading = false;

  @state()
  kudos: Kudo[] = [];

  @state()
  senderProfiles: Map<string, UserProfile | null> = new Map();

  @state()
  error: string | null = null;

  @state()
  userEmail: string | null = null;

  connectedCallback() {
    super.connectedCallback();

    // Extract email from query string
    const params = new URLSearchParams(window.location.search);
    this.userEmail = params.get('email');

    if (this.userEmail) {
      this.loadKudos();
    }
  }

  async loadKudos() {
    if (!this.userEmail) return;

    this.loading = true;
    this.error = null;

    try {
      // Fetch kudos
      this.kudos = await fetchKudos(this.userEmail);

      // Fetch sender profiles for unique senders
      const uniqueSenderEmails = [...new Set(this.kudos.map(k => k.senderEmail))];
      await Promise.all(
        uniqueSenderEmails.map(async email => {
          try {
            const profile = await fetchUserProfile(email);
            this.senderProfiles.set(email, profile);
          } catch (error) {
            // If profile fetch fails, set to null (will fallback to email)
            this.senderProfiles.set(email, null);
            console.error(`Failed to fetch profile for ${email}:`, error);
          }
        })
      );

      // Trigger re-render
      this.requestUpdate();
    } catch (error) {
      this.error = error instanceof Error ? error.message : 'Failed to load kudos';
    } finally {
      this.loading = false;
    }
  }

  render() {
    if (this.loading) {
      return this.renderLoading();
    }

    if (this.error) {
      return this.renderError();
    }

    if (this.kudos.length === 0) {
      return this.renderEmpty();
    }

    return this.renderKudos();
  }

  private renderLoading() {
    return html`<div class="loading">Loading...</div>`;
  }

  private renderError() {
    return html`<div class="error">${this.error}</div>`;
  }

  private renderEmpty() {
    return html`
      <div class="empty">
        <div class="empty-title">You haven't received any kudos yet</div>
        <div class="empty-subtitle">When someone recognizes your work, it will appear here</div>
      </div>
    `;
  }

  private renderKudos() {
    return html`
      <div class="kudos-list">
        ${this.kudos.map(kudo => {
          const senderProfile = this.senderProfiles.get(kudo.senderEmail) || null;
          return html`
            <kudo-card .kudo=${kudo} .senderProfile=${senderProfile}></kudo-card>
          `;
        })}
      </div>
    `;
  }
}
