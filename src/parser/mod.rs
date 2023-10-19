
use clap::error;
use serde::Deserialize;
use serde_json::Value;
use thiserror::Error;
extern crate serde_xml_rs;

use std::{fmt::Display, str::FromStr, collections::BTreeMap};

use crate::openapi::{Fixable, WeightScore, PathItem};
use std::error::Error;

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("Invalid input text. Either input specification text is empty or not valid")]
    InvalidInputText,
    #[error(
        "Invalid SOAP specification (WSDL definition) file. Please fix the errors and try again."
    )]
    InvalidSOAPSpec,
    #[error("Invalid Specification Type. Allowed Specification Types are in the code somewhere.")]
    InvalidSpecificationType,
    #[error("Error occurred while parsing input specification text: {0}")]
    ParseFailed(String),
    #[error("{0}")]
    JSONParseError(String),
    #[error("{0}")]
    NumberFormatError(String),
}

#[derive(Debug)]
pub enum ApiSpecificationType {
    OpenApiRest,
    SoapWSDL,

    Unknown,
}
impl Display for ApiSpecificationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiSpecificationType::OpenApiRest => write!(f, "Open API"),
            ApiSpecificationType::SoapWSDL => write!(f, "SOAP WSDL"),
            _ => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    text: &'a str,
    start: usize,
    end: usize,
}

impl<'a> Parser<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            start: 0,
            end: 0,
        }
    }

    pub fn parse(&mut self, spec_type: &ApiSpecificationType) -> Result<Vec<Fixable>, ParserError> {
        if self.text.is_empty() {
            return Err(ParserError::InvalidInputText);
        }
        let mut fixables: Vec<Fixable> = vec![];

        match *spec_type {
            ApiSpecificationType::OpenApiRest => {
                fixables = parse_open_api_rest(self.text).unwrap();
                Ok(fixables)
            }
            ApiSpecificationType::SoapWSDL => {
                fixables = parse_soap_wsdl(self.text).unwrap();
                Ok(fixables)
            }
            _ => Err(ParserError::InvalidSpecificationType),
        }
    }
}

fn parse_soap_wsdl(text: &str) -> Result<Vec<Fixable>, ParserError> {
    todo!()
}

///Use this to parse an OPENAPI specification type document
/// <br>It takes a string slice of the content of the OPEN Spec a.k.a swagger.json
fn parse_open_api_rest(text: &str) -> Result<Vec<Fixable>, ParserError> {
    let mut fixables: Vec<Fixable> = Vec::new();
    let mut final_core: u8 = 100; //final weight score...
    println!("Input Text: {}", text);

    //grabables
    let master_piece: Value = serde_json::from_str(text).map_err(|e| {
        ParserError::JSONParseError(format!("Error parsing Open API JSON spec: {}", e))
    })?;

    //INFO -> Nothing too serious => Just nameOfApi and Version

    //SERVERS -> Array [ {"url": ""} ]
    let server_base_url_exists = has_non_empty_array_item(&master_piece, "servers", "url");
    if !server_base_url_exists {
        fixables.push(Fixable::new("Invalid server. You must provide a server BASEURL for your API. Read OpenAPI specification standards for more information", 0, WeightScore::Critical));
        modify_score(&mut final_core, WeightScore::CRITICAL_VALUE);
    }
    // if let Some(servers) = master_piece.get("servers"){
    //     let server_base_url_exists = has_non_empty_array_item(servers);
    // }else{
    //     fixables.push(Fixable::new("Invalid server. You must provide a server BASEURL for your API. Read OpenAPI specification standards for more information", 0, WeightScore::Critical));
    //     modify_score(&mut finalScore, WeightScore::CRITICAL_VALUE);
    // }

    //PATHs -> a.k.a Endpoints -> Object -> has Other objects inside one for each endpoint
    //let sd: Value= serde_json::from_str(&master_piece.get("paths").unwrap().to_string()).unwrap();
    //let paths: BTreeMap<String, PathItem> = master_piece["paths"].to_owned();

    //COMPONENTS -> Logic Objects -> Request Objects' Schemas... containing fields, validation, regex, strings etc

    //SECURITY -> Array of Security Sechemes Objects -> Actually this part bears 30% of total score...

    println!("Fixables / Report {:?}", fixables);
    Ok(fixables)
}
fn has_non_empty_array_item(json_value: &Value, array_key: &str, array_item: &str) -> bool {
    if let Some(servers) = json_value.get(array_key) {
        if let Some(servers_array) = servers.as_array() {
            for server in servers_array {
                if let Some(url) = server.get(array_item) {
                    if url.is_string() && !url.as_str().unwrap_or_default().is_empty() {
                        return true;
                    }
                }
            }
        }
    }
    false
}
fn modify_score(final_score: &mut u8, weight_value: u8) {
    *final_score = final_score.saturating_sub(weight_value);
}
// pub fn is_wsdl_spec(xml_str: &str) -> Result<bool, serde_xml_rs::Error> {
//     let definitions: Definitions = serde_xml_rs::from_str(xml_str).expect("Could not parse xml file");

//     // Check if it's a WSDL definition
//     let definitions: Result<Definitions, serde_xml_rs::Error> = serde_xml_rs::from_str(xml_str);

//     match definitions {
//         Ok(parsed_definitions) => {
//             println!("{:#?}", parsed_definitions); // Access the parsed XML structure
//             Ok(true)
//             // You can now access and work with the parsed <definitions> tag and its content.
//         }
//         Err(err) => {
//             eprintln!("Error parsing XML: {:?}", err);
//             Err(ParserError::InvalidSOAPSpec)
//         }
//     }
// }
///usually the first at least 50 chars of WSDL should have the tag <definitions
pub fn is_wsdl_spec_v2(xml_str: &str) -> Result<bool, serde_xml_rs::Error> {
    Ok(xml_str[0..50]
        .to_lowercase()
        .replace(' ', "")
        .contains("<definitions"))
}

// #[derive(Debug, serde::Deserialize)]
// struct Definitions {
//     #[serde(rename = "definitions", namespace = "http://schemas.xmlsoap.org/wsdl/")]
//     wsdl_definitions: WsdlDefinitions,
// }

#[derive(Debug, Deserialize)]
struct WsdlDefinitions {
    // You can add more fields if needed
    // For example, check for specific elements within <definitions>
}
