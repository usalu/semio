//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

//! 🔌️ The repo Model Context Protocol server: the bounded JSON-RPC contract, the hash-chained event
//! log, the line-delimited stdio transport, the session state machine, request routing and the
//! repository tool/resource/prompt surface. Domain behaviour lives behind [`Repository`]; this crate
//! owns the wire and nothing else. The contract it implements is
//! `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/🧬️schema/🔣️.json`.

//#endregion 🧲️Header

//#region 🔏️Digest

const SHA256_K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6,
    0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb,
    0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3, 0x748f82ee,
    0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

/// 🔏️ Hand-rolled SHA-256 (FIPS 180-4). The event chain must not depend on an external crate, so the
/// digest is owned here and cross-checked against Node's `crypto` in the language-agnostic tests.
pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut state: [u32; 8] = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19];
    let mut padded = data.to_vec();
    let bits = (data.len() as u64).wrapping_mul(8);
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&bits.to_be_bytes());
    let mut schedule = [0u32; 64];
    for block in padded.as_chunks::<64>().0 {
        for index in 0..16 {
            schedule[index] = u32::from_be_bytes([block[index * 4], block[index * 4 + 1], block[index * 4 + 2], block[index * 4 + 3]]);
        }
        for index in 16..64 {
            let previous = schedule[index - 15];
            let ahead = schedule[index - 2];
            let sigma0 = previous.rotate_right(7) ^ previous.rotate_right(18) ^ (previous >> 3);
            let sigma1 = ahead.rotate_right(17) ^ ahead.rotate_right(19) ^ (ahead >> 10);
            schedule[index] = schedule[index - 16].wrapping_add(sigma0).wrapping_add(schedule[index - 7]).wrapping_add(sigma1);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = state;
        for index in 0..64 {
            let big1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let choose = (e & f) ^ ((!e) & g);
            let temp1 = h.wrapping_add(big1).wrapping_add(choose).wrapping_add(SHA256_K[index]).wrapping_add(schedule[index]);
            let big0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let majority = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = big0.wrapping_add(majority);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }
        for (slot, value) in state.iter_mut().zip([a, b, c, d, e, f, g, h]) {
            *slot = slot.wrapping_add(value);
        }
    }
    let mut digest = [0u8; 32];
    for (index, word) in state.iter().enumerate() {
        digest[index * 4..index * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    digest
}

/// 🔤️ Lowercase hex rendering of [`sha256`].
pub fn sha256_hex(data: &[u8]) -> String {
    let mut text = String::with_capacity(64);
    for byte in sha256(data) {
        text.push(char::from_digit((byte >> 4) as u32, 16).unwrap_or('0'));
        text.push(char::from_digit((byte & 0x0f) as u32, 16).unwrap_or('0'));
    }
    text
}

//#endregion 🔏️Digest

//#region 🧾️Json

use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, OnceLock};

/// 🧾️ An ordered JSON value written exactly the way Go's `encoding/json` writes it, so both
/// implementations produce byte-identical wire frames and byte-identical event hashes.
#[derive(Clone, Debug, PartialEq)]
pub enum J {
    Null,
    Bool(bool),
    Int(i64),
    UInt(u64),
    Float(f64),
    Str(String),
    Arr(Vec<J>),
    Obj(Vec<(String, J)>),
    Raw(String),
}

impl J {
    /// 🏷️ Builds an object from already-ordered members, dropping the ones a Go `omitempty` tag would omit.
    pub fn object(members: Vec<(&str, Option<J>)>) -> J {
        J::Obj(members.into_iter().filter_map(|(name, value)| value.map(|value| (name.to_string(), value))).collect())
    }

    /// ✍️ Renders the value into Go's exact output form.
    pub fn write(&self, out: &mut String) {
        match self {
            J::Null => out.push_str("null"),
            J::Bool(value) => out.push_str(if *value { "true" } else { "false" }),
            J::Int(value) => out.push_str(&value.to_string()),
            J::UInt(value) => out.push_str(&value.to_string()),
            J::Float(value) => out.push_str(&go_float(*value)),
            J::Str(value) => go_string(value, out),
            J::Arr(items) => {
                out.push('[');
                for (index, item) in items.iter().enumerate() {
                    if index > 0 {
                        out.push(',');
                    }
                    item.write(out);
                }
                out.push(']');
            }
            J::Obj(members) => {
                out.push('{');
                for (index, (name, value)) in members.iter().enumerate() {
                    if index > 0 {
                        out.push(',');
                    }
                    go_string(name, out);
                    out.push(':');
                    value.write(out);
                }
                out.push('}');
            }
            J::Raw(text) => out.push_str(text),
        }
    }

    /// 📤️ Renders the value into a fresh string.
    pub fn text(&self) -> String {
        let mut out = String::new();
        self.write(&mut out);
        out
    }

    /// 🔁️ Adopts an arbitrary parsed value, sorting object keys the way Go sorts map keys.
    pub fn from_value(value: &serde_json::Value) -> J {
        match value {
            serde_json::Value::Null => J::Null,
            serde_json::Value::Bool(inner) => J::Bool(*inner),
            serde_json::Value::Number(number) => J::Raw(number.to_string()),
            serde_json::Value::String(inner) => J::Str(inner.clone()),
            serde_json::Value::Array(items) => J::Arr(items.iter().map(J::from_value).collect()),
            serde_json::Value::Object(members) => J::Obj(members.iter().map(|(name, value)| (name.clone(), J::from_value(value))).collect()),
        }
    }
}

/// 🔢️ Formats a float the way Go's encoder does for the magnitudes this protocol carries.
fn go_float(value: f64) -> String {
    if value.is_finite() && value.fract() == 0.0 && value.abs() < 1e21 {
        return format!("{}", value as i64);
    }
    let text = format!("{value}");
    if text.contains('e') {
        return text.replace('e', "e+").replace("e+-", "e-");
    }
    text
}

/// 🔤️ Escapes a string exactly as Go's `encoding/json` does, HTML escaping included.
fn go_string(value: &str, out: &mut String) {
    out.push('"');
    for character in value.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '<' => out.push_str("\\u003c"),
            '>' => out.push_str("\\u003e"),
            '&' => out.push_str("\\u0026"),
            '\u{2028}' => out.push_str("\\u2028"),
            '\u{2029}' => out.push_str("\\u2029"),
            other if (other as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", other as u32)),
            other => out.push(other),
        }
    }
    out.push('"');
}

/// 🗜️ Strips insignificant whitespace and applies Go's HTML escaping to an already-valid JSON document.
pub fn compact(raw: &[u8]) -> String {
    let text = String::from_utf8_lossy(raw);
    let mut out = String::with_capacity(text.len());
    let mut in_string = false;
    let mut escaped = false;
    for character in text.chars() {
        if in_string {
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == '"' {
                in_string = false;
            }
        } else if character == '"' {
            in_string = true;
        } else if character == ' ' || character == '\t' || character == '\n' || character == '\r' {
            continue;
        }
        match character {
            '<' => out.push_str("\\u003c"),
            '>' => out.push_str("\\u003e"),
            '&' => out.push_str("\\u0026"),
            '\u{2028}' => out.push_str("\\u2028"),
            '\u{2029}' => out.push_str("\\u2029"),
            other => out.push(other),
        }
    }
    out
}

