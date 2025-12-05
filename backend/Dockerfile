FROM rust:1.91-trixie AS builder

ARG BUILDDIR=/app
WORKDIR ${BUILDDIR}
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=${BUILDDIR}/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
<<EOF
set -e
cargo build --locked --release
cp ./target/release/as1db /usr/local/bin/as1db
EOF

FROM debian:trixie-slim

RUN apt-get update && apt-get install -y ca-certificates openssl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/bin/as1db /usr/local/bin/as1db
CMD ["as1db"]
