## Context

The Kudos app currently allows team members to be identified via query-string authentication and view personalized hello messages. The system has:

- **Backend**: Rust/Axum server with SQLite database, running on port 3000
- **Frontend**: Vite/TypeScript/Lit web components, served via nginx in production
- **Communication**: RESTful JSON API over HTTP with CORS for development
- **Database**: SQLite with migration system (`schema_version` table tracking migrations)
- **Existing data model**: `users` table with display_name, full_name, email, avatar_url

We need to add the ability for users to view kudos they've received. This requires a new data model (kudos), new API endpoints, and new UI components. The implementation should follow existing architectural patterns and maintain test coverage standards (unit tests for both backend and frontend).

## Goals / Non-Goals

**Goals:**
- Add kudos data model with sender, recipient, message, timestamp, and visibility
- Create REST API endpoint to retrieve kudos for a user
- Build Lit web component to display kudos stream with loading states and empty state
- Display sender profiles (avatar, display name) by integrating with existing `/user/:email` endpoint
- Maintain existing test coverage standards (unit tests required)
- Follow existing patterns for CORS, error handling, and database migrations

**Non-Goals:**
- Creating kudos (out of scope - this feature only displays existing kudos)
- Real authentication (continues using query-string email parameter)
- Real-time updates (kudos loaded on page load only)
- Pagination or infinite scroll (display all kudos for now)
- Editing or deleting kudos
- Notification system

## Decisions

### Decision 1: Database Schema for Kudos

**Choice**: Add a new `kudos` table in SQLite with the following schema:
```sql
CREATE TABLE kudos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sender_email TEXT NOT NULL,
    recipient_email TEXT NOT NULL,
    message TEXT NOT NULL,
    created_at INTEGER NOT NULL,  -- Unix timestamp
    is_public INTEGER NOT NULL DEFAULT 1  -- SQLite boolean (1=true, 0=false)
)
```

**Rationale**:
- INTEGER PRIMARY KEY AUTOINCREMENT provides unique IDs
- Unix timestamp (INTEGER) for `created_at` matches SQLite best practices and is compact
- `is_public` as INTEGER follows SQLite boolean convention (no native BOOLEAN type)
- Email fields (not foreign keys) because users table is for display data only
- No complex relationships needed - denormalized for read performance

**Alternatives considered**:
- Using ISO 8601 strings for timestamps: More human-readable but slower to query and sort
- Foreign keys to users table: Adds complexity; kudos should work even if sender profile is missing
- Separate junction table for privacy: Over-engineering for a simple boolean flag

**Migration**: Add as migration version 2 in `db.rs`, following existing `run_migrations` pattern

### Decision 2: API Protocol and Endpoint Design

**Choice**: RESTful JSON API with new endpoint `GET /kudos/:email`

**Rationale**:
- Matches existing pattern (`GET /user/:email`)
- RESTful design is consistent with current architecture
- Email in path parameter follows established convention
- Returns JSON array of kudos objects ordered by `created_at DESC`

**API Response Format**:
```json
[
  {
    "id": 1,
    "senderEmail": "jane@deliveryhero.com",
    "recipientEmail": "john@deliveryhero.com",
    "message": "Great work on the presentation!",
    "createdAt": "2026-04-10T14:30:00Z",
    "isPublic": true
  }
]
```

**Alternatives considered**:
- GraphQL: Over-engineering; current REST pattern works well
- WebSockets: Out of scope (no real-time requirement)
- Nested endpoint `/user/:email/kudos`: Less RESTful; kudos are first-class resources

**CORS**: Use existing tower-http CORS layer (already configured for development)

### Decision 3: Frontend Component Architecture

**Choice**: Create two new Lit web components:
1. `kudos-stream` - Page component (handles routing, data fetching, state management)
2. `kudo-card` - Presentational component (displays single kudo with sender profile)

**Rationale**:
- Follows Lit/web components pattern established in `hello-world.ts`
- Separation of concerns: page component handles data, card component handles display
- `kudo-card` can be reused in future features (e.g., public kudos feed)
- Lit decorators (`@customElement`, `@state`, `@property`) for reactive state management

