WASMLINK      ?= wasmlink
WASMTIME      ?= wasmtime
CARGO         ?= cargo

build:
	$(CARGO) build --target wasm32-wasi --release

link:
	$(WASMLINK) target/wasm32-wasi/release/rust-consumer.wasm \
		--interface cache=../cache.wit \
		--profile wasmtime \
		--module cache=../rust-wasi-impl/target/wasm32-wasi/release/rust_wasi_impl.wasm \
		--output linked.wasm

run:
	$(WASMTIME) --wasm-features=module-linking,multi-memory --mapdir=/cache::. linked.wasm
