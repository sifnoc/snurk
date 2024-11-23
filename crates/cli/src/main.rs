use clap::Parser;
use color_eyre::eyre::Result;
use std::path::Path;
use std::io::Write;
use snurk::convert_zkey;

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

    // Create output file name based on input circuit name
    let input_name = Path::new(&cli.zkey)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("circuit")
        .to_lowercase();
    let input_name = input_name.strip_suffix(".zkey").unwrap_or("circuit");
    
    let output_path = format!("{}_prover.rs", input_name);
    let mut output = std::fs::File::create(&output_path)?;

    // Write the generated Rust code
    let code = format!(r#"use snurk_wasm::*;

const ZKEY: &[u8] = include_bytes!("{}");
const R1CS: &[u8] = include_bytes!("{}"); 
const WASM: &[u8] = include_bytes!("{}");

#[wasm_bindgen]
pub fn prove(inputs: &str) -> String {{
    prove_inner(inputs, ZKEY, R1CS, WASM)
}}
"#, zkey_path.display(), cli.r1cs, cli.wasm);

    output.write_all(code.as_bytes())?;

    tracing::info!("Generated prover code in {}", output_path);

    Ok(())
}
