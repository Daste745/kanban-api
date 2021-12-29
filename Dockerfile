FROM rust:1-slim-buster as base

USER root
WORKDIR /build
RUN cargo init --name backend
COPY Cargo.toml /build/Cargo.toml
RUN cargo fetch

COPY src /build/src

CMD [ "cargo", "test", "--offline" ]


FROM base as build

RUN cargo build --release --offline


FROM debian:buster-slim as run

COPY --from=build /build/target/release/main /usr/bin/backend

ENTRYPOINT [ "/usr/bin/backend" ]
