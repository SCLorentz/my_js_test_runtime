FROM debian:bookworm-slim

COPY cmd/ /root/

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    RUST_VERSION=1.83.0

RUN rm -rf ~/.bashrc \
    && mv ~/.bash ~/.bashrc

# TODO: install manualy essentials instead of using build-essential (eg. install gcc)
RUN apt-get update && apt-get install -y --no-install-recommends git ca-certificates build-essential

RUN chmod +x ~/rustup-init \
    && ~/rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION --default-host aarch64-unknown-linux-gnu \
    && rm -rf ~/rustup-init \
    && chmod -R a+w $RUSTUP_HOME $CARGO_HOME

ENV FLASK_APP=zagware
EXPOSE 8000

CMD ["/bin/bash", "-l", "-c", "flask run --host 0.0.0.0 --port 8000"]