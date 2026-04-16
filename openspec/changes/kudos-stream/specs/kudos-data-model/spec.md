## Purpose

Define the structure and storage of kudos including sender, recipient, message, timestamp, and metadata. A kudo represents recognition given from one user to another.

## ADDED Requirements

### Requirement: Kudo data structure

The system SHALL define a kudo as containing sender information, recipient information, message content, timestamp, and optional visibility metadata.

#### Scenario: Complete kudo with all fields

- **WHEN** a kudo is created with all possible fields
- **THEN** the kudo contains sender email, recipient email, message text, creation timestamp, and visibility flag

#### Scenario: Kudo with minimal required fields

- **WHEN** a kudo is created with only required fields
- **THEN** the kudo contains sender email, recipient email, message text, and creation timestamp

### Requirement: Kudo fields specification

The kudo data structure SHALL contain the following fields:
- `senderEmail` (string, required): Email address of the user giving the kudo
- `recipientEmail` (string, required): Email address of the user receiving the kudo
- `message` (string, required): The recognition message text
- `createdAt` (timestamp, required): When the kudo was created
- `id` (string, required): Unique identifier for the kudo
- `isPublic` (boolean, optional): Whether the kudo is visible to others (default: true)

#### Scenario: Sender and recipient identification

- **WHEN** a kudo record is stored
- **THEN** it includes both senderEmail and recipientEmail as valid email addresses

#### Scenario: Message content storage

- **WHEN** a kudo is created with a message
- **THEN** the message text is stored without modification

#### Scenario: Timestamp tracking

- **WHEN** a kudo is created
- **THEN** the system records the creation timestamp in ISO 8601 format

#### Scenario: Unique kudo identification

- **WHEN** a kudo is created
- **THEN** the system assigns a unique ID that persists for the lifetime of the kudo

#### Scenario: Public visibility by default

- **WHEN** a kudo is created without specifying visibility
- **THEN** the isPublic field defaults to true

#### Scenario: Private kudo

- **WHEN** a kudo is created with isPublic set to false
- **THEN** the kudo is marked as private and only visible to sender and recipient

### Requirement: Kudo persistence

The system SHALL store kudos in a persistent database that supports querying by recipient email.

#### Scenario: Store kudo in database

- **WHEN** a kudo is created
- **THEN** the system persists it to the database with all field values

#### Scenario: Query kudos by recipient

- **WHEN** querying for kudos by recipient email
- **THEN** the system returns all kudos where recipientEmail matches the query

#### Scenario: Chronological ordering

- **WHEN** retrieving kudos for a recipient
- **THEN** the system orders them by createdAt timestamp in descending order (newest first)

### Requirement: Test data availability

The system SHALL include test kudos data for the existing test users (john@deliveryhero.com and jane@deliveryhero.com).

#### Scenario: John Smith has received kudos

- **WHEN** querying kudos for john@deliveryhero.com
- **THEN** the system returns at least one kudo with test data

#### Scenario: Jane Doe has received kudos

- **WHEN** querying kudos for jane@deliveryhero.com
- **THEN** the system returns at least one kudo with test data

#### Scenario: Test kudos include sender information

- **WHEN** retrieving test kudos
- **THEN** each kudo has a valid senderEmail from the test user set
