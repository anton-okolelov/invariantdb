FROM rust:1.50 as builder
WORKDIR /usr/src/invariantdb
COPY . .
RUN cargo install --path .
COPY .docker/entrypoint.sh /
ENTRYPOINT /entrypoint.sh
