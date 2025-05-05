# ---- build stage (Rust + Trunk) ----
# Use Rust 1.81 because newer trunk dependencies (litemap, zerofrom) require it
FROM rust:1.81 AS build
WORKDIR /app

# System libs needed for trunk → reqwest → openssl-sys
RUN apt-get update \
 && apt-get install -y --no-install-recommends pkg-config libssl-dev \
 && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown

# Latest trunk now compiles on Rust 1.81; no version pin required
RUN cargo install trunk wasm-bindgen-cli --locked

# now copy source for the actual build
COPY . .

RUN trunk build --release

# ---- runtime stage (hot-reload) ----
# Re-use everything from the build stage: Rust toolchain, Trunk, source tree
FROM build AS web

# Expose the HTTP port Trunk will serve on
EXPOSE 80

# Launch Trunk’s dev server (live-reload inside the container)
CMD ["trunk", "serve", "--address", "0.0.0.0", "--port", "80", "--open=false"]
