# Snurk

A Rust tool for generating zero-knowledge proof code from Circom circuits.

## Description

Snurk takes compiled Circom circuit files (R1CS, zkey, and WebAssembly witness generator) and generates
Rust code that embeds these files using `include_bytes!`. The generated code interfaces with the
arkworks-circom prover to create zero-knowledge proofs from circuit inputs.

## Installation

```
cargo install snurk
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
