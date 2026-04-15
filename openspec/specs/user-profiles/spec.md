## Purpose

Server API endpoint to fetch and serve user profile information including display name, full legal name, email, and avatar URL.

## Requirements

### Requirement: Server exposes user profile endpoint

The system SHALL provide a REST API endpoint at `GET /user/:email` that returns user profile information for a given email address.

#### Scenario: Successful profile retrieval

- **WHEN** a valid email is provided in the URL path
- **THEN** the server returns HTTP 200 with JSON containing display name, full legal name, email, and optional avatar URL

#### Scenario: Profile not found

- **WHEN** an email that doesn't exist in the database is requested
- **THEN** the server returns HTTP 404 with an appropriate error message

#### Scenario: Invalid email format

- **WHEN** an invalid email format is provided in the URL path
- **THEN** the server returns HTTP 400 with a validation error message

### Requirement: User profile data structure

The user profile JSON response SHALL contain the following fields:
- `displayName` (string, required): The user's preferred display name
- `fullName` (string, required): The user's full legal name
- `email` (string, required): The user's email address
- `avatarUrl` (string, optional): URL to the user's avatar image

#### Scenario: Complete profile with avatar

- **WHEN** a user has all profile fields including avatar URL
- **THEN** the response includes all four fields with valid values

#### Scenario: Profile without avatar

- **WHEN** a user doesn't have an avatar URL set
- **THEN** the response includes displayName, fullName, and email, with avatarUrl as null or omitted

### Requirement: Cross-origin communication support

The server SHALL enable cross-origin communication to support both development (CORS) and production (nginx reverse proxy) scenarios.

#### Scenario: Development - CORS preflight request

- **WHEN** running in development mode and the browser sends an OPTIONS preflight request
- **THEN** the server responds with appropriate CORS headers allowing the request

#### Scenario: Development - Cross-origin GET request

- **WHEN** running in development mode and the frontend makes a GET request from a different origin
- **THEN** the server includes Access-Control-Allow-Origin header in the response

#### Scenario: Production - Single-origin via reverse proxy

- **WHEN** running in production with nginx reverse proxy
- **THEN** requests to `/api/*` are proxied to the backend at the same origin, eliminating CORS requirements

### Requirement: Test data availability

The system SHALL include two test users in the database:
- John Smith (john@deliveryhero.com)
- Jane Doe (jane@deliveryhero.com)

#### Scenario: Retrieve John Smith profile

- **WHEN** requesting `/user/john@deliveryhero.com`
- **THEN** the server returns John Smith's profile with display name "John Smith"

#### Scenario: Retrieve Jane Doe profile

- **WHEN** requesting `/user/jane@deliveryhero.com`
- **THEN** the server returns Jane Doe's profile with display name "Jane Doe"
