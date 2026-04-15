## ADDED Requirements

### Requirement: Display personalized hello world message

The web UI SHALL display a personalized hello world message using the user's profile information fetched from the server.

#### Scenario: Successful personalization with display name

- **WHEN** the user profile is successfully fetched from the server
- **THEN** the UI displays a message including the user's display name

#### Scenario: Personalization with avatar

- **WHEN** the user profile includes an avatar URL
- **THEN** the UI displays the avatar image alongside the personalized message

#### Scenario: Personalization without avatar

- **WHEN** the user profile does not include an avatar URL
- **THEN** the UI displays the personalized message without an avatar image

### Requirement: Fetch user profile on page load

The web UI SHALL automatically fetch the user profile from the server when the page loads.

#### Scenario: Initial page load with valid email

- **WHEN** the page loads with a valid email in the query string
- **THEN** the UI makes a GET request to `/user/:email` endpoint

#### Scenario: Profile fetch success

- **WHEN** the server returns a successful profile response
- **THEN** the UI renders the personalized content

#### Scenario: Profile fetch failure

- **WHEN** the server returns an error (404, 500, network error)
- **THEN** the UI displays an appropriate error message to the user

### Requirement: Server connection

The web UI SHALL connect to the backend server to fetch user profile data.

#### Scenario: Cross-origin request

- **WHEN** the frontend is served from a different domain than the backend
- **THEN** the UI successfully makes cross-origin requests using CORS

#### Scenario: API endpoint configuration

- **WHEN** the UI needs to fetch user data
- **THEN** it uses the configured server URL to construct the API endpoint

### Requirement: Loading state

The web UI SHALL display a loading indicator while fetching the user profile.

#### Scenario: Show loading indicator

- **WHEN** the profile fetch request is in progress
- **THEN** the UI displays a loading indicator

#### Scenario: Hide loading indicator on success

- **WHEN** the profile data is successfully received
- **THEN** the UI hides the loading indicator and shows the personalized content

#### Scenario: Hide loading indicator on error

- **WHEN** the profile fetch fails
- **THEN** the UI hides the loading indicator and shows an error message
