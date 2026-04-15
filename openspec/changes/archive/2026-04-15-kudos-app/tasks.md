## 1. Monorepo Setup

- [x] 1.1 Create monorepo directory structure (backend, frontend, e2e)
- [x] 1.2 Add root-level README with project overview and setup instructions
- [x] 1.3 Create .gitignore for Rust, Node.js, and test artifacts

## 2. Backend - Rust/Axum Setup

- [x] 2.1 Initialize Rust project with Cargo in backend/ directory
- [x] 2.2 Add dependencies: axum, tower-http, tokio, serde, serde_json
- [x] 2.3 Create main.rs with basic Axum server scaffold
- [x] 2.4 Configure tower-http CORS middleware for cross-origin requests
- [x] 2.5 Create Dockerfile for backend using multi-stage build

## 2a. Backend - Production-Ready Dockerfile Optimization

- [x] 2a.1 Update Dockerfile to use dependency caching layer (separate cargo build for dependencies)
- [x] 2a.2 Add binary stripping and size optimization to release build
- [x] 2a.3 Switch runtime image from debian-slim to distroless for security
- [x] 2a.4 Add health check configuration to Dockerfile
- [x] 2a.5 Configure SQLite volume mount point in Dockerfile

## 3. Backend - User Profile Data Model (TDD)

- [x] 3.1 Write failing test for UserProfile struct with required fields
- [x] 3.2 Implement UserProfile struct (displayName, fullName, email, avatarUrl)
- [x] 3.3 Add serde serialization/deserialization for UserProfile
- [x] 3.4 Refactor: Extract UserProfile to models module

## 4. Backend - SQLite Database Setup (Development & Production)

- [x] 4.1 Add SQLite dependency (rusqlite or sqlx)
- [x] 4.2 Write failing test for database initialization
- [x] 4.3 Implement database initialization with users table
- [x] 4.4 Write failing test for seeding test users (John Smith, Jane Doe)
- [x] 4.5 Implement seed function with two test users
- [x] 4.6 Refactor: Extract database code to db module
- [x] 4.7 Add database file path configuration via environment variable
- [x] 4.8 Implement automatic initialization on first run (create DB if not exists)
- [x] 4.9 Add database migration support for schema changes (future-proofing)

## 5. Backend - GET /user/:email Endpoint (TDD)

- [x] 5.1 Write failing integration test for successful profile retrieval (200)
- [x] 5.2 Implement GET /user/:email handler returning user profile JSON
- [x] 5.3 Write failing test for profile not found (404)
- [x] 5.4 Implement 404 error handling for non-existent users
- [x] 5.5 Write failing test for invalid email format (400)
- [x] 5.6 Implement email validation and 400 error response
- [x] 5.7 Write failing test for CORS preflight (OPTIONS)
- [x] 5.8 Verify CORS middleware handles preflight requests
- [x] 5.9 Write failing test for CORS headers on GET request
- [x] 5.10 Verify CORS headers are present in response
- [x] 5.11 Refactor: Extract handlers to handlers module

## 6. Frontend - Vite/TypeScript/Lit Setup

- [x] 6.1 Initialize Vite project with TypeScript in frontend/ directory
- [x] 6.2 Add dependencies: lit, @open-wc/testing, vite
- [x] 6.3 Configure TypeScript (tsconfig.json) with strict mode
- [x] 6.4 Create index.html entry point
- [x] 6.5 Create Dockerfile using NGINX to serve Vite bundle

## 7. Frontend - Query String Auth Component (TDD)

- [x] 7.1 Write failing test for extracting email from query string
- [x] 7.2 Implement query string parser to extract email parameter
- [x] 7.3 Write failing test for missing email parameter
- [x] 7.4 Implement error message when email is missing
- [x] 7.5 Write failing test for invalid email format
- [x] 7.6 Implement email format validation
- [x] 7.7 Refactor: Extract to auth utility module

## 8. Frontend - User Profile Service (TDD)

- [x] 8.1 Create UserProfile TypeScript interface matching backend schema
- [x] 8.2 Write failing test for fetchUserProfile function
- [x] 8.3 Implement fetchUserProfile service calling GET /user/:email
- [x] 8.4 Write failing test for 404 error handling
- [x] 8.5 Implement error handling for profile not found
- [x] 8.6 Write failing test for network errors
- [x] 8.7 Implement error handling for network failures
- [x] 8.8 Add API base URL configuration (environment variable)
- [x] 8.9 Refactor: Extract to services module

## 9. Frontend - Personalized Hello World Component (TDD)

