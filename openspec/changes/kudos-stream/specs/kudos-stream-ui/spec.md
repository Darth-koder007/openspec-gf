## Purpose

Web UI for displaying a stream/feed of kudos received by the current user, including individual kudo card design, empty state handling, and page layout.

## ADDED Requirements

### Requirement: Kudos stream page

The web UI SHALL provide a dedicated page that displays all kudos received by the authenticated user.

#### Scenario: Access kudos stream page

- **WHEN** a user navigates to the kudos stream page with a valid email in the query string
- **THEN** the UI displays a list of kudos received by that user

#### Scenario: Kudos stream page route

- **WHEN** the application routing is configured
- **THEN** the kudos stream is accessible at a dedicated route (e.g., /kudos)

### Requirement: Fetch kudos on page load

The kudos stream page SHALL automatically fetch kudos data for the authenticated user when the page loads.

#### Scenario: Initial page load with valid user

- **WHEN** the kudos stream page loads with a valid email in the query string
- **THEN** the UI makes a request to fetch kudos for that email

#### Scenario: Kudos fetch success

- **WHEN** the server returns kudos data successfully
- **THEN** the UI displays the kudos in the stream

#### Scenario: Kudos fetch failure

- **WHEN** the server returns an error while fetching kudos
- **THEN** the UI displays an error message to the user

### Requirement: Loading state

The kudos stream page SHALL display a loading indicator while fetching kudos data.

#### Scenario: Show loading indicator

- **WHEN** the kudos fetch request is in progress
- **THEN** the UI displays a loading indicator

#### Scenario: Hide loading on success

- **WHEN** kudos data is successfully loaded
- **THEN** the UI hides the loading indicator and shows the kudos stream

#### Scenario: Hide loading on error

- **WHEN** the kudos fetch fails
- **THEN** the UI hides the loading indicator and shows an error message

### Requirement: Individual kudo card display

The kudos stream SHALL display each kudo as a card containing sender information, message content, and timestamp.

#### Scenario: Kudo card with sender profile

- **WHEN** displaying a kudo
- **THEN** the card shows the sender's display name fetched from their user profile

#### Scenario: Kudo card with sender avatar

- **WHEN** displaying a kudo and the sender has an avatar URL
- **THEN** the card shows the sender's avatar image

#### Scenario: Kudo card without sender avatar

- **WHEN** displaying a kudo and the sender has no avatar URL
- **THEN** the card shows a placeholder or initials instead of an avatar

#### Scenario: Kudo card with message

- **WHEN** displaying a kudo
- **THEN** the card prominently displays the message text

#### Scenario: Kudo card with timestamp

- **WHEN** displaying a kudo
- **THEN** the card shows when the kudo was created in a human-readable format (e.g., "2 days ago")

### Requirement: Kudos stream layout

The kudos stream SHALL display kudos in reverse chronological order (newest first) as a vertical list.

#### Scenario: Multiple kudos ordering

- **WHEN** displaying multiple kudos
- **THEN** the most recent kudo appears at the top of the list

#### Scenario: Kudos list scrolling

- **WHEN** there are more kudos than fit on the screen
- **THEN** the user can scroll to view all kudos

### Requirement: Empty state handling

The kudos stream SHALL display a friendly empty state message when the user has received no kudos.

#### Scenario: No kudos received

- **WHEN** the kudos fetch returns an empty array
- **THEN** the UI displays an empty state message indicating no kudos have been received yet

#### Scenario: Empty state design

- **WHEN** displaying the empty state
- **THEN** the UI shows encouraging text and possibly an illustration or icon

### Requirement: Sender profile integration

The kudos stream SHALL fetch and display sender profile information for each kudo using the existing user-profiles capability.

#### Scenario: Fetch sender profile for each kudo

- **WHEN** displaying kudos
- **THEN** the UI fetches the user profile for each unique sender email

#### Scenario: Display sender display name

- **WHEN** a sender profile is successfully fetched
- **THEN** the kudo card shows the sender's displayName from their profile

#### Scenario: Sender profile fetch failure

- **WHEN** a sender profile cannot be fetched
- **THEN** the kudo card displays the sender's email address as a fallback

### Requirement: Page title and header

The kudos stream page SHALL display a clear title and header indicating the purpose of the page.

#### Scenario: Page title

- **WHEN** the kudos stream page loads
- **THEN** the page displays a title such as "Your Kudos" or "Kudos You've Received"

#### Scenario: User context in header

- **WHEN** the kudos stream page loads for a specific user
- **THEN** the header may include the user's name or avatar for context
