mod parser;
mod openapi;
use std::{fs, path::Path};

use crate::openapi::Scanner;

fn main() {
    println!("Hello, world!");
    let open_api_path = Path::new("./examples/sample_openapi.json");
    let open_api_text = fs::read_to_string(open_api_path).unwrap_or_else(|e| {
        panic!("Error while reading file {}", e)
    });

    println!("Successfully retrieved open api file >>> {}", open_api_text);

    let mut scanner: Scanner = Scanner::new(open_api_text);
    scanner.scan();
    scanner.display();

    println!("The scanner has completed work");



}
