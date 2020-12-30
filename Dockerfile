FROM rust:1.47
WORKDIR app
COPY . .
RUN cargo build --release
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./target/release/dserver"]
