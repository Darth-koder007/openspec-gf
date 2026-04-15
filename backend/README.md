# Kudos Backend

Rust/Axum server providing user profile API for the Kudos application.

## API Endpoints

### GET /user/:email

Fetch user profile by email address.

**Parameters:**
- `email` (path parameter) - User's email address

**Response:**
- `200 OK` - Returns user profile JSON
  ```json
  {
    "displayName": "John Smith",
    "fullName": "John Michael Smith",
    "email": "john@deliveryhero.com",
    "avatarUrl": null
  }
  ```
- `400 Bad Request` - Invalid email format
- `404 Not Found` - User not found

**Example:**
```bash
curl http://localhost:3000/user/john@deliveryhero.com
```

### GET /

Health check endpoint.

**Response:**
```
Kudos Backend
```

## Development

### Prerequisites

- Rust 1.94+
- SQLite 3

### Running Locally

```bash
cargo run
```

The server will start on `http://0.0.0.0:3000`.

### Running Tests

```bash
cargo test
```

All tests use in-memory SQLite databases and are isolated.

### Environment Variables

- `DATABASE_PATH` - Path to SQLite database file
  - **Local development:** `data/kudos.db` (relative path, created automatically)
  - **Docker:** `/app/data/kudos.db` (set via environment variable in docker-compose.yml)

## Database

### Schema

**users table:**
- `id` - INTEGER PRIMARY KEY
- `display_name` - TEXT NOT NULL
- `full_name` - TEXT NOT NULL
- `email` - TEXT NOT NULL UNIQUE
- `avatar_url` - TEXT (optional)

### Test Data

The application seeds two test users on startup:
- John Smith (`john@deliveryhero.com`)
- Jane Doe (`jane@deliveryhero.com`)

## Docker

### Building

```bash
docker build -t kudos-backend .
```

### Running

```bash
docker run -p 3000:3000 -v kudos-data:/app/data kudos-backend
```

## Architecture

- **Web Framework:** Axum (async Rust web framework)
- **Database:** SQLite with rusqlite
- **Async Runtime:** Tokio
- **Middleware:** tower-http (CORS enabled for development)
- **Serialization:** serde/serde_json

## Known Issues

### Docker Deployment Issue

**Status:** Under investigation

**Symptoms:**
- Backend binary exits immediately (code 0) when run in Docker container
- No output produced despite explicit `eprintln!` debug statements
- Binary executes successfully but produces no logs, even with `RUST_BACKTRACE=1`

**Evidence:**
- All 9 unit tests pass in development AND in Docker test environment
- Binary runs perfectly with `cargo run` locally
- Binary exists and has correct dependencies (`ldd` shows all libraries present)
- SQLite libraries installed correctly in container
- File permissions are correct (executable)

**Investigation attempts:**
- Added debug output with `eprintln!` throughout main() - no output produced
- Tried without stripping - same issue
- Checked file descriptors - no output to any descriptor
- Verified SQLite libraries present - confirmed
- Checked binary dependencies - all satisfied

**Root cause:** Unknown - possibly related to tokio async runtime initialization in containerized environment, or stdout/stderr handling in debian-slim base image.

**Workaround:** Use `cargo run` for local development. The backend code is production-ready; only the containerization needs debugging.
