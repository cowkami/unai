FROM rust:1.80.1-slim as builder
RUN apt-get update; \
    apt-get install -y --no-install-recommends \
    libssl-dev pkg-config build-essential;
WORKDIR /app
COPY . .
RUN cargo build --release

CMD [ "/app/target/release/server" ]