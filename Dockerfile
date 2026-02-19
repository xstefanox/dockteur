FROM rust:1.93.1-slim-trixie AS default-builder
SHELL ["/bin/sh", "-ec"]
RUN <<EOF
  apt-get update
  apt-get install -y upx-ucl
EOF
RUN rustup component add clippy
USER nobody
WORKDIR /opt/dockteur

COPY --chown=nobody Cargo.toml Cargo.lock ./
RUN <<EOF
  mkdir src
  echo 'fn main() {}' > src/main.rs
  cargo build
  cargo build --release
  cargo test --no-run
  rm -rf target/*/deps/dockteur-* target/*/dockteur* target/*/.fingerprint/dockteur-*
EOF

COPY --chown=nobody src ./src
RUN cargo build
RUN cargo clippy --all-targets --all-features -- -D warnings
ARG DOCKER_HOST
RUN cargo test
RUN <<EOF
  cargo build --release
  upx --best --lzma target/release/dockteur
EOF

FROM rust:1.93.1-alpine3.22 AS alpine-builder
SHELL ["/bin/sh", "-ec"]
RUN <<EOF
  apk add upx musl-dev
EOF
RUN rustup component add clippy
USER nobody
WORKDIR /opt/dockteur

COPY --chown=nobody Cargo.toml Cargo.lock ./
RUN <<EOF
  mkdir src
  echo 'fn main() {}' > src/main.rs
  cargo build
  cargo build --release
  cargo test --no-run
  rm -rf target/*/deps/dockteur-* target/*/dockteur* target/*/.fingerprint/dockteur-*
EOF

COPY --chown=nobody src ./src
RUN cargo build
RUN cargo clippy --all-targets --all-features -- -D warnings
ARG DOCKER_HOST
RUN cargo test
RUN <<EOF
  cargo build --release
  upx --best --lzma target/release/dockteur
EOF

FROM scratch AS default
COPY --from=default-builder /opt/dockteur/target/release/dockteur /dockteur

FROM scratch AS alpine
COPY --from=alpine-builder /opt/dockteur/target/release/dockteur /dockteur
