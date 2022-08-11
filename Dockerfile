FROM rust AS builder

WORKDIR /app

# Cache build dependencies
COPY .gitignore Cargo.toml Cargo.lock ./
COPY src src
COPY entity entity

RUN --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/usr/local/rustup \
    set -eux; \
    rustup install stable; \
    cargo build --release; \
    objcopy --compress-debug-sections target/release/fikabot ./fikabot

# ===================================
FROM gcr.io/distroless/cc

ENV RUST_LOG "info"
ENV ENV "prod"

ENV WEBHOOK_TOKEN ""

ENV SLACK_TOKEN ""
ENV SLACK_SIGNING_SECRET ""

ENV PORT "8080"

ENV DB_USERNAME ""
ENV DB_PASSWORD ""
ENV DB_HOST ""
ENV DB_PORT ""
ENV DB_DATABASE ""

WORKDIR /app
COPY --from=builder /app/fikabot /app/fikabot

ENTRYPOINT [ "/app/fikabot" ]