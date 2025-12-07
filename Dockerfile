# Builder stage: build release binary
FROM rust:1.91-slim AS builder

# Install build deps required by some crates (openssl, pkg-config)
RUN apt-get update && \
    apt-get install -y --no-install-recommends pkg-config libssl-dev ca-certificates build-essential && \
    rm -rf /var/lib/apt/lists/*

# Set runtime debug envs you want available
ENV RUST_BACKTRACE=1
ENV RUST_LOG=debug

WORKDIR /app

# Copy Cargo manifests first to leverage Docker layer cache for dependencies
COPY Cargo.toml Cargo.lock ./

# Create empty src to allow cargo to fetch deps (works even if workspace)
#RUN mkdir -p src && echo "fn main() {println!(\"dummy\");}" > src/main.rs && \
#    cargo fetch

# Now copy the full source and build a release binary
COPY . .
RUN cargo build --release

# Runtime stage: minimal image with necessary runtime libs only
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies: CA certs and libssl (ensure matches build-time lib)
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

# Create a non-root user to run the app
RUN useradd --create-home --shell /bin/false appuser

WORKDIR /app

# Copy the statically built binary from builder
COPY --from=builder /app/target/release/kodabi_lightrag_mcp /app/kodabi_lightrag_mcp

# Ensure binary is executable and owned by non-root user
RUN chown appuser:appuser /app/kodabi_lightrag_mcp && chmod +x /app/kodabi_lightrag_mcp

USER appuser

# Expose the port you expect (default in your app = 8080)
EXPOSE 9699

# Use a simple healthcheck (optional)
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s \
  CMD wget -q -T 2 -O - http://127.0.0.1:9699/health >/dev/null || exit 1

CMD ["/app/kodabi_lightrag_mcp"]