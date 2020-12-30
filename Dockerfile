# Planner stage 
FROM rust:1.47 AS planner
WORKDIR app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Cacher stage
FROM rust:1.47 AS cacher
WORKDIR app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Builder stage 
FROM rust:1.47 AS builder
WORKDIR app
COPY --from=cacher /app/target target 
COPY --from=cacher /usr/local/cargo /usr/local/cargo
COPY . .
RUN cargo build --release --bin dserver

# Runtime stage
FROM debian:buster-slim AS runtime
WORKDIR app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/dserver dserver
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./dserver"]
