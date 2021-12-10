URL ?= 

cargo-run:
	cargo run -- ${URL}

release: 
	CROSS_COMPILE="x86_64-linux-musl-" RUSTFLAGS="-C linker=x86_64-linux-musl-gcc" RUST_BACKTRACE=full cargo build --release --target=x86_64-unknown-linux-musl