**Component State Management**:
- `kudos-stream`: Loading state, error state, kudos array, user email
- `kudo-card`: Receives kudo data and sender profile as properties

**Alternatives considered**:
- Single monolithic component: Less reusable, harder to test
- External state management (Redux, MobX): Over-engineering for this feature
- Vue/React: Inconsistent with existing Lit architecture

### Decision 4: Sender Profile Integration

**Choice**: Frontend fetches sender profiles using existing `/user/:email` endpoint

**Implementation**:
- `kudos-stream` component fetches all kudos first
- Extracts unique sender emails from kudos array
- Makes parallel requests to `/user/:email` for each unique sender
- Passes sender profile to each `kudo-card` component
- Falls back to displaying email if profile fetch fails

**Rationale**:
- Reuses existing API endpoint and service function (`fetchUserProfile`)
- Client-side join is acceptable for small datasets (test users only)
- Parallel requests are fast for small number of unique senders
- Graceful degradation if profile service is down

**Alternatives considered**:
- Backend joins: More complex queries; SQLite doesn't have proper foreign keys here
- Embedded sender profiles in kudos response: Denormalizes data, increases API response size
- Server-side aggregation: Over-engineering for current scale

### Decision 5: UI Design and Styling

**Choice**: Follow existing Lit CSS patterns from `hello-world.ts`:
- Use Lit's `static styles` with `css` tagged template
- System font stack: `system-ui, -apple-system, sans-serif`
- Consistent color palette:
  - Error states: `#d32f2f` with `#ffebee` background
  - Loading states: `#666` gray, italic text
  - Primary text: `#333`
- Border radius: `4px` for cards
- Spacing: `1rem` and `2rem` units

**Kudo Card Design**:
```
┌─────────────────────────────────────┐
│  [Avatar]  Jane Doe                 │
│            "Great work on..."       │
│            2 days ago               │
└─────────────────────────────────────┘
```

**Empty State Design**:
- Centered message: "You haven't received any kudos yet"
- Encouraging subtext: "When someone recognizes your work, it will appear here"
- Soft gray color, no error styling

**Rationale**:
- Maintains visual consistency with existing UI
- Avatar reuse from `hello-world` component
- Card-based layout is familiar and scannable
- Relative timestamps ("2 days ago") are more human-friendly than ISO dates

**Alternatives considered**:
- Tailwind CSS: Requires new dependency, inconsistent with current approach
- CSS modules: Over-engineering for current scale
- Material Design library: Unnecessary dependency

### Decision 6: Timestamp Formatting

**Choice**: Display relative timestamps in the UI ("2 days ago", "just now")

**Implementation**:
- Store as Unix timestamp (INTEGER) in database
- Backend returns ISO 8601 string in API response
- Frontend converts to relative time string using simple utility function
- Utility function handles: seconds, minutes, hours, days, weeks, months

**Rationale**:
- More human-friendly than absolute timestamps
- Lightweight - no need for date library (Moment.js, date-fns)
- ISO 8601 in API follows JSON best practices
- Unix timestamp in DB is efficient for queries and sorting

**Alternatives considered**:
- Absolute timestamps only: Less friendly ("2026-04-10T14:30:00Z")
- Client-side date library: Over-engineering, increases bundle size
- Server-side relative formatting: Not scalable (user timezone issues)

### Decision 7: Error Handling Strategy

**Choice**: Follow existing error handling patterns:

**Backend** (matching `handlers.rs` patterns):
- Email validation: Return `400 BAD_REQUEST` with JSON error
- User not found: Return `200 OK` with empty array (not 404)
- Database errors: Return `500 INTERNAL_SERVER_ERROR` with JSON error
- Use `IntoResponse` trait for consistent error responses

**Frontend** (matching `hello-world.ts` patterns):
- Loading state: Display "Loading..." in italic gray
- Error state: Display error message in red box with left border
- Empty state: Display encouraging message (not treated as error)
- Network errors: Caught and displayed as error state

