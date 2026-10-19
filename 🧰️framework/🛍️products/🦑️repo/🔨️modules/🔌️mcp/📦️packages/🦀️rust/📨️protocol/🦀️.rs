//! 📨️ Protocol: the bounded JSON-RPC contract: codes, identifiers, errors, capabilities and wire shapes.
//!
//! Holds the `📜️Protocol` region of the crate root; the root re-exports its items.

use super::*;

//#region 📜️Protocol

/// 🏷️ The JSON-RPC dialect version every frame carries.
pub const JSONRPC_VERSION: &str = "2.0";

/// 🗓️ The preferred protocol version, echoed whenever the client asks for something unsupported.
pub const PROTOCOL_VERSION: &str = "2025-11-25";

/// 🗓️ Every protocol version this server answers with unchanged.
pub const SUPPORTED_PROTOCOL_VERSIONS: [&str; 5] = [PROTOCOL_VERSION, "2025-06-18", "2025-03-26", "2024-11-05", "2024-10-07"];

/// 🤝️ Negotiates the response protocol version: a supported request is echoed, anything else falls back.
pub fn negotiate_protocol_version(requested: &str) -> &'static str {
    SUPPORTED_PROTOCOL_VERSIONS.into_iter().find(|supported| *supported == requested).unwrap_or(PROTOCOL_VERSION)
}

pub const CODE_PARSE_ERROR: i64 = -32700;
pub const CODE_INVALID_REQUEST: i64 = -32600;
pub const CODE_METHOD_NOT_FOUND: i64 = -32601;
pub const CODE_INVALID_PARAMS: i64 = -32602;
pub const CODE_INTERNAL_ERROR: i64 = -32603;
pub const CODE_PAYLOAD_TOO_LARGE: i64 = -32001;
pub const CODE_NOT_INITIALIZED: i64 = -32002;
pub const CODE_DUPLICATE_REQUEST: i64 = -32003;
pub const CODE_STALE_SESSION: i64 = -32004;
pub const CODE_SERVER_BUSY: i64 = -32005;
pub const CODE_REQUEST_CANCELLED: i64 = -32800;
pub const CODE_TOOL_ERROR: i64 = -32010;

/// 🚨️ Every failure this crate reports to its caller.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    PayloadTooLarge,
    NestingTooDeep,
    StaleSession,
    PeerDropped,
    Closed,
    Limit,
    Invalid(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::PayloadTooLarge => formatter.write_str("mcp: payload too large"),
            Error::NestingTooDeep => formatter.write_str("mcp: nesting too deep"),
            Error::StaleSession => formatter.write_str("mcp: stale session"),
            Error::PeerDropped => formatter.write_str("mcp: peer dropped"),
            Error::Closed => formatter.write_str("mcp: session closed"),
            Error::Limit => formatter.write_str("mcp: limit exceeded"),
            Error::Invalid(message) => write!(formatter, "mcp: {message}"),
        }
    }
}

impl std::error::Error for Error {}

/// 🔑️ A JSON-RPC request id. Only strings and integers are valid.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Id {
    Text(String),
    Number(i64),
}

impl Id {
    /// 🗝️ The stable dedup key, disjoint between the string and the numeric namespace.
    pub fn key(&self) -> String {
        match self {
            Id::Text(value) => format!("s:{value}"),
            Id::Number(value) => format!("n:{value}"),
        }
    }

    /// 📥️ Reads an id out of a parsed JSON value.
    pub fn from_value(value: &serde_json::Value) -> Option<Id> {
        match value {
            serde_json::Value::String(text) => Some(Id::Text(text.clone())),
            serde_json::Value::Number(number) => number.as_i64().map(Id::Number),
            _ => None,
        }
    }
}

/// 🚨️ A JSON-RPC error object.
#[derive(Clone, Debug, PartialEq)]
pub struct RpcError {
    pub code: i64,
    pub message: String,
    pub data: Option<String>,
}

impl RpcError {
    pub fn new(code: i64, message: &str) -> RpcError {
        RpcError { code, message: message.to_string(), data: None }
    }

    pub(super) fn json(&self) -> J {
        J::object(vec![
            ("code", Some(J::Int(self.code))),
            ("message", Some(J::Str(self.message.clone()))),
            ("data", self.data.as_ref().map(|data| J::Raw(data.clone()))),
        ])
    }
}

/// 🧰️ The failure a domain handler raises. Codes in `-32099..=-32000` reach the peer unchanged.
#[derive(Clone, Debug, PartialEq)]
pub struct HandlerError {
    pub code: i64,
    pub message: String,
    pub data: Option<String>,
}

impl HandlerError {
    pub fn tool(message: &str) -> HandlerError {
        HandlerError { code: CODE_TOOL_ERROR, message: message.to_string(), data: None }
    }
}

impl std::fmt::Display for HandlerError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

