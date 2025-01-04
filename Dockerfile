# Usa una imagen base oficial de Rust
FROM rust:latest AS builder

# Establece el directorio de trabajo
WORKDIR /usr/src/app

# Copia los archivos de tu proyecto al contenedor
COPY . .

# Compila el proyecto
RUN cargo build --release --package web-server

# Usa una imagen base más reciente para el runtime
FROM ubuntu:latest

# Instala las dependencias necesarias
RUN apt-get update && apt-get install -y libssl-dev

# Copia el binario compilado desde la imagen de compilación
COPY --from=builder /usr/src/app/target/release/web-server /usr/local/bin/web-server

# Establece el directorio de trabajo
WORKDIR /usr/local/bin

# Establece las variables de entorno necesarias
ENV SERVICE_WEB_FOLDER="web-folder/"
ENV RUST_BACKTRACE=1

# Expone el puerto en el que tu aplicación escucha
EXPOSE 8081

# Comando para ejecutar tu aplicación
CMD ["web-server"]