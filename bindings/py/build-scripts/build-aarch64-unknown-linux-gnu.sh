#!/bin/bash

set -e

curl --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --profile minimal --default-toolchain 1.57.0 -y
export PATH=/root/.cargo/bin:"$PATH"

cat >/root/.cargo/config <<EOF
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
EOF

rustup target add aarch64-unknown-linux-gnu

apt-get update

apt-get install -y gcc-aarch64-linux-gnu

cargo install maturin

cd /io

maturin build --release --target aarch64-unknown-linux-gnu