//#endregion 🧾️Json

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

    fn json(&self) -> J {
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

    fn json(&self) -> J {
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
    fn json(&self) -> J {
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
    fn json(&self) -> J {
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
    fn json(&self) -> J {
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
    fn json(&self) -> J {
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

    fn json(&self) -> J {
        J::object(vec![
            ("type", Some(J::Str(self.kind.clone()))),
            ("text", (!self.text.is_empty()).then(|| J::Str(self.text.clone()))),
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
    fn json(&self) -> J {
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
    fn json(&self) -> J {
        J::object(vec![
            ("uri", Some(J::Str(self.uri.clone()))),
            ("mimeType", (!self.mime_type.is_empty()).then(|| J::Str(self.mime_type.clone()))),
            ("text", (!self.text.is_empty()).then(|| J::Str(self.text.clone()))),
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
    fn json(&self) -> J {
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

//#region 📡️Event

/// 🏷️ The schema identifier every event record carries.
pub const EVENT_SCHEMA: &str = "semio.mcp.event/1";

/// 🔗️ One hash-chained event record.
#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    pub schema: String,
    pub sequence: u64,
    pub kind: String,
    pub peer: String,
    pub generation: u64,
    pub request_id: String,
    pub payload: String,
    pub previous: String,
    pub hash: String,
}

impl Event {
    /// ✍️ Renders the record exactly as the Go implementation does, so both chains agree bit for bit.
    pub fn text(&self, hash: &str) -> String {
        let mut out = String::new();
        out.push_str("{\"schema\":");
        go_string(&self.schema, &mut out);
        out.push_str(",\"sequence\":");
        out.push_str(&self.sequence.to_string());
        out.push_str(",\"kind\":");
        go_string(&self.kind, &mut out);
        out.push_str(",\"peer\":");
        go_string(&self.peer, &mut out);
        out.push_str(",\"generation\":");
        out.push_str(&self.generation.to_string());
        if !self.request_id.is_empty() {
            out.push_str(",\"requestId\":");
            go_string(&self.request_id, &mut out);
        }
        out.push_str(",\"payload\":");
        out.push_str(&self.payload);
        if !self.previous.is_empty() {
            out.push_str(",\"previous\":");
            go_string(&self.previous, &mut out);
        }
        out.push_str(",\"hash\":");
        go_string(hash, &mut out);
        out.push('}');
        out
    }

    /// 🔏️ The chain digest of this record, taken over its own rendering with an empty `hash`.
    pub fn digest(&self) -> String {
        sha256_hex(self.text("").as_bytes())
    }
}

/// 📥️ What a caller commits into the log.
#[derive(Clone, Debug)]
pub struct EventInput {
    pub kind: String,
    pub peer: String,
    pub generation: u64,
    pub request_id: String,
    pub payload: String,
}

impl EventInput {
    pub fn new(kind: &str, peer: &str, generation: u64, payload: String) -> EventInput {
        EventInput { kind: kind.to_string(), peer: peer.to_string(), generation, request_id: String::new(), payload }
    }
}

/// 📚️ The append-only, hash-chained, bounded event log.
#[derive(Debug, Default)]
pub struct EventLog {
    state: Mutex<EventLogState>,
    max_bytes: usize,
    max_count: usize,
}

#[derive(Debug, Default)]
struct EventLogState {
    events: Vec<Event>,
    data: String,
}

impl EventLog {
    pub fn new(max_bytes: usize, max_count: usize) -> EventLog {
        EventLog { state: Mutex::new(EventLogState::default()), max_bytes, max_count }
    }

    /// ✅️ Appends a whole batch or nothing at all.
    pub fn commit(&self, inputs: &[EventInput]) -> Result<(), Error> {
        if inputs.is_empty() {
            return Ok(());
        }
        let mut state = self.state.lock().unwrap_or_else(|poison| poison.into_inner());
        if self.max_count > 0 && state.events.len() + inputs.len() > self.max_count {
            return Err(Error::Limit);
        }
        let mut previous = state.events.last().map(|event| event.hash.clone()).unwrap_or_default();
        let mut staged_events = Vec::with_capacity(inputs.len());
        let mut staged = String::new();
        for (index, input) in inputs.iter().enumerate() {
            if input.kind.is_empty() || serde_json::from_str::<serde_json::Value>(&input.payload).is_err() {
                return Err(Error::Invalid("invalid event".to_string()));
            }
            let mut event = Event {
                schema: EVENT_SCHEMA.to_string(),
                sequence: (state.events.len() + index + 1) as u64,
                kind: input.kind.clone(),
                peer: input.peer.clone(),
                generation: input.generation,
                request_id: input.request_id.clone(),
                payload: compact(input.payload.as_bytes()),
                previous: previous.clone(),
                hash: String::new(),
            };
            event.hash = event.digest();
            staged.push_str(&event.text(&event.hash));
            staged.push('\n');
            previous = event.hash.clone();
            staged_events.push(event);
        }
        if self.max_bytes > 0 && state.data.len() + staged.len() > self.max_bytes {
            return Err(Error::Limit);
        }
        state.events.extend(staged_events);
        state.data.push_str(&staged);
        Ok(())
    }

    /// 📄️ The whole log as JSONL.
    pub fn snapshot(&self) -> String {
        self.state.lock().unwrap_or_else(|poison| poison.into_inner()).data.clone()
    }

    /// 📋️ Every committed record.
    pub fn events(&self) -> Vec<Event> {
        self.state.lock().unwrap_or_else(|poison| poison.into_inner()).events.clone()
    }
}

/// ✂️ Lifts one top-level member out of an already-compact JSON object without re-serialising it, so a
/// replayed payload keeps the exact bytes its digest was taken over.
pub fn raw_member(line: &str, name: &str) -> Option<String> {
    let bytes = line.as_bytes();
    let needle = format!("\"{name}\":");
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    let mut index = 0usize;
    while index < bytes.len() {
        let byte = bytes[index];
        if in_string {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                in_string = false;
            }
            index += 1;
            continue;
        }
        match byte {
            b'{' | b'[' => depth += 1,
            b'}' | b']' => depth = depth.saturating_sub(1),
            b'"' => {
                if depth == 1 && line[index..].starts_with(&needle) {
                    let start = index + needle.len();
                    let end = value_end(bytes, start)?;
                    return Some(line[start..end].to_string());
                }
                in_string = true;
            }
            _ => {}
        }
        index += 1;
    }
    None
}

fn value_end(bytes: &[u8], start: usize) -> Option<usize> {
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    let mut index = start;
    while index < bytes.len() {
        let byte = bytes[index];
        if in_string {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                in_string = false;
                if depth == 0 {
                    return Some(index + 1);
                }
            }
            index += 1;
            continue;
        }
        match byte {
            b'"' => in_string = true,
            b'{' | b'[' => depth += 1,
            b'}' | b']' => {
                if depth == 0 {
                    return Some(index);
                }
                depth -= 1;
                if depth == 0 {
                    return Some(index + 1);
                }
            }
            b',' if depth == 0 => return Some(index),
            _ => {}
        }
        index += 1;
    }
    None
}

/// 🔁️ Reloads a JSONL log, rejecting any break in the sequence or in the hash chain.
pub fn replay_events(data: &str, max_bytes: usize, max_count: usize) -> Result<Vec<Event>, Error> {
    if max_bytes > 0 && data.len() > max_bytes {
        return Err(Error::Limit);
    }
    let mut result: Vec<Event> = Vec::new();
    let mut previous = String::new();
    for line in data.lines() {
        if line.is_empty() {
            continue;
        }
        if max_count > 0 && result.len() == max_count {
            return Err(Error::Limit);
        }
        let position = result.len() + 1;
        let corrupt = || Error::Invalid(format!("corrupt event {position}"));
        let parsed: serde_json::Value = serde_json::from_str(line).map_err(|_| corrupt())?;
        let members = parsed.as_object().ok_or_else(corrupt)?;
        let text = |name: &str| members.get(name).and_then(|value| value.as_str()).unwrap_or_default().to_string();
        let event = Event {
            schema: text("schema"),
            sequence: members.get("sequence").and_then(serde_json::Value::as_u64).unwrap_or_default(),
            kind: text("kind"),
            peer: text("peer"),
            generation: members.get("generation").and_then(serde_json::Value::as_u64).unwrap_or_default(),
            request_id: text("requestId"),
            payload: raw_member(line, "payload").ok_or_else(corrupt)?,
            previous: text("previous"),
            hash: text("hash"),
        };
        if event.schema != EVENT_SCHEMA || event.sequence != position as u64 || event.previous != previous || event.kind.is_empty() || event.digest() != event.hash {
            return Err(corrupt());
        }
        previous = event.hash.clone();
        result.push(event);
    }
    Ok(result)
}

//#endregion 📡️Event

//#region 🔐️Session

/// 📏️ Every bounded resource of one server.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Limits {
    pub max_payload_bytes: usize,
    pub max_nesting: usize,
    pub max_page_items: usize,
    pub max_registry_items: usize,
    pub max_recent_ids: usize,
    pub max_event_bytes: usize,
    pub max_events: usize,
    pub max_handlers: usize,
}

impl Default for Limits {
    fn default() -> Limits {
        Limits { max_payload_bytes: 1 << 20, max_nesting: 64, max_page_items: 64, max_registry_items: 4096, max_recent_ids: 4096, max_event_bytes: 64 << 20, max_events: 100_000, max_handlers: 8 }
    }
}

impl Limits {
    fn normalized(self) -> Limits {
        let defaults = Limits::default();
        Limits {
            max_payload_bytes: if self.max_payload_bytes == 0 { defaults.max_payload_bytes } else { self.max_payload_bytes },
            max_nesting: if self.max_nesting == 0 { defaults.max_nesting } else { self.max_nesting },
            max_page_items: if self.max_page_items == 0 { defaults.max_page_items } else { self.max_page_items },
            max_registry_items: if self.max_registry_items == 0 { defaults.max_registry_items } else { self.max_registry_items },
            max_recent_ids: if self.max_recent_ids == 0 { defaults.max_recent_ids } else { self.max_recent_ids },
            max_event_bytes: if self.max_event_bytes == 0 { defaults.max_event_bytes } else { self.max_event_bytes },
            max_events: if self.max_events == 0 { defaults.max_events } else { self.max_events },
            max_handlers: if self.max_handlers == 0 { defaults.max_handlers } else { self.max_handlers },
        }
    }
}

/// ⚙️ What a server needs before it can accept a peer.
#[derive(Clone, Debug)]
pub struct Config {
    pub server_name: String,
    pub server_version: String,
    pub instructions: String,
    pub limits: Limits,
}

/// 🧭️ The per-request handle a handler observes for cancellation.
#[derive(Clone, Debug, Default)]
pub struct Context {
    cancelled: Arc<AtomicBool>,
}

impl Context {
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }

    fn cancel(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
    }
}

/// 📶️ Emits `notifications/progress` for the request it belongs to.
pub struct Progress<'a> {
    session: &'a Session,
    token: Option<String>,
}

impl Progress<'_> {
    /// 📣️ Reports one progress step; a request without a progress token reports nothing.
    pub fn report(&self, context: &Context, value: f64, total: Option<f64>, message: &str) -> Result<(), Error> {
        if context.is_cancelled() {
            return Err(Error::Closed);
        }
        let Some(token) = self.token.clone() else { return Ok(()) };
        let params = J::object(vec![
            ("progressToken", Some(J::Raw(token))),
            ("progress", Some(J::Float(value))),
            ("total", total.map(J::Float)),
            ("message", (!message.is_empty()).then(|| J::Str(message.to_string()))),
        ]);
        let payload = J::Obj(vec![
            ("jsonrpc".to_string(), J::Str(JSONRPC_VERSION.to_string())),
            ("method".to_string(), J::Str("notifications/progress".to_string())),
            ("params".to_string(), params),
        ])
        .text();
        if payload.len() > self.session.server.config.limits.max_payload_bytes {
            return Err(Error::PayloadTooLarge);
        }
        self.session.server.log.commit(&[EventInput::new("notification.sent", &self.session.peer, self.session.generation, payload.clone())])?;
        self.session.emit(payload.as_bytes())
    }
}

type ToolHandler = Box<dyn Fn(&Context, &CallToolParams, &Progress<'_>) -> Result<CallToolResult, HandlerError> + Send + Sync>;
type ResourceHandler = Box<dyn Fn(&Context, &str, &Progress<'_>) -> Result<Vec<ResourceContent>, HandlerError> + Send + Sync>;
type PromptHandler = Box<dyn Fn(&Context, &str, &BTreeMap<String, String>) -> Result<GetPromptResult, HandlerError> + Send + Sync>;

/// 🖥️ The registry every session routes into.
pub struct Server {
    config: Config,
    log: EventLog,
    tools: BTreeMap<String, (Tool, ToolHandler)>,
    resources: BTreeMap<String, (Resource, ResourceHandler)>,
    templates: BTreeMap<String, ResourceTemplate>,
    prompts: BTreeMap<String, (Prompt, PromptHandler)>,
    generation: AtomicU64,
    handler_queued: AtomicI64,
    handler_active: AtomicI64,
}

impl Server {
    /// 🏗️ Builds an empty server; a name and a version are mandatory.
    pub fn new(mut config: Config) -> Result<Server, Error> {
        if config.server_name.trim().is_empty() || config.server_version.trim().is_empty() {
            return Err(Error::Invalid("server name and version are required".to_string()));
        }
        config.limits = config.limits.normalized();
        let log = EventLog::new(config.limits.max_event_bytes, config.limits.max_events);
        Ok(Server {
            config,
            log,
            tools: BTreeMap::new(),
            resources: BTreeMap::new(),
            templates: BTreeMap::new(),
            prompts: BTreeMap::new(),
            generation: AtomicU64::new(0),
            handler_queued: AtomicI64::new(0),
            handler_active: AtomicI64::new(0),
        })
    }

    pub fn register_tool(&mut self, schema: Tool, handler: ToolHandler) -> Result<(), Error> {
        if schema.name.trim().is_empty() || schema.input_schema.kind != "object" {
            return Err(Error::Invalid("invalid tool registration".to_string()));
        }
        if self.tools.contains_key(&schema.name) {
            return Err(Error::Invalid("duplicate tool".to_string()));
        }
        if self.tools.len() == self.config.limits.max_registry_items {
            return Err(Error::Limit);
        }
        self.tools.insert(schema.name.clone(), (schema, handler));
        Ok(())
    }

    pub fn register_resource(&mut self, schema: Resource, handler: ResourceHandler) -> Result<(), Error> {
        if schema.uri.trim().is_empty() || schema.name.trim().is_empty() {
            return Err(Error::Invalid("invalid resource registration".to_string()));
        }
        if self.resources.contains_key(&schema.uri) {
            return Err(Error::Invalid("duplicate resource".to_string()));
        }
        if self.resources.len() == self.config.limits.max_registry_items {
            return Err(Error::Limit);
        }
        self.resources.insert(schema.uri.clone(), (schema, handler));
        Ok(())
    }

    pub fn register_resource_template(&mut self, schema: ResourceTemplate) -> Result<(), Error> {
        if schema.uri_template.trim().is_empty() || schema.name.trim().is_empty() {
            return Err(Error::Invalid("invalid resource template registration".to_string()));
        }
        if self.templates.contains_key(&schema.uri_template) {
            return Err(Error::Invalid("duplicate resource template".to_string()));
        }
        self.templates.insert(schema.uri_template.clone(), schema);
        Ok(())
    }

    pub fn register_prompt(&mut self, schema: Prompt, handler: PromptHandler) -> Result<(), Error> {
        if schema.name.trim().is_empty() {
            return Err(Error::Invalid("invalid prompt registration".to_string()));
        }
        if self.prompts.contains_key(&schema.name) {
            return Err(Error::Invalid("duplicate prompt".to_string()));
        }
        self.prompts.insert(schema.name.clone(), (schema, handler));
        Ok(())
    }

    /// 📚️ The event log this server writes into.
    pub fn events(&self) -> &EventLog {
        &self.log
    }

    /// 🔌️ Opens a session for one peer, bumping the peer generation and recording `session.opened`.
    pub fn connect(self: &Arc<Self>, peer: &str, sink: Option<Sink>) -> Result<Arc<Session>, Error> {
        if peer.trim().is_empty() {
            return Err(Error::Invalid("peer is required".to_string()));
        }
        let generation = self.generation.fetch_add(1, Ordering::SeqCst) + 1;
        let session = Arc::new(Session {
            server: Arc::clone(self),
            peer: peer.to_string(),
            generation,
            sink,
            state: Mutex::new(SessionState::default()),
            finished: Condvar::new(),
            handshake: Condvar::new(),
        });
        self.log.commit(&[EventInput::new("session.opened", peer, generation, format!("{{\"generation\":{generation}}}"))])?;
        Ok(session)
    }

    fn capabilities(&self) -> J {
        J::object(vec![
            ("prompts", (!self.prompts.is_empty()).then(|| J::Obj(Vec::new()))),
            ("resources", (!self.resources.is_empty() || !self.templates.is_empty()).then(|| J::Obj(Vec::new()))),
            ("tools", (!self.tools.is_empty()).then(|| J::Obj(Vec::new()))),
        ])
    }

    /// 📊️ The live handler-pool occupancy, mirroring the Go server's counters.
    pub fn handler_stats(&self) -> (usize, i64, i64) {
        (self.config.limits.max_handlers, self.handler_active.load(Ordering::SeqCst), self.handler_queued.load(Ordering::SeqCst))
    }
}

/// 📤️ Where an out-of-band frame goes.
pub type Sink = Arc<dyn Fn(&[u8]) -> Result<(), Error> + Send + Sync>;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
enum Phase {
    #[default]
    Connected,
    Initialized,
    Ready,
    Closing,
    Closed,
}

#[derive(Default)]
struct SessionState {
    phase: Phase,
    /// 📥️ A `notifications/initialized` that overtook its own `initialize` response. A client is
    /// entitled to pipeline the two, and the reader hands notifications straight through while the
    /// request is still on a worker — so the notification is REMEMBERED here instead of dropped, and
    /// the initialize path applies it the moment the phase it is waiting for exists.
    pending_initialized: bool,
    /// 🤝️ How many `initialize` requests the transport has delivered and no worker has answered yet.
    /// A burst is one write, so a request that follows `initialize` on the wire can reach a second
    /// worker before the first has promoted the phase; without this latch it answers `-32002`.
    handshake_pending: usize,
    active: BTreeMap<String, Context>,
    seen: Vec<String>,
    pre_cancelled: BTreeMap<String, String>,
    running: usize,
}

/// 🔐️ One peer connection: the lifecycle phase, the in-flight requests and the dedup window.
pub struct Session {
    server: Arc<Server>,
    peer: String,
    generation: u64,
    sink: Option<Sink>,
    state: Mutex<SessionState>,
    finished: Condvar,
    handshake: Condvar,
}

impl Session {
    pub fn peer(&self) -> &str {
        &self.peer
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    fn emit(&self, payload: &[u8]) -> Result<(), Error> {
        match &self.sink {
            None => Ok(()),
            Some(sink) => sink(payload).inspect_err(|_| self.drop_session()),
        }
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, SessionState> {
        self.state.lock().unwrap_or_else(|poison| poison.into_inner())
    }

    /// 🛑️ Cancels every in-flight request and refuses new ones.
    pub fn drop_session(&self) {
        let mut state = self.lock();
        if state.phase >= Phase::Closing {
            return;
        }
        state.phase = Phase::Closing;
        for context in state.active.values() {
            context.cancel();
        }
        drop(state);
        self.handshake.notify_all();
    }

    /// 🤝️ Records that an `initialize` request has been delivered and is not answered yet. The
    /// transport calls it before the payload reaches the worker pool, so a request delivered after
    /// it in the same burst cannot overtake the phase promotion.
    pub fn expect_handshake(&self) {
        self.lock().handshake_pending += 1;
    }

    /// 🤝️ Releases every request that queued behind a delivered `initialize`.
    pub fn settle_handshake(&self) {
        let mut state = self.lock();
        state.handshake_pending = state.handshake_pending.saturating_sub(1);
        drop(state);
        self.handshake.notify_all();
    }

    /// ⏳️ Blocks while an `initialize` delivered before this request is still unanswered.
    fn await_handshake(&self) {
        let mut state = self.lock();
        while state.handshake_pending > 0 && state.phase < Phase::Closing {
            state = self.handshake.wait(state).unwrap_or_else(|poison| poison.into_inner());
        }
    }

    /// 🚪️ Closes the session, waits for the in-flight requests and records `session.closed`.
    pub fn close(&self) -> Result<(), Error> {
        self.drop_session();
        let mut state = self.lock();
        while state.running > 0 {
            state = self.finished.wait(state).unwrap_or_else(|poison| poison.into_inner());
        }
        if state.phase == Phase::Closed {
            return Ok(());
        }
        state.phase = Phase::Closed;
        drop(state);
        self.server.log.commit(&[EventInput::new("session.closed", &self.peer, self.generation, "{\"reason\":\"closed\"}".to_string())])
    }

    fn begin(&self, key: &str) -> Result<Context, RpcError> {
        let mut state = self.lock();
        if state.phase >= Phase::Closing {
            return Err(RpcError::new(CODE_STALE_SESSION, "session closed"));
        }
        if state.active.contains_key(key) {
            return Err(RpcError::new(CODE_DUPLICATE_REQUEST, "duplicate request id"));
        }
        if state.seen.iter().any(|seen| seen == key) {
            return Err(RpcError::new(CODE_DUPLICATE_REQUEST, "stale request id"));
        }
        let context = Context::default();
        if state.pre_cancelled.remove(key).is_some() {
            context.cancel();
        }
        state.active.insert(key.to_string(), context.clone());
        state.running += 1;
        Ok(context)
    }

    fn finish(&self, key: &str) {
        let mut state = self.lock();
        state.active.remove(key);
        state.seen.push(key.to_string());
        let window = self.server.config.limits.max_recent_ids;
        if state.seen.len() > window {
            state.seen.remove(0);
        }
        state.running -= 1;
        self.finished.notify_all();
    }

    fn cancel(&self, key: &str, reason: &str) {
        let mut state = self.lock();
        if let Some(context) = state.active.get(key) {
            context.cancel();
            return;
        }
        if state.seen.iter().any(|seen| seen == key) {
            return;
        }
        if state.pre_cancelled.len() < self.server.config.limits.max_recent_ids {
            state.pre_cancelled.insert(key.to_string(), reason.to_string());
        }
    }
}

//#endregion 🔐️Session

//#region 🚦️Routing

/// 🤝️ Settles the handshake latch on every path an `initialize` dispatch can leave by, including the
/// early refusals that never reach the router.
struct HandshakeSettle<'a>(&'a Session);

impl Drop for HandshakeSettle<'_> {
    fn drop(&mut self) {
        self.0.settle_handshake();
    }
}

impl Session {
    /// 📬️ Handles one framed payload and returns the frame to write back, if any.
    pub fn dispatch(&self, payload: &[u8]) -> Result<Option<String>, Error> {
        let request = match self.decode_request(payload) {
            Ok(request) => request,
            Err((id_raw, error)) => return Ok(Some(self.encode_error(&id_raw, &error))),
        };
        let Some(id) = request.id.clone() else {
            self.handle_notification(&request, payload)?;
            return Ok(None);
        };
        let _settle = if request.method == "initialize" {
            Some(HandshakeSettle(self))
        } else {
            self.await_handshake();
            None
        };
        let key = id.key();
        let context = match self.begin(&key) {
            Ok(context) => context,
            Err(error) => {
                let response = self.encode_error(&request.id_raw, &error);
                self.commit_exchange(&key, payload, &response)?;
                return Ok(Some(response));
            }
        };
        let outcome = self.route(&context, &request);
        self.finish(&key);
        let response = match outcome {
            Ok(result) => self.encode_result(&request.id_raw, &result),
            Err(error) => self.encode_error(&request.id_raw, &error),
        };
        self.commit_exchange(&key, payload, &response)?;
        Ok(Some(response))
    }

    fn route(&self, context: &Context, request: &Request) -> Result<J, RpcError> {
        if context.is_cancelled() {
            return Err(RpcError::new(CODE_REQUEST_CANCELLED, "request cancelled"));
        }
        let phase = self.lock().phase;
        if request.method == "initialize" {
            if phase != Phase::Connected {
                return Err(RpcError::new(CODE_INVALID_REQUEST, "already initialized"));
            }
            let invalid = || RpcError::new(CODE_INVALID_PARAMS, "invalid initialize params");
            let members = decode_params(&request.params).map_err(|_| invalid())?;
            let protocol = members.get("protocolVersion").and_then(serde_json::Value::as_str).unwrap_or_default();
            let info = members.get("clientInfo").and_then(serde_json::Value::as_object);
            let name = info.and_then(|info| info.get("name")).and_then(serde_json::Value::as_str).unwrap_or_default();
            let version = info.and_then(|info| info.get("version")).and_then(serde_json::Value::as_str).unwrap_or_default();
            if protocol.is_empty() || name.is_empty() || version.is_empty() {
                return Err(invalid());
            }
            let mut state = self.lock();
            if state.phase != Phase::Connected {
                return Err(RpcError::new(CODE_INVALID_REQUEST, "already initialized"));
            }
            state.phase = if std::mem::take(&mut state.pending_initialized) { Phase::Ready } else { Phase::Initialized };
            drop(state);
            return Ok(J::object(vec![
                ("protocolVersion", Some(J::Str(negotiate_protocol_version(protocol).to_string()))),
                ("capabilities", Some(self.server.capabilities())),
                (
                    "serverInfo",
                    Some(J::Obj(vec![("name".to_string(), J::Str(self.server.config.server_name.clone())), ("version".to_string(), J::Str(self.server.config.server_version.clone()))])),
                ),
                ("instructions", (!self.server.config.instructions.is_empty()).then(|| J::Str(self.server.config.instructions.clone()))),
            ]));
        }
        if phase != Phase::Ready {
            return Err(RpcError::new(CODE_NOT_INITIALIZED, "session not initialized"));
        }
        match request.method.as_str() {
            "ping" => {
                decode_params(&request.params).map_err(|_| RpcError::new(CODE_INVALID_PARAMS, "invalid params"))?;
                Ok(J::Obj(Vec::new()))
            }
            "tools/list" => self.list(&request.params, "tools", |server| server.tools.values().map(|(schema, _)| schema.json()).collect()),
            "tools/call" => self.call_tool(context, request),
            "resources/list" => self.list(&request.params, "resources", |server| server.resources.values().map(|(schema, _)| schema.json()).collect()),
            "resources/templates/list" => self.list(&request.params, "resourceTemplates", |server| server.templates.values().map(ResourceTemplate::json).collect()),
            "resources/read" => self.read_resource(context, request),
            "prompts/list" => self.list(&request.params, "prompts", |server| server.prompts.values().map(|(schema, _)| schema.json()).collect()),
            "prompts/get" => self.get_prompt(context, request),
            _ => Err(RpcError::new(CODE_METHOD_NOT_FOUND, "method not found")),
        }
    }

    fn list(&self, params: &serde_json::Value, field: &str, collect: impl Fn(&Server) -> Vec<J>) -> Result<J, RpcError> {
        let members = decode_params(params).map_err(|_| RpcError::new(CODE_INVALID_PARAMS, "invalid list params"))?;
        let cursor = members.get("cursor").and_then(serde_json::Value::as_str).unwrap_or_default();
        let offset = if cursor.is_empty() { 0 } else { cursor.parse::<usize>().map_err(|_| RpcError::new(CODE_INVALID_PARAMS, "invalid cursor"))? };
        let items = collect(&self.server);
        let (end, next) = page(offset, items.len(), self.server.config.limits.max_page_items).ok_or_else(|| RpcError::new(CODE_INVALID_PARAMS, "invalid cursor"))?;
        Ok(J::object(vec![(field, Some(J::Arr(items[offset..end].to_vec()))), ("nextCursor", next.map(J::Str))]))
    }

    fn call_tool(&self, context: &Context, request: &Request) -> Result<J, RpcError> {
        let invalid = || RpcError::new(CODE_INVALID_PARAMS, "invalid tool params");
        let members = decode_params(&request.params).map_err(|_| invalid())?;
        let name = members.get("name").and_then(serde_json::Value::as_str).unwrap_or_default().to_string();
        if name.is_empty() {
            return Err(invalid());
        }
        let arguments = members.get("arguments").cloned().unwrap_or(serde_json::Value::Object(serde_json::Map::new()));
        if !arguments.is_object() {
            return Err(RpcError::new(CODE_INVALID_PARAMS, "invalid tool arguments"));
        }
        let params = CallToolParams { name: name.clone(), arguments, progress_token: progress_token(members) };
        let Some((_, handler)) = self.server.tools.get(&name) else {
            return Err(RpcError::new(CODE_INVALID_PARAMS, "tool not found"));
        };
        let progress = self.progress(params.progress_token.clone());
        let result = handler(context, &params, &progress).map_err(|error| handler_rpc_error(context, &error))?;
        if context.is_cancelled() {
            return Err(RpcError::new(CODE_REQUEST_CANCELLED, "request cancelled"));
        }
        Ok(result.json())
    }

    fn read_resource(&self, context: &Context, request: &Request) -> Result<J, RpcError> {
        let invalid = || RpcError::new(CODE_INVALID_PARAMS, "invalid resource params");
        let members = decode_params(&request.params).map_err(|_| invalid())?;
        let uri = members.get("uri").and_then(serde_json::Value::as_str).unwrap_or_default().to_string();
        if uri.is_empty() {
            return Err(invalid());
        }
        let Some((_, handler)) = self.server.resources.get(&uri) else {
            return Err(RpcError::new(CODE_INVALID_PARAMS, "resource not found"));
        };
        let progress = self.progress(progress_token(members));
        let contents = handler(context, &uri, &progress).map_err(|error| handler_rpc_error(context, &error))?;
        if context.is_cancelled() {
            return Err(RpcError::new(CODE_REQUEST_CANCELLED, "request cancelled"));
        }
        Ok(J::Obj(vec![("contents".to_string(), J::Arr(contents.iter().map(ResourceContent::json).collect()))]))
    }

    fn get_prompt(&self, context: &Context, request: &Request) -> Result<J, RpcError> {
        let invalid = || RpcError::new(CODE_INVALID_PARAMS, "invalid prompt params");
        let members = decode_params(&request.params).map_err(|_| invalid())?;
        let name = members.get("name").and_then(serde_json::Value::as_str).unwrap_or_default().to_string();
        if name.is_empty() {
            return Err(invalid());
        }
        let mut arguments = BTreeMap::new();
        if let Some(given) = members.get("arguments").and_then(serde_json::Value::as_object) {
            for (key, value) in given {
                arguments.insert(key.clone(), value.as_str().unwrap_or_default().to_string());
            }
        }
        let Some((_, handler)) = self.server.prompts.get(&name) else {
            return Err(RpcError::new(CODE_INVALID_PARAMS, "prompt not found"));
        };
        let result = handler(context, &name, &arguments).map_err(|error| handler_rpc_error(context, &error))?;
        if context.is_cancelled() {
            return Err(RpcError::new(CODE_REQUEST_CANCELLED, "request cancelled"));
        }
        Ok(result.json())
    }

    fn progress(&self, token: Option<String>) -> Progress<'_> {
        Progress { session: self, token }
    }

    fn handle_notification(&self, request: &Request, payload: &[u8]) -> Result<(), Error> {
        match request.method.as_str() {
            "notifications/initialized" => {
                let mut state = self.lock();
                match state.phase {
                    Phase::Initialized => state.phase = Phase::Ready,
                    Phase::Connected => state.pending_initialized = true,
                    _ => {}
                }
            }
            "notifications/cancelled" => {
                if let Ok(members) = decode_params(&request.params) {
                    if let Some(id) = members.get("requestId").and_then(Id::from_value) {
                        let reason = members.get("reason").and_then(serde_json::Value::as_str).unwrap_or("cancelled by peer");
                        self.cancel(&id.key(), reason);
                    }
                }
            }
            _ => {}
        }
        self.server.log.commit(&[EventInput::new("notification.received", &self.peer, self.generation, compact(payload))])
    }

    fn decode_request(&self, payload: &[u8]) -> Result<Request, (String, RpcError)> {
        let null = || "null".to_string();
        if payload.len() > self.server.config.limits.max_payload_bytes {
            return Err((null(), RpcError::new(CODE_PAYLOAD_TOO_LARGE, "payload too large")));
        }
        match validate_nesting(payload, self.server.config.limits.max_nesting) {
            Err(Error::NestingTooDeep) => return Err((null(), RpcError::new(CODE_INVALID_REQUEST, "nesting too deep"))),
            Err(_) => return Err((null(), RpcError::new(CODE_PARSE_ERROR, "parse error"))),
            Ok(()) => {}
        }
        let mut stream = serde_json::Deserializer::from_slice(payload).into_iter::<serde_json::Value>();
        let Some(Ok(document)) = stream.next() else {
            return Err((null(), RpcError::new(CODE_PARSE_ERROR, "parse error")));
        };
        if stream.next().is_some() {
            return Err((null(), RpcError::new(CODE_INVALID_REQUEST, "trailing data")));
        }
        let Some(members) = document.as_object() else {
            return Err((null(), RpcError::new(CODE_INVALID_REQUEST, "invalid request")));
        };
        for name in members.keys() {
            if !matches!(name.as_str(), "jsonrpc" | "id" | "method" | "params") {
                return Err((null(), RpcError::new(CODE_INVALID_REQUEST, "invalid request")));
            }
        }
        let id_raw = members.get("id").map_or_else(null, |id| compact(serde_json::to_string(id).unwrap_or_default().as_bytes()));
        let id = match members.get("id") {
            None => None,
            Some(value) => match Id::from_value(value) {
                Some(id) => Some(id),
                None => return Err((null(), RpcError::new(CODE_INVALID_REQUEST, "invalid request id"))),
            },
        };
        let request = Request {
            jsonrpc: members.get("jsonrpc").and_then(serde_json::Value::as_str).unwrap_or_default().to_string(),
            id,
            id_raw: id_raw.clone(),
            method: members.get("method").and_then(serde_json::Value::as_str).unwrap_or_default().to_string(),
            params: members.get("params").cloned().unwrap_or(serde_json::Value::Object(serde_json::Map::new())),
        };
        if request.jsonrpc != JSONRPC_VERSION || request.method.is_empty() {
            return Err((id_raw, RpcError::new(CODE_INVALID_REQUEST, "invalid request")));
        }
        Ok(request)
    }

    fn encode_result(&self, id_raw: &str, result: &J) -> String {
        let encoded = format!("{{\"jsonrpc\":\"2.0\",\"id\":{id_raw},\"result\":{}}}", result.text());
        let limits = self.server.config.limits;
        if encoded.len() > limits.max_payload_bytes || validate_nesting(encoded.as_bytes(), limits.max_nesting).is_err() {
            return self.encode_error(id_raw, &RpcError::new(CODE_PAYLOAD_TOO_LARGE, "response too large"));
        }
        encoded
    }

    fn encode_error(&self, id_raw: &str, error: &RpcError) -> String {
        let encoded = format!("{{\"jsonrpc\":\"2.0\",\"id\":{id_raw},\"error\":{}}}", error.json().text());
        let limits = self.server.config.limits;
        if encoded.len() <= limits.max_payload_bytes && validate_nesting(encoded.as_bytes(), limits.max_nesting).is_ok() {
            return encoded;
        }
        format!("{{\"jsonrpc\":\"2.0\",\"id\":{id_raw},\"error\":{}}}", RpcError::new(CODE_PAYLOAD_TOO_LARGE, "response too large").json().text())
    }

    fn commit_exchange(&self, key: &str, request: &[u8], response: &str) -> Result<(), Error> {
        let mut received = EventInput::new("request.received", &self.peer, self.generation, compact(request));
        received.request_id = key.to_string();
        let mut sent = EventInput::new("response.sent", &self.peer, self.generation, compact(response.as_bytes()));
        sent.request_id = key.to_string();
        self.server.log.commit(&[received, sent])
    }

    fn reject(&self, payload: &[u8], rejection: RpcError) -> Result<String, Error> {
        let (id_raw, key, error) = match self.decode_request(payload) {
            Ok(request) => (request.id_raw.clone(), request.id.map(|id| id.key()).unwrap_or_default(), rejection),
            Err((id_raw, protocol)) => (id_raw, String::new(), protocol),
        };
        let response = self.encode_error(&id_raw, &error);
        self.commit_exchange(&key, payload, &response)?;
        Ok(response)
    }
}

fn progress_token(members: &serde_json::Map<String, serde_json::Value>) -> Option<String> {
    members.get("_meta").and_then(serde_json::Value::as_object).and_then(|meta| meta.get("progressToken")).map(|token| compact(serde_json::to_string(token).unwrap_or_default().as_bytes()))
}

fn handler_rpc_error(context: &Context, error: &HandlerError) -> RpcError {
    if context.is_cancelled() {
        return RpcError::new(CODE_REQUEST_CANCELLED, "request cancelled");
    }
    if (-32099..=-32000).contains(&error.code) && !error.message.is_empty() {
        return RpcError { code: error.code, message: error.message.clone(), data: error.data.clone() };
    }
    RpcError::new(CODE_INTERNAL_ERROR, "internal error")
}

/// 📄️ Computes the end index and the next cursor of one page.
pub fn page(offset: usize, length: usize, size: usize) -> Option<(usize, Option<String>)> {
    if offset > length {
        return None;
    }
    let end = offset + size;
    if end >= length {
        return Some((length, None));
    }
    Some((end, Some(end.to_string())))
}

/// 🪆️ Rejects a frame whose bracket nesting exceeds the configured depth or is unbalanced.
pub fn validate_nesting(payload: &[u8], maximum: usize) -> Result<(), Error> {
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    for byte in payload {
        if in_string {
            if escaped {
                escaped = false;
            } else if *byte == b'\\' {
                escaped = true;
            } else if *byte == b'"' {
                in_string = false;
            }
            continue;
        }
        match byte {
            b'"' => in_string = true,
            b'{' | b'[' => {
                depth += 1;
                if depth > maximum {
                    return Err(Error::NestingTooDeep);
                }
            }
            b'}' | b']' => {
                if depth == 0 {
                    return Err(Error::Invalid("invalid nesting".to_string()));
                }
                depth -= 1;
            }
            _ => {}
        }
    }
    if depth != 0 || in_string {
        return Err(Error::Invalid("invalid nesting".to_string()));
    }
    Ok(())
}

//#endregion 🚦️Routing

//#region 🗄️Repository

/// 🪪️ The per-IDE surface selected by `SEMIO_REPO_MCP_CLIENT`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Profile {
    #[default]
    Generic,
    Cursor,
    Kiro,
    Copilot,
    Claude,
    Codex,
}

impl Profile {
    /// 🔤️ Parses a profile slug; the empty string and `client` both mean the generic surface.
    pub fn parse(raw: &str) -> Result<Profile, Error> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "" | "generic" | "client" => Ok(Profile::Generic),
            "cursor" => Ok(Profile::Cursor),
            "kiro" => Ok(Profile::Kiro),
            "copilot" => Ok(Profile::Copilot),
            "claude" => Ok(Profile::Claude),
            "codex" => Ok(Profile::Codex),
            other => Err(Error::Invalid(format!("unknown mcp kind {other:?} (expected client, cursor, copilot, claude, codex, or kiro)"))),
        }
    }

    /// 🏷️ The stable kind slug used as the description-table column.
    pub fn kind(self) -> &'static str {
        match self {
            Profile::Generic => "generic",
            Profile::Cursor => "cursor",
            Profile::Kiro => "kiro",
            Profile::Copilot => "copilot",
            Profile::Claude => "claude",
            Profile::Codex => "codex",
        }
    }

    /// 🖥️ The server name reported in `initialize`.
    pub fn server_name(self) -> &'static str {
        match self {
            Profile::Generic | Profile::Cursor | Profile::Kiro | Profile::Copilot | Profile::Claude | Profile::Codex => "repo",
        }
    }
}

const DESCRIPTIONS_JSON: &str = include_str!("../../🖼️assets/🔣️descriptions.json");

static DESCRIPTIONS: OnceLock<BTreeMap<String, BTreeMap<String, String>>> = OnceLock::new();

/// 🗣️ The per-profile description table, authored once at `🔌️mcp/🖼️assets/🔣️descriptions.json`.
pub fn descriptions() -> &'static BTreeMap<String, BTreeMap<String, String>> {
    DESCRIPTIONS.get_or_init(|| {
        let parsed: serde_json::Value = serde_json::from_str(DESCRIPTIONS_JSON).expect("description table must parse");
        let mut table = BTreeMap::new();
        if let Some(members) = parsed.get("descriptions").and_then(serde_json::Value::as_object) {
            for (key, value) in members {
                let mut column = BTreeMap::new();
                if let Some(kinds) = value.as_object() {
                    for (kind, text) in kinds {
                        column.insert(kind.clone(), text.as_str().unwrap_or_default().to_string());
                    }
                }
                table.insert(key.clone(), column);
            }
        }
        table
    })
}

/// 🗣️ Picks one description for one profile, falling back to the generic wording.
pub fn describe(profile: Profile, key: &str) -> String {
    let Some(column) = descriptions().get(key) else { return String::new() };
    if let Some(text) = column.get(profile.kind()) {
        if !text.is_empty() {
            return text.clone();
        }
    }
    column.get("generic").cloned().unwrap_or_default()
}

/// 🧰️ What a domain tool returns to the protocol layer.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RepositoryResult {
    pub text: String,
    pub structured: Option<String>,
    pub is_error: bool,
}

/// 🔗️ The domain boundary. Everything the repo MCP server can do that is not protocol lives behind
/// this trait; the `⌨️cli` crate supplies the production implementation backed by the tickets, goals
/// and move crates.
pub trait Repository: Send + Sync {
    /// 🧰️ Runs one tool. `arguments` is the raw, already-validated-as-object `arguments` member.
    fn call(&self, context: &Context, name: &str, arguments: &serde_json::Value) -> Result<RepositoryResult, HandlerError>;
    /// 📚️ Reads one resource URI.
    fn read(&self, context: &Context, uri: &str) -> Result<ResourceContent, HandlerError>;
    /// 💬️ Renders one prompt.
    fn prompt(&self, context: &Context, name: &str, arguments: &BTreeMap<String, String>) -> Result<GetPromptResult, HandlerError>;
}

/// 🧪️ An in-memory repository that records every call and answers deterministically.
#[derive(Debug, Default)]
pub struct RecordingRepository {
    calls: Mutex<Vec<(String, String)>>,
}

impl RecordingRepository {
    pub fn new() -> RecordingRepository {
        RecordingRepository::default()
    }

    /// 📋️ Every `(surface, argument)` pair observed so far, in order.
    pub fn calls(&self) -> Vec<(String, String)> {
        self.calls.lock().unwrap_or_else(|poison| poison.into_inner()).clone()
    }

    fn record(&self, surface: &str, argument: &str) {
        self.calls.lock().unwrap_or_else(|poison| poison.into_inner()).push((surface.to_string(), argument.to_string()));
    }
}

impl Repository for RecordingRepository {
    fn call(&self, _context: &Context, name: &str, arguments: &serde_json::Value) -> Result<RepositoryResult, HandlerError> {
        if !TOOL_NAMES.contains(&name) {
            return Err(HandlerError::tool("tool not found"));
        }
        self.record("call", name);
        Ok(RepositoryResult { text: name.to_string(), structured: Some(serde_json::to_string(arguments).unwrap_or_else(|_| "null".to_string())), is_error: false })
    }

    fn read(&self, _context: &Context, uri: &str) -> Result<ResourceContent, HandlerError> {
        if !RESOURCE_URIS.contains(&uri) {
            return Err(HandlerError::tool("resource not found"));
        }
        self.record("read", uri);
        Ok(ResourceContent { uri: uri.to_string(), mime_type: "text/plain".to_string(), text: uri.to_string(), blob: String::new() })
    }

    fn prompt(&self, _context: &Context, name: &str, arguments: &BTreeMap<String, String>) -> Result<GetPromptResult, HandlerError> {
        let Some(instruction) = prompt_instruction(name) else { return Err(HandlerError::tool("prompt not found")) };
        self.record("prompt", name);
        let given = arguments.get("prompt").cloned().unwrap_or_default();
        Ok(GetPromptResult { description: instruction.to_string(), messages: vec![("user".to_string(), Content::text(&format!("{instruction}\n\n{given}")))] })
    }
}

/// 💬️ The fixed instruction each prompt prefixes to the caller's own prompt.
pub fn prompt_instruction(name: &str) -> Option<&'static str> {
    match name {
        "enhance" => Some("Enhance the request while preserving its intent and constraints."),
        "refactor" => Some("Refactor the requested scope completely and preserve observable behavior."),
        "test" => Some("Test the requested scope with executable success and hostile cases."),
        "comply" => Some("Apply the repository instructions and resolve every in-scope breach."),
        _ => None,
    }
}

