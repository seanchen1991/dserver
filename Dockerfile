FROM rust:1.47

WORKDIR app

COPY . .

RUN cargo build --release

ENTRYPOINT ["./target/release/dserver"]
