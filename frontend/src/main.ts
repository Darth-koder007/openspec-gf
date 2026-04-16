import './style.css';
import './hello-world';
import './kudos-stream';
import { getEmailFromQuery, isValidEmail } from './auth';
import { fetchUserProfile } from './services';
import type { HelloWorld } from './hello-world';

export async function initApp(queryString: string): Promise<HelloWorld> {
  const component = document.createElement('hello-world') as HelloWorld;

  // Extract email from query string
  const email = getEmailFromQuery(queryString);

  if (!email) {
    component.loading = false;
    component.error = 'Please provide an email parameter in the URL';
    return component;
  }

  if (!isValidEmail(email)) {
    component.loading = false;
    component.error = 'Invalid email format';
    return component;
  }

  // Fetch user profile
  try {
    const profile = await fetchUserProfile(email);
    component.loading = false;
    component.profile = profile;
  } catch (error) {
    component.loading = false;
    component.error = error instanceof Error ? error.message : 'Failed to fetch user profile';
  }

  return component;
}

// Initialize app on page load
if (typeof window !== 'undefined' && document.querySelector('#app')) {
  const app = document.querySelector<HTMLDivElement>('#app');

  // Simple routing based on path
  const path = window.location.pathname;

  if (path === '/kudos' || path === '/kudos.html') {
    // Render kudos stream
    const kudosStream = document.createElement('kudos-stream');
    app!.appendChild(kudosStream);
  } else {
    // Default: render hello-world
    initApp(window.location.search).then((component) => {
      app!.appendChild(component);
    });
  }
}
