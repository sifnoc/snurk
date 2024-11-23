# Snurk

A Rust tool for generating optimized wasm prover binaries from Circom circuits.

## Description

Snurk takes compiled Circom circuit files (R1CS, zkey, and WebAssembly witness generator) and generates
Rust code that optimizes and inlines the prover assets to the binary.

## Installation

```
git clone https://github.com/thefrozenfire/snurk.git
cd snurk

rustup toolchain install nightly-2024-05-22
rustup target add --toolchain nightly-2024-05-22 wasm32-unknown-unknown
cargo install wasm-pack

cargo build -p snurk --release
```

## Usage

```
Usage: ./target/release/snurk --r1cs <R1CS> --zkey <ZKEY> --wasm <WASM>

Options:
      --r1cs <R1CS>
          Path to the R1CS constraint system file

      --zkey <ZKEY>
          Path to the proving key file

      --wasm <WASM>
          Path to the witness generator WebAssembly file

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Known Issues

> [!IMPORTANT]
> **Note on Rust-to-WASM Compilation**: This project requires compiling Rust into WASM, which needs [`clang`](https://clang.llvm.org/) version 16.0.0 or newer. MacOS users, be aware that Xcode's default `clang` might be older. If you encounter the error `No available targets are compatible with triple "wasm32-unknown-unknown"`, it's likely due to an outdated `clang`. Updating `clang` to a newer version should resolve this issue.
> 
> For MacOS aarch64 users, if Apple's default `clang` isn't working, try installing `llvm` via Homebrew (`brew install llvm`). You can then prioritize the Homebrew `clang` over the default macOS version by modifying your `PATH`. Add the following line to your shell configuration file (e.g., `.bashrc`, `.zshrc`):
> ```sh
> export PATH="/opt/homebrew/opt/llvm/bin:$PATH"
> ```