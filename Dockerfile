FROM rust:1.50 as builder

WORKDIR /usr/src/sqlactix

RUN cargo install --version=0.5.1 sqlx-cli --no-default-features --features postgres

COPY Cargo.toml Cargo.lock sqlx-data.json ./
COPY migrations ./migrations
COPY src ./src

RUN cargo build --release

FROM debian:buster-slim

WORKDIR /usr/src/sqlactix

COPY ./migrations ./migrations
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/bin/sqlx
COPY --from=builder /usr/src/sqlactix/target/release/sqlactix /usr/bin/sqlactix

ENTRYPOINT [ "sqlx", "migrate", "run" ]

CMD [ "sqlactix" ]