## 1. Monorepo Setup

- [ ] 1.1 Create monorepo directory structure (backend, frontend, e2e)
- [ ] 1.2 Add root-level README with project overview and setup instructions
- [ ] 1.3 Create .gitignore for Rust, Node.js, and test artifacts

## 2. Backend - Rust/Axum Setup

- [ ] 2.1 Initialize Rust project with Cargo in backend/ directory
- [ ] 2.2 Add dependencies: axum, tower-http, tokio, serde, serde_json
- [ ] 2.3 Create main.rs with basic Axum server scaffold
- [ ] 2.4 Configure tower-http CORS middleware for cross-origin requests
- [ ] 2.5 Create Dockerfile for backend using multi-stage build

## 3. Backend - User Profile Data Model (TDD)

- [ ] 3.1 Write failing test for UserProfile struct with required fields
- [ ] 3.2 Implement UserProfile struct (displayName, fullName, email, avatarUrl)
- [ ] 3.3 Add serde serialization/deserialization for UserProfile
- [ ] 3.4 Refactor: Extract UserProfile to models module

## 4. Backend - Test Database Setup

- [ ] 4.1 Add SQLite dependency (rusqlite or sqlx)
- [ ] 4.2 Write failing test for database initialization
- [ ] 4.3 Implement database initialization with users table
- [ ] 4.4 Write failing test for seeding test users (John Smith, Jane Doe)
- [ ] 4.5 Implement seed function with two test users
- [ ] 4.6 Refactor: Extract database code to db module

## 5. Backend - GET /user/:email Endpoint (TDD)

- [ ] 5.1 Write failing integration test for successful profile retrieval (200)
- [ ] 5.2 Implement GET /user/:email handler returning user profile JSON
- [ ] 5.3 Write failing test for profile not found (404)
- [ ] 5.4 Implement 404 error handling for non-existent users
- [ ] 5.5 Write failing test for invalid email format (400)
- [ ] 5.6 Implement email validation and 400 error response
- [ ] 5.7 Write failing test for CORS preflight (OPTIONS)
- [ ] 5.8 Verify CORS middleware handles preflight requests
- [ ] 5.9 Write failing test for CORS headers on GET request
- [ ] 5.10 Verify CORS headers are present in response
- [ ] 5.11 Refactor: Extract handlers to handlers module

## 6. Frontend - Vite/TypeScript/Lit Setup

- [ ] 6.1 Initialize Vite project with TypeScript in frontend/ directory
- [ ] 6.2 Add dependencies: lit, @open-wc/testing, vite
- [ ] 6.3 Configure TypeScript (tsconfig.json) with strict mode
- [ ] 6.4 Create index.html entry point
- [ ] 6.5 Create Dockerfile using NGINX to serve Vite bundle

## 7. Frontend - Query String Auth Component (TDD)

- [ ] 7.1 Write failing test for extracting email from query string
- [ ] 7.2 Implement query string parser to extract email parameter
- [ ] 7.3 Write failing test for missing email parameter
- [ ] 7.4 Implement error message when email is missing
- [ ] 7.5 Write failing test for invalid email format
- [ ] 7.6 Implement email format validation
- [ ] 7.7 Refactor: Extract to auth utility module

## 8. Frontend - User Profile Service (TDD)

- [ ] 8.1 Create UserProfile TypeScript interface matching backend schema
- [ ] 8.2 Write failing test for fetchUserProfile function
- [ ] 8.3 Implement fetchUserProfile service calling GET /user/:email
- [ ] 8.4 Write failing test for 404 error handling
- [ ] 8.5 Implement error handling for profile not found
- [ ] 8.6 Write failing test for network errors
- [ ] 8.7 Implement error handling for network failures
- [ ] 8.8 Add API base URL configuration (environment variable)
- [ ] 8.9 Refactor: Extract to services module

## 9. Frontend - Personalized Hello World Component (TDD)

- [ ] 9.1 Write failing test for HelloWorld Lit component rendering
- [ ] 9.2 Create HelloWorld Lit component with basic structure
- [ ] 9.3 Write failing test for loading state display
- [ ] 9.4 Implement loading indicator while fetching profile
- [ ] 9.5 Write failing test for personalized message with display name
- [ ] 9.6 Implement personalized message rendering with profile data
- [ ] 9.7 Write failing test for avatar display when avatarUrl present
- [ ] 9.8 Implement conditional avatar image rendering
- [ ] 9.9 Write failing test for no avatar when avatarUrl absent
- [ ] 9.10 Implement fallback when avatar is not available
- [ ] 9.11 Write failing test for error message display
- [ ] 9.12 Implement error message rendering on fetch failure
- [ ] 9.13 Refactor: Extract component logic and improve styling

## 10. Frontend - Main App Integration

- [ ] 10.1 Write failing test for app initialization on page load
- [ ] 10.2 Implement main app that extracts email and fetches profile
- [ ] 10.3 Wire HelloWorld component to main app
- [ ] 10.4 Add component registration and index.html integration

## 11. End-to-End Testing Setup

- [ ] 11.1 Initialize Playwright in e2e/ directory
- [ ] 11.2 Create docker-compose.yml for backend, frontend, and test database
- [ ] 11.3 Configure Playwright to use Docker Compose services
- [ ] 11.4 Add test helpers for seeding SQLite database

## 12. End-to-End Tests - User Journeys

- [ ] 12.1 Write E2E test: John Smith views personalized hello world
- [ ] 12.2 Write E2E test: Jane Doe views personalized hello world
- [ ] 12.3 Write E2E test: User with avatar sees avatar displayed
- [ ] 12.4 Write E2E test: User without avatar sees no avatar
- [ ] 12.5 Write E2E test: Invalid email shows error message
- [ ] 12.6 Write E2E test: Missing email parameter shows prompt
- [ ] 12.7 Verify all E2E tests pass with Docker Compose

## 13. CI/CD - DroneCI Configuration

- [ ] 13.1 Create .drone.yml pipeline configuration
- [ ] 13.2 Add pipeline step: Build backend (cargo build)
- [ ] 13.3 Add pipeline step: Run backend unit tests (cargo test)
- [ ] 13.4 Add pipeline step: Build frontend (npm run build)
- [ ] 13.5 Add pipeline step: Run frontend unit tests (npm test)
- [ ] 13.6 Add pipeline step: Build Docker images
- [ ] 13.7 Add pipeline step: Run E2E tests with Docker Compose
- [ ] 13.8 Configure pipeline triggers and notifications

## 14. Documentation

- [ ] 14.1 Document API endpoint in backend README
- [ ] 14.2 Document frontend component architecture in frontend README
- [ ] 14.3 Add development setup instructions (how to run locally)
- [ ] 14.4 Add testing instructions (unit tests and E2E tests)
- [ ] 14.5 Document Docker usage for local development
- [ ] 14.6 Add example URLs for testing (John and Jane)

## 15. Verification

- [ ] 15.1 Run all backend unit tests and verify 100% pass
- [ ] 15.2 Run all frontend unit tests and verify 100% pass
- [ ] 15.3 Run all E2E tests and verify complete user journeys work
- [ ] 15.4 Test CORS by serving frontend and backend on different ports
- [ ] 15.5 Verify Docker builds succeed for both backend and frontend
- [ ] 15.6 Manual test: Access app with John's email, see personalized message
- [ ] 15.7 Manual test: Access app with Jane's email, see personalized message
