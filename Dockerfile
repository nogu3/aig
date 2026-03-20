# Development stage (default) - includes toolchain for build/test/lint
FROM rust:latest AS dev

WORKDIR /usr/src/app

# Create an empty project and copy manifests to cache dependencies
RUN cargo init --vcs none .
COPY Cargo.toml Cargo.lock ./

# Fetch and build dependencies (debug + dev-dependencies for testing)
RUN mkdir -p src \
    && echo "fn main() {}" > src/main.rs \
    && echo "" > src/lib.rs \
    && cargo test --no-run 2>/dev/null || true \
    && rm -rf src

# Copy the actual source code and tests
COPY src ./src
COPY tests ./tests

RUN touch src/main.rs src/lib.rs \
    && cargo build

# Builder stage - optimized release build
FROM dev AS builder

RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim AS runtime

# Install CA certificates to enable HTTPS requests (reqwest needs this)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/aig /usr/local/bin/aig

# Set the entrypoint to the CLI tool
ENTRYPOINT ["aig"]
