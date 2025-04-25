use std::io;
use std::io::Read;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
}

fn main() {
    let arg = Cli::parse();
    let mut input = String::new();
    
    // Get piped input
    io::stdin()
        .read_to_string(&mut input)
        .expect("[-] Failed to read input.");

    // Find pattern in input
    for line in input.lines() {
        if line.contains(&arg.pattern) {
            println!("{line}");
        }
    }  
}