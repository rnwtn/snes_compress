use std::{env, fs};

use snes_compress::CompressionType;

#[derive(Debug)]
struct Inputs {
    option: String,
    format: String,
    input_file: String,
    output_file: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        print_all_info();
        return;
    } else if args.len() != 5 {
        print_usage();
        return;
    }
    let inputs = get_inputs(&args);
    if !validate_inputs(&inputs) {
        return;
    }

    let in_file = inputs.input_file;
    let out_file = inputs.output_file;
    let compression_type = map_compression_type(&inputs.format);

    let source = fs::read(&in_file).unwrap();

    if inputs.option == "-d" {
        let decompressed = snes_compress::decompress(&source, compression_type).unwrap();
        fs::write(&out_file, &decompressed).unwrap();
        let dlen = decompressed.len();
        let clen = source.len();
        print_finished_stats(&in_file, &out_file, compression_type, dlen, clen);
    }
    if inputs.option == "-c" {
        let compressed = snes_compress::compress(&source, compression_type).unwrap();
        fs::write(&out_file, &compressed).unwrap();
        let dlen = source.len();
        let clen = compressed.len();
        print_finished_stats(&in_file, &out_file, compression_type, dlen, clen);
    }
}

fn print_finished_stats(
    in_file: &str,
    out_file: &str,
    compression_type: CompressionType,
    decompressed_len: usize,
    compressed_len: usize,
) {
    println!("input_file: {in_file}");
    println!("output_file: {out_file}");
    println!("format: {compression_type}");
    println!("decompressed_size: {decompressed_len:X}");
    println!("compressed_size: {compressed_len:X}");
    println!();
    println!("Done!");
}

fn validate_inputs(inputs: &Inputs) -> bool {
    if inputs.option != "-d" && inputs.option != "-c" {
        print_options();
        return false;
    }
    if inputs.format.to_lowercase() != "-lz5" {
        print_formats();
        return false;
    }
    return true;
}

fn map_compression_type(compression_type: &str) -> CompressionType {
    let compression_type: &str = &compression_type.to_lowercase();
    match compression_type {
        "-lz5" => CompressionType::LZ5,
        _ => panic!(),
    }
}

fn get_inputs(args: &Vec<String>) -> Inputs {
    let option = args[1].clone();
    let format = args[2].clone();
    let input_file = args[3].clone();
    let output_file = args[4].clone();
    Inputs {
        option,
        format,
        input_file,
        output_file,
    }
}

fn print_all_info() {
    print_info();
    print_usage();
    print_options();
    print_formats();
}

fn print_info() {
    println!("snes_compress:");
    println!("    For compressing and decompressing data for old games.");
    println!();
}

fn print_usage() {
    println!("Usage:");
    println!("    snes_compress [option] [format] <input_file> <output_file>");
    println!();
}

fn print_options() {
    println!("Options:");
    println!("    -d: decompress");
    println!("    -c: compress");
    println!();
}

fn print_formats() {
    println!("Formats:");
    println!("    -LZ5: LZ5");
    println!();
}
