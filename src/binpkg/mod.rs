use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use serde_json::Value;
use tar::{Archive, Builder};

use crate::binpkg::metadata::Metadata;

pub mod create;
pub mod read;
pub mod metadata;

pub struct BinPkg {
    pub metadata: Metadata,
    pub source: Option<String>,
    pub output: Option<String>,
}

impl BinPkg {
    pub fn create(metadata: Metadata, source_directory: impl ToString, output_file: impl ToString) -> eyre::Result<Self> {
        let metadata_json = serde_json::to_string(&metadata)?;
        let metadata_length = metadata_json.len();
        let header = format!("LENGTH={}\n{}", metadata_length, metadata_json);

        let mut output = File::create(output_file.to_string())?;
        output.write_all(header.as_bytes())?;
        output.write_all(b"\n")?;

        let tar_gz = GzEncoder::new(output, Compression::default());
        let mut tar = Builder::new(tar_gz);
        tar.append_dir_all(".", source_directory.to_string())?;
                 
        Ok(Self {
            metadata,
            source: Some(source_directory.to_string()),
            output: Some(output_file.to_string())
        })
    }
    
    pub fn read(input_file: impl ToString) -> eyre::Result<Self> {
        let file = File::open(input_file.to_string())?;
        let mut reader = BufReader::new(file);

        let mut length_line = String::new();
        reader.read_line(&mut length_line)?;
        let metadata_length: usize = length_line.trim().split('=').nth(1).unwrap().parse().unwrap();

        let mut metadata_json = vec![0; metadata_length];
        reader.read_exact(&mut metadata_json)?;
        let metadata: Metadata = serde_json::from_slice(&metadata_json)?;

        Ok(Self {
            metadata,
            source: Some(input_file.to_string()),
            output: None
        })
    }
}