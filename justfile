# cleans the rust core
core-clean:
    rm -rf ./target
    cargo clean

# builds the rust core
core-build:
    cargo build --all

# builds the rust core in release mode
core-build-release:
    cargo build --release --all

# tests the rust core
core-test:
    cargo nextest run

# reformats the rust core to fit standards
core-format:
    cargo fmt

# checks that all core files are formatted to standard
core-format-check:
    cargo fmt --all -- --check

# lints the core
core-lint:
    cargo clippy --all --all-targets -- -D warnings -A clippy::empty_line_after_doc_comments

alias c := core-incremental

# runs an incremental (local) build on the core
core-incremental: core-format core-lint core-build core-test

core-full: core-clean core-format-check core-lint core-build-release core-test