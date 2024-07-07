use std::fs::File;
use std::io::Write;
use flate2::Compression;
use flate2::write::GzEncoder;
use tar::Builder;

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
}