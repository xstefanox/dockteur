FROM rust:1.82-slim-bullseye AS default-builder
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

FROM rust:1.82-alpine3.20 AS alpine-builder
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
COPY --from=default-builder /opt/dockteur/target/release/dockteur /dockteur

FROM scratch AS alpine
COPY --from=alpine-builder /opt/dockteur/target/release/dockteur /dockteur
