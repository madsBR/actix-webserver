ARG branch=TEST

FROM rust:1.70-bookworm AS builder

COPY . .

WORKDIR /actix-engine/engine
ENV BRANCH=${branch}

RUN cargo build --release
FROM debian:bookworm-slim as runtime

COPY --from=builder /actix-engine/target/release /app
COPY actix-engine/engine/static /app/static
WORKDIR /app

RUN groupadd host && useradd -m -g host host && chmod -R 744 ./ && chown -R host:host ./

EXPOSE 8080

USER host

ENTRYPOINT ["/bin/bash","-c","./engine"]