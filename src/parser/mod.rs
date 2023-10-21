use clap::error;
use oapi::{OApi, OApiDocument};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
extern crate serde_xml_rs;

use std::{collections::BTreeMap, fmt::Display, str::FromStr};

use crate::openapi::{EndpointPath, Fixable, IssueScoreImpact, OpenApi, OpenApiDoc, WeightScore};
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
    #[error("{0}")]
    DataValidationError(String),
    #[error("Specification type: {0} is compatible with {0} type documents")]
    IncompatibleSpecificationAndDocumentType(String, String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Document{
    OpenAPI(OApiDocument),

}
impl Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Document::OpenAPI(_) => write!(f, "Open API"),
            _ => unimplemented!(),
        }
    }
}
impl Into<String> for Document{
    fn into(self) -> String {
        match self{
            Document::OpenAPI(_) => "OpenAPI".into(),
            _ => unimplemented!()
        }
    }
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
impl Into<String> for ApiSpecificationType{
    fn into(self) -> String {
        match self{
            ApiSpecificationType::OpenApiRest => "OpenAPI".into(),
            ApiSpecificationType::SoapWSDL => "SoapWSDL".into(),
           _ => unimplemented!()
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

/// <p>Use this to parse a plain OpenApi spec text to an OpenAPIDoc object<br></p>
/// <p>It returns A result of type OpenApiDoc or a ParserError</p>
/// <p>parameters -> @text </p>
fn parse_json_open_api(text: &str) -> Result<OApiDocument, ParserError> {
    // let mut open_api_doc: OpenApiDoc = OpenApiDoc::new("3.0.1");
    //let open_api: OpenApi = serde_json::from_str(&text).expect("Could not retrieve paths");
    let open_api: OApiDocument = serde_json::from_str(text).map_err(|e| {
        ParserError::ParseFailed(format!("Failed to parse open api JSON specification {}", e))
    })?;

    //collect ->
    //1 - openapi(a.k.a version 3.0.1), info
    //2 -  -> paths -->[ URLS/endpoints, httpMethod, responses (200, 201, 400, 500)]
    //3 - components [schema (typeObjects), fields/PropertiesUnderEach, ]

    Ok(open_api)
}
fn parse_yaml_open_api(text: &str) -> Result<(), ParserError> {
    Ok(())
}
///Use this to parse an OPENAPI specification type document
/// <br>It takes a string slice of the content of the OPEN Spec a.k.a swagger.json
fn parse_open_api_rest(text: &str) -> Result<Vec<Fixable>, ParserError> {
    let mut fixables: Vec<Fixable> = Vec::new();
    let mut final_core: u8 = 100; //final weight score...
    //two options - 1... we penalize them for each error...only from the total max obtainable
    //or 
    let mut final_data_validation_category_score: u8 = 70; 
    let mut final_security_category_score: u8 = 30;

    println!("Input Text: {}", text);
    //grabables
    let master_piece: Value = serde_json::from_str(text).map_err(|e| {
        ParserError::JSONParseError(format!("Error parsing Open API JSON spec: {}", e))
    })?;

    //get open api document...
    let document = parse_json_open_api(text)?;

    //INFO -> Nothing too serious => Just nameOfApi and Version

    //SERVERS -> Array [ {"url": ""} ]
    if let Some(server) = document.servers() {
        if server.is_empty() {
            modify_score(
                &mut final_security_category_score,
                IssueScoreImpact::NO_SERVER_BASE_URL_DEFINED_VALUE,
            );
            fixables.push(Fixable::new("Invalid server. You must provide a server BASEURL for your API. Read OpenAPI specification standards for more information", 0, WeightScore::Critical));
        }
    } else {
        modify_score(
            &mut final_security_category_score,
            IssueScoreImpact::NO_SERVER_BASE_URL_DEFINED_VALUE,
        );
        fixables.push(Fixable::new("Invalid server. You must provide a server BASEURL for your API. Read OpenAPI specification standards for more information", 0, WeightScore::Critical));
    }
    
    //PATHs -> a.k.a Endpoints -> Object -> has Other objects inside one for each endpoint
    // DATA validations...
    do_data_validations(&Document::OpenAPI(document.clone()), &mut final_data_validation_category_score, ApiSpecificationType::OpenApiRest)?;

    //COMPONENTS -> Logic Objects -> Request Objects' Schemas... containing fields, validation, regex, strings etc

    //SECURITY -> Array of Security Sechemes Objects -> Actually this part bears 30% of total score...

    println!("Fixables / Report {:?}", fixables);
    Ok(fixables)
}

fn do_data_validations(document: &Document, final_data_validation_category_score: &mut u8, spec_type: ApiSpecificationType) -> Result<Vec<Fixable>, ParserError> {
    let mut fixables: Vec<Fixable> = Vec::new();

    match spec_type {
        ApiSpecificationType::OpenApiRest => {
            match document{
                Document::OpenAPI(d) => {
                    if let Some(path) = Some(d.paths()){
                        println!("Displaying path info in console: {:?}", path);
                        println!("-------------------------------------------------\n");
                        for(url, path_item) in path{
                            println!("URL for this path item: {}", url);
                            println!("Path Item for this current URL: {:?}", path_item);
                            println!("--------------------------------------------------------------\n");
                            

                            if let Some(get_op) = path_item.get(){
                                // this ENDPOINT... has a GET Operation... 
                                //let's gather info about it...
                                let params = get_op.parameters();
                                if !params.is_empty(){
                                    
                                }

                            }
                            if let Some(post_op) = path_item.post(){
                                // this ENDPOINT... has a POST Operation... 
                                //let's gather info about it...
                            }



                        }
                    }
                }
                _ => return Err(ParserError::IncompatibleSpecificationAndDocumentType(spec_type.to_string(), document.to_string()))
            }
        },
        ApiSpecificationType::SoapWSDL => todo!(),
        _ => unimplemented!()
    }


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
