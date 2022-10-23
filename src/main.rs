use snes_address::errors::AddressError;
use std::env;

#[derive(Debug)]
struct Inputs {
    option: String,
    address: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        print_all_info();
        return;
    } else if args.len() != 3 {
        print_usage();
        return;
    }
    let inputs = get_inputs(&args);
    if !validate_inputs(&inputs) {
        return;
    }

    let address = usize::from_str_radix(&inputs.address, 16).unwrap();

    if inputs.option == "-P2L" {
        match snes_address::pc_to_lorom(address) {
            Ok(mapped_address) => println!("LoRom address: {mapped_address:X?}"),
            Err(AddressError::OutOfBounds) => println!("Address out of bounds"),
        }
    }
    if inputs.option == "-L2P" {
        match snes_address::lorom_to_pc(address) {
            Ok(mapped_address) => println!("PC address: {mapped_address:X?}"),
            Err(AddressError::OutOfBounds) => println!("Address out of bounds"),
        }
    }
}

fn validate_inputs(inputs: &Inputs) -> bool {
    if inputs.option != "-P2L" && inputs.option != "-L2P" {
        print_options();
        return false;
    }
    if let Err(e) = usize::from_str_radix(&inputs.address, 16) {
        print_usage();
        return false;
    }
    return true;
}

fn get_inputs(args: &Vec<String>) -> Inputs {
    let option = args[1].clone();
    let address = args[2].clone();
    Inputs { option, address }
}

fn print_all_info() {
    print_info();
    print_usage();
    print_options();
}

fn print_info() {
    println!("snes_address:");
    println!("    For changing between PC and SNES memory map addresses.");
    println!();
}

fn print_usage() {
    println!("Usage:");
    println!("    snes_address [option] <address_in_hex>");
    println!();
}

fn print_options() {
    println!("Options:");
    println!("    -P2L: decompress");
    println!("    -L2P: LoRom to PC");
    println!();
}
