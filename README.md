# data-inspector

Rust web service that generates insights for unstructured text data.

# Dependencies
```sh
brew install pytorch jq
```

# Dev
```sh
export LIBTORCH=$(brew --cellar pytorch)/$(brew info --json pytorch | jq -r '.[0].installed[0].version')
export LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH

RUST_LOG=info cargo watch -x run
```
