# Use a Rust base image for building
FROM rust:1.82 AS builder

# Set working directory
WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY init-cocktail-db init-cocktail-db/
COPY rest-api rest-api/

# Build the application
RUN cargo build --release --package init-cocktail-db

# Create a smaller runtime image
FROM debian:stable-slim

# Install runtime dependencies (e.g., libpq for PostgreSQL)
RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/init-cocktail-db /usr/local/bin/init-cocktail-db


# Set environment variables (override in docker-compose.yml if needed)
ENV DATABASE_URL=postgres://crosskdb:ming@cocktailbar:5432/cocktailbar

# Run the binary
CMD ["init-cocktail-db"]