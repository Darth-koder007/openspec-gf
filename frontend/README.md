# Kudos Frontend

Vite + TypeScript + Lit web application displaying personalized hello world messages.

## Component Architecture

### HelloWorld Component (`src/hello-world.ts`)

Main Lit web component that displays personalized greetings based on user profile data.

**Features:**
- Loading state with indicator
- Personalized message with user's display name
- Conditional avatar display
- Error handling and display

**Properties:**
- `loading` (boolean) - Loading state
- `profile` (UserProfile | null) - User profile data
- `error` (string | null) - Error message

**States:**
1. **Loading** - Shows "Loading..." indicator
2. **Success** - Shows personalized message with optional avatar
3. **Error** - Shows error message

### Services (`src/services.ts`)

API client for fetching user profiles from the backend.

**Functions:**
- `fetchUserProfile(email: string): Promise<UserProfile>` - Fetches user profile by email

**Configuration:**
- `VITE_API_BASE_URL` - Backend API URL (default: `http://localhost:3000`)

### Auth Utilities (`src/auth.ts`)

Simple query-string based authentication for development.

**Functions:**
- `getEmailFromQuery(queryString: string): string | null` - Extracts email from URL query parameters
- `isValidEmail(email: string): boolean` - Validates email format

### Main App (`src/main.ts`)

Application entry point that initializes the HelloWorld component.

**Functions:**
- `initApp(queryString: string): Promise<HelloWorld>` - Initializes app with email from query string

**Flow:**
1. Extract email from URL query string
2. Validate email format
3. Fetch user profile from backend
4. Render HelloWorld component with profile data
5. Handle errors gracefully

### Types (`src/types.ts`)

TypeScript interfaces matching backend schema.

```typescript
interface UserProfile {
  displayName: string;
  fullName: string;
  email: string;
  avatarUrl: string | null;
}
```

## Development

### Prerequisites

- Node.js 22.12.0+ (use `nvm use` to switch to correct version)
- npm 10+

### Running Locally

```bash
npm install
npm run dev
```

The app will be available at `http://localhost:5173` (Vite dev server).

### Building for Production

```bash
npm run build
```

Output will be in `dist/` directory.

### Running Tests

```bash
npm test
```

**Test Framework:** Vitest with jsdom environment
**Component Testing:** @open-wc/testing helpers

## Docker

### Building

```bash
docker build -t kudos-frontend .
```

### Running

```bash
docker run -p 80:80 kudos-frontend
```

## Production Architecture

In production, the frontend is served by nginx with the following features:

### Reverse Proxy

All `/api/*` requests are proxied to the backend service, eliminating CORS complexity.

### SPA Routing

All routes fall back to `index.html` for client-side routing.

### Optimizations

- Gzip compression for text assets
- Cache headers for static assets (1 year)
- Security headers (X-Frame-Options, X-Content-Type-Options, X-XSS-Protection)

## Environment Variables

- `VITE_API_BASE_URL` - Backend API URL
  - Development: `http://localhost:3000`
  - Production: `/api` (proxied by nginx)

## Testing

### Unit Tests

Located in `src/*.test.ts` files:
- `auth.test.ts` - Auth utility tests
- `services.test.ts` - API service tests
- `hello-world.test.ts` - Component tests
- `main.test.ts` - App initialization tests

All tests follow strict TDD practices with Red-Green-Refactor cycles.

### Test Coverage

- Auth: Email extraction and validation
- Services: Profile fetching and error handling
- Component: Loading, success, error states, avatar display
- Main: App initialization with query string

## Example Usage

### With Query String

```
http://localhost/?email=john@deliveryhero.com
```

Displays personalized greeting for John Smith.

### Test Users

- `john@deliveryhero.com` - John Smith
- `jane@deliveryhero.com` - Jane Doe