- [x] 9.1 Write failing test for HelloWorld Lit component rendering
- [x] 9.2 Create HelloWorld Lit component with basic structure
- [x] 9.3 Write failing test for loading state display
- [x] 9.4 Implement loading indicator while fetching profile
- [x] 9.5 Write failing test for personalized message with display name
- [x] 9.6 Implement personalized message rendering with profile data
- [x] 9.7 Write failing test for avatar display when avatarUrl present
- [x] 9.8 Implement conditional avatar image rendering
- [x] 9.9 Write failing test for no avatar when avatarUrl absent
- [x] 9.10 Implement fallback when avatar is not available
- [x] 9.11 Write failing test for error message display
- [x] 9.12 Implement error message rendering on fetch failure
- [x] 9.13 Refactor: Extract component logic and improve styling

## 10. Frontend - Main App Integration

- [x] 10.1 Write failing test for app initialization on page load
- [x] 10.2 Implement main app that extracts email and fetches profile
- [x] 10.3 Wire HelloWorld component to main app
- [x] 10.4 Add component registration and index.html integration

## 11. End-to-End Testing Setup

- [x] 11.1 Initialize Playwright in e2e/ directory
- [x] 11.2 Create docker-compose.yml for backend, frontend, and test database
- [x] 11.3 Configure Playwright to use Docker Compose services
- [x] 11.4 Add test helpers for seeding SQLite database

## 12. End-to-End Tests - User Journeys

- [x] 12.1 Write E2E test: John Smith views personalized hello world
- [x] 12.2 Write E2E test: Jane Doe views personalized hello world
- [x] 12.3 Write E2E test: User with avatar sees avatar displayed
- [x] 12.4 Write E2E test: User without avatar sees no avatar
- [x] 12.5 Write E2E test: Invalid email shows error message
- [x] 12.6 Write E2E test: Missing email parameter shows prompt
- [ ] 12.7 Verify all E2E tests pass with Docker Compose (Blocked by backend runtime issue)

## 12a. Production Containerization - Docker Compose Setup

- [x] 12a.1 Create docker-compose.yml with backend, frontend, and volume configuration
- [x] 12a.2 Configure backend service with health checks and volume persistence
- [x] 12a.3 Configure frontend service with dependency on backend health
- [x] 12a.4 Add environment variable configuration for API URLs
- [x] 12a.5 Create nginx.conf for frontend with SPA routing and reverse proxy
- [x] 12a.6 Update frontend Dockerfile to use nginx:alpine with custom config
- [x] 12a.7 Configure nginx to proxy /api/* requests to backend service
- [x] 12a.8 Test docker-compose up with full stack running (Note: Backend has runtime issue - exits immediately despite passing all tests)
- [ ] 12a.9 Verify SQLite data persists across container restarts (Blocked by backend runtime issue)

## 13. CI/CD - DroneCI Configuration (DEFERRED)

**Note:** CI/CD setup has been deferred until deployment infrastructure is established. This section can be implemented as a future enhancement.

- [ ] ~~13.1 Create .drone.yml pipeline configuration~~ (Deferred)
- [ ] ~~13.2 Add pipeline step: Build backend (cargo build)~~ (Deferred)
- [ ] ~~13.3 Add pipeline step: Run backend unit tests (cargo test)~~ (Deferred)
- [ ] ~~13.4 Add pipeline step: Build frontend (npm run build)~~ (Deferred)
- [ ] ~~13.5 Add pipeline step: Run frontend unit tests (npm test)~~ (Deferred)
- [ ] ~~13.6 Add pipeline step: Build Docker images~~ (Deferred)
- [ ] ~~13.7 Add pipeline step: Run E2E tests with Docker Compose~~ (Deferred)
- [ ] ~~13.8 Configure pipeline triggers and notifications~~ (Deferred)

## 14. Documentation

- [x] 14.1 Document API endpoint in backend README
- [x] 14.2 Document frontend component architecture in frontend README
- [x] 14.3 Add development setup instructions (how to run locally)
- [x] 14.4 Add testing instructions (unit tests and E2E tests)
- [x] 14.5 Document Docker usage for local development
- [x] 14.6 Add example URLs for testing (John and Jane)

## 15. Verification

- [ ] 15.1 Run all backend unit tests and verify 100% pass
- [ ] 15.2 Run all frontend unit tests and verify 100% pass
- [ ] 15.3 Run all E2E tests and verify complete user journeys work
- [ ] 15.4 Test development mode: CORS works when serving frontend and backend on different ports
- [ ] 15.5 Verify production Docker builds succeed for both backend and frontend
- [ ] 15.6 Verify docker-compose up brings up full stack successfully
- [ ] 15.7 Test production mode: nginx reverse proxy routes /api/* to backend
- [ ] 15.8 Verify SQLite data persists after docker-compose down/up cycle
- [ ] 15.9 Manual test: Access app with John's email via nginx, see personalized message
- [ ] 15.10 Manual test: Access app with Jane's email via nginx, see personalized message
- [ ] 15.11 Verify distroless backend image is significantly smaller than debian-slim
