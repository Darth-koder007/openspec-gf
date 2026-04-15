# Kudos App

A lightweight, positive social application where team members can recognize and celebrate each other's achievements.

## Project Structure

This is a monorepo containing:

- **backend/** - Rust/Axum server application
- **frontend/** - Vite/TypeScript/Lit web application
- **e2e/** - Playwright end-to-end tests

## Quick Start

### Prerequisites

- Rust 1.94+ (latest stable)
- Node.js v22.12.0 or later - Use `nvm use` in the project root to switch to the correct version
- Docker & Docker Compose (for containerized deployment)

### Development Setup

**1. Clone and navigate to the repository:**
```bash
cd kudos
nvm use  # Switch to Node 22.12.0
```

**2. Backend setup:**
```bash
cd backend
cargo build
cargo test  # Verify all tests pass
cargo run   # Start backend on http://localhost:3000
```

**3. Frontend setup (in a new terminal):**
```bash
cd frontend
npm install
npm test    # Verify all tests pass
npm run dev # Start frontend on http://localhost:5173
```

**4. Access the application:**
```
http://localhost:5173/?email=john@deliveryhero.com
```

### Testing

**Backend Unit Tests:**
```bash
cd backend
cargo test
```
- 9 tests covering user profiles, database, API endpoints, CORS
- All tests use in-memory SQLite databases
- Tests follow strict TDD principles

**Frontend Unit Tests:**
```bash
cd frontend
npm test
```
- 14 tests covering auth, services, components, app initialization
- Uses Vitest with jsdom environment
- Component tests use @open-wc/testing helpers

**End-to-End Tests:**
```bash
cd e2e
npm install
npm test
```
- 6 E2E scenarios using Playwright
- Tests complete user journeys with Docker Compose
- **Note:** Currently blocked by backend Docker deployment issue

### Docker Usage

**Development with Docker Compose:**
```bash
# Build images
docker-compose build

# Start services
docker-compose up

# Stop services
docker-compose down

# View logs
docker-compose logs backend
docker-compose logs frontend
```

**Individual Services:**

Backend:
```bash
cd backend
docker build -t kudos-backend .
docker run -p 3000:3000 -v kudos-data:/app/data kudos-backend
```

Frontend:
```bash
cd frontend
docker build -t kudos-frontend .
docker run -p 80:80 kudos-frontend
```

**Production Stack:**
- Backend: Rust binary in debian:12-slim with SQLite
- Frontend: nginx:alpine serving Vite build with reverse proxy
- Network: Bridge network for service communication
- Volumes: Persistent SQLite data storage

**Known Issue:** Backend exits immediately in Docker container despite passing all tests. This is a deployment issue being investigated. The backend works correctly when run locally with `cargo run`.

### Example URLs

**Local Development (Vite dev server):**
- John Smith: `http://localhost:5173/?email=john@deliveryhero.com`
- Jane Doe: `http://localhost:5173/?email=jane@deliveryhero.com`
- Invalid email: `http://localhost:5173/?email=invalid`
- Missing email: `http://localhost:5173/`

**Production (Docker Compose with nginx):**
- John Smith: `http://localhost/?email=john@deliveryhero.com`
- Jane Doe: `http://localhost/?email=jane@deliveryhero.com`

**Backend API (direct access in development):**
- Get user: `curl http://localhost:3000/user/john@deliveryhero.com`
- Health check: `curl http://localhost:3000/`

## Test Users

Two users are automatically seeded in the database on startup:

- **John Smith**
  - Email: `john@deliveryhero.com`
  - Display Name: John Smith
  - Full Name: John Michael Smith
  - Avatar: None

- **Jane Doe**
  - Email: `jane@deliveryhero.com`
  - Display Name: Jane Doe
  - Full Name: Jane Elizabeth Doe
  - Avatar: None

## Architecture

### Technology Stack

**Backend:**
- Language: Rust 1.94
- Web Framework: Axum (async, built on tower)
- Database: SQLite with rusqlite
- Async Runtime: Tokio
- Middleware: tower-http (CORS)
- Container: debian:12-slim with SQLite libraries

**Frontend:**
- Runtime: Node.js 22.12.0+
- Build Tool: Vite 8.x
- Language: TypeScript (strict mode)
- UI Framework: Lit 3.x (web components)
- Testing: Vitest + @open-wc/testing
- Container: nginx:alpine with custom configuration

**E2E Testing:**
- Framework: Playwright
- Infrastructure: Docker Compose
- Browser: Chromium

### System Architecture

```
┌─────────────┐
│   Browser   │
└──────┬──────┘
       │
       ▼
┌─────────────────┐
│  nginx:alpine   │  Frontend (port 80)
│  - Serves HTML  │
│  - Reverse proxy│
└────────┬────────┘
         │
         │ /api/* → http://backend:3000
         ▼
┌─────────────────┐
│  debian:slim    │  Backend (port 3000)
│  - Rust/Axum    │
│  - SQLite DB    │
└─────────────────┘
```

### Development vs Production

**Development:**
- Frontend: Vite dev server (port 5173)
- Backend: Direct access (port 3000)
- CORS: Enabled for cross-origin requests
- Hot Module Replacement: Enabled

**Production:**
- Frontend: nginx reverse proxy (port 80)
- Backend: Internal network only (port 3000)
- CORS: Not needed (same-origin via proxy)
- Optimizations: Gzip, caching, security headers

## Features

- ✅ RESTful API endpoint: `GET /user/:email`
- ✅ Personalized hello world message
- ✅ Query-string authentication (development)
- ✅ SQLite with persistent volumes
- ✅ Docker Compose orchestration
- ✅ nginx reverse proxy (production)
- ✅ Comprehensive unit and E2E tests
- ✅ Strict TDD development practices
- ⏳ CI/CD pipeline (deferred)
