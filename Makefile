fmt:
	cargo fmt --all
	cargo check --all
	cargo clippy --all

run: 
	cargo run --all 

release:
	cargo build --release --all