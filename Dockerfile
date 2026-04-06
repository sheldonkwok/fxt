FROM rust:slim AS builder

WORKDIR /opt/app

COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/

RUN cargo build --release

FROM debian:bookworm-slim

COPY --from=builder /opt/app/target/release/fxt /usr/local/bin/fxt

CMD ["fxt"]
