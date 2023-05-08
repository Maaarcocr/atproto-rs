use std::collections::HashMap;

use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum LexBoolean {
    #[serde(rename_all = "camelCase")]
    Boolean {
        description: Option<String>,
        default: Option<bool>,
        #[serde(rename = "const")]
        r#const: Option<bool>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum LexInteger {
    #[serde(rename_all = "camelCase")]
    Integer {
        description: Option<String>,
        default: Option<i64>, // Use i64 to represent integer numbers in Rust
        minimum: Option<i64>,
        maximum: Option<i64>,
        #[serde(rename = "enum")]
        enum_values: Option<Vec<i64>>, // `enum` is a reserved keyword in Rust, so we use `enum_values` instead
        const_value: Option<i64>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub enum LexStringFormat {
    #[serde(rename = "datetime")]
    Datetime,
    #[serde(rename = "uri")]
    Uri,
    #[serde(rename = "at-uri")]
    AtUri,
    #[serde(rename = "did")]
    Did,
    #[serde(rename = "handle")]
    Handle,
    #[serde(rename = "at-identifier")]
    AtIdentifier,
    #[serde(rename = "nsid")]
    Nsid,
    #[serde(rename = "cid")]
    Cid,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum LexString {
    #[serde(rename_all = "camelCase")]
    String {
        format: Option<LexStringFormat>,
        description: Option<String>,
        default: Option<String>,
        min_length: Option<i64>,
        max_length: Option<i64>,
        min_graphemes: Option<i64>,
        max_graphemes: Option<i64>,
        #[serde(rename = "enum")]
        enum_values: Option<Vec<String>>, // `enum` is a reserved keyword in Rust, so we use `enum_values` instead
        const_value: Option<String>,
        known_values: Option<Vec<String>>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum LexUnknown {
    #[serde(rename_all = "camelCase")]
    Unknown { description: Option<String> },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum LexPrimitive {
    Boolean(LexBoolean),
    Integer(LexInteger),
    String(LexString),
    Unknown(LexUnknown),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum LexBytes {
    #[serde(rename_all = "camelCase")]
    Bytes {
        description: Option<String>,
        default: Option<String>,
        #[serde(rename = "const")]
        r#const: Option<String>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum LexCidLink {
    #[serde(rename_all = "camelCase")]
    CidLink { description: Option<String> },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum LexIpldType {
    Bytes(LexBytes),
    CidLink(LexCidLink),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum LexRef {
    #[serde(rename_all = "camelCase")]
    Ref {
        description: Option<String>,
        #[serde(rename = "ref")]
        r#ref: String, // Note: Rename `ref` field to avoid clash with Rust keyword
    },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum LexRefUnion {
    #[serde(rename_all = "camelCase")]
    Union {
        description: Option<String>,
        refs: Vec<String>,
        closed: Option<bool>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum LexRefVariant {
    Ref(LexRef),
    RefUnion(LexRefUnion),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum LexBlob {
    #[serde(rename_all = "camelCase")]
    Blob {
        description: Option<String>,
        accept: Option<Vec<String>>,
        max_size: Option<i64>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum LexArray {
    #[serde(rename_all = "camelCase")]
    Array {
        description: Option<String>,
        items: LexItems, // Use a Box to represent the recursive structure of the `items` field
        min_length: Option<i64>, // Rename fields to conform with Rust naming conventions
        max_length: Option<i64>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum LexItems {
    Primitive(LexPrimitive),
    IpldType(LexIpldType),
    Blob(LexBlob),
    RefVariant(LexRefVariant),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum LexPrimitiveArray {
    #[serde(rename_all = "camelCase")]
    Array {
        description: Option<String>,
        items: LexPrimitive, // Use a Box to represent the recursive structure of the `items` field
        min_length: Option<i64>, // Rename fields to conform with Rust naming conventions
        max_length: Option<i64>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum LexToken {
    #[serde(rename_all = "camelCase")]
    Token { description: Option<String> },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum LexObject {
    #[serde(rename_all = "camelCase")]
    Object {
        description: Option<String>,
        required: Option<Vec<String>>,
        nullable: Option<Vec<String>>,
        properties: Option<HashMap<String, LexProperty>>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum LexProperty {
    RefVariant(LexRefVariant),
    IpldType(LexIpldType),
    Array(LexArray),
    Blob(LexBlob),
    Primitive(LexPrimitive),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum LexXrpcParameters {
    #[serde(rename_all = "camelCase")]
    Params {
        description: Option<String>,
        required: Option<Vec<String>>,
        properties: HashMap<String, LexXrpcParameter>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum LexXrpcParameter {
    Primitive(LexPrimitive),
    PrimitiveArray(LexPrimitiveArray),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LexXrpcBody {
    description: Option<String>,
    encoding: String,
    schema: Option<LexXrpcBodySchema>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum LexXrpcBodySchema {
    RefVariant(LexRefVariant),
    Object(LexObject),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LexXrpcSubscriptionMessage {
    description: Option<String>,
    schema: Option<LexXrpcSubscriptionMessageSchema>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum LexXrpcSubscriptionMessageSchema {
    RefVariant(LexRefVariant),
    Object(LexObject),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LexXrpcError {
    name: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum LexXrpcQuery {
    #[serde(rename_all = "camelCase")]
    Query {
        description: Option<String>,
        parameters: Option<LexXrpcParameters>,
        output: Option<LexXrpcBody>,
        errors: Option<Vec<LexXrpcError>>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum LexXrpcProcedure {
    Procedure {
        description: Option<String>,
        parameters: Option<LexXrpcParameters>,
        input: Option<LexXrpcBody>,
        output: Option<LexXrpcBody>,
        errors: Option<Vec<LexXrpcError>>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum LexXrpcSubscription {
    #[serde(rename_all = "camelCase")]
    Subscription {
        description: Option<String>,
        parameters: Option<LexXrpcParameters>,
        message: Option<LexXrpcSubscriptionMessage>,
        infos: Option<Vec<LexXrpcError>>,
        errors: Option<Vec<LexXrpcError>>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum LexRecord {
    #[serde(rename_all = "camelCase")]
    Record {
        description: Option<String>,
        key: Option<String>,
        record: LexObject,
    },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum LexUserType {
    Record(LexRecord),
    XrpcQuery(LexXrpcQuery),
    XrpcProcedure(LexXrpcProcedure),
    XrpcSubscription(LexXrpcSubscription),
    Blob(LexBlob),
    Array(LexArray),
    Token(LexToken),
    Object(LexObject),
    Boolean(LexPrimitive),
    Integer(LexPrimitive),
    String(LexPrimitive),
    Bytes(LexPrimitive),
    CidLink(LexPrimitive),
    Unknown(LexPrimitive),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LexiconDoc {
    pub lexicon: u8,
    pub id: String,
    pub revision: Option<f64>,
    pub description: Option<String>,
    pub defs: HashMap<String, LexUserType>,
}

pub fn parse_lexicon_doc(doc: &str) -> Result<LexiconDoc, serde_json::Error> {
    serde_json::from_str(doc)
}