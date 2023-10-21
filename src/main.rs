mod lexer;
mod openapi;
mod parser;
use std::{
    fs,
    io::Error,
    path::{Path, PathBuf},
};

use oapi::OApi;
use serde_json::Value;
use sppparse::SparseRoot;

use crate::{
    lexer::{Lexer, Token},
    openapi::{OpenApi, PathData, Scanner},
};

fn run(text: &str) -> anyhow::Result<()> {
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
    let open_api_text = fs::read_to_string(open_api_path)
        .unwrap_or_else(|e| panic!("Error while reading file {}", e));

    // let open_api: OpenApi = serde_json::from_str(&open_api_text).expect("Could not retrieve paths");
    // if let Some(path) = &open_api.paths{
    //     for (url, path_data) in path {
    //         println!("URL: {}", url);
    //         println!("Path Data >>> {:?}", path_data);
    //     }
    // }

    let json: Value = serde_json::from_str(&open_api_text).expect("Could not parse JSON text");

    if let Some(paths) = json.get("paths") {
        //println!("{:?}", paths);
        println!("Paths exists in the json text");
    } else {
        println!("The 'paths' array does not exist in the JSON.");
    }
    //so I gotta write my own open api json parser...
    //println!("Paths object as retrieved from the specification: {}", &paths.to_string());
    let json_string = r#"
    {
        "name": "John Doe",
        "age": 30,
        "city": "Sample City"
    }
    "#;
    let mut tokens: Vec<Token> = Vec::new();
    let mut lexer: Lexer = Lexer::new(json_string);
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    println!("{:?}", tokens);

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
