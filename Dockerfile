# stage 1: builder with musl target
FROM rust:1.93-alpine AS builder

RUN apk add --no-cache musl-dev

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/app

COPY ./static/favicon.ico ./static/favicon.ico

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --locked
RUN rm src/main.rs

COPY ./src ./src

RUN cargo build --release --locked --target x86_64-unknown-linux-musl

# stage 2: static distroless runner
FROM gcr.io/distroless/static-debian13 AS runner

WORKDIR /app

COPY --from=builder \
    /usr/src/app/target/x86_64-unknown-linux-musl/release/youtube-api-proxy-rust \
    /app/youtube-api-proxy-rust

USER nonroot:nonroot

EXPOSE 3000

CMD ["/app/youtube-api-proxy-rust"]