/// 🧰️ Every tool name, in the order `tools/list` sorts them.
pub const TOOL_NAMES: [&str; 6] = ["goal_close", "goal_open", "goal_reopen", "ticket_close", "ticket_open", "ticket_reopen"];

/// 📚️ Every resource URI, in the order `resources/list` sorts them.
pub const RESOURCE_URIS: [&str; 8] = ["repo://", "repo://bundles", "repo://contributors", "repo://files", "repo://folders", "repo://goals", "repo://policies", "repo://tickets"];

/// 💬️ Every prompt name, in the order `prompts/list` sorts them.
pub const PROMPT_NAMES: [&str; 4] = ["comply", "enhance", "refactor", "test"];

/// 🧰️ The advertised tool table for one profile.
pub fn tool_schemas(profile: Profile) -> Vec<Tool> {
    let mut open = vec![
        ("emoji", Schema::string("Ticket emoji.")),
        ("title", Schema::string("Ticket title.")),
        ("prompt", Schema::string("Task description.")),
        ("goal", Schema::string("Goal id.")),
        ("client", Schema::string("Agent client.")),
        ("llm", Schema::string("Model.")),
        ("effort", Schema::string("Reasoning effort.")),
        ("draft", Schema::string("Draft id.")),
        ("parent", Schema::string("Parent ticket.")),
        ("issue", Schema::string("Existing issue URL.")),
        ("no_issue", Schema::boolean("Skip issue creation.")),
        ("no_management", Schema::boolean("Skip management integration.")),
    ];
    let mut reopen = vec![
        ("path", Schema::string("YY/MM/DD/SLUG path.")),
        ("prompt", Schema::string("Additional task description.")),
        ("client", Schema::string("Agent client.")),
        ("llm", Schema::string("Model.")),
        ("effort", Schema::string("Reasoning effort.")),
        ("draft", Schema::string("Draft id.")),
        ("title", Schema::string("Updated title.")),
        ("goal", Schema::string("Goal id.")),
        ("parent", Schema::string("Parent ticket.")),
        ("no_management", Schema::boolean("Skip management integration.")),
    ];
    match profile {
        Profile::Cursor | Profile::Copilot | Profile::Claude | Profile::Codex => {
            open.push(("plan_id", Schema::string(&describe(profile, "arg_plan_id"))));
            reopen.push(("plan_id", Schema::string(&describe(profile, "arg_plan_id"))));
        }
        Profile::Kiro => {
            open.push(("spec_id", Schema::string(&describe(profile, "arg_spec_id"))));
            reopen.push(("spec_id", Schema::string(&describe(profile, "arg_spec_id"))));
        }
        Profile::Generic => {}
    }
    let tool = |name: &str, schema: Schema| Tool { name: name.to_string(), title: String::new(), description: describe(profile, &format!("tool_{name}")), input_schema: schema };
    vec![
        tool("ticket_open", Schema::object(open, &["emoji", "title", "prompt", "goal"])),
        tool(
            "ticket_close",
            Schema::object(
                vec![
                    ("path", Schema::string("YY/MM/DD/SLUG path.")),
                    ("summary", Schema::string("Completion summary.")),
                    ("files", Schema::array("Changed files.")),
                    ("title", Schema::string("Updated title.")),
                    ("no_management", Schema::boolean("Skip management integration.")),
                ],
                &["summary"],
            ),
        ),
        tool("ticket_reopen", Schema::object(reopen, &[])),
        tool(
            "goal_open",
            Schema::object(
                vec![
                    ("title", Schema::string("Goal title.")),
                    ("description", Schema::string("Goal description.")),
                    ("prompt", Schema::string("Goal prompt.")),
                    ("due_date", Schema::string("YYYY-MM-DD due date.")),
                    ("llm", Schema::string("Model.")),
                    ("client", Schema::string("Agent client.")),
                    ("parent", Schema::string("Parent goal id.")),
                    ("milestone", Schema::string("Management milestone.")),
                    ("no_management", Schema::boolean("Skip management integration.")),
                ],
                &["title", "prompt"],
            ),
        ),
        tool(
            "goal_close",
            Schema::object(vec![("id", Schema::string("Goal id.")), ("summary", Schema::string("Completion summary.")), ("no_management", Schema::boolean("Skip management integration."))], &["id", "summary"]),
        ),
        tool(
            "goal_reopen",
            Schema::object(
                vec![
                    ("id", Schema::string("Goal id.")),
                    ("prompt", Schema::string("Additional goal prompt.")),
                    ("llm", Schema::string("Model.")),
                    ("client", Schema::string("Agent client.")),
                    ("title", Schema::string("Updated title.")),
                    ("description", Schema::string("Updated description.")),
                    ("due_date", Schema::string("YYYY-MM-DD due date.")),
                    ("no_management", Schema::boolean("Skip management integration.")),
                ],
                &["id", "prompt", "llm", "client"],
            ),
        ),
    ]
}

