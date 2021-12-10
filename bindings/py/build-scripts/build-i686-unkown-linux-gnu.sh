#!/bin/bash

set -e

rustup target add i686-unknown-linux-gnu

yum install -y libgcc.i686

/usr/bin/maturin build --release --target i686-unknown-linux-gnu --manylinux 2010
