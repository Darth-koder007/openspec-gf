import { LitElement, html, css } from 'lit';
import { customElement, state } from 'lit/decorators.js';
import type { UserProfile } from './types';

@customElement('hello-world')
export class HelloWorld extends LitElement {
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

    .container {
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 1rem;
    }

    .avatar {
      width: 80px;
      height: 80px;
      border-radius: 50%;
      object-fit: cover;
    }

    .message {
      font-size: 1.5rem;
      font-weight: 500;
      color: #333;
    }
  `;

  @state()
  loading = true;

  @state()
  profile: UserProfile | null = null;

  @state()
  error: string | null = null;

  render() {
    if (this.loading) {
      return this.renderLoading();
    }

    if (this.error) {
      return this.renderError();
    }

    if (this.profile) {
      return this.renderProfile();
    }

    return html`<div>Hello World</div>`;
  }

  private renderLoading() {
    return html`<div class="loading">Loading...</div>`;
  }

  private renderError() {
    return html`<div class="error">${this.error}</div>`;
  }

  private renderProfile() {
    return html`
      <div class="container">
        ${this.profile!.avatarUrl
          ? html`<img class="avatar" src="${this.profile!.avatarUrl}" alt="Avatar" />`
          : ''}
        <div class="message">
          Hello, ${this.profile!.displayName}!
        </div>
      </div>
    `;
  }
}
