#!/bin/bash

export LIBTORCH=~/devving/data-inspector/libtorch
export LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH

RUST_LOG=info cargo watch -x run
