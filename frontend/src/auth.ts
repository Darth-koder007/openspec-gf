export function getEmailFromQuery(queryString: string): string | null {
  const params = new URLSearchParams(queryString);
  return params.get('email');
}

export function isValidEmail(email: string): boolean {
  // Basic email validation: contains @ and has characters before and after it
  const parts = email.split('@');
  if (parts.length !== 2) {
    return false;
  }
  return parts[0].length > 0 && parts[1].length > 0 && parts[1].includes('.');
}
