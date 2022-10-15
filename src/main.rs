use std::{env, fs};

use snes_tools::data_compression::{self, CompressionType};

#[derive(Debug)]
struct Inputs {
    option: String,
    input_file: String,
    output_file: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        print_all_info();
        return;
    } else if args.len() != 4 {
        print_usage();
        return;
    }
    let inputs = get_inputs(&args);
    if !validate_inputs(&inputs) {
        return;
    }

    let source = fs::read(&inputs.input_file).unwrap();

    if inputs.option == "-d" {
        let decompressed = data_compression::decompress(&source, CompressionType::LZ5).unwrap();
        fs::write(&inputs.output_file, decompressed).unwrap();
    }
    if inputs.option == "-c" {
        let compressed = data_compression::compress(&source, CompressionType::LZ5).unwrap();
        fs::write(&inputs.output_file, compressed).unwrap();
    }
}

fn validate_inputs(inputs: &Inputs) -> bool {
    if inputs.option != "-d" && inputs.option != "-c" {
        print_options();
        return false;
    }
    return true;
}

fn get_inputs(args: &Vec<String>) -> Inputs {
    let option = args[1].clone();
    let input_file = args[2].clone();
    let output_file = args[3].clone();
    Inputs {
        option,
        input_file,
        output_file,
    }
}

fn print_all_info() {
    print_info();
    print_usage();
    print_options();
}

fn print_info() {
    println!("snes_tools:");
    println!("    A compression library that only supports LZ5 right");
    println!("    now, but may support others later.");
    println!();
}

fn print_usage() {
    println!("Usage:");
    println!("    snes_tools [option] <input_file> <output_file>");
    println!();
}

fn print_options() {
    println!("Options:");
    println!("    -d: decompress");
    println!("    -c: compress");
    println!();
}
