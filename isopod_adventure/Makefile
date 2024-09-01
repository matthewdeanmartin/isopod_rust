# Makefile for building, formatting, linting, testing, and packaging Rust project

.PHONY: all format lint test build clean

all: format lint test build

format:
	cargo fmt --all

lint:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	cargo test

build:
	cargo build --release

clean:
	cargo clean

run:
	cargo run --release