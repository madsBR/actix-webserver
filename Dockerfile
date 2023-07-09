ARG branch=TEST

FROM rust:1.70-bookworm AS builder

COPY . .

WORKDIR /actix-engine/engine
ENV BRANCH=${branch}

RUN cargo build --release
FROM debian:bookworm-slim as runtime

WORKDIR /app

EXPOSE 8080
RUN useradd -ms /bin/bash host && mkdir -p /app && chown -R host:host /app && chmod -R 744 /app
USER host
COPY --from=builder --chown=host:host /actix-engine/target/release /app
COPY --chown=host:host /actix-engine/engine/static /app/static

ENTRYPOINT ["/bin/bash","-c","./engine"]