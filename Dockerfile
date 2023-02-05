FROM rust:slim AS builder

WORKDIR /usr/src/inventory-service

RUN apt-get update && apt-get install -y libssl-dev pkg-config

COPY Cargo.toml .
COPY Cargo.lock .
COPY src src

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /usr/src/inventory-service

COPY --from=builder /usr/src/inventory-service/target/release/inventory-service .

EXPOSE 8200

CMD ["./inventory-service"]