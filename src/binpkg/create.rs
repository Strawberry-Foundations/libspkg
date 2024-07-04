use std::fs::File;
use std::io::Write;
use tar::Builder;
use flate2::write::GzEncoder;
use flate2::Compression;

pub fn create_package(metadata: serde_json::Value, source_dir: &str, output_file: &str) -> std::io::Result<()> {
    let metadata_json = serde_json::to_string(&metadata)?;
    let metadata_length = metadata_json.len();
    let header = format!("LENGTH={}\n{}", metadata_length, metadata_json);

    let mut output = File::create(output_file)?;
    output.write_all(header.as_bytes())?;
    output.write_all(b"\n")?;

    let tar_gz = GzEncoder::new(output, Compression::default());
    let mut tar = Builder::new(tar_gz);
    tar.append_dir_all(".", source_dir)?;

    Ok(())
}
