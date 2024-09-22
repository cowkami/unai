FROM rust:1.80.1-slim as builder
WORKDIR /app
COPY server server
COPY api-client api-client
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo build --release

FROM debian:stable-slim
COPY --from=builder /app/target/release/server .
CMD [ "/server" ]