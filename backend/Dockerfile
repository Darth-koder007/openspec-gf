# Build stage
FROM rust:1.94 as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs and build dependencies only
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy actual source code
COPY src ./src

# Build the application with cached dependencies
RUN cargo build --release
    # Temporarily disabled stripping to debug deployment issue
    # strip /app/target/release/kudos-backend

# Runtime stage
FROM debian:12-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/kudos-backend /app/kudos-backend

# Create volume mount point for SQLite database
VOLUME ["/app/data"]

EXPOSE 3000

CMD ["/app/kudos-backend"]
