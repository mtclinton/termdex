FROM rust:1.67.1

WORKDIR /pokemon

RUN cargo install diesel_cli --no-default-features --features postgres
