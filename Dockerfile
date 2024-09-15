FROM rust:1.80.1-slim as builder
WORKDIR /app
COPY chatbot/src src
COPY chatbot/Cargo.toml .
COPY chatbot/Cargo.lock .
RUN cargo build --release

FROM debian:stable-slim
COPY --from=builder /app/target/release/chatbot .
CMD [ "/chatbot" ]