/// 📨️ A decoded JSON-RPC request. `params` keeps the raw member so decoding stays lazy and open.
#[derive(Clone, Debug)]
pub struct Request {
    pub jsonrpc: String,
    pub id: Option<Id>,
    pub id_raw: String,
    pub method: String,
    pub params: serde_json::Value,
}

/// 🪟️ Reads a `params` object. Unknown members are ignored — every params object is open for extension,
/// and clients legitimately send `clientInfo.title`, extra capability objects and `_meta` keys.
pub fn decode_params(params: &serde_json::Value) -> Result<&serde_json::Map<String, serde_json::Value>, Error> {
    match params {
        serde_json::Value::Null => Err(Error::Invalid("params must be an object".to_string())),
        serde_json::Value::Object(members) => Ok(members),
        _ => Err(Error::Invalid("params must be an object".to_string())),
    }
}

/// 🔒️ Reads a `tools/call` arguments object, rejecting every member the tool does not declare. Tool
/// arguments are the one closed surface of this protocol.
pub fn decode_arguments(arguments: &serde_json::Value, allowed: &[&str]) -> Result<(), HandlerError> {
    let members = match arguments {
        serde_json::Value::Object(members) => members,
        _ => return Err(HandlerError::tool("arguments must be an object")),
    };
    for name in members.keys() {
        if !allowed.contains(&name.as_str()) {
            return Err(HandlerError::tool("unknown argument"));
        }
    }
    Ok(())
}

/// 🧱️ A JSON Schema fragment describing one tool argument or one tool input object.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Schema {
    pub kind: String,
    pub description: String,
    pub properties: BTreeMap<String, Schema>,
    pub required: Vec<String>,
    pub items: Option<Box<Schema>>,
    pub additional_properties: Option<bool>,
    pub enumeration: Vec<String>,
}

impl Schema {
    pub fn string(description: &str) -> Schema {
        Schema { kind: "string".to_string(), description: description.to_string(), ..Schema::default() }
    }

    pub fn boolean(description: &str) -> Schema {
        Schema { kind: "boolean".to_string(), description: description.to_string(), ..Schema::default() }
    }

    pub fn array(description: &str) -> Schema {
        Schema { kind: "array".to_string(), description: description.to_string(), items: Some(Box::new(Schema { kind: "string".to_string(), ..Schema::default() })), ..Schema::default() }
    }

    pub fn object(properties: Vec<(&str, Schema)>, required: &[&str]) -> Schema {
        Schema {
            kind: "object".to_string(),
            properties: properties.into_iter().map(|(name, schema)| (name.to_string(), schema)).collect(),
            required: required.iter().map(|name| name.to_string()).collect(),
            additional_properties: Some(false),
            ..Schema::default()
        }
    }

    pub(super) fn json(&self) -> J {
        J::object(vec![
            ("type", (!self.kind.is_empty()).then(|| J::Str(self.kind.clone()))),
            ("description", (!self.description.is_empty()).then(|| J::Str(self.description.clone()))),
            ("properties", (!self.properties.is_empty()).then(|| J::Obj(self.properties.iter().map(|(name, schema)| (name.clone(), schema.json())).collect()))),
            ("required", (!self.required.is_empty()).then(|| J::Arr(self.required.iter().map(|name| J::Str(name.clone())).collect()))),
            ("items", self.items.as_ref().map(|items| items.json())),
            ("additionalProperties", self.additional_properties.map(J::Bool)),
            ("enum", (!self.enumeration.is_empty()).then(|| J::Arr(self.enumeration.iter().map(|value| J::Str(value.clone())).collect()))),
        ])
    }
}

/// 🧰️ One advertised tool.
#[derive(Clone, Debug, PartialEq)]
pub struct Tool {
    pub name: String,
    pub title: String,
    pub description: String,
    pub input_schema: Schema,
}

impl Tool {
    pub(super) fn json(&self) -> J {
        J::object(vec![
            ("name", Some(J::Str(self.name.clone()))),
            ("title", (!self.title.is_empty()).then(|| J::Str(self.title.clone()))),
            ("description", (!self.description.is_empty()).then(|| J::Str(self.description.clone()))),
            ("inputSchema", Some(self.input_schema.json())),
        ])
    }
}

/// 📚️ One advertised resource.
#[derive(Clone, Debug, PartialEq)]
pub struct Resource {
    pub uri: String,
    pub name: String,
    pub title: String,
    pub description: String,
    pub mime_type: String,
}

impl Resource {
    pub(super) fn json(&self) -> J {
        J::object(vec![
            ("uri", Some(J::Str(self.uri.clone()))),
            ("name", Some(J::Str(self.name.clone()))),
            ("title", (!self.title.is_empty()).then(|| J::Str(self.title.clone()))),
            ("description", (!self.description.is_empty()).then(|| J::Str(self.description.clone()))),
            ("mimeType", (!self.mime_type.is_empty()).then(|| J::Str(self.mime_type.clone()))),
        ])
    }
}

