FROM rust:1.75-alpine as builder
WORKDIR /app
RUN apk add --no-cache musl-dev
COPY . .
RUN cargo build --release

FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/target/release/takeoff-mailer /usr/local/bin/
CMD ["takeoff-mailer"]