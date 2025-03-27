# Build image
FROM rust:1.85-slim as builder

RUN apt-get update && apt-get install -y \
    libasound2-dev \
    libudev-dev \
    libwayland-dev \
    libxkbcommon-dev \
    pkg-config \
    gcc \
    g++ \
    make \
    cmake \
    build-essential \
    libssl-dev \
    openssl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/
COPY deployment/server/options ./options
COPY deployment/server/certs ./certs

ENV CARGO_TERM_COLOR=never
ENV CARGO_HTTP_MULTIPLEXING=false
ENV CARGO_NET_RETRY=5

RUN bash -c 'cargo build --locked --release 2>&1'

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libasound2 \
    libudev1 \
    libwayland-client0 \
    libxkbcommon0 \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/launcher /app/launcher
COPY --from=builder /app/crates/assets/assets/ /app/assets/
COPY --from=builder /app/options /app/options
COPY --from=builder /app/certs /app/certs

EXPOSE 12025

CMD ["/app/launcher", "server", "--server-options", "/app/options/server_options.ron", "--shared-options", "/app/options/shared_options.ron"]
