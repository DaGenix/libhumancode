#!/bin/bash

docker run --rm -it -v $(pwd):/io --entrypoint "" konstin2/maturin:latest /io/build-scripts/build-i686-unkown-linux-gnu.sh
docker run --rm -it -v $(pwd):/io --entrypoint "" konstin2/maturin:latest /io/build-scripts/build-x86_64-unknown-linux-gnu.sh
docker run --rm -it -v $(pwd):/io quay.io/pypa/manylinux_2_24_x86_64 /io/build-scripts/build-aarch64-unknown-linux-gnu.sh
docker run --rm -it -v $(pwd):/io quay.io/pypa/manylinux_2_24_x86_64 /io/build-scripts/build-armv7-unknown-linux-gnueabihf.sh
