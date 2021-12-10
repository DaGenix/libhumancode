#!/bin/bash

set -e

curl --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --profile minimal --default-toolchain 1.57.0 -y

export PATH=/root/.cargo/bin:"$PATH"

cat >/root/.cargo/config <<EOF
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
EOF

rustup target add armv7-unknown-linux-gnueabihf

apt-get update

apt-get install -y gcc-arm-linux-gnueabihf

cargo install maturin

cd /io

maturin build --release --target armv7-unknown-linux-gnueabihf
