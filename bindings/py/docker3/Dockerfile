FROM konstin2/maturin

RUN yum install -y gcc-arm-linux-gnu gcc-aarch64-linux-gnu

RUN rustup target add armv7-unknown-linux-gnueabihf aarch64-unknown-linux-gnu

ADD cargo-config.toml /root/.cargo/config

ENTRYPOINT ["/usr/bin/maturin"]
