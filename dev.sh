#!/bin/bash

LIBTORCH=~/devving/data-inspector/libtorch \
LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH \
RUST_LOG=debug \
RUST_BACKTRACE=1 \
cargo watch -x run
