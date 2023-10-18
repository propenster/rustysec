use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::parser::*;

const OPEN_API_INFO_STR: &str = "openapi";

#[derive(Debug)]
pub enum WeightScore {
    Minimum(i64),
    Medium(i64),
    High(i64),
    Critical(i64),
}
#[derive(Debug)]
pub enum FixableType{
    Error,
    Warning, //ignorable...
}
#[derive(Debug)]
pub struct Fixable {
    error: String,
    line: u64,
    weight_score: WeightScore,
    fixable_type: FixableType,
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
    // pub fn from_text(open_api_string: &str) -> Self {
    //     let obj: OpenApi =
    //         serde_json::from_str(open_api_string).expect("Failed to parse open api json file");
    //     Scanner {
    //         open_api: obj,
    //         fixables: vec![],
    //     }
    // }
    pub fn scan(&mut self) -> Result<Vec<Fixable>, ParserError> {
        let mut fixables: Vec<Fixable> = vec![];

        let mut parser = Parser::new(&self.text);

        let spec_type: ApiSpecificationType = guess_spec_type_from_text(&self.text);

        match parser.parse(&spec_type) {
            Ok(fixes) => Ok(fixes),
            Err(e) => Err(ParserError::ParseFailed(format!(
                "error parsing {} type specification {}",
                spec_type,
                e
            ).into())),
        }  
        
    }
    pub fn display(&mut self) {
        println!("{:?}", self);
        println!("Here are some discovered security vulnerabilities after scanning code against OWASP-top-10");
        println!("{:?}", self.fixables);
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

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenApi {
    pub openapi: Option<String>,
    pub info: Option<Info>,
    pub servers: Option<Vec<Server>>,
    pub paths: Option<Paths>,
    pub components: Option<Components>,
    pub security: Option<Vec<Security>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub title: String,
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paths {
    #[serde(rename = "/WeatherForecast/mangoes/all")]
    pub weather_forecast_mangoes_all: WeatherForecastMangoesAll,
    #[serde(rename = "/WeatherForecast/mangoes")]
    pub weather_forecast_mangoes: WeatherForecastMangoes,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherForecastMangoesAll {
    pub get: Get,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Get {
    pub tags: Vec<String>,
    pub responses: Responses,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Responses {
    #[serde(rename = "200")]
    pub n200: n200,
    #[serde(rename = "401")]
    pub n401: n401,
    #[serde(rename = "404")]
    pub n404: n404,
    #[serde(rename = "406")]
    pub n406: n406,
    #[serde(rename = "429")]
    pub n429: n429,
    #[serde(rename = "403")]
    pub n403: n403,
    #[serde(rename = "415")]
    pub n415: n415,
    pub default: Default,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n200 {
    pub description: String,
    pub content: Content,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    #[serde(rename = "text/plain")]
    pub text_plain: TextPlain,
    #[serde(rename = "application/json")]
    pub application_json: ApplicationJson,
    #[serde(rename = "text/json")]
    pub text_json: TextJson,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextPlain {
    pub schema: Schema,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationJson {
    pub schema: Schema2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema2 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextJson {
    pub schema: Schema3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema3 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n401 {
    pub description: String,
    pub content: Content2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content2 {
    #[serde(rename = "text/plain")]
    pub text_plain: TextPlain2,
    #[serde(rename = "application/json")]
    pub application_json: ApplicationJson2,
    #[serde(rename = "text/json")]
    pub text_json: TextJson2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextPlain2 {
    pub schema: Schema4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema4 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationJson2 {
    pub schema: Schema5,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema5 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextJson2 {
    pub schema: Schema6,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema6 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n404 {
    pub description: String,
    pub content: Content3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content3 {
    #[serde(rename = "text/plain")]
    pub text_plain: TextPlain3,
    #[serde(rename = "application/json")]
    pub application_json: ApplicationJson3,
    #[serde(rename = "text/json")]
    pub text_json: TextJson3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextPlain3 {
    pub schema: Schema7,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema7 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationJson3 {
    pub schema: Schema8,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema8 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextJson3 {
    pub schema: Schema9,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema9 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n406 {
    pub description: String,
    pub content: Content4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content4 {
    #[serde(rename = "text/plain")]
    pub text_plain: TextPlain4,
    #[serde(rename = "application/json")]
    pub application_json: ApplicationJson4,
    #[serde(rename = "text/json")]
    pub text_json: TextJson4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextPlain4 {
    pub schema: Schema10,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema10 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationJson4 {
    pub schema: Schema11,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema11 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextJson4 {
    pub schema: Schema12,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema12 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n429 {
    pub description: String,
    pub content: Content5,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content5 {
    #[serde(rename = "text/plain")]
    pub text_plain: TextPlain5,
    #[serde(rename = "application/json")]
    pub application_json: ApplicationJson5,
    #[serde(rename = "text/json")]
    pub text_json: TextJson5,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextPlain5 {
    pub schema: Schema13,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema13 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationJson5 {
    pub schema: Schema14,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema14 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextJson5 {
    pub schema: Schema15,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema15 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n403 {
    pub description: String,
    pub content: Content6,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content6 {
    #[serde(rename = "text/plain")]
    pub text_plain: TextPlain6,
    #[serde(rename = "application/json")]
    pub application_json: ApplicationJson6,
    #[serde(rename = "text/json")]
    pub text_json: TextJson6,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextPlain6 {
    pub schema: Schema16,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema16 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationJson6 {
    pub schema: Schema17,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema17 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextJson6 {
    pub schema: Schema18,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema18 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n415 {
    pub description: String,
    pub content: Content7,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content7 {
    #[serde(rename = "text/plain")]
    pub text_plain: TextPlain7,
    #[serde(rename = "application/json")]
    pub application_json: ApplicationJson7,
    #[serde(rename = "text/json")]
    pub text_json: TextJson7,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextPlain7 {
    pub schema: Schema19,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema19 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationJson7 {
    pub schema: Schema20,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema20 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextJson7 {
    pub schema: Schema21,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema21 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Default {
    pub description: String,
    pub content: Content8,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content8 {
    #[serde(rename = "text/plain")]
    pub text_plain: TextPlain8,
    #[serde(rename = "application/json")]
    pub application_json: ApplicationJson8,
    #[serde(rename = "text/json")]
    pub text_json: TextJson8,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextPlain8 {
    pub schema: Schema22,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema22 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationJson8 {
    pub schema: Schema23,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema23 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextJson8 {
    pub schema: Schema24,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema24 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherForecastMangoes {
    pub post: Post,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub tags: Vec<String>,
    pub operation_id: String,
    pub request_body: RequestBody,
    pub responses: Responses2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestBody {
    pub content: Content9,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content9 {
    #[serde(rename = "application/json")]
    pub application_json: ApplicationJson9,
    #[serde(rename = "text/json")]
    pub text_json: TextJson9,
    #[serde(rename = "application/*+json")]
    pub application_json2: ApplicationJson10,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationJson9 {
    pub schema: Schema25,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema25 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextJson9 {
    pub schema: Schema26,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema26 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationJson10 {
    pub schema: Schema27,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema27 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Responses2 {
    #[serde(rename = "200")]
    pub n200: n2002,
    #[serde(rename = "401")]
    pub n401: n4012,
    #[serde(rename = "404")]
    pub n404: n4042,
    #[serde(rename = "406")]
    pub n406: n4062,
    #[serde(rename = "429")]
    pub n429: n4292,
    #[serde(rename = "403")]
    pub n403: n4032,
    #[serde(rename = "415")]
    pub n415: n4152,
    pub default: Default2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n2002 {
    pub description: String,
    pub content: Content10,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content10 {
    #[serde(rename = "text/plain")]
    pub text_plain: TextPlain9,
    #[serde(rename = "application/json")]
    pub application_json: ApplicationJson11,
    #[serde(rename = "text/json")]
    pub text_json: TextJson10,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextPlain9 {
    pub schema: Schema28,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema28 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationJson11 {
    pub schema: Schema29,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema29 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextJson10 {
    pub schema: Schema30,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema30 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n4012 {
    pub description: String,
    pub content: Content11,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content11 {
    #[serde(rename = "text/plain")]
    pub text_plain: TextPlain10,
    #[serde(rename = "application/json")]
    pub application_json: ApplicationJson12,
    #[serde(rename = "text/json")]
    pub text_json: TextJson11,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextPlain10 {
    pub schema: Schema31,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema31 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationJson12 {
    pub schema: Schema32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema32 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextJson11 {
    pub schema: Schema33,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema33 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n4042 {
    pub description: String,
    pub content: Content12,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content12 {
    #[serde(rename = "text/plain")]
    pub text_plain: TextPlain11,
    #[serde(rename = "application/json")]
    pub application_json: ApplicationJson13,
    #[serde(rename = "text/json")]
    pub text_json: TextJson12,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextPlain11 {
    pub schema: Schema34,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema34 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationJson13 {
    pub schema: Schema35,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema35 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextJson12 {
    pub schema: Schema36,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema36 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n4062 {
    pub description: String,
    pub content: Content13,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content13 {
    #[serde(rename = "text/plain")]
    pub text_plain: TextPlain12,
    #[serde(rename = "application/json")]
    pub application_json: ApplicationJson14,
    #[serde(rename = "text/json")]
    pub text_json: TextJson13,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextPlain12 {
    pub schema: Schema37,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema37 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationJson14 {
    pub schema: Schema38,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema38 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextJson13 {
    pub schema: Schema39,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema39 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n4292 {
    pub description: String,
    pub content: Content14,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content14 {
    #[serde(rename = "text/plain")]
    pub text_plain: TextPlain13,
    #[serde(rename = "application/json")]
    pub application_json: ApplicationJson15,
    #[serde(rename = "text/json")]
    pub text_json: TextJson14,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextPlain13 {
    pub schema: Schema40,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema40 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationJson15 {
    pub schema: Schema41,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema41 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextJson14 {
    pub schema: Schema42,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema42 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n4032 {
    pub description: String,
    pub content: Content15,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content15 {
    #[serde(rename = "text/plain")]
    pub text_plain: TextPlain14,
    #[serde(rename = "application/json")]
    pub application_json: ApplicationJson16,
    #[serde(rename = "text/json")]
    pub text_json: TextJson15,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextPlain14 {
    pub schema: Schema43,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema43 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationJson16 {
    pub schema: Schema44,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema44 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextJson15 {
    pub schema: Schema45,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema45 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n4152 {
    pub description: String,
    pub content: Content16,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content16 {
    #[serde(rename = "text/plain")]
    pub text_plain: TextPlain15,
    #[serde(rename = "application/json")]
    pub application_json: ApplicationJson17,
    #[serde(rename = "text/json")]
    pub text_json: TextJson16,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextPlain15 {
    pub schema: Schema46,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema46 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationJson17 {
    pub schema: Schema47,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema47 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextJson16 {
    pub schema: Schema48,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema48 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Default2 {
    pub description: String,
    pub content: Content17,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content17 {
    #[serde(rename = "text/plain")]
    pub text_plain: TextPlain16,
    #[serde(rename = "application/json")]
    pub application_json: ApplicationJson18,
    #[serde(rename = "text/json")]
    pub text_json: TextJson17,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextPlain16 {
    pub schema: Schema49,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema49 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationJson18 {
    pub schema: Schema50,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema50 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextJson17 {
    pub schema: Schema51,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema51 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Components {
    pub schemas: Schemas,
    pub security_schemes: SecuritySchemes,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schemas {
    #[serde(rename = "DefaultErrorResponse")]
    pub default_error_response: DefaultErrorResponse,
    #[serde(rename = "MangoDto")]
    pub mango_dto: MangoDto,
    #[serde(rename = "MangoResponse")]
    pub mango_response: MangoResponse,
    #[serde(rename = "MangoesResponse")]
    pub mangoes_response: MangoesResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultErrorResponse {
    pub required: Vec<String>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties,
    pub additional_properties: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties {
    pub error: Error,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    pub max_length: i64,
    pub min_length: i64,
    pub pattern: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MangoDto {
    pub required: Vec<String>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties2,
    pub additional_properties: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties2 {
    pub name: Name,
    pub description: Description,
    pub membership_id: MembershipId,
    pub amount: Amount,
    pub serial_no: SerialNo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    pub max_length: i64,
    pub min_length: i64,
    pub pattern: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description {
    pub max_length: i64,
    pub min_length: i64,
    pub pattern: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MembershipId {
    pub max_length: i64,
    pub min_length: i64,
    pub pattern: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Amount {
    pub maximum: i64,
    pub minimum: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub format: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SerialNo {
    pub maximum: i64,
    pub minimum: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub format: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MangoResponse {
    pub required: Vec<String>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties3,
    pub additional_properties: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties3 {
    pub mango_id: MangoId,
    pub name: Name2,
    pub description: Description2,
    pub amount: Amount2,
    pub serial_no: SerialNo2,
    pub created_at: CreatedAt,
    pub last_updated_at: LastUpdatedAt,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MangoId {
    pub max_length: i64,
    pub min_length: i64,
    pub pattern: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Name2 {
    pub max_length: i64,
    pub min_length: i64,
    pub pattern: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description2 {
    pub max_length: i64,
    pub min_length: i64,
    pub pattern: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Amount2 {
    pub maximum: i64,
    pub minimum: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub format: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SerialNo2 {
    pub maximum: i64,
    pub minimum: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub format: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatedAt {
    pub max_length: i64,
    pub min_length: i64,
    pub pattern: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastUpdatedAt {
    pub max_length: i64,
    pub min_length: i64,
    pub pattern: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MangoesResponse {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties4,
    pub additional_properties: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties4 {
    pub mangoes: Mangoes,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mangoes {
    pub max_items: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub items: Items,
    pub nullable: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Items {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecuritySchemes {
    #[serde(rename = "Bearer")]
    pub bearer: Bearer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bearer {
    #[serde(rename = "type")]
    pub type_field: String,
    pub description: String,
    pub scheme: String,
    pub bearer_format: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Security {
    #[serde(rename = "Bearer")]
    pub bearer: Vec<Value>,
}
