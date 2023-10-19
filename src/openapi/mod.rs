use std::collections::BTreeMap;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::parser::*;

const OPEN_API_INFO_STR: &str = "openapi";

#[derive(Debug, Clone)]
pub enum WeightScore {
    Low, // 1
    Medium, // 2
    High, // 3
    Critical, //5 or 4
}
impl WeightScore {
    pub const LOW_VALUE: u8 = 1;
    pub const MEDIUM_VALUE: u8 = 2;
    pub const HIGH_VALUE: u8 = 3;
    pub const CRITICAL_VALUE: u8 = 4;
}
#[derive(Debug, Clone)]
pub enum FixableType {
    Error,
    Warning, //ignorable...
}
impl From<WeightScore> for FixableType {
    fn from(value: WeightScore) -> Self {
        match value {
            WeightScore::Medium | WeightScore::Critical | WeightScore::High => {
                FixableType::Error
            }
            _ => FixableType::Warning,
        }
    }
}
#[derive(Debug)]
pub struct Fixable {
    error: String,
    line: u64,
    weight_score: WeightScore,
    fixable_type: FixableType,
}
impl Fixable {
    pub fn new(error: impl Into<String>, line: u64, weight_score: WeightScore) -> Self {
        let error_str = error.into();
        let fixable_type = weight_score.clone().into();

        Self {
            error: error_str,
            line,
            weight_score,
            fixable_type,
        }
    }
}

#[derive(Debug)]
pub struct Scanner {
    //open_api:  OpenApi,
    text: String,
    fixables: Vec<Fixable>,
}
impl Scanner {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            fixables: vec![],
        }
    }
    /// Scan an input TEXT for Application Vulnerabilities according to OWASP-Top-10
    pub fn scan(&mut self) -> Result<Vec<Fixable>, ParserError> {
        let mut parser = Parser::new(&self.text);

        let spec_type: ApiSpecificationType = guess_spec_type_from_text(&self.text);

        match parser.parse(&spec_type) {
            Ok(fixes) => Ok(fixes),
            Err(e) => Err(ParserError::ParseFailed(
                format!("error parsing {} type specification {}", spec_type, e).into(),
            )),
        }
    }
    /// Display the results/output of a scan in the CLI console or API caller in the future
    pub fn display(&mut self) -> anyhow::Result<()> {
        println!("{:?}", self);
        println!("Here are some discovered security vulnerabilities after scanning code against OWASP-top-10");
        println!("{:?}", self.fixables);

        Ok(())
    }
}

fn guess_spec_type_from_text(text: &str) -> ApiSpecificationType {
    match serde_json::from_str::<Value>(text) {
        Ok(value) => {
            //now search for the open_api info property...
            //openapi
            if let Some(_) = value.get(OPEN_API_INFO_STR) {
                ApiSpecificationType::OpenApiRest
            } else {
                println!("Unknown Api specification type detected. JSON is not a valid OpenAPI specification.");
                ApiSpecificationType::Unknown
            }
        }

        Err(e) => {
            //either it's not a JSON -> it's prolly an XML in case of WSDl...

            if is_wsdl_spec_v2(text).unwrap() {
                ApiSpecificationType::SoapWSDL
            } else {
                println!("Unknown Api specification type detected. XML is not a valid SOAP WSDL specification. {}", e);
                ApiSpecificationType::Unknown
            }
        }
    }
}






#[derive(Debug, Deserialize)]
pub struct PathItem {
    get: Option<Operation>,
    post: Option<Operation>,
    put: Option<Operation>,
    patch: Option<Operation>,
    delete: Option<Operation>,
    // Add fields for other HTTP methods as needed
}

#[derive(Debug, Deserialize)]
struct Operation {
    responses: BTreeMap<String, Response>,
    // Add other fields you need here
}

#[derive(Debug, Deserialize)]
struct Response {
    description: String,
    content: BTreeMap<String, Content>,
    // Add other fields as needed
}

#[derive(Debug, Deserialize)]
struct Content {
    schema: Schema,
}

#[derive(Debug, Deserialize)]
struct Schema {
    // You may need to define additional fields based on your schema structure
    #[serde(rename = "$ref")]
    reference: String,
}
