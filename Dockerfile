# ETAPA 1: Builder
FROM rust:1.84-slim-bookworm as builder

RUN apt-get update && apt-get install -y \
    pkg-config libssl-dev libpoppler-glib-dev libglib2.0-dev libc6-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./

# Cache de dependencias
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Compilar código real
COPY src ./src
COPY templates ./templates
RUN touch src/main.rs
RUN cargo build --release

# ETAPA 2: Runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libssl3 libpoppler-glib8 libglib2.0-0 ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
# NOTA: El nombre del binario cambia aquí
COPY --from=builder /usr/src/app/target/release/acompanante_espiritual .
COPY --from=builder /usr/src/app/templates ./templates
RUN mkdir -p /tmp

CMD ["./acompanante_espiritual"]