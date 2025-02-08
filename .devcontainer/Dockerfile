FROM debian:bookworm-slim

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    RUST_VERSION=1.83.0

RUN apt-get update && apt-get install -y --no-install-recommends git ca-certificates build-essential pkg-config

RUN chmod +x ~/rustup-init \
    && ~/rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION --default-host aarch64-unknown-linux-gnu \
    && rm -rf ~/rustup-init \
    && chmod -R a+w $RUSTUP_HOME $CARGO_HOME

ENV FLASK_APP=rusty-cb
EXPOSE 8000

ENTRYPOINT ["/bin/bash", "-l", "-c", "flask run --host 0.0.0.0 --port 8000"]