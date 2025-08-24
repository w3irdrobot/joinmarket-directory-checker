set dotenv-load

default:
    just --list

lint:
    cargo +nightly fmt
    cargo check
    cargo clippy --all-targets --all-features

start:
    cargo run

migrate cmd='run':
    cargo sqlx migrate {{cmd}}

prepare:
    cargo sqlx prepare