**Rationale**:
- Consistency with existing error handling
- Empty kudos array is not an error (user hasn't received kudos yet)
- Client-side error display is user-friendly

### Decision 8: Testing Strategy

**Choice**: Maintain existing test coverage standards:

**Backend Tests** (following `main.rs::tests` pattern):
- Unit tests for kudos database functions (`db.rs`)
- Unit tests for kudos handler (`handlers.rs`)
- Integration tests for `/kudos/:email` endpoint using axum test helpers
- All tests use in-memory SQLite (`:memory:`)
- Test scenarios: success, empty array, invalid email, database errors

**Frontend Tests** (following existing patterns):
- Component tests for `kudos-stream` using `@open-wc/testing`
- Component tests for `kudo-card` using `@open-wc/testing`
- Service tests for kudos API client function
- Test scenarios: loading, error, empty state, populated stream

**E2E Tests**:
- Deferred until Docker deployment issue is resolved
- Future test: Navigate to kudos page, verify kudos display

**Rationale**:
- TDD approach matches existing development practices
- Comprehensive test coverage catches regressions
- In-memory SQLite makes backend tests fast and isolated

## Decisions: Common Mistakes and Patterns to Avoid

### Pattern: DO NOT use snake_case in JSON API responses

**Issue**: Backend is Rust (snake_case) but frontend is TypeScript (camelCase)

**Solution**: Use serde `rename` attribute to convert field names:
```rust
#[derive(Serialize)]
pub struct Kudo {
    pub id: i64,
    #[serde(rename = "senderEmail")]
    pub sender_email: String,
    #[serde(rename = "recipientEmail")]
    pub recipient_email: String,
    pub message: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
}
```

**Rationale**: Existing `UserProfile` struct already uses this pattern. Maintaining consistency is critical.

### Pattern: DO NOT return 404 for empty results

**Issue**: New developers might return 404 when a user has no kudos

**Solution**: Return `200 OK` with empty array `[]`

**Rationale**: 404 means "resource not found" (user doesn't exist), not "no data". Empty array is valid data.

### Pattern: DO handle missing sender profiles gracefully

**Issue**: If `/user/:email` fails for a sender, the kudo card might break

**Solution**:
- Catch fetch errors for sender profiles
- Display sender email as fallback if profile fetch fails
- Do not block rendering entire kudos stream on profile failures

**Rationale**: Kudos should still be visible even if one sender's profile is unavailable.

### Pattern: DO use INTEGER for timestamps in SQLite

**Issue**: SQLite has no native timestamp type; developers might use TEXT

**Solution**: Use INTEGER for Unix timestamps (seconds since epoch)

**Rationale**: More efficient for sorting and range queries. Convert to ISO 8601 in API layer.

### Pattern: DO validate email format before database queries

**Issue**: Invalid email formats can cause database errors or unexpected behavior

**Solution**: Use existing `is_valid_email` function in `handlers.rs` before any database operations

**Rationale**: Fail fast with clear error message. Consistent with existing `/user/:email` endpoint.

### Pattern: DO use migration version tracking

**Issue**: Running schema changes directly can cause inconsistencies

**Solution**: Add new migration in `run_migrations` function with version check:
```rust
if current_version < 2 {
    conn.execute("CREATE TABLE kudos (...)", [])?;
    set_schema_version(conn, 2)?;
}
```

**Rationale**: Existing migration system in `db.rs` ensures schema is applied exactly once.

### Pattern: DO lock database connection for entire query

**Issue**: Concurrent access to SQLite connection can cause locking issues

**Solution**: Use existing pattern:
```rust
let conn = db.lock().unwrap();
// Perform query with conn
// Lock released automatically when conn goes out of scope
```

**Rationale**: SQLite is single-writer. Mutex ensures thread safety. Matches existing handler pattern.

### Pattern: DO NOT use default Lit styles for everything

**Issue**: Developers might add global styles or use shadow DOM styles incorrectly

**Solution**: Use `static styles = css` in each component for scoped styles. Follow existing patterns in `hello-world.ts`.

**Rationale**: Shadow DOM provides style encapsulation. Consistent with Lit best practices.

## Risks / Trade-offs

### [Risk] Performance with large number of kudos → Mitigation

**Risk**: Fetching all kudos for a user with hundreds of kudos could be slow

**Mitigation**:
- Current scope: Test users only (small dataset)
- Future: Add pagination when scaling (LIMIT/OFFSET in SQL)
- Future: Add database index on `recipient_email` column

**Trade-off**: Simplicity now vs. scalability later. Acceptable for MVP.

### [Risk] Multiple API calls for sender profiles → Mitigation

**Risk**: N+1 query problem - one call per unique sender could be slow

**Mitigation**:
- Current scope: Small number of test users (2 senders max)
- Parallel fetch requests minimize latency
- Future: Add batch endpoint `/users?emails=a,b,c` if needed

**Trade-off**: Simple implementation now vs. optimization later. Acceptable for current scale.

### [Risk] No real-time updates → Mitigation

**Risk**: User must refresh page to see new kudos

**Mitigation**:
- Current scope: Explicitly a non-goal
- Future: Add WebSocket or polling if requirement emerges

**Trade-off**: Simplicity vs. real-time UX. Acceptable for initial release.

### [Risk] SQLite single-writer limitation → Mitigation

**Risk**: SQLite has limited concurrency for writes

**Mitigation**:
- Current scope: Only reads in this feature (no kudo creation yet)
- Read operations can run concurrently
- Future: If write contention becomes issue, migrate to Postgres

**Trade-off**: SQLite is perfect for read-heavy MVP. Acceptable for current load.

### [Risk] Privacy field not enforced → Mitigation

**Risk**: `isPublic` field exists but API returns all kudos to recipient

**Mitigation**:
- Current implementation: Recipient can always see their own kudos (public + private)
- Privacy enforcement will matter when adding "public kudos feed" feature
- Document in specs that privacy applies to non-recipients viewing kudos

**Trade-off**: Simple implementation now. Privacy enforcement deferred until public feed feature.

### [Risk] No error recovery for partial failures → Mitigation

**Risk**: If some sender profiles fail to load, UX could be confusing

**Mitigation**:
- Fall back to displaying sender email if profile fetch fails
- Log errors to console for debugging
- Do not block entire kudos stream rendering

**Trade-off**: Degraded UX for some cards vs. complete failure. Graceful degradation is better.

## Migration Plan

### Database Migration

1. Add migration version 2 in `backend/src/db.rs`
2. Create `kudos` table with schema defined above
3. Seed test kudos data in new `seed_test_kudos` function
4. Call seeding function in `main.rs` after `seed_test_users`

### Backend Deployment

1. Add new `Kudo` struct in `models.rs`
2. Add database functions in `db.rs`: `get_kudos_by_recipient`
3. Add handler in `handlers.rs`: `get_kudos`
4. Register route in `main.rs`: `.route("/kudos/:email", get(handlers::get_kudos))`
5. Write and run tests: `cargo test`
6. No environment variables needed (uses existing database)

### Frontend Deployment

1. Add new TypeScript interface `Kudo` in `types.ts`
2. Add API client function `fetchKudos` in `services.ts`
3. Create `kudo-card.ts` component
4. Create `kudos-stream.ts` component
5. Add utility function `formatRelativeTime` in new `utils.ts`
6. Write and run tests: `npm test`
7. Update routing in `main.ts` if needed (or use hash-based routing)

### Rollback Strategy

- Database: Migration is additive (new table only), no risk to existing data
- Backend: New endpoint does not affect existing `/user/:email` endpoint
- Frontend: New components are independent, can be removed without affecting `hello-world`
- If issues arise: Remove route registration, migration persists harmlessly

### Testing Before Release

1. Run backend tests: `cargo test` (should have 15+ tests)
2. Run frontend tests: `npm test` (should have 20+ tests)
3. Manual testing:
   - Access `http://localhost:5173/kudos?email=john@deliveryhero.com`
   - Verify kudos display with sender profiles
   - Verify empty state for user with no kudos
   - Verify error handling for invalid email

## Open Questions

**Resolved**:
- ✅ Communication protocol: REST over HTTP (matches existing)
- ✅ UI design guidelines: Follow existing Lit patterns in `hello-world.ts`
- ✅ Database choice: SQLite (existing)
- ✅ Common mistakes to avoid: Documented in design patterns section above

**Outstanding**: None
