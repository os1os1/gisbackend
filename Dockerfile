FROM rustlang/rust:nightly-2024-07-12 AS builder
WORKDIR /app

# 先に Cargo.toml のみコピーして依存を解決
COPY Cargo.toml .
# Cargo.lock を生成させる
RUN cargo generate-lockfile

# ソースをコピー
COPY src/ ./src/

# ビルド
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /app
COPY --from=builder /app/target/release/gissample_backend /usr/local/bin/gissample_backend
CMD ["gissample_backend"]
