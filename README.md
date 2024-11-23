# Snurk

A Rust tool for generating optimized wasm prover binaries from Circom circuits.

## Description

Snurk takes compiled Circom circuit files (R1CS, zkey, and WebAssembly witness generator) and generates
Rust code that optimizes and inlines the prover assets to the binary.

## Installation

```
git clone https://github.com/thefrozenfire/snurk.git
cd snurk
cargo build -p snurk
./target/release/snurk --r1cs <R1CS> --zkey <ZKEY> --wasm <WASM>
```

## Usage

```
Usage: snurk --r1cs <R1CS> --zkey <ZKEY> --wasm <WASM>

Options:
      --r1cs <R1CS>
          Path to the R1CS constraint system file

      --zkey <ZKEY>
          Path to the proving/verification key file

      --wasm <WASM>
          Path to the witness generator WebAssembly file

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
