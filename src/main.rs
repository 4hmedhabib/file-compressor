use std::env::args;
use std::process;

use file_compressor::compress_file;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 3 {
        eprintln!("Usage: `source` `target` (e.g., input.pdf output.pdf.gz)");
        process::exit(1);
    }

    println!("Compressing: {} -> {}", &args[1], &args[2]);

    if let Err(e) = compress_file(&args[1], &args[2]) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }

    println!("Compression successful!")
}
