FROM rust:1.79-slim-bullseye AS build-default
RUN apt-get update && apt-get install -y upx-ucl
USER nobody
WORKDIR /opt/dockteur
COPY --chown=nobody . ./
RUN cargo build
RUN cargo test
RUN cargo build --release
RUN upx --best --lzma target/release/dockteur

FROM rust:1.79-alpine3.19 AS build-alpine
RUN apk add upx musl-dev
USER nobody
WORKDIR /opt/dockteur
COPY --chown=nobody . ./
RUN cargo build
RUN cargo test
RUN cargo build --release
RUN upx --best --lzma target/release/dockteur

FROM scratch AS default
COPY --from=build-default /opt/dockteur/target/release/dockteur /dockteur

FROM scratch AS alpine
COPY --from=build-alpine /opt/dockteur/target/release/dockteur /dockteur
