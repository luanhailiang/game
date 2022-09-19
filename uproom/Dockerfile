FROM rust:1.62 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

FROM docker.clhero.fun/hero/debian:buster-slim

FROM debian:bookworm-slim
WORKDIR /

COPY --from=builder /usr/local/cargo/bin/app /app
COPY script /script
RUN chmod +x /app

ENTRYPOINT [ "/app" ]