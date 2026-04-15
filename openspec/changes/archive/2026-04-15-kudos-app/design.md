## Context

This is a greenfield project establishing the foundation for a lightweight social application for team recognition. The current repository is empty, so we're building from scratch with a focus on modern, type-safe tooling and test-driven development practices.

**Goal**: Establish the technical foundation by setting up a monorepo architecture and proving that the Rust backend and Vite/Lit frontend can communicate securely.

## Goals / Non-Goals

**Goals:**
- Create a working monorepo with backend and frontend applications
- Implement a basic user profile API endpoint (`GET /user/:email`)
- Build a web UI that displays personalized hello world message using profile data
- Set up production-ready Docker containerization with multi-stage builds
- Configure nginx reverse proxy for single-origin architecture
- Establish strict TDD practices for both frontend and backend development
- Set up end-to-end testing infrastructure with Docker Compose
- Implement SQLite with persistent volumes for production data storage

**Non-Goals:**
- Real authentication/authorization (using query-string based email for now)
- Production-grade user management or admin interfaces
- Multi-server deployment or horizontal scaling (single-server with Docker Compose)
- Advanced UI features beyond the personalized hello world message
- Observability stack (structured logging, metrics, tracing can be added incrementally)
- CI/CD pipeline setup (can be added later when deployment infrastructure is established)

## Decisions

### Architecture: Monorepo

Use a monorepo structure to keep frontend and backend code in a single repository. This simplifies dependency management, versioning, and enables atomic commits across both applications.

**Structure:**
```
/backend    - Rust/Axum server
/frontend   - Vite/Lit web application
/e2e        - Playwright end-to-end tests
```

### Backend: Rust + Axum

**Technology Stack:**
- **Language**: Rust (type safety, performance, memory safety)
- **Web Framework**: Axum (ergonomic, built on tower ecosystem)
- **Middleware**: tower-http (CORS, logging, tracing)
- **Build Tool**: Cargo
- **Containerization**: Docker

**Rationale**: Rust provides strong type safety and performance. Axum is modern, well-maintained, and integrates seamlessly with the tower ecosystem for middleware.

### Frontend: Vite + TypeScript + Lit

**Technology Stack:**
- **Runtime**: Node.js >=22.12.0 (required for Vite 8+ and rolldown bundler)
- **Build Tool**: Vite (fast dev server, optimized production builds)
- **Language**: TypeScript (type safety)
- **UI Framework**: Lit (lightweight web components, reactive)
- **Testing**: @open-wc/testing (web component testing utilities), Vitest
- **Deployment**: NGINX-based Docker image serving the Vite bundle

**Rationale**: Vite provides excellent DX with fast HMR. Lit is lightweight and standards-based (web components). TypeScript ensures type safety across the stack. Node.js 22.12.0+ is required for the rolldown bundler used by Vite 8+.

### Testing Strategy: Strict TDD + E2E

**Test-Driven Development (Strict Red-Green-Refactor):**
- **RED**: Write failing test → Run test → Verify it fails with expected error
- **GREEN**: Implement minimal code → Run test → Verify it passes
- **REFACTOR**: Improve code structure → Run test → Verify it still passes
- Apply strict TDD to both frontend and backend
- **NEVER skip the verification steps** - always run tests to confirm red/green states
- Each TDD cycle must include test execution and verification
- Reference: https://github.com/obra/superpowers/tree/main/skills/test-driven-development

**End-to-End Testing:**
- **Framework**: Playwright
- **Infrastructure**: Docker Compose
- **Database**: SQLite (ephemeral test database)
- **Coverage**: One E2E test per functionality covering complete user journeys

**Test Data:**
- John Smith (john@deliveryhero.com)
- Jane Doe (jane@deliveryhero.com)

### Database: SQLite with Persistent Storage

**Decision**: Use SQLite for both testing and production.

**Rationale**: This is a lightweight kudos application with modest traffic expectations. SQLite provides:
- Zero configuration and maintenance overhead
- Perfect performance for single-server deployments
- File-based storage with Docker volume persistence
- Easy backup (copy the DB file)
- Migration path to Postgres if scaling requirements change

**Production Setup**:
- SQLite database file stored in Docker volume (`/app/data/kudos.db`)
- Automatic initialization and seeding on first run
- Volume ensures data persistence across container restarts

**Future Considerations**: Can migrate to Postgres/MySQL if multi-server deployment or higher concurrency becomes necessary.

