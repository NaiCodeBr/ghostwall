FROM rust:1.75 as builder

WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./
COPY core ./core
COPY scanner ./scanner
COPY risk-engine ./risk-engine
COPY storage ./storage
COPY reporting ./reporting
COPY threat-intel ./threat-intel
COPY api ./api

# Build the API
RUN cargo build --release --bin ghostwall-api

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/ghostwall-api /app/ghostwall-api

EXPOSE 8080

CMD ["./ghostwall-api"]
