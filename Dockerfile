FROM rust:1.81-slim-bullseye AS build-default
RUN apt-get update && apt-get install -y upx-ucl
RUN rustup component add clippy
USER nobody
WORKDIR /opt/dockteur
COPY --chown=nobody . ./
RUN cargo build
RUN cargo clippy --all-targets --all-features -- -D warnings
RUN cargo test
RUN cargo build --release
RUN upx --best --lzma target/release/dockteur

FROM rust:1.81-alpine3.20 AS build-alpine
RUN apk add upx musl-dev
RUN rustup component add clippy
USER nobody
WORKDIR /opt/dockteur
COPY --chown=nobody . ./
RUN cargo build
RUN cargo clippy --all-targets --all-features -- -D warnings
RUN cargo test
RUN cargo build --release
RUN upx --best --lzma target/release/dockteur

FROM scratch AS default
COPY --from=build-default /opt/dockteur/target/release/dockteur /dockteur

FROM scratch AS alpine
COPY --from=build-alpine /opt/dockteur/target/release/dockteur /dockteur