### Containerization: Production-Ready Docker Strategy

**Architecture**: Multi-container setup with separate services for frontend and backend.

**Backend Container** (Rust/Axum):
```dockerfile
# Multi-stage build with dependency caching
1. Builder stage:
   - Layer caching for Cargo dependencies (rebuild only when Cargo.toml changes)
   - Release build with optimizations
   - Strip debug symbols

2. Runtime stage:
   - distroless base image (minimal attack surface, ~2.5MB)
   - Only the compiled binary + runtime dependencies
   - Health check endpoint
```

**Frontend Container** (Vite/Lit + NGINX):
```dockerfile
1. Builder stage:
   - Node.js for npm install and Vite build
   - Production optimized bundle

2. Runtime stage:
   - nginx:alpine (~40MB)
   - Custom nginx.conf for:
     * SPA routing (fallback to index.html)
     * Reverse proxy /api/* to backend
     * Gzip compression
     * Security headers
```

**Benefits of nginx reverse proxy**:
- Single origin (eliminates CORS complexity in production)
- Standard industry pattern
- NGINX handles static files efficiently
- Can add caching, rate limiting later

**Docker Compose Orchestration**:
```yaml
services:
  backend:
    - Exposes port 3000 (internal only)
    - Volume mount for SQLite persistence
    - Health checks for graceful startup

  frontend:
    - Exposes port 80 (public)
    - Depends on backend health
    - Proxies /api/* to http://backend:3000

volumes:
  db-data: SQLite persistence
```

**Development vs Production**:
- Development: CORS enabled, services run independently
- Production: nginx reverse proxy, single-origin architecture

### CI/CD: Deferred

**Decision**: CI/CD pipeline setup is deferred until deployment infrastructure is established.

**Rationale**: The application can be tested and run locally using Docker Compose. Setting up a CI/CD pipeline (DroneCI, GitHub Actions, etc.) requires decisions about deployment targets, container registries, and infrastructure that are not yet determined. This can be added as a future enhancement without impacting the core functionality.

**Future Considerations**:
- When CI/CD is added, suggested pipeline stages:
  1. Build backend and frontend Docker images (with layer caching)
  2. Run backend unit tests (cargo test)
  3. Run frontend unit tests (npm test)
  4. Spin up Docker Compose environment
  5. Run E2E tests with Playwright
  6. Push images to registry (on main branch)
  7. Deploy to target environment

## Risks / Trade-offs

**[Risk] No real authentication** → Acceptable for initial foundation. Query-string email parameter is explicitly a temporary solution. Real auth will be added in future changes.

**[Trade-off] SQLite for production** → Sufficient for single-server, low-to-medium traffic. Trade simplicity and zero-maintenance for horizontal scaling limitations. Can migrate to Postgres if traffic demands multi-server deployment. For this lightweight kudos app, SQLite is the right choice.

**[Risk] Monorepo complexity** → Mitigated by keeping frontend and backend as independent projects with clear boundaries. Each can be built and tested independently.

**[Trade-off] Rust learning curve** → Team may need time to ramp up on Rust. However, the type safety and performance benefits outweigh this cost for a server application.

**[Trade-off] Web Components adoption** → Lit/web components are standards-based but less mainstream than React/Vue. The lightweight nature and lack of framework lock-in justify this choice for a small application.

**[Trade-off] distroless runtime** → Backend uses distroless for security and minimal size. Trade-off: no shell for debugging (must debug via logs or attach debugger). Can use alpine variant if shell access becomes necessary.

## Migration Plan

Not applicable - this is a new application with no existing system to migrate from.

## Open Questions

1. **Deployment Target** (Deferred): Where will the production application be deployed? Options: Single server with Docker Compose, Kubernetes cluster, cloud platform (AWS ECS, Cloud Run, Azure Container Apps), on-premises. Decision can be made when deployment is needed.

2. **Container Registry** (Deferred): What registry should be used for Docker images? Options: Docker Hub, private registry, cloud provider registry (ECR, GCR, ACR). Decision deferred until CI/CD setup.

3. **API Versioning**: Should we add versioning from the start (e.g., `/api/v1/user/:email`)? For a greenfield app with no existing clients, we can start without it and add versioning when needed.

4. **Observability**: Should we add structured logging, metrics, and tracing from the start? Or add incrementally as needed? Current approach: add incrementally.