/// 📚️ The advertised resource table for one profile.
pub fn resource_schemas(profile: Profile) -> Vec<Resource> {
    [("repo://", "repo", "res_root"), ("repo://bundles", "bundles", "res_bundles"), ("repo://folders", "folders", "res_folders"), ("repo://files", "files", "res_files"), ("repo://tickets", "tickets", "res_tickets"), ("repo://goals", "goals", "res_goals"), ("repo://policies", "policies", "res_policies"), ("repo://contributors", "contributors", "res_contributors")]
        .into_iter()
        .map(|(uri, name, key)| Resource { uri: uri.to_string(), name: name.to_string(), title: String::new(), description: describe(profile, key), mime_type: "text/plain".to_string() })
        .collect()
}

/// 💬️ The advertised prompt table for one profile.
pub fn prompt_schemas(profile: Profile) -> Vec<Prompt> {
    ["enhance", "refactor", "test", "comply"]
        .into_iter()
        .map(|name| Prompt {
            name: name.to_string(),
            title: String::new(),
            description: describe(profile, &format!("prompt_{name}")),
            arguments: vec![PromptArgument { name: "prompt".to_string(), description: String::new(), required: true }],
        })
        .collect()
}

/// 🏭️ Builds the production server for one repository and one profile.
pub fn repository_server(repository: impl Repository + 'static, profile: Profile, limits: Limits) -> Result<Server, Error> {
    let repository: Arc<dyn Repository> = Arc::new(repository);
    let mut server = Server::new(Config {
        server_name: profile.server_name().to_string(),
        server_version: "1.0.0".to_string(),
        instructions: "Use repository tools and resources through their owned schemas.".to_string(),
        limits,
    })?;
    for schema in tool_schemas(profile) {
        let name = schema.name.clone();
        let owned = Arc::clone(&repository);
        server.register_tool(
            schema,
            Box::new(move |context, params, progress| {
                progress.report(context, 0.0, None, "started").map_err(|_| HandlerError::tool("progress failed"))?;
                let result = owned.call(context, &name, &params.arguments)?;
                progress.report(context, 1.0, None, "completed").map_err(|_| HandlerError::tool("progress failed"))?;
                Ok(CallToolResult { content: vec![Content::text(&result.text)], structured_content: result.structured, is_error: result.is_error })
            }),
        )?;
    }
    for schema in resource_schemas(profile) {
        let uri = schema.uri.clone();
        let owned = Arc::clone(&repository);
        server.register_resource(
            schema,
            Box::new(move |context, _requested, progress| {
                progress.report(context, 0.0, None, "started").map_err(|_| HandlerError::tool("progress failed"))?;
                Ok(vec![owned.read(context, &uri)?])
            }),
        )?;
    }
    for schema in prompt_schemas(profile) {
        let name = schema.name.clone();
        let owned = Arc::clone(&repository);
        server.register_prompt(schema, Box::new(move |context, _requested, arguments| owned.prompt(context, &name, arguments)))?;
    }
    Ok(server)
}

