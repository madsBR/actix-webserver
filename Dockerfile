ARG branch=TEST

FROM node:14-alpine AS static_env
WORKDIR /build
COPY actix-engine/engine/static ./static
COPY actix-engine/engine/package.json actix-engine/engine/package-lock.json actix-engine/engine/webpack.config.js actix-engine/engine/tsconfig.json ./
RUN npm install webpack --yes && npm install webpack-cli --yes && npm install typescript && npm install ts-loader && npm install --production
# Copy only package files and install dependencies


FROM static_env AS static_builder
WORKDIR /build

RUN npm run build --production


FROM rust:1.71-bookworm as rust_builder
LABEL homepage_stage="rust_build"
WORKDIR /build
COPY . .
ENV BRANCH=${branch}
WORKDIR /build/actix-engine/engine
RUN cargo build --release



FROM debian:bookworm-slim as runtime

WORKDIR /app
EXPOSE 8080
RUN useradd -ms /bin/bash host && mkdir -p /app && chown -R host:host /app && chmod -R 744 /app
USER host
COPY --from=rust_builder --chown=host:host /build/actix-engine/target/release /app
COPY --from=static_builder --chown=host:host /build/static /app/static

ENTRYPOINT ["/bin/bash","-c","./engine"]