use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use serde_json::Value;
use tar::Archive;
use flate2::read::GzDecoder;

pub fn extract_package(input_file: &str, output_dir: &str) -> std::io::Result<Value> {
    let file = File::open(input_file)?;
    let mut reader = BufReader::new(file);

    let mut length_line = String::new();
    reader.read_line(&mut length_line)?;
    let metadata_length: usize = length_line.trim().split('=').nth(1).unwrap().parse().unwrap();

    let mut metadata_json = vec![0; metadata_length];
    reader.read_exact(&mut metadata_json)?;
    let metadata: Value = serde_json::from_slice(&metadata_json)?;

    let mut separator = [0; 1];
    reader.read_exact(&mut separator)?;

    let tar_gz = GzDecoder::new(reader);
    let mut archive = Archive::new(tar_gz);
    archive.unpack(output_dir)?;

    Ok(metadata)
}