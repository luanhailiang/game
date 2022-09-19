FROM docker.clhero.fun/hero/debian:bookworm-slim

WORKDIR /

COPY target/release/app /app
COPY script /script
RUN chmod +x /app

ENTRYPOINT [ "/app" ]