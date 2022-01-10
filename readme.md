# Wasm components example

This repository contains an example of a WebAssembly interface, implemented in
Rust as a WebAssembly component, then consumed from other Rust and C++
components.

It is a showcase of the
[`wit-bindgen`](https://github.com/bytecodealliance/wit-bindgen) tooling from
the Bytecode Alliance.

### Building, linking, and executing

```bash
# building the implementation
$ cd rust-wasi-impl && make build

# building, linking, and running the C++ example
$ cd cpp-consumer && make bindgen build link run

# building, linking, and running the Rust example
$ cd rust-consumer && make build link run
```

### Prerequisites

- [Wasmtime](https://github.com/bytecodealliance/wasmtime) at
  [v0.33](https://github.com/bytecodealliance/wasmtime/releases/tag/v0.33.0)
- [`wit-bindgen`](https://github.com/bytecodealliance/wit-bindgen) at
  [2e654dc82b](https://github.com/bytecodealliance/wit-bindgen/commit/2e654dc82b7f9331719ba617a36ed5967b2aecb0)
- [WASI SDK](https://github.com/WebAssembly/wasi-sdk) at
  [v12+](https://github.com/WebAssembly/wasi-sdk/releases/tag/wasi-sdk-14) in
  `/opt/wasi-sdk/` (configurable in `Makefile`)
- [Rust](https://www.rust-lang.org/) at
  [1.56+](https://www.rust-lang.org/tools/install) with the `wasm32-wasi` target
  configured
