#!/bin/bash

set -e

/usr/bin/maturin build --release --target x86_64-unknown-linux-gnu --manylinux 2010
