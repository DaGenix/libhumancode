#!/bin/bash

set -e

docker run --rm -v $(pwd):/io konstin2/maturin:latest build --release --target x86_64-unknown-linux-gnu --manylinux 2010

docker run --rm -v $(pwd):/io --entrypoint "" konstin2/maturin:latest /io/build-scripts/build-i686-unkown-linux-gnu.sh

docker run --rm -v $(pwd):/io quay.io/dagenix/docker-maturin-cross build --release --target armv7-unknown-linux-gnueabihf --manylinux 2_24

docker run --rm -v $(pwd):/io quay.io/dagenix/docker-maturin-cross build --release --target aarch64-unknown-linux-gnu --manylinux 2_24
