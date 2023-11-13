format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run 

run-release:
	cargo run --release --bin my_binary

build-release:
    cargo build --release

all: format lint test run
