import './style.css';
import './hello-world';
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
if (typeof window !== 'undefined') {
  initApp(window.location.search).then((component) => {
    const app = document.querySelector<HTMLDivElement>('#app');
    if (app) {
      app.appendChild(component);
    }
  });
}
