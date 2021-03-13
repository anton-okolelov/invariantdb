FROM rust:1.50 as builder
WORKDIR /usr/src/invariantdb
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=target \
   cargo install --path .
COPY .docker/entrypoint.sh /
ENTRYPOINT /entrypoint.sh
