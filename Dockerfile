FROM --platform=$BUILDPLATFORM rust:1.71-slim AS build-default
ENV CARGO_HOME=cargo
RUN apt-get update && apt-get install -y upx-ucl
WORKDIR /opt/dockteur
COPY --chown=nobody . ./
RUN cargo build
RUN cargo test
RUN cargo build --release
RUN upx --best --lzma target/release/dockteur

FROM --platform=$BUILDPLATFORM rust:1.71-alpine3.18 AS build-alpine
ENV CARGO_HOME=cargo
RUN apk add upx musl-dev
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
