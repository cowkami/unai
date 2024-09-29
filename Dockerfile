FROM rust:1.80.1-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release 

FROM debian:stable-slim
COPY --from=builder /app/config/google_service_account_key.json /config/
COPY --from=builder /app/target/release/server .
CMD [ "/server" ]