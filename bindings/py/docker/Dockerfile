FROM debian:bullseye

RUN apt-get update

RUN DEBIAN_FRONTEND=noninteractive apt-get install -y curl python3 build-essential

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o ./install.sh && \
    chmod +x ./install.sh && \
    ./install.sh --no-modify-path -y -q --profile minimal \
        -t armv7-unknown-linux-gnueabihf && \
    rm ./install.sh

RUN /root/.cargo/bin/cargo install maturin

RUN cat >>~/.cargo/config <<EOF
    [target.armv7-unknown-linux-gnueabihf]
    linker = "arm-linux-gnueabihf-gcc"
    EOF

RUN DEBIAN_FRONTEND=noninteractive apt-get install -y gcc-arm-linux-gnueabihf

RUN PATH=/root/.cargo/bin/:"$PATH"

PYO3_NO_PYTHON=1 PYO3_CROSS=1 maturin build --release --target armv7-unknown-linux-gnueabihf --compatibility linux \
    \
