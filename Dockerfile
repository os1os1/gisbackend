FROM rustlang/rust:nightly AS builder
WORKDIR /app

# 最初から src も一緒にコピー
COPY Cargo.toml .
COPY src/ ./src/

# これならターゲットが見えるのでOK
RUN cargo fetch
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && \
    apt-get install -y ca-certificates libssl-dev && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gissample_backend /usr/local/bin/gissample_backend
CMD ["gissample_backend"]