/// 🧭️ One advertised resource template.
#[derive(Clone, Debug, PartialEq)]
pub struct ResourceTemplate {
    pub uri_template: String,
    pub name: String,
    pub title: String,
    pub description: String,
    pub mime_type: String,
}

impl ResourceTemplate {
    pub(super) fn json(&self) -> J {
        J::object(vec![
            ("uriTemplate", Some(J::Str(self.uri_template.clone()))),
            ("name", Some(J::Str(self.name.clone()))),
            ("title", (!self.title.is_empty()).then(|| J::Str(self.title.clone()))),
            ("description", (!self.description.is_empty()).then(|| J::Str(self.description.clone()))),
            ("mimeType", (!self.mime_type.is_empty()).then(|| J::Str(self.mime_type.clone()))),
        ])
    }
}

/// 💬️ One advertised prompt argument.
#[derive(Clone, Debug, PartialEq)]
pub struct PromptArgument {
    pub name: String,
    pub description: String,
    pub required: bool,
}

/// 💬️ One advertised prompt.
#[derive(Clone, Debug, PartialEq)]
pub struct Prompt {
    pub name: String,
    pub title: String,
    pub description: String,
    pub arguments: Vec<PromptArgument>,
}

impl Prompt {
    pub(super) fn json(&self) -> J {
        J::object(vec![
            ("name", Some(J::Str(self.name.clone()))),
            ("title", (!self.title.is_empty()).then(|| J::Str(self.title.clone()))),
            ("description", (!self.description.is_empty()).then(|| J::Str(self.description.clone()))),
            (
                "arguments",
                (!self.arguments.is_empty()).then(|| {
                    J::Arr(
                        self.arguments
                            .iter()
                            .map(|argument| {
                                J::object(vec![
                                    ("name", Some(J::Str(argument.name.clone()))),
                                    ("description", (!argument.description.is_empty()).then(|| J::Str(argument.description.clone()))),
                                    ("required", argument.required.then_some(J::Bool(true))),
                                ])
                            })
                            .collect(),
                    )
                }),
            ),
        ])
    }
}

/// 🧩️ One content block of a tool result or a prompt message.
#[derive(Clone, Debug, PartialEq)]
pub struct Content {
    pub kind: String,
    pub text: String,
    pub data: String,
    pub mime_type: String,
    pub uri: String,
}

impl Content {
    pub fn text(text: &str) -> Content {
        Content { kind: "text".to_string(), text: text.to_string(), data: String::new(), mime_type: String::new(), uri: String::new() }
    }

    pub(super) fn json(&self) -> J {
        J::object(vec![
            ("type", Some(J::Str(self.kind.clone()))),
            ("text", self.blob.is_empty().then(|| J::Str(self.text.clone()))),
            ("data", (!self.data.is_empty()).then(|| J::Str(self.data.clone()))),
            ("mimeType", (!self.mime_type.is_empty()).then(|| J::Str(self.mime_type.clone()))),
            ("uri", (!self.uri.is_empty()).then(|| J::Str(self.uri.clone()))),
        ])
    }
}

/// 🧰️ What a tool handler returns.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CallToolResult {
    pub content: Vec<Content>,
    pub structured_content: Option<String>,
    pub is_error: bool,
}

impl CallToolResult {
    pub(super) fn json(&self) -> J {
        J::object(vec![
            ("content", Some(J::Arr(self.content.iter().map(Content::json).collect()))),
            ("structuredContent", self.structured_content.as_ref().map(|raw| J::Raw(raw.clone()))),
            ("isError", self.is_error.then_some(J::Bool(true))),
        ])
    }
}

/// 📚️ One resource body.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceContent {
    pub uri: String,
    pub mime_type: String,
    pub text: String,
    pub blob: String,
}

impl ResourceContent {
    pub(super) fn json(&self) -> J {
        J::object(vec![
            ("uri", Some(J::Str(self.uri.clone()))),
            ("mimeType", (!self.mime_type.is_empty()).then(|| J::Str(self.mime_type.clone()))),
            ("text", self.blob.is_empty().then(|| J::Str(self.text.clone()))),
            ("blob", (!self.blob.is_empty()).then(|| J::Str(self.blob.clone()))),
        ])
    }
}

/// 💬️ What a prompt handler returns.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GetPromptResult {
    pub description: String,
    pub messages: Vec<(String, Content)>,
}

impl GetPromptResult {
    pub(super) fn json(&self) -> J {
        J::object(vec![
            ("description", (!self.description.is_empty()).then(|| J::Str(self.description.clone()))),
            (
                "messages",
                Some(J::Arr(self.messages.iter().map(|(role, content)| J::Obj(vec![("role".to_string(), J::Str(role.clone())), ("content".to_string(), content.json())])).collect())),
            ),
        ])
    }
}

/// 📞️ The decoded `tools/call` params.
#[derive(Clone, Debug)]
pub struct CallToolParams {
    pub name: String,
    pub arguments: serde_json::Value,
    pub progress_token: Option<String>,
}

//#endregion 📜️Protocol
