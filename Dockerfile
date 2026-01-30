FROM rust:1.75 AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN groupadd -r appgroup && useradd -r -g appgroup appuser
WORKDIR /app
COPY --from=builder /app/target/release/inventory-service .
USER appuser
EXPOSE 8083
CMD ["./inventory-service"]
