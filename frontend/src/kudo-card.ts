import { LitElement, html, css } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import type { Kudo, UserProfile } from './types';
import { formatRelativeTime } from './utils';

@customElement('kudo-card')
export class KudoCard extends LitElement {
  static styles = css`
    :host {
      display: block;
      font-family: system-ui, -apple-system, sans-serif;
    }

    .card {
      padding: 1rem;
      border: 1px solid #e0e0e0;
      border-radius: 4px;
      background-color: #fff;
    }

    .header {
      display: flex;
      align-items: center;
      gap: 0.75rem;
      margin-bottom: 0.5rem;
    }

    .avatar {
      width: 40px;
      height: 40px;
      border-radius: 50%;
      object-fit: cover;
      background-color: #e0e0e0;
    }

    .sender-info {
      display: flex;
      flex-direction: column;
    }

    .sender-name {
      font-weight: 500;
      color: #333;
    }

    .timestamp {
      font-size: 0.875rem;
      color: #666;
    }

    .message {
      color: #333;
      line-height: 1.5;
      margin-top: 0.5rem;
    }
  `;

  @property({ type: Object })
  kudo!: Kudo;

  @property({ type: Object })
  senderProfile: UserProfile | null = null;

  render() {
    const displayName = this.senderProfile?.displayName || this.kudo.senderEmail;
    const avatarUrl = this.senderProfile?.avatarUrl;
    const relativeTime = formatRelativeTime(this.kudo.createdAt);

    return html`
      <div class="card">
        <div class="header">
          ${avatarUrl
            ? html`<img class="avatar" src="${avatarUrl}" alt="Avatar" />`
            : html`<div class="avatar"></div>`}
          <div class="sender-info">
            <div class="sender-name">${displayName}</div>
            <div class="timestamp">${relativeTime}</div>
          </div>
        </div>
        <div class="message">${this.kudo.message}</div>
      </div>
    `;
  }
}
