FROM rust:1.62-slim-bullseye

RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app
COPY . ./