## Context

This is a greenfield project establishing the foundation for a lightweight social application for team recognition. The current repository is empty, so we're building from scratch with a focus on modern, type-safe tooling and test-driven development practices.

**Goal**: Establish the technical foundation by setting up a monorepo architecture and proving that the Rust backend and Vite/Lit frontend can communicate securely.

## Goals / Non-Goals

**Goals:**
- Create a working monorepo with backend and frontend applications
- Implement a basic user profile API endpoint (`GET /user/:email`)
- Build a web UI that displays personalized hello world message using profile data
- Enable CORS for cross-domain communication between server and client
- Establish TDD practices for both frontend and backend development
- Set up end-to-end testing infrastructure covering user journeys
- Configure CI/CD pipeline with DroneCI

**Non-Goals:**
- Real authentication/authorization (using query-string based email for now)
- Production-grade user management
- Persistent database (using SQLite for testing)
- Advanced UI features beyond the hello world message

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
- **Build Tool**: Vite (fast dev server, optimized production builds)
- **Language**: TypeScript (type safety)
- **UI Framework**: Lit (lightweight web components, reactive)
- **Testing**: @open-wc/testing (web component testing utilities)
- **Deployment**: NGINX-based Docker image serving the Vite bundle

**Rationale**: Vite provides excellent DX with fast HMR. Lit is lightweight and standards-based (web components). TypeScript ensures type safety across the stack.

### Testing Strategy: TDD + E2E

**Test-Driven Development:**
- Write failing test → implement code → refactor
- Apply TDD to both frontend and backend
- Reference: https://github.com/obra/superpowers/tree/main/skills/test-driven-development

**End-to-End Testing:**
- **Framework**: Playwright
- **Infrastructure**: Docker Compose
- **Database**: SQLite (ephemeral test database)
- **Coverage**: One E2E test per functionality covering complete user journeys

**Test Data:**
- John Smith (john@deliveryhero.com)
- Jane Doe (jane@deliveryhero.com)

### CORS Configuration

Enable CORS on the backend to allow the frontend to be served from a different domain. Use tower-http CORS middleware with appropriate configuration for development and testing environments.

### CI/CD: DroneCI

Use DroneCI for continuous integration and deployment.

**Open Question**: How do we configure DroneCI in a consistent/standard way? Need to establish conventions for pipeline configuration, artifact handling, and deployment targets.

## Risks / Trade-offs

**[Risk] No real authentication** → Acceptable for initial foundation. Query-string email parameter is explicitly a temporary solution. Real auth will be added in future changes.

**[Risk] SQLite for testing only** → E2E tests use ephemeral SQLite databases. This is sufficient for testing but not production. Future changes will add production-grade database support.

**[Risk] Monorepo complexity** → Mitigated by keeping frontend and backend as independent projects with clear boundaries. Each can be built and tested independently.

**[Trade-off] Rust learning curve** → Team may need time to ramp up on Rust. However, the type safety and performance benefits outweigh this cost for a server application.

**[Trade-off] Web Components adoption** → Lit/web components are standards-based but less mainstream than React/Vue. The lightweight nature and lack of framework lock-in justify this choice for a small application.

## Migration Plan

Not applicable - this is a new application with no existing system to migrate from.

## Open Questions

1. **DroneCI Configuration**: What is the standard/consistent approach for DroneCI pipeline setup? Need to define conventions for build stages, test execution, and deployment.

2. **Production Database**: What database will be used in production? (Postgres, MySQL, etc.)

3. **Deployment Strategy**: Where will the application be deployed? (Kubernetes, cloud platform, on-prem?)

4. **Frontend-Backend Communication**: Should we add API versioning from the start (e.g., `/api/v1/user/:email`)?
