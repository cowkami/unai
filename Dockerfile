FROM rust:1.80.1-slim as builder
# for firestore
RUN apt-get update && apt-get install -y ca-certificates

WORKDIR /app
COPY . .
RUN cargo build --release 

FROM debian:stable-slim
# for firestore
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder /app/config/google_service_account_key.json /config/
COPY --from=builder /app/target/release/server .
CMD [ "/server" ]
