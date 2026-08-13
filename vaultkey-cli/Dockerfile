# Build stage
FROM rust:1.75-slim as builder

WORKDIR /app

# Install OpenSSL dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy Cargo files
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
COPY --from=builder /app/target/release/vaultkey-cli /app/vaultkey-cli

# Create config directory
RUN mkdir -p /root/.config/vaultkey

# Run
ENTRYPOINT ["/app/vaultkey-cli"]