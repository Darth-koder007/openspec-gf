## ADDED Requirements

### Requirement: Email-based query string authentication

The web UI SHALL extract the user's email from the URL query string to identify the user. This is a temporary authentication mechanism and not production-ready security.

#### Scenario: Email present in query string

- **WHEN** the URL contains an `email` query parameter (e.g., `?email=john@deliveryhero.com`)
- **THEN** the UI uses this email to fetch the user profile

#### Scenario: Email missing from query string

- **WHEN** the URL does not contain an `email` query parameter
- **THEN** the UI displays a message prompting the user to provide an email

#### Scenario: Invalid email format in query string

- **WHEN** the `email` query parameter contains an invalid email format
- **THEN** the UI displays a validation error message

### Requirement: No real authentication

The system SHALL NOT implement real authentication or authorization at this stage. The query string email parameter is explicitly a placeholder for future authentication.

#### Scenario: No password verification

- **WHEN** a user accesses the application with an email in the query string
- **THEN** no password or credentials are required or validated

#### Scenario: No session management

- **WHEN** a user accesses the application
- **THEN** no session cookies or tokens are created or managed

### Requirement: URL construction for testing

The system SHALL allow direct URL access with email parameter for testing purposes.

#### Scenario: Test user John Smith access

- **WHEN** accessing the URL with `?email=john@deliveryhero.com`
- **THEN** the UI displays John Smith's personalized content

#### Scenario: Test user Jane Doe access

- **WHEN** accessing the URL with `?email=jane@deliveryhero.com`
- **THEN** the UI displays Jane Doe's personalized content

### Requirement: Email parameter persistence

The web UI MAY preserve the email parameter when navigating within the application (future consideration).

#### Scenario: Email parameter in initial URL

- **WHEN** the user first loads the page with `?email=john@deliveryhero.com`
- **THEN** the email parameter is extracted and used for profile fetching
