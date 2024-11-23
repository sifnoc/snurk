use ark_bn254::Bn254;
use ark_circom::read_zkey;
use ark_groth16::Groth16;
use ark_serialize::CanonicalSerialize;
use ark_snark::SNARK;
use std::path::Path;
use std::path::PathBuf;
use color_eyre::eyre::Result;

type GrothBn = Groth16<Bn254>;

pub fn convert_zkey(input_zkey: &Path) -> Result<((std::fs::File, PathBuf), (std::fs::File, PathBuf))> {
    let mut zkey_file = std::fs::File::open(input_zkey).unwrap();
    let (params, _matrices) = read_zkey(&mut zkey_file).unwrap();

    let input_name = input_zkey.file_name().and_then(|n| n.to_str()).unwrap_or("circuit");
    let input_name = input_name
        .to_lowercase();
    let input_name = input_name
        .strip_suffix(".zkey")
        .unwrap_or("circuit");
    let temp_zkey = std::env::temp_dir().join(format!("{}_temp.zkey", input_name));
    let mut writer = std::fs::File::create(&temp_zkey).unwrap();
    params.serialize_uncompressed(&mut writer).unwrap();

    let pvk = GrothBn::process_vk(&params.vk).unwrap();

    let temp_vkey = std::env::temp_dir().join(format!("{}_temp.vkey", input_name));
    let mut vkey_writer = std::fs::File::create(&temp_vkey).unwrap();
    pvk.serialize_uncompressed(&mut vkey_writer).unwrap();

    let zkey_file = std::fs::File::open(temp_zkey.clone()).unwrap();
    let vkey_file = std::fs::File::open(temp_vkey.clone()).unwrap();

    // Reopen the temp zkey file in read mode and return the handle
    Ok(((zkey_file, temp_zkey), (vkey_file, temp_vkey)))
}
