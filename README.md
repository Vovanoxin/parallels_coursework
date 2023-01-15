# Parallel Reverted Index Building
To build the project you need to install the Rust toolchain
it can be downloaded with the official tutorial by the link https://www.rust-lang.org/learn/get-started.

To build project run cargo build --release. This will create `index_client` and `index_server`
binaries in `target/release/` directory.

The `index_server` has a list of parameters:
- `--files-dir` - directory with files to index. Note that it automatically goes through directories recursively
- `--index-path` - path to JSON file to write index (if `--build-index` specified) or read index from (if `--build-index` not specified).
- `--build_index` - build index.
- `--start_server` - start TCP server. Note that if there is nor `--build_index` nor `--index-path` the program will fail.
- `--thread_number` - specify number of threads to use in threadpool.
- `--benchmark` - run benchmark of build_index function. Requires `--files-dir`.