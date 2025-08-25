# Builder stage for creating the Dioxus bundle
FROM rust:1.89-slim-trixie AS builder

WORKDIR /app

# Install build dependencies including GUI libraries needed for Dioxus desktop
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    curl \
    libglib2.0-dev \
    libgtk-3-dev \
    libjavascriptcoregtk-4.1-dev \
    libsoup-3.0-dev \
    libwebkit2gtk-4.1-dev \
    libxdo-dev \
    && rm -rf /var/lib/apt/lists/*

# Install dioxus-cli
RUN cargo install dioxus-cli

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY api/ ./api/
COPY ui/ ./ui/
COPY desktop/ ./desktop/
COPY mobile/ ./mobile/
COPY server/ ./server/
COPY Dioxus.toml ./

# Build the server binary
RUN cargo build --release --bin server

# Production stage
FROM debian:trixie-slim AS production

# Install minimal runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false -m -d /app appuser

WORKDIR /app

# Copy the built server from builder stage
COPY --from=builder /app/target/release/server ./server
RUN chmod +x ./server && chown appuser:appuser ./server

# Switch to app user
USER appuser

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Run the Dioxus server
CMD ["./server"]