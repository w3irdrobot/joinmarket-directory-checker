set dotenv-load

default:
    just --list

lint:
    cargo +nightly fmt
    cargo check
    cargo clippy --all-targets --all-features

start:
    cargo run
