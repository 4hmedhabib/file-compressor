use file_compressor::compress_file;
use flate2::read::GzDecoder;
use std::fs::{self, File, write};
use std::io::Read;

#[test]
fn test_pdf_compression_cycle() {
    let src = "test_input.pdf";
    let tgt = "test_output.pdf.gz";

    let dummy_content = b"%PDF-1.4 test content";

    write(src, dummy_content).unwrap();

    // Test the logic
    compress_file(src, tgt).expect("Compression failed");

    // Verify
    let compressed_file = File::open(tgt).unwrap();
    let mut decoder = GzDecoder::new(compressed_file);
    let mut result = Vec::new();
    decoder.read_to_end(&mut result).unwrap();

    assert_eq!(dummy_content.to_vec(), result);

    let _ = fs::remove_file(src);
    let _ = fs::remove_file(tgt);
}
