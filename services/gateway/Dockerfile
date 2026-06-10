# Dockerfile - Multi-stage build for minimal image size
# Stage 1: Build the application
FROM rust:1.86 AS builder

WORKDIR /app

# Copy and build dependencies
COPY Cargo.toml Cargo.lock ./
COPY services/gateway/Cargo.toml services/gateway/
RUN mkdir services/gateway/src && echo "fn main() {}" >> services/gateway/src/main.rs
RUN cargo build --release

# Copy code and build release binary with optimizations
COPY services/gateway ./services/gateway
RUN touch services/gateway/src/*
RUN cargo build --release --package gateway-service

# Stage 2a: Test the application
FROM builder AS test
ENTRYPOINT ["cargo", "test", "--release"]

# Stage 2b: Create minimal runtime image
FROM debian:bookworm-slim AS deploy

# Install only required runtime dependencies
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN useradd -r -s /bin/false appuser
USER appuser

# Copy only the compiled binary
COPY --from=builder /app/target/release/gateway-service /usr/local/bin/

# Expose service port
EXPOSE 3000

# Set entrypoint
ENTRYPOINT ["gateway-service"]
