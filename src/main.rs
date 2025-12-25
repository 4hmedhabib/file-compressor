use flate2::Compression;
use flate2::write::GzEncoder;
use std::env::args;
use std::fs::File;
use std::io::BufReader;
use std::io::copy;
use std::time::Instant;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 3 {
        eprintln!("Usage: `source` `target` (e.g., input.pdf output.pdf.gz)");
        return;
    }

    let source_path = &args[1];
    let target_path = &args[2];

    let input_file = File::open(source_path).expect("Failed to open source file");
    let source_len = input_file.metadata().unwrap().len();
    let mut input = BufReader::new(input_file);

    println!("Source: {}", source_path);
    println!("Target: {}", target_path);

    let output = File::create(target_path).expect("Failed to create target file");

    let mut encoder = GzEncoder::new(output, Compression::default());

    let start = Instant::now();
    copy(&mut input, &mut encoder).expect("Compression failed");

    let output_file = encoder.finish().expect("Failed to finish encoder");
    let target_len = output_file.metadata().unwrap().len();

    println!("---");
    println!("Source len: {} bytes", source_len);
    println!("Target len: {} bytes", target_len);
    println!("Elapsed: {:?}", start.elapsed());
    println!(
        "Compression Ratio: {:.2}%",
        (target_len as f64 / source_len as f64) * 100.0
    )
}
