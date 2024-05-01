# Usar una imagen base de Rust
FROM rust:latest as builder

# Crear un nuevo directorio
WORKDIR /usr/src/myapp

# Copiar el archivo Cargo.toml y el código fuente
COPY . .

# Compilar la aplicación para producción
RUN cargo install --path ./crates/services/web-server

# Iniciar un nuevo stage
FROM debian:buster-slim

# Copiar el ejecutable
COPY --from=builder /usr/local/cargo/bin/web-server /usr/local/bin/web-server

# Ejecutar la aplicación
CMD ["web-server"]
