WASI_CC       ?= /opt/wasi-sdk/bin/clang
WIT_BINDGEN   ?= wit-bindgen
WASMLINK      ?= wasmlink
WASMTIME      ?= wasmtime

bindgen:
	$(WIT_BINDGEN) c --import ../cache.wit --out-dir bindings

build:
	$(WASI_CC) -I . -I ./bindings -c -o cache.o bindings/cache.c
	$(WASI_CC) main.cpp cache.o -o cpp_consumer.wasm

link:
	$(WASMLINK) cpp_consumer.wasm \
		--interface cache=../cache.wit \
		--profile wasmtime \
		--module cache=../rust-wasi-impl/target/wasm32-wasi/release/rust_wasi_impl.wasm \
		--output linked.wasm

run:
	$(WASMTIME) --wasm-features=module-linking,multi-memory --mapdir=/cache::. linked.wasm

clean:
	rm *.wasm && rm *.o
