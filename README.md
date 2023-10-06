# data-inspector

Rust web service that generates insights for unstructured text data.

# Dependencies

mac
```sh
brew install pytorch jq
```

ubuntu
```sh
sudo apt install jq

wget https://download.pytorch.org/libtorch/cu118/libtorch-cxx11-abi-shared-with-deps-2.0.0%2Bcu118.zip 

export LIBTORCH=~/devving/data-inspector/libtorch
export LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH
```

# Dev
```sh
export LIBTORCH=$(brew --cellar pytorch)/$(brew info --json pytorch | jq -r '.[0].installed[0].version')
export LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH

RUST_LOG=info cargo watch -x run
```
