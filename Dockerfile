# stage 1: builder with musl target
FROM rust:1.87-alpine AS builder

RUN apk add --no-cache musl-dev openssl-dev pkgconfig

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --locked
RUN rm src/main.rs

COPY ./src ./src

RUN cargo build --release --locked --target x86_64-unknown-linux-musl

# stage 2: static distroless runner
FROM gcr.io/distroless/static-debian12 AS runner

WORKDIR /app

COPY --from=builder \
    /usr/src/app/target/x86_64-unknown-linux-musl/release/youtube-api-proxy-rust \
    /app/youtube-api-proxy-rust

EXPOSE 3000

CMD ["/app/youtube-api-proxy-rust"]
