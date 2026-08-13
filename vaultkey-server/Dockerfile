# Build stage
FROM rust:latest as builder

WORKDIR /app

# Install OpenSSL dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy Cargo files first for caching
COPY Cargo.toml ./
COPY src/ ./src/

# Build
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

# Copy binary
COPY --from=builder /app/target/release/vaultkey-server /app/vaultkey-server

# Create directories AND set permissions
RUN mkdir -p /app/data /app/storage && \
    chmod 777 /app/data /app/storage

# Create empty database file
RUN touch /app/data/vaultkey.db && \
    chmod 666 /app/data/vaultkey.db

# Expose port
EXPOSE 8000

# Run
CMD ["/app/vaultkey-server"]