use flate2::Compression;
use flate2::write::GzEncoder;
use std::fs::File;
use std::io::{BufReader, Error, copy};
use std::time::Instant;

pub fn compress_file(source_path: &str, target_path: &str) -> Result<(), Error> {
    let input_file = File::open(source_path)
        .map_err(|e| Error::new(e.kind(), format!("Could not open source: {}", e)))?;

    let source_len = input_file.metadata()?.len();
    let mut input = BufReader::new(input_file);

    let output_file = File::create(target_path)
        .map_err(|e| Error::new(e.kind(), format!("Could not create target: {}", e)))?;

    let mut encoder = GzEncoder::new(output_file, Compression::default());
    let start = Instant::now();

    copy(&mut input, &mut encoder)?;

    let compressed_file = encoder.finish()?;
    let target_len = compressed_file.metadata()?.len();

    println!("Source len:  {} bytes", source_len);
    println!("Target len:  {} bytes", target_len);
    println!("Elapsed:     {:?}", start.elapsed());

    Ok(())
}
