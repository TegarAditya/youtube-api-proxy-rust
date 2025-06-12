# stage 1
FROM rust:1.87-slim AS builder

RUN apt update && apt install -y libssl-dev pkg-config && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --locked
RUN rm src/main.rs

COPY ./src ./src
RUN cargo build --release --locked

# stage 2
FROM gcr.io/distroless/cc-debian12 AS runner

WORKDIR /app

COPY --from=builder /usr/src/app/target/release/youtube-api-proxy-rust /app/youtube-api-proxy-rust

EXPOSE 3000

CMD ["/app/youtube-api-proxy-rust"]
