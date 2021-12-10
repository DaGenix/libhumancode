FROM quay.io/pypa/manylinux_2_24_x86_64

RUN curl --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --profile minimal --default-toolchain 1.57.0 -y

ENV PATH /root/.cargo/bin:$PATH

RUN cargo install maturin

RUN rustup target add aarch64-unknown-linux-gnu armv7-unknown-linux-gnueabihf

ADD cargo-config.toml /root/.cargo/config

RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y gcc-aarch64-linux-gnu gcc-arm-linux-gnueabihf && \
    rm -rf /var/cache/apt/

WORKDIR /io

ENTRYPOINT ["/root/.cargo/bin/maturin"]