/// 🧪️ The synthetic server the `2️⃣g2-contract.json` golden vectors are recorded against: one `echo`
/// tool, one JSON `repo://goals` resource, one ticket template and one `review` prompt.
pub fn contract_server(limits: Limits) -> Result<Server, Error> {
    let mut server = Server::new(Config { server_name: "repo".to_string(), server_version: "1.0.0".to_string(), instructions: "owned".to_string(), limits })?;
    server.register_tool(
        Tool { name: "echo".to_string(), title: String::new(), description: String::new(), input_schema: Schema { kind: "object".to_string(), ..Schema::default() } },
        Box::new(|context, params, progress| {
            let text = params.arguments.get("text").and_then(serde_json::Value::as_str).unwrap_or_default().to_string();
            progress.report(context, 1.0, None, "echo").map_err(|_| HandlerError::tool("progress failed"))?;
            Ok(CallToolResult { content: vec![Content::text(&text)], structured_content: None, is_error: false })
        }),
    )?;
    server.register_resource(
        Resource { uri: "repo://goals".to_string(), name: "goals".to_string(), title: String::new(), description: String::new(), mime_type: "application/json".to_string() },
        Box::new(|_context, _uri, _progress| Ok(vec![ResourceContent { uri: "repo://goals".to_string(), mime_type: "application/json".to_string(), text: "[]".to_string(), blob: String::new() }])),
    )?;
    server.register_resource_template(ResourceTemplate { uri_template: "repo://ticket/{id}".to_string(), name: "ticket".to_string(), title: String::new(), description: String::new(), mime_type: String::new() })?;
    server.register_prompt(
        Prompt { name: "review".to_string(), title: String::new(), description: String::new(), arguments: vec![PromptArgument { name: "scope".to_string(), description: String::new(), required: true }] },
        Box::new(|_context, _name, arguments| {
            let scope = arguments.get("scope").cloned().unwrap_or_default();
            Ok(GetPromptResult { description: format!("Review {scope}"), messages: vec![("user".to_string(), Content::text(&scope))] })
        }),
    )?;
    Ok(server)
}

