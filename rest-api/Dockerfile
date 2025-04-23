# Use a Rust base image for building
FROM rust:1.82 AS builder

# Set working directory
WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY rest-api rest-api/
COPY init-cocktail-db init-cocktail-db

# Build the specific crate
RUN cargo build --release --package rest-api

# Create a smaller runtime image
FROM debian:bookworm-slim

# Install runtime dependencies (e.g., libpq for PostgreSQL)
RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/rest-api /usr/local/bin/rest-api

# Set environment variables
ENV DATABASE_URL=postgres://crosskdb:ming@cocktailbar:5432/cocktailbar

# Expose the API port (adjust based on your API)
EXPOSE 8080

# Run the binary
CMD ["rest-api"]