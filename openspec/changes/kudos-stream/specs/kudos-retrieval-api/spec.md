## Purpose

Server API endpoint to fetch and serve kudos data for a specific user, supporting the kudos stream UI.

## ADDED Requirements

### Requirement: Kudos retrieval endpoint

The system SHALL provide a REST API endpoint that returns all kudos received by a specific user.

#### Scenario: Endpoint path and method

- **WHEN** the API is configured
- **THEN** kudos are retrievable via GET request at `/kudos/:email`

#### Scenario: Successful kudos retrieval

- **WHEN** a valid email is provided in the URL path
- **THEN** the server returns HTTP 200 with JSON array of kudos for that recipient

#### Scenario: User with no kudos

- **WHEN** requesting kudos for a user who has received none
- **THEN** the server returns HTTP 200 with an empty JSON array

#### Scenario: Invalid email format

- **WHEN** an invalid email format is provided in the URL path
- **THEN** the server returns HTTP 400 with a validation error message

### Requirement: Kudos response structure

The kudos retrieval endpoint SHALL return an array of kudo objects with complete information.

#### Scenario: Kudo object fields

- **WHEN** returning kudos data
- **THEN** each kudo object contains id, senderEmail, recipientEmail, message, createdAt, and isPublic fields

#### Scenario: Multiple kudos response

- **WHEN** a user has received multiple kudos
- **THEN** the response contains all kudos in a JSON array

#### Scenario: Chronological ordering in response

- **WHEN** returning multiple kudos
- **THEN** the kudos are ordered by createdAt timestamp in descending order (newest first)

### Requirement: Cross-origin communication support

The kudos retrieval endpoint SHALL enable cross-origin communication to support both development (CORS) and production (nginx reverse proxy) scenarios.

#### Scenario: Development - CORS preflight request

- **WHEN** running in development mode and the browser sends an OPTIONS preflight request
- **THEN** the server responds with appropriate CORS headers allowing the request

#### Scenario: Development - Cross-origin GET request

- **WHEN** running in development mode and the frontend makes a GET request from a different origin
- **THEN** the server includes Access-Control-Allow-Origin header in the response

#### Scenario: Production - Single-origin via reverse proxy

- **WHEN** running in production with nginx reverse proxy
- **THEN** requests to `/api/kudos/*` are proxied to the backend at the same origin, eliminating CORS requirements

### Requirement: Privacy filtering

The kudos retrieval endpoint SHALL only return kudos where the requesting user is the recipient or where the kudo is marked as public.

#### Scenario: Recipient can see all their kudos

- **WHEN** requesting kudos for a user's own email
- **THEN** the response includes both public and private kudos received by that user

#### Scenario: Private kudos excluded from others' view

- **WHEN** a kudo is marked with isPublic: false
- **THEN** it only appears in responses for the recipient's email

### Requirement: Error handling

The kudos retrieval endpoint SHALL handle errors gracefully and return appropriate HTTP status codes.

#### Scenario: Database connection error

- **WHEN** the database is unavailable
- **THEN** the server returns HTTP 500 with an error message

#### Scenario: Database query error

- **WHEN** the database query fails
- **THEN** the server returns HTTP 500 with an error message

### Requirement: Response format

The kudos retrieval endpoint SHALL return JSON with consistent structure.

#### Scenario: JSON content type

- **WHEN** returning kudos data
- **THEN** the response Content-Type header is application/json

#### Scenario: Timestamp format

- **WHEN** returning kudos with timestamps
- **THEN** the createdAt field is formatted as ISO 8601 string

#### Scenario: Email field format

- **WHEN** returning kudos
- **THEN** senderEmail and recipientEmail are valid email address strings
