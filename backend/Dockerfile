FROM rust:1.85-alpine AS builder
WORKDIR /app
RUN apk add --no-cache musl-dev openssl-dev pkgconf
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM alpine:3.21
RUN apk add --no-cache ca-certificates libssl3
COPY --from=builder /app/target/release/backend /usr/local/bin/tartarus
EXPOSE 3001
CMD ["tartarus"]