//#endregion 🗄️Repository

//#region 🦀️Entrypoint

/// 🪪️ The environment variable that selects the profile.
pub const PROFILE_ENVIRONMENT: &str = "SEMIO_REPO_MCP_CLIENT";

/// 🧺️ The queue every handler thread of one served peer pulls its next payload from: the pending
/// payloads, the condition the readers wait on, and the flag that closes the queue.
type JobQueue = (Mutex<Vec<Vec<u8>>>, Condvar, AtomicBool);

/// 🚚️ Serves one peer over a line-delimited byte stream until the peer drops.
pub fn serve(server: &Arc<Server>, peer: &str, reader: impl Read + Send, writer: impl Write + Send + 'static) -> Result<(), Error> {
    let output = Arc::new(Mutex::new(writer));
    let sink_output = Arc::clone(&output);
    let sink: Sink = Arc::new(move |payload: &[u8]| {
        let mut guard = sink_output.lock().unwrap_or_else(|poison| poison.into_inner());
        guard.write_all(payload).and_then(|()| guard.write_all(b"\n")).and_then(|()| guard.flush()).map_err(|_| Error::PeerDropped)
    });
    let session = server.connect(peer, Some(Arc::clone(&sink)))?;
    let limits = server.config.limits;
    let jobs: Arc<JobQueue> = Arc::new((Mutex::new(Vec::new()), Condvar::new(), AtomicBool::new(false)));
    let mut workers = Vec::new();
    for _ in 0..limits.max_handlers {
        let jobs = Arc::clone(&jobs);
        let session = Arc::clone(&session);
        let server = Arc::clone(server);
        let sink = Arc::clone(&sink);
        workers.push(std::thread::spawn(move || loop {
            let payload = {
                let (queue, ready, closed) = &*jobs;
                let mut guard = queue.lock().unwrap_or_else(|poison| poison.into_inner());
                while guard.is_empty() {
                    if closed.load(Ordering::SeqCst) {
                        return;
                    }
                    guard = ready.wait(guard).unwrap_or_else(|poison| poison.into_inner());
                }
                guard.remove(0)
            };
            server.handler_queued.fetch_sub(1, Ordering::SeqCst);
            server.handler_active.fetch_add(1, Ordering::SeqCst);
            if let Ok(Some(response)) = session.dispatch(&payload) {
                let _ = sink(response.as_bytes());
            }
            server.handler_active.fetch_sub(1, Ordering::SeqCst);
        }));
    }
    let lines = BufReader::new(reader).split(b'\n');
    let mut outcome = Ok(());
    for line in lines {
        let Ok(mut payload) = line else {
            outcome = Err(Error::PeerDropped);
            break;
        };
        if payload.last() == Some(&b'\r') {
            payload.pop();
        }
        if payload.is_empty() {
            continue;
        }
        if payload.len() > limits.max_payload_bytes {
            outcome = Err(Error::PayloadTooLarge);
            break;
        }
        let decoded = session.decode_request(&payload);
        if matches!(&decoded, Ok(request) if request.id.is_none()) {
            let _ = session.dispatch(&payload);
            continue;
        }
        if server.handler_queued.load(Ordering::SeqCst) >= limits.max_handlers as i64 {
            let response = session.reject(&payload, RpcError::new(CODE_SERVER_BUSY, "handler queue full"))?;
            sink(response.as_bytes())?;
            continue;
        }
        // 🤝️The latch is armed BEFORE the payload reaches a worker: arming it afterwards races the
        // worker that already answered, and the decrement would then be lost for good.
        if matches!(&decoded, Ok(request) if request.method == "initialize") {
            session.expect_handshake();
        }
        server.handler_queued.fetch_add(1, Ordering::SeqCst);
        let (queue, ready, _) = &*jobs;
        queue.lock().unwrap_or_else(|poison| poison.into_inner()).push(payload);
        ready.notify_one();
    }
    let (_, ready, closed) = &*jobs;
    closed.store(true, Ordering::SeqCst);
    ready.notify_all();
    for worker in workers {
        let _ = worker.join();
    }
    session.close()?;
    outcome
}

/// ▶️ Serves the repository over stdio until stdin closes.
pub fn serve_stdio(repository: impl Repository + 'static, profile: Profile) -> Result<(), Error> {
    let server = Arc::new(repository_server(repository, profile, Limits::default())?);
    match serve(&server, "stdio", std::io::stdin(), std::io::stdout()) {
        Err(Error::PeerDropped) => Ok(()),
        other => other,
    }
}

/// 🚀️ The process entry point: no arguments are accepted, the profile comes from the environment.
pub fn run_with(argv: &[String], repository: impl Repository + 'static) -> i32 {
    if !argv.is_empty() {
        eprintln!("repo MCP accepts no command arguments");
        return 1;
    }
    let profile = match Profile::parse(&std::env::var(PROFILE_ENVIRONMENT).unwrap_or_default()) {
        Ok(profile) => profile,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    match serve_stdio(repository, profile) {
        Ok(()) => 0,
        Err(error) => {
            eprintln!("{error}");
            1
        }
    }
}

//#endregion 🦀️Entrypoint

//#region 🧪️Tests

#[cfg(test)]
#[path = "../../🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#endregion 🧪️Tests
