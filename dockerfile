FROM rust:1.93-alpine3.23 as builder
WORKDIR /app
RUN apk add --no-cache \
    musl-dev \
    protobuf-dev \
    pkgconfig \
    openssl-dev \
    make \
    gcc
COPY . .
RUN cargo build --release

FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/target/release/takeoff-mailer /usr/local/bin/
RUN chmod +x /usr/local/bin/takeoff-mailer
CMD ["takeoff-mailer"]