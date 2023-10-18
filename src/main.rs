mod parser;
mod openapi;
use std::{fs, path::Path, io::Error};

use crate::openapi::Scanner;


fn run(text: &str) -> anyhow::Result<()>{
    //in the future, when we add CLI capabilities,
    //we extract CLI args here...
    let mut scanner: Scanner = Scanner::new(text);
    scanner.scan()?;
    scanner.display()?;

    Ok(())
}
fn main() {
    println!("Hello, world!");
    let open_api_path = Path::new("./examples/sample_openapi.json");
    let open_api_text = fs::read_to_string(open_api_path).unwrap_or_else(|e| {
        panic!("Error while reading file {}", e)
    });

    println!("Successfully retrieved open api file >>> {}", open_api_text);

    match run(&open_api_text) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{} {:#}", "Error:", e);

            std::process::exit(1);
        }
    }
    
    println!("The scanner has completed work");



}
