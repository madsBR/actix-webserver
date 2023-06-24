FROM rust:1.70-buster

COPY . .

WORKDIR /actix-engine/engine

RUN cargo build --release

EXPOSE 8080

ENTRYPOINT [ "cargo","run","--release" ]