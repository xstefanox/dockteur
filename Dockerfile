FROM rust:1.61-slim AS build
ENV CARGO_HOME=cargo
RUN apt-get update && apt-get install -y upx-ucl
WORKDIR /opt/dockteur
COPY --chown=nobody . ./
RUN cargo build
RUN cargo test
RUN cargo build --release
RUN upx --best --lzma target/release/dockteur

FROM scratch AS production
COPY --from=build /opt/dockteur/target/release/dockteur /dockteur
