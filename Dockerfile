FROM rust:1.91-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release
FROM rust:1.91-slim
WORKDIR /app
COPY --from=builder /app/target/release/kodabi_lightrag_mcp .
CMD ["./kodabi_lightrag_mcp"]
