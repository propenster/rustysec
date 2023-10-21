use std::collections::BTreeMap;
use std::collections::HashMap;

use oapi::OApi;
use oapi::OApiOperation;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::parser::*;

const OPEN_API_INFO_STR: &str = "openapi";

#[derive(Debug, Clone)]
pub enum WeightScore {
    Low,      // 1
    Medium,   // 2
    High,     // 3
    Critical, //5 or 4
}
pub enum IssueScoreImpact{
    ArrayWithoutMaxItems, //scoreImpact - 7
    StringPropertyWithoutMaxLength, // ScoreImpact - 2 [MEDIUM 2]
    StringPropertyWithoutREgexPattern, // scoreImpace - 3 [MEDIUM 3]
    NoServerBaseUrlDefined,
}


impl IssueScoreImpact{
    pub const ARRAY_WITHOUT_MAX_ITEMS_VALUE: u8 = 7;
    pub const STRING_PROPERTY_WITHOUT_MAX_LENGTH_VALUE: u8 = 2;
    pub const STRING_PROPERTY_WITHOUT_REGEX_PATTERN_VALUE: u8 = 3;

    pub const NO_SERVER_BASE_URL_DEFINED_VALUE: u8 = 10;

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
            WeightScore::Medium | WeightScore::Critical | WeightScore::High => FixableType::Error,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    description: String,
    content: HashMap<String, Schema>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    schema: SchemaRef,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SchemaRef {
    #[serde(rename = "$ref")]
    reference: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PathData {
    get: Option<Get>,
    post: Option<Post>,
    put: Option<Put>,
    patch: Option<Patch>,
    delete: Option<Delete>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Get {
    tags: Vec<String>,
    responses: HashMap<String, Response>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    tags: Vec<String>,
    operationId: String,
    requestBody: RequestBody,
    responses: HashMap<String, Response>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Put {
    tags: Vec<String>,
    operationId: String,
    requestBody: RequestBody,
    responses: HashMap<String, Response>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Patch {
    tags: Vec<String>,
    operationId: String,
    requestBody: RequestBody,
    responses: HashMap<String, Response>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Delete {
    tags: Vec<String>,
    operationId: String,
    requestBody: RequestBody,
    responses: HashMap<String, Response>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestBody {
    content: HashMap<String, Schema>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub title: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Components {
    pub schemas: HashMap<String, Schema>,
    pub security_schemes: HashMap<String, SecurityScheme>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ComponentSchema {
    pub required: Vec<String>,
    pub type_: String, // Note: "type" is a reserved keyword, so we use "type_" instead
    pub properties: HashMap<String, Property>,
    pub additional_properties: bool,
    // You may add other fields as needed
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Property {
    pub max_length: u32,
    pub min_length: u32,
    pub pattern: String,
    pub type_: String, // Again, use "type_" for the "type" field
                       // Add other properties
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SecurityScheme {
    pub type_: String, // Use "type_" for the "type" field
    pub description: String,
    pub scheme: String,
    pub bearer_format: String,
    // Add other fields
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SecurityRequirement {
    // The structure of this field may vary depending on your OpenAPI specification
    // For now, we'll use a HashMap, but you should adapt it to your specific schema
    #[serde(flatten)]
    pub security: std::collections::HashMap<String, Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenApi {
    pub openapi: Option<String>,
    pub info: Option<Info>,
    pub servers: Option<Vec<Server>>,
    pub paths: Option<HashMap<String, PathData>>,
    pub components: Option<Components>,
    pub security: Option<SecurityRequirement>,
}

/// <p>This is custom OpenApiDoc object...</p>
/// <p>We might expose this as a library later on</p>
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenApiDoc {
    pub openapi: Option<String>,
    pub info: Option<Info>,
    pub servers: Option<Vec<Server>>,
    pub paths: Option<HashMap<String, PathData>>,
}
impl OpenApiDoc {
    pub fn new(version: impl Into<String>) -> Self {
        Self {
            openapi: Some(version.into()),
            info: None,
            servers: None,
            paths: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndpointPath {
    pub url: String,
    pub tags: Vec<String>,
    pub responses: Option<Vec<EndpointResponse>>,
}
impl EndpointPath {
    pub(crate) fn new(url: &str) -> EndpointPath {
        Self {
            url: String::from(url),
            tags: vec![],
            responses: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// enum for Response Schema...
pub enum EndpointResponse {
    /// code, description, schema
    Http200OK(String, String, String),
    Http201Created(String, String, String),
    Http400BadRequest(String, String, String),
    Http401Unauthorized(String, String, String),
    Http403Forbidden(String, String, String),
    Http500InternalServerError(String, String, String),
}

