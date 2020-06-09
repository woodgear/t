set -e
cargo fmt --all -- --check
cargo check --all
cargo clippy --all-targets --all-features -- -D warnings

cargo tarpaulin -v
