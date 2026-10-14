# syntax=docker/dockerfile:1

FROM rust:1-alpine3.24 AS builder

RUN apk upgrade --no-cache \
    && apk add --no-cache musl-dev

WORKDIR /build

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --locked --release --bin unifi-exporter

FROM alpine:3.24

LABEL org.opencontainers.image.description="Read-only Prometheus exporter for UniFi Dream Machines."

RUN apk upgrade --no-cache \
    && apk add --no-cache ca-certificates

COPY --from=builder /build/target/release/unifi-exporter /usr/local/bin/unifi-exporter

EXPOSE 9130

USER nobody

ENTRYPOINT ["/usr/local/bin/unifi-exporter"]
