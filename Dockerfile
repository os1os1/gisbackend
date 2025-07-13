FROM rustlang/rust:nightly AS builder
WORKDIR /app

COPY Cargo.toml .
RUN cargo fetch
COPY src/ ./src/
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /app
COPY --from=builder /app/target/release/gissample_backend /usr/local/bin/gissample_backend
CMD ["gissample_backend"]
