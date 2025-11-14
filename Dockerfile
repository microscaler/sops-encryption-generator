# Dockerfile for sops-encryption-generator GitHub Action
FROM rust:1.75-slim as builder

WORKDIR /build

# Install build dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy Cargo files
COPY Cargo.toml Cargo.lock* ./
COPY src ./src

# Build the binary
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install SOPS, GPG, and ca-certificates
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        gnupg2 \
        curl && \
    SOPS_VERSION="3.10.2" && \
    curl -LO "https://github.com/mozilla/sops/releases/download/v${SOPS_VERSION}/sops-v${SOPS_VERSION}.linux" && \
    chmod +x sops-v${SOPS_VERSION}.linux && \
    mv sops-v${SOPS_VERSION}.linux /usr/local/bin/sops && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /build/target/release/sops-encryption-generator /usr/local/bin/

# Set entrypoint
ENTRYPOINT ["/usr/local/bin/sops-encryption-generator"]

