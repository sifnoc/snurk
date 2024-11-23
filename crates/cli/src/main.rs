use clap::Parser;
use color_eyre::eyre::Result;
use std::path::Path;
use std::io::Write;
use snurk::convert_zkey;

use wasm_pack::{
    command::run_wasm_pack,
    Cli as WasmPackCli, PBAR,
};

#[derive(Parser)]
#[command(
    author, 
    version, 
    about = "A tool for generating Rust code that embeds Circom circuit files and generates zero-knowledge proofs",
    long_about = "This tool takes R1CS, zkey, and witness generator files as input and generates \
    a Rust source file that uses include_bytes! to embed these files. The generated code interfaces \
    with the arkworks-circom prover to create zero-knowledge proofs from circuit inputs."
)]
struct Cli {
    /// Path to the R1CS constraint system file
    #[arg(long)]
    r1cs: String,

    /// Path to the proving/verification key file
    #[arg(long)]
    zkey: String,

    /// Path to the witness generator WebAssembly file
    #[arg(long)]
    wasm: String,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    tracing::info!("Using R1CS file: {}", cli.r1cs);
    tracing::info!("Using zkey file: {}", cli.zkey);
    tracing::info!("Using witness generator: {}", cli.wasm);

    let zkey_path = Path::new(&cli.zkey);

    let ((zkey_file, zkey_path), (vkey_file, _vkey_path)) = convert_zkey(&zkey_path).unwrap();

    println!("zkey_file: {:?}", zkey_file);
    println!("vkey_file: {:?}", vkey_file);
    
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let wasm_src = Path::new(manifest_dir).join("../wasm/src/circuit.rs");

    let mut output = std::fs::File::create(&wasm_src)?;

    // Write the generated Rust code
    let code = format!(r#"use crate::*;
use wasm_bindgen::prelude::*;

const ZKEY: &[u8] = include_bytes!("{}");
const R1CS: &[u8] = include_bytes!("{}"); 
const WASM: &[u8] = include_bytes!("{}");

#[wasm_bindgen]
pub fn prove(inputs: &str) -> String {{
    prove_inner(inputs, ZKEY, R1CS, WASM)
}}
"#, zkey_path.display(), cli.r1cs, cli.wasm);

    output.write_all(code.as_bytes())?;

    tracing::info!("Generated prover code in {}", wasm_src.display());

    let wasm_pack_args = WasmPackCli::parse_from(&["wasm-pack", "build", "crates/wasm", "--target", "web", "--out-dir", "../../pkg", "--", "--no-default-features", "--features", "wasm,build", "-Z", "build-std=std,panic_abort"]);

    PBAR.set_log_level(wasm_pack_args.log_level);

    if wasm_pack_args.quiet {
        PBAR.set_quiet(true);
    }

    run_wasm_pack(wasm_pack_args.cmd).unwrap();

    Ok(())
}
