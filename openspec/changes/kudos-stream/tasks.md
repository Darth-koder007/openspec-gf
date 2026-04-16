## 1. Backend: Database Schema and Migration

- [x] 1.1 Add migration version 2 in `backend/src/db.rs` to create `kudos` table with schema (id, sender_email, recipient_email, message, created_at, is_public)
- [x] 1.2 Add `seed_test_kudos` function in `db.rs` to populate test kudos data for john@deliveryhero.com and jane@deliveryhero.com
- [x] 1.3 Call `seed_test_kudos` function in `backend/src/main.rs` after `seed_test_users`
- [x] 1.4 Write unit tests for migration (verify table creation and schema)
- [x] 1.5 Write unit tests for `seed_test_kudos` (verify test data exists)

## 2. Backend: Data Model and Database Functions

- [x] 2.1 Create `Kudo` struct in `backend/src/models.rs` with serde rename for camelCase JSON fields
- [x] 2.2 Add `get_kudos_by_recipient` function in `db.rs` to query kudos by recipient email, ordered by created_at DESC
- [x] 2.3 Write unit tests for `get_kudos_by_recipient` (success, empty array, invalid email)

## 3. Backend: API Handler

- [x] 3.1 Add `get_kudos` handler function in `backend/src/handlers.rs` that validates email and calls `get_kudos_by_recipient`
- [x] 3.2 Implement error handling in handler (400 for invalid email, 500 for database errors, 200 with empty array for no kudos)
- [x] 3.3 Convert Unix timestamp to ISO 8601 string in handler response
- [x] 3.4 Register route `GET /kudos/:email` in `backend/src/main.rs` using the `get_kudos` handler
- [x] 3.5 Write integration tests for `/kudos/:email` endpoint (success, empty array, invalid email, not found)
- [x] 3.6 Run `cargo test` to verify all backend tests pass

## 4. Frontend: Type Definitions and Services

- [x] 4.1 Add `Kudo` interface in `frontend/src/types.ts` with fields matching backend JSON response (id, senderEmail, recipientEmail, message, createdAt, isPublic)
- [x] 4.2 Add `fetchKudos` function in `frontend/src/services.ts` to call `/kudos/:email` endpoint
- [x] 4.3 Write unit tests for `fetchKudos` service function (success, error handling)

## 5. Frontend: Utility Functions

- [x] 5.1 Create `frontend/src/utils.ts` file with `formatRelativeTime` function to convert ISO timestamps to relative time strings ("2 days ago", "just now")
- [x] 5.2 Write unit tests for `formatRelativeTime` (seconds, minutes, hours, days, weeks, months)

## 6. Frontend: Kudo Card Component

- [x] 6.1 Create `frontend/src/kudo-card.ts` Lit component with properties for kudo data and sender profile
- [x] 6.2 Add component styles following existing patterns (system font, card layout, border radius, spacing)
- [x] 6.3 Implement render method showing sender avatar (or placeholder), sender display name, message, and relative timestamp
- [x] 6.4 Handle missing sender profile gracefully (display sender email as fallback)
- [x] 6.5 Write component tests for `kudo-card` (with avatar, without avatar, missing profile)

## 7. Frontend: Kudos Stream Page Component

- [x] 7.1 Create `frontend/src/kudos-stream.ts` Lit component with state for loading, error, kudos array, and user email
- [x] 7.2 Add component styles following existing patterns (loading state, error state, card list layout)
- [x] 7.3 Implement `connectedCallback` to extract email from query string and fetch kudos on page load
- [x] 7.4 Implement loading state rendering ("Loading..." in italic gray)
- [x] 7.5 Implement error state rendering (red box with left border, error message)
- [x] 7.6 Implement empty state rendering when kudos array is empty (encouraging message, no error styling)
- [x] 7.7 Implement kudos stream rendering (map kudos array to kudo-card components)
- [x] 7.8 Add logic to fetch sender profiles for unique sender emails (parallel requests to `/user/:email`)
- [x] 7.9 Pass sender profile data to each kudo-card component
- [x] 7.10 Write component tests for `kudos-stream` (loading, error, empty state, populated stream, profile fetch failure)

## 8. Frontend: Integration and Routing

- [x] 8.1 Import `kudos-stream` component in `frontend/src/main.ts` to register the custom element
- [x] 8.2 Update routing or add navigation to kudos stream page (if needed)
- [x] 8.3 Run `npm test` to verify all frontend tests pass

## 9. Manual Testing

- [ ] 9.1 Start backend with `cargo run` and verify database migration runs successfully
- [ ] 9.2 Test backend API endpoint directly: `curl http://localhost:3000/kudos/john@deliveryhero.com`
- [ ] 9.3 Start frontend with `npm run dev` and navigate to kudos stream page with `?email=john@deliveryhero.com`
- [ ] 9.4 Verify kudos display correctly with sender avatars, names, messages, and timestamps
- [ ] 9.5 Test empty state by checking a user with no kudos (if test data allows)
- [ ] 9.6 Test error state by providing invalid email format
- [ ] 9.7 Test loading state (verify it shows briefly during fetch)

## 10. Documentation

- [ ] 10.1 Update backend README if needed to document new `/kudos/:email` endpoint
- [ ] 10.2 Update main README with example URL for kudos stream page
