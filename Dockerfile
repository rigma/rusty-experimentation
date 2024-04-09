FROM clux/muslrust AS builder
ARG TARGET_ARCH="x86_64"

COPY .cargo ./.cargo
COPY crates ./crates
COPY Cargo.toml .

RUN --mount=type=cache,target=/volume/target \
  --mount=type=cache,target=/root/.cargo/registry \
  cargo build --target ${TARGET_ARCH:-"x86_64"}-unknown-linux-musl --bin backbone-metadata --release \
  && mv /volume/target/${TARGET_ARCH:-"x86_64"}-unknown-linux-musl/release/backbone-metadata .

FROM cgr.dev/chainguard/static

COPY --from=builder --chown=nonroot:nonroot /volume/backbone-metadata /app/backbone-metadata

EXPOSE 80

ENTRYPOINT [ "/app/backbone-metadata", "--host", "0.0.0.0", "--port", "80"]
