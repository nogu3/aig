# Builder stage
FROM rust:latest AS builder

WORKDIR /usr/src/app

# Create an empty project and copy manifests to cache dependencies
RUN cargo init --vcs none .
COPY Cargo.toml Cargo.lock ./

# Fetch and build dependencies
# We touch src/main.rs and src/lib.rs to ensure Cargo sees them
RUN mkdir -p src \
    && echo "fn main() {}" > src/main.rs \
    && echo "" > src/lib.rs \
    && cargo build --release \
    && rm -rf src

# Copy the actual source code
COPY src ./src
# Build the actual application
# We need to touch the main and lib files so cargo knows they changed
RUN touch src/main.rs src/lib.rs \
    && cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install CA certificates to enable HTTPS requests (reqwest needs this)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/aig /usr/local/bin/aig

# Set the entrypoint to the CLI tool
ENTRYPOINT ["aig"]
