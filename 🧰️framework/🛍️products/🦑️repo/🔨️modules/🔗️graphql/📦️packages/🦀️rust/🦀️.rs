//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

//! 🔗️ The repository's owned GraphQL document grammar: lexer, AST, recursive-descent parser,
//! validation and variable coercion. Byte-for-byte behavioural twin of
//! `🔗️graphql/📦️packages/🐹️go/🐹️.go`, down to the diagnostic text and the byte offsets, so the
//! executor can be ported against this AST without re-deciding a single grammar question.

//#endregion 🧲️Header

use semio_framework_repo_identity::{emoji_text, entity, flat};
use semio_framework_repo_model::{
    derive_technology_kind, AnalyzeMetrics, AnalyzeResult, Breach, BreachPriority, Bundle, BundleKind, Checkpoint, Contributor, ContributorAddInput, ContributorContributionsStorage, ContributorLink,
    Definition, DefinitionKind, Draft, DraftCreateInput, File, FilterInput, Folder, FolderKind, Goal, GoalChangeInput, GoalCloseInput, GoalCreateInput, GoalDates, GoalDeleteInput,
    GoalManagementData, GoalReopenInput, GoalStatus, Interaction, InteractionFile, InteractionResource, Policy, PriorityCount, Range, Section, Statute, StatuteMeta, Technology, Territory, Ticket,
    TicketAgent, TicketChangeInput, TicketCloseInput, TicketDeleteInput, TicketManagementData, TicketOpenInput, TicketReopenInput, TicketStatus, Todo, TodoChangeInput, TodoCreateInput,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value as Json};
use std::collections::BTreeMap;
use std::fmt;

/// 🔌️ Explicit re-export: the projection and the coercion helpers speak `serde_json`, so a client of
/// this crate reaches that vocabulary through this crate and never declares the dependency itself.
pub use serde_json;

//#region 🔤️Lexer

/// 🔤️ The five token classes the grammar distinguishes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenKind {
    #[default]
    Eof,
    Name,
    Str,
    Number,
    Punct,
}

/// 🔤️ One lexed token and the byte offset it starts at.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
    pub offset: usize,
}

/// 🔤️ Bytes the reference implementation treats as insignificant whitespace.
fn is_space(byte: u8) -> bool {
    matches!(byte, b'\t' | b'\n' | 0x0B | 0x0C | b'\r' | b' ' | 0x85 | 0xA0)
}

/// 🔤️ Bytes the reference implementation accepts as a name letter.
fn is_letter(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || matches!(byte, 0xAA | 0xB5 | 0xBA | 0xC0..=0xD6 | 0xD8..=0xF6 | 0xF8..=0xFF)
}

/// 🔤️ Bytes the reference implementation accepts as a name digit.
fn is_digit(byte: u8) -> bool {
    byte.is_ascii_digit()
}

/// 🔤️ A byte-oriented scanner over one document source.
#[derive(Clone, Debug)]
pub struct Lexer {
    source: Vec<u8>,
    offset: usize,
}

impl Lexer {
    /// 🔤️ Starts a scanner at the beginning of a source.
    pub fn new(source: &str) -> Self {
        Self { source: source.as_bytes().to_vec(), offset: 0 }
    }

    /// 🔤️ Reads the next token, skipping insignificant bytes, commas and comments.
    ///
    /// Returns the token AND the failure, never one instead of the other: the reference
    /// implementation assigns both to the parser in one statement, so a failed string literal
    /// still leaves a stale token behind, and that is observable in the diagnostics.
    pub fn next_token(&mut self) -> (Token, Option<ParseError>) {
        while self.offset < self.source.len() {
            let byte = self.source[self.offset];
            if is_space(byte) || byte == b',' {
                self.offset += 1;
                continue;
            }
            if byte == b'#' {
                while self.offset < self.source.len() && self.source[self.offset] != b'\n' {
                    self.offset += 1;
                }
                continue;
            }
            break;
        }
        if self.offset >= self.source.len() {
            return (Token { kind: TokenKind::Eof, text: String::new(), offset: self.offset }, None);
        }
        let start = self.offset;
        let byte = self.source[self.offset];
        if b"{}()[]:$!@=".contains(&byte) {
            self.offset += 1;
            return (Token { kind: TokenKind::Punct, text: (byte as char).to_string(), offset: start }, None);
        }
        if byte == b'"' {
            self.offset += 1;
            while self.offset < self.source.len() {
                if self.source[self.offset] == b'\\' {
                    self.offset += 2;
                    continue;
                }
                if self.source[self.offset] == b'"' {
                    self.offset += 1;
                    let raw = String::from_utf8_lossy(&self.source[start..self.offset]).into_owned();
                    return match unquote(&raw) {
                        Ok(value) => (Token { kind: TokenKind::Str, text: value, offset: start }, None),
                        Err(error) => (Token { kind: TokenKind::Str, text: String::new(), offset: start }, Some(error)),
                    };
                }
                self.offset += 1;
            }
            return (Token::default(), Some(ParseError { kind: ParseErrorKind::UnterminatedString, message: format!("unterminated string at {start}"), offset: Some(start) }));
        }
        if byte == b'-' || byte.is_ascii_digit() {
            self.offset += 1;
            while self.offset < self.source.len() && b"0123456789.eE+-".contains(&self.source[self.offset]) {
                self.offset += 1;
            }
            return (Token { kind: TokenKind::Number, text: self.slice(start, self.offset), offset: start }, None);
        }
        if byte == b'_' || is_letter(byte) {
            self.offset += 1;
            while self.offset < self.source.len() {
                let byte = self.source[self.offset];
                if byte != b'_' && !is_letter(byte) && !is_digit(byte) {
                    break;
                }
                self.offset += 1;
            }
            return (Token { kind: TokenKind::Name, text: self.slice(start, self.offset), offset: start }, None);
        }
        (Token::default(), Some(ParseError { kind: ParseErrorKind::UnexpectedCharacter, message: format!("unexpected character {} at {start}", quote_byte(byte)), offset: Some(start) }))
    }

    fn slice(&self, from: usize, to: usize) -> String {
        String::from_utf8_lossy(&self.source[from..to]).into_owned()
    }
}

/// 🔤️ The reference implementation's interpreted-string decoder, escape for escape.
fn unquote(raw: &str) -> Result<String, ParseError> {
    let invalid = || ParseError { kind: ParseErrorKind::InvalidStringSyntax, message: "invalid syntax".to_string(), offset: None };
    let bytes: Vec<char> = raw.chars().collect();
    if bytes.len() < 2 || bytes[0] != '"' || bytes[bytes.len() - 1] != '"' {
        return Err(invalid());
    }
    let body = &bytes[1..bytes.len() - 1];
    let mut out = String::new();
    let mut index = 0;
    while index < body.len() {
        let current = body[index];
        if current == '\n' || current == '"' {
            return Err(invalid());
        }
        if current != '\\' {
            out.push(current);
            index += 1;
            continue;
        }
        index += 1;
        if index >= body.len() {
            return Err(invalid());
        }
        let escape = body[index];
        index += 1;
        match escape {
            'a' => out.push('\u{7}'),
            'b' => out.push('\u{8}'),
            'f' => out.push('\u{c}'),
            'n' => out.push('\n'),
            'r' => out.push('\r'),
            't' => out.push('\t'),
            'v' => out.push('\u{b}'),
            '\\' => out.push('\\'),
            '"' => out.push('"'),
            'x' | 'u' | 'U' => {
                let width = match escape {
                    'x' => 2,
                    'u' => 4,
                    _ => 8,
                };
                if index + width > body.len() {
                    return Err(invalid());
                }
                let digits: String = body[index..index + width].iter().collect();
                index += width;
                let code = u32::from_str_radix(&digits, 16).map_err(|_| invalid())?;
                if escape == 'x' {
                    out.push(char::from(code as u8));
                } else {
                    out.push(char::from_u32(code).ok_or_else(invalid)?);
                }
            }
            '0'..='7' => {
                if index + 2 > body.len() {
                    return Err(invalid());
                }
                let digits: String = std::iter::once(escape).chain(body[index..index + 2].iter().copied()).collect();
                index += 2;
                let code = u32::from_str_radix(&digits, 8).map_err(|_| invalid())?;
                if code > 255 {
                    return Err(invalid());
                }
                out.push(char::from(code as u8));
            }
            _ => return Err(invalid()),
        }
    }
    Ok(out)
}

/// 🔤️ `%q` of a single byte, as the reference implementation renders it in diagnostics.
fn quote_byte(byte: u8) -> String {
    match byte {
        b'\'' => "'\\''".to_string(),
        b'\\' => "'\\\\'".to_string(),
        0x07 => "'\\a'".to_string(),
        0x08 => "'\\b'".to_string(),
        0x0C => "'\\f'".to_string(),
        b'\n' => "'\\n'".to_string(),
        b'\r' => "'\\r'".to_string(),
        b'\t' => "'\\t'".to_string(),
        0x0B => "'\\v'".to_string(),
        0x20..=0x7E => format!("'{}'", byte as char),
        0xA0..=0xFF => format!("'{}'", char::from(byte)),
        _ => format!("'\\x{byte:02x}'"),
    }
}

/// 🔤️ `%q` of a string, as the reference implementation renders it in diagnostics.
fn quote_string(value: &str) -> String {
    let mut out = String::from("\"");
    for character in value.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(character),
        }
    }
    out.push('"');
    out
}

//#endregion 🔤️Lexer

//#region 🌳️Ast

/// 🌳️ One parsed operation: its kind and its root selection set.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Document {
    pub operation: String,
    pub selections: Vec<Selection>,
}

/// 🌳️ One selected field, its alias, its arguments and its sub-selection.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Selection {
    pub alias: Option<String>,
    pub name: String,
    pub arguments: BTreeMap<String, Value>,
    pub fields: Vec<Selection>,
}

/// 🌳️ One literal constant. `None` on `Value::literal` is the reference implementation's nil,
/// which is both the `null` literal and the zero value.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Literal {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
}

/// 🌳️ One argument value: a variable reference, a list, an input object or a literal.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Value {
    pub literal: Option<Literal>,
    pub variable: Option<String>,
    pub list: Option<Vec<Value>>,
    pub object: Option<BTreeMap<String, Value>>,
}

impl Literal {
    /// 🌳️ The literal as JSON, the way the executor hands it to a resolver.
    pub fn to_json(&self) -> Json {
        match self {
            Literal::Bool(value) => Json::Bool(*value),
            Literal::Int(value) => json!(value),
            Literal::Float(value) => json!(value),
            Literal::Str(value) => Json::String(value.clone()),
        }
    }
}

impl Value {
    /// 🔢️ Substitutes variables into a parsed value, recursing through lists and input objects.
    pub fn resolve(&self, variables: &Map<String, Json>) -> Json {
        if let Some(name) = &self.variable {
            return variables.get(name).cloned().unwrap_or(Json::Null);
        }
        if let Some(items) = &self.list {
            return Json::Array(items.iter().map(|item| item.resolve(variables)).collect());
        }
        if let Some(fields) = &self.object {
            return Json::Object(fields.iter().map(|(name, item)| (name.clone(), item.resolve(variables))).collect());
        }
        self.literal.as_ref().map_or(Json::Null, Literal::to_json)
    }

    /// 🖼️ Canonical, language-neutral projection of one value, as `🧬️schema/🔣️.json` defines it.
    pub fn projection(&self) -> Json {
        if let Some(name) = &self.variable {
            return json!({ "kind": "variable", "name": name });
        }
        if let Some(items) = &self.list {
            return json!({ "kind": "list", "items": items.iter().map(Value::projection).collect::<Vec<_>>() });
        }
        if let Some(fields) = &self.object {
            return json!({ "kind": "object", "fields": fields.iter().map(|(name, item)| json!({ "name": name, "value": item.projection() })).collect::<Vec<_>>() });
        }
        match &self.literal {
            None => json!({ "kind": "null" }),
            Some(literal) => json!({ "kind": "literal", "value": literal.to_json() }),
        }
    }
}

impl Selection {
    /// 🔑️ The result key one selection writes under — its alias when it has one.
    pub fn key(&self) -> &str {
        self.alias.as_deref().unwrap_or(&self.name)
    }

    /// 🖼️ Canonical projection of one selection, with arguments sorted by name.
    pub fn projection(&self) -> Json {
        json!({
            "name": self.name,
            "alias": self.alias,
            "arguments": self.arguments.iter().map(|(name, value)| json!({ "name": name, "value": value.projection() })).collect::<Vec<_>>(),
            "fields": self.fields.iter().map(Selection::projection).collect::<Vec<_>>(),
        })
    }
}

impl Document {
    /// 🧩️ Parses one document, the single entry point the executor is expected to call.
    pub fn parse(source: &str) -> Result<Document, ParseError> {
        parse(source)
    }

    /// 🖼️ Canonical projection of one document — the shape every implementation and the oracle emit.
    pub fn projection(&self) -> Json {
        json!({ "operation": self.operation, "selections": self.selections.iter().map(Selection::projection).collect::<Vec<_>>() })
    }
}

/// 🔢️ Coerces one selection's arguments, filling declared defaults only for absent argument names.
pub fn coerce_arguments(arguments: &BTreeMap<String, Value>, defaults: &Map<String, Json>, variables: &Map<String, Json>) -> Map<String, Json> {
    let mut args: Map<String, Json> = arguments.iter().map(|(name, value)| (name.clone(), value.resolve(variables))).collect();
    for (name, fallback) in defaults {
        if !args.contains_key(name) && !fallback.is_null() {
            args.insert(name.clone(), fallback.clone());
        }
    }
    args
}

//#endregion 🌳️Ast

//#region 🧩️Parser

/// ❌️ The closed set of grammar failures, so a caller can branch without matching on prose.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParseErrorKind {
    UnterminatedString,
    UnexpectedCharacter,
    InvalidStringSyntax,
    InvalidNumberSyntax,
    ExpectedToken,
    UnexpectedToken,
    UnterminatedSelectionSet,
    ExpectedFieldName,
    ExpectedAliasedFieldName,
    ExpectedArgumentName,
    ExpectedVariableName,
    ExpectedObjectField,
    InvalidValue,
}

/// ❌️ One grammar failure: its class, the reference implementation's exact prose and its offset.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub message: String,
    pub offset: Option<usize>,
}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for ParseError {}

/// 🧩️ A single-token-lookahead recursive descent parser over the lexer.
struct Parser {
    lexer: Lexer,
    current: Token,
    error: Option<ParseError>,
}

impl Parser {
    fn new(source: &str) -> Self {
        let mut parser = Self { lexer: Lexer::new(source), current: Token::default(), error: None };
        parser.advance();
        parser
    }

    fn advance(&mut self) {
        if self.error.is_some() {
            return;
        }
        let (token, error) = self.lexer.next_token();
        self.current = token;
        self.error = error;
    }

    fn take(&mut self, text: &str) -> Result<(), ParseError> {
        if let Some(error) = &self.error {
            return Err(error.clone());
        }
        if self.current.text != text {
            return Err(ParseError {
                kind: ParseErrorKind::ExpectedToken,
                message: format!("expected {} at {}, got {}", quote_string(text), self.current.offset, quote_string(&self.current.text)),
                offset: Some(self.current.offset),
            });
        }
        self.advance();
        Ok(())
    }

    /// 🧩️ Skips a balanced parenthesised group — variable definitions and directive arguments.
    fn skip_parenthesized(&mut self) {
        let mut depth = 0i32;
        while self.current.kind != TokenKind::Eof {
            if self.current.text == "(" {
                depth += 1;
            }
            if self.current.text == ")" {
                depth -= 1;
                self.advance();
                if depth == 0 {
                    break;
                }
                continue;
            }
            self.advance();
        }
    }

    fn selection_set(&mut self) -> Result<Vec<Selection>, ParseError> {
        self.take("{")?;
        let mut selections = Vec::new();
        while self.current.text != "}" {
            if self.current.kind == TokenKind::Eof {
                return Err(ParseError { kind: ParseErrorKind::UnterminatedSelectionSet, message: "unterminated selection set".to_string(), offset: None });
            }
            if self.current.kind != TokenKind::Name {
                return Err(ParseError { kind: ParseErrorKind::ExpectedFieldName, message: format!("expected field name at {}", self.current.offset), offset: Some(self.current.offset) });
            }
            let mut item = Selection { name: self.current.text.clone(), ..Selection::default() };
            self.advance();
            if self.current.text == ":" {
                self.advance();
                item.alias = Some(item.name.clone());
                if self.current.kind != TokenKind::Name {
                    return Err(ParseError { kind: ParseErrorKind::ExpectedAliasedFieldName, message: "expected aliased field name".to_string(), offset: None });
                }
                item.name = self.current.text.clone();
                self.advance();
            }
            if self.current.text == "(" {
                self.advance();
                while self.current.text != ")" {
                    if self.current.kind != TokenKind::Name {
                        return Err(ParseError { kind: ParseErrorKind::ExpectedArgumentName, message: format!("expected argument name at {}", self.current.offset), offset: Some(self.current.offset) });
                    }
                    let name = self.current.text.clone();
                    self.advance();
                    self.take(":")?;
                    let argument = self.value()?;
                    item.arguments.insert(name, argument);
                }
                self.advance();
            }
            while self.current.text == "@" {
                self.advance();
                if self.current.kind == TokenKind::Name {
                    self.advance();
                }
                if self.current.text == "(" {
                    self.skip_parenthesized();
                }
            }
            if self.current.text == "{" {
                item.fields = self.selection_set()?;
            }
            selections.push(item);
        }
        self.advance();
        Ok(selections)
    }

    fn value(&mut self) -> Result<Value, ParseError> {
        let current = self.current.clone();
        if current.text == "$" {
            self.advance();
            if self.current.kind != TokenKind::Name {
                return Err(ParseError { kind: ParseErrorKind::ExpectedVariableName, message: "expected variable name".to_string(), offset: None });
            }
            let result = Value { variable: Some(self.current.text.clone()), ..Value::default() };
            self.advance();
            return Ok(result);
        }
        if current.text == "[" {
            self.advance();
            let mut result = Value::default();
            while self.current.text != "]" {
                let item = self.value()?;
                result.list.get_or_insert_with(Vec::new).push(item);
            }
            self.advance();
            return Ok(result);
        }
        if current.text == "{" {
            self.advance();
            let mut result = Value { object: Some(BTreeMap::new()), ..Value::default() };
            while self.current.text != "}" {
                if self.current.kind != TokenKind::Name {
                    return Err(ParseError { kind: ParseErrorKind::ExpectedObjectField, message: "expected object field".to_string(), offset: None });
                }
                let name = self.current.text.clone();
                self.advance();
                self.take(":")?;
                let item = self.value()?;
                result.object.get_or_insert_with(BTreeMap::new).insert(name, item);
            }
            self.advance();
            return Ok(result);
        }
        match current.kind {
            TokenKind::Str => {
                self.advance();
                Ok(Value { literal: Some(Literal::Str(current.text)), ..Value::default() })
            }
            TokenKind::Number => {
                self.advance();
                if current.text.contains(['.', 'e', 'E']) {
                    let parsed = current.text.parse::<f64>().map_err(|_| ParseError {
                        kind: ParseErrorKind::InvalidNumberSyntax,
                        message: format!("strconv.ParseFloat: parsing {}: invalid syntax", quote_string(&current.text)),
                        offset: Some(current.offset),
                    })?;
                    Ok(Value { literal: Some(Literal::Float(parsed)), ..Value::default() })
                } else {
                    let parsed = current.text.parse::<i64>().map_err(|_| ParseError {
                        kind: ParseErrorKind::InvalidNumberSyntax,
                        message: format!("strconv.Atoi: parsing {}: invalid syntax", quote_string(&current.text)),
                        offset: Some(current.offset),
                    })?;
                    Ok(Value { literal: Some(Literal::Int(parsed)), ..Value::default() })
                }
            }
            TokenKind::Name => {
                self.advance();
                match current.text.as_str() {
                    "true" => Ok(Value { literal: Some(Literal::Bool(true)), ..Value::default() }),
                    "false" => Ok(Value { literal: Some(Literal::Bool(false)), ..Value::default() }),
                    "null" => Ok(Value::default()),
                    _ => Ok(Value { literal: Some(Literal::Str(current.text)), ..Value::default() }),
                }
            }
            _ => Err(ParseError { kind: ParseErrorKind::InvalidValue, message: format!("invalid value at {}", current.offset), offset: Some(current.offset) }),
        }
    }
}

/// 🧩️ Parses one document: an optional operation header followed by the root selection set.
pub fn parse(source: &str) -> Result<Document, ParseError> {
    let mut parser = Parser::new(source);
    let mut document = Document { operation: "query".to_string(), selections: Vec::new() };
    if parser.current.kind == TokenKind::Name && (parser.current.text == "query" || parser.current.text == "mutation") {
        document.operation = parser.current.text.clone();
        parser.advance();
        if parser.current.kind == TokenKind::Name {
            parser.advance();
        }
        if parser.current.text == "(" {
            parser.skip_parenthesized();
        }
    }
    document.selections = parser.selection_set()?;
    if let Some(error) = parser.error {
        return Err(error);
    }
    if parser.current.kind != TokenKind::Eof {
        return Err(ParseError { kind: ParseErrorKind::UnexpectedToken, message: format!("unexpected token {} at {}", quote_string(&parser.current.text), parser.current.offset), offset: Some(parser.current.offset) });
    }
    Ok(document)
}

//#endregion 🧩️Parser

//#region ✅️Validation

/// ✅️ Reports whether a request string is a document this grammar accepts.
pub fn validate(source: &str) -> Result<(), ParseError> {
    parse(source).map(|_| ())
}

/// ✅️ The operation kind of a request string, without executing it.
pub fn operation_type(source: &str) -> Result<String, ParseError> {
    parse(source).map(|document| document.operation)
}

//#endregion ✅️Validation


//#region 📜️Schema

/// 🗺️ A type reference in a field, argument or input-field position. Mirrors the reference
/// implementation's `NonNull`/`List` wrappers around a named type, except that the name is
/// resolved against [`Schema::types`] instead of a pointer, so the graph may be cyclic without a
/// thunk.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeRef {
    /// 🏷️ A named type, resolved against the schema's type table.
    Named(String),
    /// ❗️ A wrapper asserting the value is never null.
    NonNull(Box<TypeRef>),
    /// 📃️ A wrapper for a homogeneous sequence.
    List(Box<TypeRef>),
}

impl TypeRef {
    /// 🏷️ The innermost named type, ignoring every wrapper.
    pub fn type_name(&self) -> &str {
        match self {
            TypeRef::Named(name) => name,
            TypeRef::NonNull(inner) | TypeRef::List(inner) => inner.type_name(),
        }
    }

    /// 📜️ The SDL rendering of this reference.
    pub fn render(&self) -> String {
        match self {
            TypeRef::Named(name) => name.clone(),
            TypeRef::NonNull(inner) => format!("{}!", inner.render()),
            TypeRef::List(inner) => format!("[{}]", inner.render()),
        }
    }
}

/// 🏷️ A named type reference.
fn ty(name: &str) -> TypeRef {
    TypeRef::Named(name.to_string())
}

/// ❗️ Wraps a reference as non-null.
fn nn(inner: TypeRef) -> TypeRef {
    TypeRef::NonNull(Box::new(inner))
}

/// 📃️ Wraps a reference as a list.
fn lst(inner: TypeRef) -> TypeRef {
    TypeRef::List(Box::new(inner))
}

/// 📃️ The `[T!]!` shape every collection field of this schema uses.
fn nn_list(inner: TypeRef) -> TypeRef {
    nn(lst(nn(inner)))
}

/// 🎚️ One declared field argument and its default.
#[derive(Clone, Debug, PartialEq)]
pub struct Argument {
    /// 🏷️ The argument name.
    pub name: String,
    /// 🗺️ The declared argument type.
    pub ty: TypeRef,
    /// 🎁️ The declared default, `Json::Null` when there is none.
    pub default: Json,
}

/// 🧩️ One selectable field of an object or interface type.
#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    /// 🏷️ The field name.
    pub name: String,
    /// 🗺️ The declared field type.
    pub ty: TypeRef,
    /// 🎚️ The declared arguments, in declaration order.
    pub args: Vec<Argument>,
    /// 🔑️ The resolver key, or `None` when the field is read off the source.
    pub resolver: Option<String>,
}

impl Field {
    /// 🧩️ A field resolved by reading the source object.
    fn new(name: &str, ty: TypeRef) -> Field {
        Field { name: name.to_string(), ty, args: Vec::new(), resolver: None }
    }

    /// 🔑️ A field resolved by the named resolver.
    fn resolved(name: &str, ty: TypeRef, resolver: &str) -> Field {
        Field { name: name.to_string(), ty, args: Vec::new(), resolver: Some(resolver.to_string()) }
    }

    /// 🎚️ Declares one argument without a default.
    fn arg(mut self, name: &str, ty: TypeRef) -> Field {
        self.args.push(Argument { name: name.to_string(), ty, default: Json::Null });
        self
    }
}

/// 🏛️ A composite type with a field table.
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectType {
    /// 🏷️ The type name.
    pub name: String,
    /// 🎭️ The interfaces this object declares.
    pub interfaces: Vec<String>,
    /// 🧩️ The field table.
    pub fields: Vec<Field>,
}

impl ObjectType {
    /// 🧩️ Looks one field up by name.
    pub fn field(&self, name: &str) -> Option<&Field> {
        self.fields.iter().find(|field| field.name == name)
    }
}

/// 🎭️ An abstract type resolved to a concrete object at execution time.
#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceType {
    /// 🏷️ The type name.
    pub name: String,
    /// 🧩️ The field table every implementor carries.
    pub fields: Vec<Field>,
}

/// 🎭️ A closed set of object types resolved at execution time.
#[derive(Clone, Debug, PartialEq)]
pub struct UnionType {
    /// 🏷️ The type name.
    pub name: String,
    /// 🏛️ The member object type names.
    pub types: Vec<String>,
}

/// 🔢️ A closed set of named constants and the wire value each stands for.
#[derive(Clone, Debug, PartialEq)]
pub struct EnumType {
    /// 🏷️ The type name.
    pub name: String,
    /// 🔢️ Member name to wire value, in declaration order.
    pub values: Vec<(String, String)>,
}

/// 📥️ A composite type accepted as an argument.
#[derive(Clone, Debug, PartialEq)]
pub struct InputObjectType {
    /// 🏷️ The type name.
    pub name: String,
    /// 📥️ The input field table, in declaration order.
    pub fields: Vec<(String, TypeRef)>,
}

/// 📜️ Anything nameable in the owned schema vocabulary.
#[derive(Clone, Debug, PartialEq)]
pub enum NamedType {
    /// 🔤️ A leaf type serialised straight into the result.
    Scalar(String),
    /// 🏛️ A composite type with a field table.
    Object(ObjectType),
    /// 🎭️ An abstract type with a field table.
    Interface(InterfaceType),
    /// 🎭️ A closed set of object types.
    Union(UnionType),
    /// 🔢️ A closed set of named constants.
    Enum(EnumType),
    /// 📥️ A composite argument type.
    InputObject(InputObjectType),
}

impl NamedType {
    /// 🏷️ The name this type is registered under.
    pub fn name(&self) -> &str {
        match self {
            NamedType::Scalar(name) => name,
            NamedType::Object(object) => &object.name,
            NamedType::Interface(interface) => &interface.name,
            NamedType::Union(union) => &union.name,
            NamedType::Enum(enumeration) => &enumeration.name,
            NamedType::InputObject(input) => &input.name,
        }
    }
}

/// 🗺️ The executable schema: a type table plus the operation roots.
#[derive(Clone, Debug, PartialEq)]
pub struct Schema {
    /// 📚️ Every declared type, by name.
    pub types: BTreeMap<String, NamedType>,
    /// 🔍️ The query root type name.
    pub query: String,
    /// ✍️ The mutation root type name, when one is configured.
    pub mutation: Option<String>,
}

impl Schema {
    /// 🏛️ Looks one object type up by name.
    pub fn object(&self, name: &str) -> Option<&ObjectType> {
        match self.types.get(name) {
            Some(NamedType::Object(object)) => Some(object),
            _ => None,
        }
    }
}

/// 🔤️ The five scalar names the repo schema is built from.
const SCALARS: &[&str] = &["String", "Int", "Boolean", "ID", "DateTime"];

//#endregion 📜️Schema

//#region 🎥️ContextPort

/// ❌️ Every failure a context operation can report.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContextError {
    /// 📝️ The failure text, reproduced verbatim into the execution error chain.
    pub message: String,
}

impl ContextError {
    /// ❌️ Builds a context failure from any displayable value.
    pub fn new(message: impl Into<String>) -> ContextError {
        ContextError { message: message.into() }
    }
}

impl fmt::Display for ContextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for ContextError {}

/// 🔌️ The port every repo aggregate is reached through.
///
/// The reference implementation declares this interface next to the domain model, because its
/// executor lives in the same package as every aggregate. Here the port sits in the module that
/// *needs* it: `📐️model` is at the bottom of the dependency DAG and no crate below `🔗️graphql`
/// ever calls a context, so placing the trait here keeps the DAG acyclic without a second
/// abstraction. Both placements are behaviourally identical — the method set is the same.
pub trait RepoContext {
    /// 📁️ The absolute repository root every file URI is built against.
    fn root_dir(&self) -> String;
    /// 📜️ Every technology of the repository.
    fn technologies(&self) -> Vec<Technology>;
    /// 🔵️ Every bundle of the repository.
    fn bundles(&self) -> Vec<Bundle>;
    /// ✔️ The most recent checkpoints, newest first, capped by `limit` when given.
    fn checkpoints(&self, limit: Option<i64>) -> Result<Vec<Checkpoint>, ContextError>;
    /// 🩷️ Every folder of the repository.
    fn folders(&self) -> Vec<Folder>;
    /// 📄️ Every file of the repository.
    fn files(&self) -> Vec<File>;
    /// 💗️ Every section of the repository.
    fn sections(&self) -> Vec<Section>;
    /// 💕️ Every definition of the repository.
    fn definitions(&self) -> Vec<Definition>;
    /// 🧑️‍💻️ Every contributor of the repository.
    fn contributors(&self) -> Result<Vec<Contributor>, ContextError>;
    /// 🎯️ Every goal of the repository.
    fn goals(&self) -> Result<Vec<Goal>, ContextError>;
    /// 🎫️ Every ticket matching the date and status filters.
    fn tickets(&self, year: Option<i64>, month: Option<i64>, day: Option<i64>, status: Option<TicketStatus>) -> Result<Vec<Ticket>, ContextError>;
    /// 👮️ Every policy of the repository.
    fn policies(&self) -> Vec<Policy>;
    /// 📝️ Every draft of the repository.
    fn drafts(&self) -> Result<Vec<Draft>, ContextError>;
    /// ✅️ Every todo matching the filter.
    fn todos(&self, filter: Option<&FilterInput>) -> Result<Vec<Todo>, ContextError>;
    /// 📜️ Every declared statute.
    fn statutes(&self) -> Vec<StatuteMeta>;
    /// 💬️ Every recorded interaction, flattened with its source artifact.
    fn interactions(&self) -> Result<Vec<InteractionResource>, ContextError>;
    /// 🔬️ Runs the analyzer over an optional scope.
    fn analyze(&self, scope: Option<&str>) -> Result<AnalyzeResult, ContextError>;
    /// 📩️ Creates a goal.
    fn goal_create(&self, input: GoalCreateInput) -> Result<Goal, ContextError>;
    /// 📐️ Changes a goal.
    fn goal_change(&self, input: GoalChangeInput) -> Result<Goal, ContextError>;
    /// 🏁️ Closes a goal.
    fn goal_close(&self, input: GoalCloseInput) -> Result<Goal, ContextError>;
    /// 🎯️ Reopens a goal.
    fn goal_reopen(&self, input: GoalReopenInput) -> Result<Goal, ContextError>;
    /// 🔸️ Deletes a goal.
    fn goal_delete(&self, input: GoalDeleteInput) -> Result<bool, ContextError>;
    /// 🆕️ Creates a todo.
    fn todo_create(&self, input: TodoCreateInput) -> Result<Todo, ContextError>;
    /// ✏️ Changes a todo.
    fn todo_change(&self, input: TodoChangeInput) -> Result<Todo, ContextError>;
    /// 🔹️ Deletes a todo.
    fn todo_delete(&self, id: &str) -> Result<bool, ContextError>;
    /// 🆕️ Creates a draft.
    fn draft_create(&self, input: DraftCreateInput) -> Result<Draft, ContextError>;
    /// 🗑️ Deletes a draft.
    fn draft_delete(&self, id: &str) -> Result<bool, ContextError>;
    /// 📬️ Opens a ticket.
    fn ticket_open(&self, input: TicketOpenInput) -> Result<Ticket, ContextError>;
    /// 📪️ Closes a ticket.
    fn ticket_close(&self, input: TicketCloseInput) -> Result<Ticket, ContextError>;
    /// 🔓️ Reopens a ticket.
    fn ticket_reopen(&self, input: TicketReopenInput) -> Result<Ticket, ContextError>;
    /// ♻️ Changes a ticket.
    fn ticket_change(&self, input: TicketChangeInput) -> Result<Ticket, ContextError>;
    /// 🔺️ Deletes a ticket.
    fn ticket_delete(&self, input: TicketDeleteInput) -> Result<bool, ContextError>;
    /// ➕️ Adds a contributor.
    fn contributor_add(&self, input: ContributorAddInput) -> Result<Contributor, ContextError>;
    /// 🚚️ Removes a contributor.
    fn contributor_remove(&self, github: &str) -> Result<(), ContextError>;
    /// 📁️ Creates a folder.
    fn folder_create(&self, path: &str) -> Result<Folder, ContextError>;
    /// 📄️ Moves a folder.
    fn folder_move(&self, source: &str, destination: &str) -> Result<Folder, ContextError>;
    /// 🔻️ Deletes a folder.
    fn folder_delete(&self, path: &str) -> Result<(), ContextError>;
    /// ⬛️ Creates a file.
    fn file_create(&self, path: &str) -> Result<File, ContextError>;
    /// ⬜️ Moves a file.
    fn file_move(&self, source: &str, destination: &str) -> Result<File, ContextError>;
    /// 🟥️ Deletes a file.
    fn file_delete(&self, path: &str) -> Result<(), ContextError>;
    /// 📑️ Creates a section.
    fn section_create(&self, file: &str, name: &str, parent: Option<&str>) -> Result<Section, ContextError>;
    /// 🟧️ Moves a section.
    fn section_move(&self, file: &str, old_name: &str, new_name: &str) -> Result<Section, ContextError>;
    /// 🟨️ Deletes a section.
    fn section_delete(&self, file: &str, name: &str) -> Result<(), ContextError>;
    /// 🧬️ Integrates a source into a target section.
    fn integrate(&self, source: Option<&str>, target_section: Option<&str>, target_file: Option<&str>, target_parent: Option<&str>) -> Result<File, ContextError>;
    /// 🧲️ Extracts a section into a target file.
    fn extract(&self, source_file: Option<&str>, source_section: Option<&str>, target_file: Option<&str>) -> Result<File, ContextError>;
    /// 🐙️ Synchronises the management provider.
    fn sync_management(&self) -> Result<bool, ContextError>;
}

//#endregion 🎥️ContextPort

//#region 🖼️Sources

/// 🖼️ Serialises a domain value into the JSON the executor reads fields off.
fn to_json<T: Serialize>(value: &T) -> Json {
    serde_json::to_value(value).unwrap_or(Json::Null)
}

/// 🖼️ The object body of a serialised domain value, empty when it is not an object.
fn body<T: Serialize>(value: &T) -> Map<String, Json> {
    match to_json(value) {
        Json::Object(fields) => fields,
        _ => Map::new(),
    }
}

/// 🏷️ Stamps the concrete type name an interface or union member is discriminated by.
fn tagged(mut fields: Map<String, Json>, typename: &str) -> Json {
    fields.insert("__typename".to_string(), Json::String(typename.to_string()));
    Json::Object(fields)
}

/// 📁️ Joins a repository-relative path onto the root, always with forward slashes.
fn file_uri(root_dir: &str, relative: &str) -> String {
    let root = root_dir.replace('\\', "/");
    let tail = relative.replace('\\', "/");
    if tail.is_empty() {
        format!("file://{root}")
    } else {
        format!("file://{}/{}", root.trim_end_matches('/'), tail.trim_start_matches('/'))
    }
}

/// 🔤️ Title-cases one dash separated slug, word by word.
fn titleize_slug(slug: &str) -> String {
    slug.split('-')
        .map(|word| match word.chars().next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + &word[first.len_utf8()..].to_lowercase(),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// 📜️ The identifier value of a statute path: every segment titleized, joined with `#`.
fn statute_path_to_id_value(path: &str) -> String {
    path.split('/').map(titleize_slug).collect::<Vec<_>>().join("#")
}

/// 🏷️ The emoji a technology name derives when the record carries none.
fn technology_emoji(name: &str) -> String {
    if name.contains("repo") {
        entity("technology-infrastructure").to_string()
    } else if name.starts_with("coda") {
        entity("technology-research").to_string()
    } else {
        String::new()
    }
}

/// 📜️ The identifier of a technology.
fn technology_id(technology: &Technology) -> String {
    let emoji = if technology.emoji.is_empty() { technology_emoji(&technology.name) } else { technology.emoji.clone() };
    emoji_text(&emoji) + &flat(&technology.name)
}

/// 🔵️ The emoji a bundle kind stands for.
fn bundle_kind_emoji(kind: BundleKind) -> &'static str {
    match kind {
        BundleKind::Library => entity("bundle-library"),
        BundleKind::Schema => entity("bundle-schema"),
        BundleKind::Binary => entity("bundle-binary"),
        BundleKind::Ui => entity("bundle-ui"),
        BundleKind::Site => entity("bundle-site"),
        BundleKind::Assets => entity("bundle-assets"),
        BundleKind::Repo => entity("bundle-repo"),
    }
}

/// 🔵️ The identifier of a bundle: technology emoji and code, then bundle emoji and code.
fn bundle_id(bundle: &Bundle, technologies: &[Technology]) -> String {
    let emoji = if bundle.emoji.is_empty() { bundle_kind_emoji(bundle.kind).to_string() } else { bundle.emoji.clone() };
    let (technology_code, bundle_code) = match bundle.name.split_once('/') {
        Some((head, tail)) => (head.to_string(), tail.to_string()),
        None => (bundle.name.clone(), bundle.name.clone()),
    };
    let technology_emoji_value = technologies
        .iter()
        .find(|candidate| candidate.name == technology_code && !candidate.emoji.is_empty())
        .map_or_else(|| derive_technology_kind(&technology_code).as_str().to_string(), |candidate| candidate.emoji.clone());
    emoji_text(&technology_emoji_value) + &flat(&technology_code) + &emoji_text(&emoji) + &flat(&bundle_code)
}

/// 💗️ The identifier of a section, falling back to the emoji form when it carries none.
fn section_id(section: &Section) -> String {
    if !section.id.is_empty() {
        return section.id.clone();
    }
    let emoji = if section.emoji.is_empty() { entity("section").to_string() } else { section.emoji.clone() };
    emoji_text(&emoji) + &flat(&section.name)
}

/// 💕️ The emoji a definition kind stands for.
fn definition_kind_emoji(kind: DefinitionKind) -> &'static str {
    match kind {
        DefinitionKind::Implementation => entity("definition-impl"),
        DefinitionKind::Interface => entity("definition-interface"),
        DefinitionKind::Constant => entity("definition-constant"),
        DefinitionKind::Test => entity("definition-test"),
    }
}

/// 💕️ The identifier of a definition, falling back to the emoji form when it carries none.
fn definition_id(definition: &Definition) -> String {
    if !definition.id.is_empty() {
        return definition.id.clone();
    }
    emoji_text(definition_kind_emoji(definition.kind)) + &flat(&definition.name)
}

/// 🕰️ Normalises a recorded timestamp into the RFC 3339 rendering the `DateTime` scalar emits.
fn to_rfc3339(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.len() >= 19 && trimmed.is_char_boundary(10) && (trimmed.as_bytes()[10] == b' ' || trimmed.as_bytes()[10] == b'T') {
        let (date, rest) = trimmed.split_at(10);
        let time = rest[1..].split(['+', 'Z', '.']).next().unwrap_or("");
        if time.len() == 8 {
            return Some(format!("{date}T{time}Z"));
        }
    }
    None
}

/// 🕰️ Midnight UTC of a year/month/day triple, the zero date a ticket falls back to.
fn midnight(year: i64, month: i64, day: i64) -> String {
    format!("{year:04}-{month:02}-{day:02}T00:00:00Z")
}

/// 🎫️ Whether an interaction kind is the expected one, tolerating the `.ended` suffix.
fn is_ticket_interaction_kind(kind: &str, expected: &str) -> bool {
    let kind = kind.trim();
    kind == expected.trim() || kind.strip_suffix(".ended").is_some_and(|head| head == expected.trim())
}

/// 🎫️ The date a ticket started: its open interaction, else its first, else midnight.
fn ticket_started(ticket: &Ticket) -> String {
    for interaction in &ticket.interactions {
        if is_ticket_interaction_kind(&interaction.kind, "ticket.open") {
            if let Some(date) = to_rfc3339(&interaction.date) {
                return date;
            }
        }
    }
    if let Some(first) = ticket.interactions.first() {
        if let Some(date) = to_rfc3339(&first.date) {
            return date;
        }
    }
    midnight(ticket.year, ticket.month, ticket.day)
}

/// 🎫️ The date a ticket finished: its last close interaction, or none.
fn ticket_finished(ticket: &Ticket) -> Option<String> {
    ticket.interactions.iter().rev().find(|interaction| is_ticket_interaction_kind(&interaction.kind, "ticket.close")).and_then(|interaction| to_rfc3339(&interaction.date))
}

/// 🎫️ The last interaction's field, then the last agent's, then blank.
fn ticket_latest(ticket: &Ticket, pick: fn(&Interaction) -> &str, fallback: fn(&TicketAgent) -> &str) -> String {
    if let Some(interaction) = ticket.interactions.last() {
        return pick(interaction).to_string();
    }
    ticket.agents.last().map(fallback).unwrap_or_default().to_string()
}

/// 🎫️ The author of a ticket: its first interaction's, then its first agent's.
fn ticket_author(ticket: &Ticket) -> String {
    if let Some(interaction) = ticket.interactions.first() {
        return interaction.author.clone();
    }
    ticket.agents.first().map(|agent| agent.contributor.clone()).unwrap_or_default()
}

/// 🎫️ The checkpoint of a ticket: its first interaction's.
fn ticket_checkpoint(ticket: &Ticket) -> String {
    ticket.interactions.first().map(|interaction| interaction.checkpoint.clone()).unwrap_or_default()
}

/// 🔤️ Wraps a non-empty string as JSON, mapping the empty string to null.
fn optional(value: String) -> Json {
    if value.is_empty() {
        Json::Null
    } else {
        Json::String(value)
    }
}

/// 🤝️ Splits `Name <email>` the way the reference implementation does.
fn parse_git_author(value: &str) -> (String, String) {
    match value.split_once(" <") {
        Some((name, rest)) => (name.trim().to_string(), rest.trim_end_matches('>').to_string()),
        None => (value.to_string(), String::new()),
    }
}

/// 🟡️ The milestone number a management link carries.
fn parse_milestone_number(milestone: &str) -> Option<i64> {
    milestone.parse::<i64>().ok().or_else(|| milestone.rsplit('/').next().and_then(|tail| tail.parse::<i64>().ok()))
}

/// 📜️ The technology source object.
fn technology_source(technology: &Technology, technologies: &[Technology], root_dir: &str) -> Json {
    let mut fields = body(technology);
    let id = technology_id(technology);
    fields.insert("uri".to_string(), Json::String(format!("repo://technology/{id}")));
    fields.insert("id".to_string(), Json::String(id));
    fields.insert("kind".to_string(), Json::String(technology.kind.as_str().to_string()));
    let bundles = technology.bundles.clone().unwrap_or_default();
    fields.insert("bundles".to_string(), Json::Array(bundles.iter().map(|bundle| bundle_source(bundle, technologies, root_dir)).collect()));
    tagged(fields, "Technology")
}

/// 🔵️ The bundle source object.
fn bundle_source(bundle: &Bundle, technologies: &[Technology], root_dir: &str) -> Json {
    let mut fields = body(bundle);
    fields.insert("id".to_string(), Json::String(bundle_id(bundle, technologies)));
    fields.insert("uri".to_string(), Json::String(file_uri(root_dir, &bundle.root)));
    fields.insert("kind".to_string(), Json::String(bundle.kind.as_str().to_string()));
    fields.insert("tags".to_string(), Json::Array(bundle.tags.iter().cloned().map(Json::String).collect()));
    fields.insert("packages".to_string(), Json::Array(bundle.packages.iter().map(|package| tagged(body(package), "Package")).collect()));
    tagged(fields, "Bundle")
}

/// 🩷️ The folder source object.
fn folder_source(folder: &Folder) -> Json {
    let mut fields = body(folder);
    fields.insert("kind".to_string(), Json::String(folder.kind.as_str().to_string()));
    tagged(fields, "Folder")
}

/// 📄️ The file source object.
fn file_source(file: &File) -> Json {
    tagged(body(file), "File")
}

/// 📐️ The line range of a start/end pair.
fn range_source(start: i64, end: i64) -> Json {
    tagged(body(&Range { start, end }), "Range")
}

/// 💗️ The section source object, recursing through children and definitions.
fn section_source(section: &Section) -> Json {
    let mut fields = body(section);
    fields.insert("id".to_string(), Json::String(section_id(section)));
    fields.insert("range".to_string(), range_source(section.start_line, section.end_line));
    fields.insert("children".to_string(), Json::Array(section.children.iter().map(section_source).collect()));
    fields.insert("definitions".to_string(), Json::Array(section.definitions.iter().map(definition_source).collect()));
    tagged(fields, "Section")
}

/// 💕️ The definition source object.
fn definition_source(definition: &Definition) -> Json {
    let mut fields = body(definition);
    fields.insert("id".to_string(), Json::String(definition_id(definition)));
    fields.insert("kind".to_string(), Json::String(definition.kind.as_str().to_string()));
    fields.insert("range".to_string(), range_source(definition.start_line, definition.end_line));
    tagged(fields, "Definition")
}

/// 📜️ The declared metadata of one statute, or the reference implementation's unknown default.
fn statute_meta_of(kind: &Statute, statutes: &[StatuteMeta]) -> StatuteMeta {
    statutes.iter().find(|candidate| candidate.kind == *kind).cloned().unwrap_or(StatuteMeta {
        kind: kind.clone(),
        policy_id: String::new(),
        priority: BreachPriority::Low,
        reason: "Unknown breach".to_string(),
        solution: "Fix the breach".to_string(),
        autofixable: false,
    })
}

/// 📜️ The statute source object.
fn statute_source(meta: &StatuteMeta) -> Json {
    let mut fields = body(meta);
    fields.insert("id".to_string(), Json::String(emoji_text(entity("statute")) + &statute_path_to_id_value(&meta.kind.0)));
    fields.insert("priority".to_string(), Json::String(meta.priority.as_str().to_string()));
    tagged(fields, "Statute")
}

/// 🔶️ The breach source object, with the statute metadata overlaid by the breach's own overrides.
fn breach_source(breach: &Breach, statutes: &[StatuteMeta]) -> Json {
    let mut meta = statute_meta_of(&breach.kind, statutes);
    if let Some(priority) = breach.lint_priority {
        meta.priority = priority;
    }
    if let Some(autofixable) = breach.lint_autofixable {
        meta.autofixable = autofixable;
    }
    if !breach.reason.is_empty() {
        meta.reason = breach.reason.clone();
    }
    if !breach.solution.is_empty() {
        meta.solution = breach.solution.clone();
    }
    let mut fields = body(breach);
    fields.insert("id".to_string(), Json::String(emoji_text(entity("breach")) + &breach.id));
    fields.insert("kindId".to_string(), Json::String(breach.kind.0.clone()));
    fields.insert("kind".to_string(), statute_source(&meta));
    fields.insert("priority".to_string(), Json::String(meta.priority.as_str().to_string()));
    fields.insert("autofixable".to_string(), Json::Bool(meta.autofixable));
    fields.insert("line".to_string(), Json::from(breach.line));
    fields.insert("column".to_string(), Json::from(breach.column));
    fields.insert("excerpt".to_string(), optional(breach.excerpt.clone()));
    tagged(fields, "Breach")
}

/// 🟣️ The territory source object.
fn territory_source(territory: &Territory, statutes: &[StatuteMeta]) -> Json {
    let mut fields = body(territory);
    fields.insert("groups".to_string(), Json::Array(territory.groups.iter().map(|group| territory_source(group, statutes)).collect()));
    fields.insert("kinds".to_string(), Json::Array(territory.kinds.iter().map(|kind| statute_source(&statute_meta_of(kind, statutes))).collect()));
    tagged(fields, "Territory")
}

/// 👮️ The policy source object.
fn policy_source(policy: &Policy, statutes: &[StatuteMeta]) -> Json {
    let mut fields = body(policy);
    fields.insert("scopes".to_string(), Json::Array(policy.scopes.clone().unwrap_or_default().into_iter().map(Json::String).collect()));
    fields.insert("groups".to_string(), Json::Array(policy.groups.clone().unwrap_or_default().iter().map(|group| territory_source(group, statutes)).collect()));
    fields.insert("statutes".to_string(), Json::Array(policy.statutes.clone().unwrap_or_default().iter().map(statute_source).collect()));
    tagged(fields, "Policy")
}

/// 💬️ The interaction source object.
fn interaction_source(interaction: &Interaction) -> Json {
    let mut fields = body(interaction);
    for (key, value) in [("prompt", &interaction.prompt), ("summary", &interaction.summary), ("llm", &interaction.llm), ("effort", &interaction.effort)] {
        fields.insert(key.to_string(), Json::String(value.clone()));
    }
    tagged(fields, "Interaction")
}

/// 🎁️ The flattened interaction-resource source object.
fn interaction_resource_source(resource: &InteractionResource) -> Json {
    let Json::Object(mut fields) = interaction_source(&resource.interaction) else { return Json::Null };
    for (key, value) in [("goalId", &resource.goal_id), ("ticketId", &resource.ticket_id)] {
        fields.insert(key.to_string(), Json::String(value.clone()));
    }
    fields.insert("sourceKind".to_string(), Json::String(resource.source_kind.clone()));
    fields.insert("sourceId".to_string(), Json::String(resource.source_id.clone()));
    tagged(fields, "InteractionResource")
}

/// 🎫️ The ticket source object, with every derived field the reference resolvers compute.
fn ticket_source(ticket: &Ticket, root_dir: &str) -> Json {
    let mut fields = body(ticket);
    fields.insert("id".to_string(), Json::String(emoji_text(entity("ticket")) + &flat(&ticket.slug)));
    fields.insert("year".to_string(), Json::from(ticket.year));
    fields.insert("month".to_string(), Json::from(ticket.month));
    fields.insert("day".to_string(), Json::from(ticket.day));
    fields.insert("slug".to_string(), Json::String(ticket.slug.clone()));
    let path = if ticket.folder_path.is_empty() { ticket.json_path.clone() } else { ticket.folder_path.clone() };
    fields.insert("path".to_string(), Json::String(path));
    fields.insert("uri".to_string(), Json::String(file_uri(root_dir, &ticket.folder_path)));
    fields.insert("title".to_string(), Json::String(ticket.title.clone()));
    fields.insert("emoji".to_string(), optional(ticket.emoji.clone()));
    let prompt = if ticket.description.is_empty() { ticket.interactions.first().map(|interaction| interaction.prompt.clone()).unwrap_or_default() } else { ticket.description.clone() };
    fields.insert("prompt".to_string(), Json::String(prompt));
    fields.insert("summary".to_string(), Json::Null);
    fields.insert("status".to_string(), Json::String(ticket.status.as_str().to_string()));
    fields.insert("llm".to_string(), optional(ticket_latest(ticket, |interaction| &interaction.llm, |agent| &agent.llm)));
    fields.insert("effort".to_string(), optional(ticket_latest(ticket, |interaction| &interaction.effort, |agent| &agent.effort)));
    fields.insert("client".to_string(), optional(ticket_latest(ticket, |interaction| &interaction.client, |agent| &agent.client)));
    fields.insert("checkpoint".to_string(), optional(ticket_checkpoint(ticket)));
    fields.insert("goal".to_string(), optional(ticket.goal.clone()));
    fields.insert("parent".to_string(), optional(ticket.parent.clone()));
    fields.insert("interactions".to_string(), Json::Array(ticket.interactions.iter().map(interaction_source).collect()));
    let mut dates = Map::new();
    dates.insert("started".to_string(), Json::String(ticket_started(ticket)));
    dates.insert("finished".to_string(), ticket_finished(ticket).map_or(Json::Null, Json::String));
    fields.insert("dates".to_string(), tagged(dates, "TicketDate"));
    tagged(fields, "Ticket")
}

/// 🎯️ The goal source object.
fn goal_source(goal: &Goal) -> Json {
    let mut fields = body(goal);
    fields.insert("id".to_string(), Json::String(goal.id.clone()));
    fields.insert("dueDate".to_string(), optional(goal.dates.due.clone()));
    fields.insert("createdAt".to_string(), Json::Null);
    fields.insert("status".to_string(), Json::String(goal.status.as_str().to_string()));
    let management = goal.management.clone().unwrap_or_default();
    fields.insert("milestone".to_string(), parse_milestone_number(&management.milestone).map_or(Json::Null, Json::from));
    fields.insert("issue".to_string(), optional(management.issue));
    fields.insert("parent".to_string(), optional(goal.parent.clone()));
    fields.insert("interactions".to_string(), Json::Array(Vec::new()));
    tagged(fields, "Goal")
}

/// 📝️ The draft source object.
fn draft_source(draft: &Draft) -> Json {
    tagged(body(draft), "Draft")
}

/// ✅️ The todo source object.
fn todo_source(todo: &Todo) -> Json {
    let mut fields = body(todo);
    fields.insert("id".to_string(), Json::String(emoji_text(entity("todo")) + &flat(&todo.id)));
    fields.insert("description".to_string(), optional(todo.description.clone()));
    fields.insert("location".to_string(), todo.location.as_ref().map_or(Json::Null, |location| tagged(body(location), "Location")));
    tagged(fields, "Todo")
}

/// ✔️ The checkpoint source object.
fn checkpoint_source(checkpoint: &Checkpoint) -> Json {
    tagged(body(checkpoint), "Checkpoint")
}

/// 🧑️‍💻️ The contributor source object.
fn contributor_source(contributor: &Contributor) -> Json {
    let mut fields = body(contributor);
    fields.insert("id".to_string(), Json::String(emoji_text(entity("contributor")) + &flat(&contributor.alias)));
    let links = contributor.links.iter().map(|(name, url)| tagged(body(&ContributorLink { name: name.clone(), url: url.clone() }), "ContributorLink")).collect();
    fields.insert("links".to_string(), Json::Array(links));
    fields.insert("emoji".to_string(), optional(contributor.emoji.clone()));
    fields.insert("fingerprint".to_string(), optional(contributor.fingerprint.clone()));
    for (key, values) in [("names", &contributor.names), ("emails", &contributor.emails), ("fingerprints", &contributor.fingerprints), ("githubs", &contributor.githubs), ("aliases", &contributor.aliases)] {
        fields.insert(key.to_string(), Json::Array(values.iter().cloned().map(Json::String).collect()));
    }
    fields.insert("icons".to_string(), Json::Null);
    tagged(fields, "Contributor")
}

/// 🔬️ The analyze-result source object.
fn analyze_result_source(result: &AnalyzeResult, statutes: &[StatuteMeta]) -> Json {
    let mut fields = Map::new();
    fields.insert("breachs".to_string(), Json::Array(result.breachs.clone().unwrap_or_default().iter().map(|breach| breach_source(breach, statutes)).collect()));
    let metrics = result.metrics.clone().unwrap_or(AnalyzeMetrics { total: 0, by_priority: None, autofixable: 0 });
    let mut metric_fields = Map::new();
    metric_fields.insert("total".to_string(), Json::from(metrics.total));
    metric_fields.insert("autofixable".to_string(), Json::from(metrics.autofixable));
    metric_fields.insert("byPriority".to_string(), tagged(body(&metrics.by_priority.unwrap_or_default()), "PriorityCount"));
    fields.insert("metrics".to_string(), tagged(metric_fields, "AnalyzeMetrics"));
    tagged(fields, "AnalyzeResult")
}

//#endregion 🖼️Sources

//#region 🏗️SchemaCommon

/// 🔢️ The enum types the repo schema declares.
fn schema_enums() -> Vec<NamedType> {
    let enumeration = |name: &str, values: &[(&str, &str)]| {
        NamedType::Enum(EnumType { name: name.to_string(), values: values.iter().map(|(member, wire)| ((*member).to_string(), (*wire).to_string())).collect() })
    };
    vec![
        enumeration("DefinitionKind", &[("IMPLEMENTATION", "implementation"), ("INTERFACE", "interface"), ("CONSTANT", "constant")]),
        enumeration("BundleKind", &[("LIBRARY", "library"), ("SCHEMA", "schema"), ("BINARY", "binary"), ("UI", "ui"), ("SITE", "site"), ("ASSETS", "assets"), ("REPO", "repo")]),
        enumeration("FolderKind", &[("ORGANIZATION", "organization"), ("REQUIRED", "required")]),
        enumeration("TicketStatus", &[("OPEN", "open"), ("CLOSED", "closed")]),
        enumeration(
            "TicketClient",
            &[
                ("COPILOT_CHAT", "copilot-chat"),
                ("WINDSURF", "windsurf"),
                ("WINDSURF_CHAT", "windsurf-chat"),
                ("ANTIGRAVITY", "antigravity"),
                ("ANTIGRAVITY_CHAT", "antigravity-chat"),
                ("CURSOR", "cursor"),
                ("CURSOR_CHAT", "cursor-chat"),
                ("VSCODE", "vscode"),
                ("CLAUDE_CODE", "claude-code"),
                ("CODEX", "codex"),
                ("DROID", "droid"),
                ("KIRO_CLI", "kiro-cli"),
            ],
        ),
        enumeration("BreachPriority", &[("HIGH", "high"), ("MEDIUM", "medium"), ("LOW", "low")]),
        enumeration(
            "FileKind",
            &[
                ("CODE", "code"),
                ("SCRIPT", "script"),
                ("CONFIG", "config"),
                ("LAB", "lab"),
                ("DOCS", "docs"),
                ("RESOURCE", "resource"),
                ("TEMPLATE", "template"),
                ("LICENSE", "license"),
            ],
        ),
    ]
}

/// 🔢️ The metric objects every aggregate shares.
fn schema_metrics() -> Vec<NamedType> {
    vec![
        NamedType::Object(ObjectType {
            name: "Range".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("start", nn(ty("Int"))), Field::new("end", nn(ty("Int")))],
        }),
        NamedType::Object(ObjectType {
            name: "CountMetrics".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("added", nn(ty("Int"))), Field::new("updated", nn(ty("Int"))), Field::new("removed", nn(ty("Int")))],
        }),
        NamedType::Object(ObjectType {
            name: "PriorityCount".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("high", nn(ty("Int"))), Field::new("medium", nn(ty("Int"))), Field::new("low", nn(ty("Int")))],
        }),
        NamedType::Object(ObjectType {
            name: "AnalyzeMetrics".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("total", nn(ty("Int"))), Field::new("byPriority", ty("PriorityCount")), Field::new("autofixable", nn(ty("Int")))],
        }),
        NamedType::Object(ObjectType {
            name: "AnalyzeResult".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("breachs", nn_list(ty("Breach"))), Field::new("metrics", nn(ty("AnalyzeMetrics")))],
        }),
    ]
}

//#endregion 🏗️SchemaCommon

//#region 🏗️SchemaRepo

/// 💠️ The repository root type: every aggregate reachable from one query.
fn schema_repo() -> Vec<NamedType> {
    vec![NamedType::Object(ObjectType {
        name: "Repo".to_string(),
        interfaces: Vec::new(),
        fields: vec![
            Field::new("id", nn(ty("ID"))),
            Field::new("name", nn(ty("String"))),
            Field::new("path", nn(ty("String"))),
            Field::resolved("technologies", nn_list(ty("Technology")), "technologies"),
            Field::resolved("checkpoints", nn_list(ty("Checkpoint")), "checkpoints").arg("limit", ty("Int")),
            Field::resolved("bundles", nn_list(ty("Bundle")), "bundles"),
            Field::resolved("folders", nn_list(ty("Folder")), "folders"),
            Field::resolved("files", nn_list(ty("File")), "files"),
            Field::resolved("sections", nn_list(ty("Section")), "sections"),
            Field::resolved("definitions", nn_list(ty("Definition")), "definitions"),
            Field::resolved("contributors", nn_list(ty("Contributor")), "contributors"),
            Field::resolved("goals", nn_list(ty("Goal")), "goals"),
            Field::resolved("tickets", nn_list(ty("Ticket")), "tickets")
                .arg("year", ty("Int"))
                .arg("month", ty("Int"))
                .arg("day", ty("Int"))
                .arg("status", ty("TicketStatus")),
            Field::resolved("policies", nn_list(ty("Policy")), "policies"),
            Field::resolved("statutes", nn_list(ty("Statute")), "statutes"),
            Field::resolved("breachs", nn_list(ty("Breach")), "breachs").arg("scope", ty("String")),
        ],
    })]
}

//#endregion 🏗️SchemaRepo

//#region 🏗️SchemaTechnology

/// 📜️ The technology type and the package it publishes.
fn schema_technology() -> Vec<NamedType> {
    vec![
        NamedType::Object(ObjectType {
            name: "Technology".to_string(),
            interfaces: Vec::new(),
            fields: vec![
                Field::new("id", nn(ty("ID"))),
                Field::new("name", nn(ty("String"))),
                Field::new("root", nn(ty("String"))),
                Field::new("kind", nn(ty("String"))),
                Field::new("bundles", nn_list(ty("Bundle"))),
                Field::new("uri", nn(ty("String"))),
            ],
        }),
        NamedType::Object(ObjectType {
            name: "Package".to_string(),
            interfaces: Vec::new(),
            fields: vec![
                Field::new("name", nn(ty("String"))),
                Field::new("version", nn(ty("String"))),
                Field::new("path", nn(ty("String"))),
                Field::new("kind", nn(ty("String"))),
            ],
        }),
    ]
}

//#endregion 🏗️SchemaTechnology

//#region 🏗️SchemaBundle

/// 🔵️ The bundle type.
fn schema_bundle() -> Vec<NamedType> {
    vec![NamedType::Object(ObjectType {
        name: "Bundle".to_string(),
        interfaces: Vec::new(),
        fields: vec![
            Field::new("id", nn(ty("ID"))),
            Field::new("name", nn(ty("String"))),
            Field::new("root", nn(ty("String"))),
            Field::new("sourceRoot", ty("String")),
            Field::new("projectType", ty("String")),
            Field::new("tags", nn_list(ty("String"))),
            Field::new("packages", nn_list(ty("Package"))),
            Field::new("kind", nn(ty("String"))),
            Field::new("uri", nn(ty("String"))),
            Field::resolved("folders", nn_list(ty("Folder")), "empty-list"),
            Field::resolved("files", nn_list(ty("File")), "empty-list"),
            Field::resolved("breachs", nn_list(ty("Breach")), "empty-list"),
        ],
    })]
}

//#endregion 🏗️SchemaBundle

//#region 🏗️SchemaFolder

/// 🩷️ The folder type.
fn schema_folder() -> Vec<NamedType> {
    vec![NamedType::Object(ObjectType {
        name: "Folder".to_string(),
        interfaces: Vec::new(),
        fields: vec![
            Field::new("id", nn(ty("ID"))),
            Field::new("path", nn(ty("String"))),
            Field::new("uri", nn(ty("String"))),
            Field::new("name", nn(ty("String"))),
            Field::new("kind", nn(ty("String"))),
            Field::new("parent", ty("Folder")),
            Field::resolved("children", nn_list(ty("Folder")), "folder.children"),
            Field::resolved("files", nn_list(ty("File")), "folder.files"),
            Field::new("ignored", nn(ty("Boolean"))),
            Field::new("generated", nn(ty("Boolean"))),
            Field::new("bundle", ty("Bundle")),
            Field::resolved("breachs", nn_list(ty("Breach")), "empty-list"),
        ],
    })]
}

//#endregion 🏗️SchemaFolder

//#region 🏗️SchemaFile

/// 📄️ The file type.
fn schema_file() -> Vec<NamedType> {
    vec![NamedType::Object(ObjectType {
        name: "File".to_string(),
        interfaces: Vec::new(),
        fields: vec![
            Field::new("id", nn(ty("ID"))),
            Field::new("path", nn(ty("String"))),
            Field::new("uri", nn(ty("String"))),
            Field::new("name", nn(ty("String"))),
            Field::new("extension", nn(ty("String"))),
            Field::new("folder", ty("Folder")),
            Field::new("kind", nn(ty("String"))),
            Field::new("ignored", nn(ty("Boolean"))),
            Field::new("generated", nn(ty("Boolean"))),
            Field::new("bundle", ty("Bundle")),
            Field::resolved("sections", lst(ty("Section")), "file.sections"),
            Field::resolved("definitions", lst(ty("Definition")), "file.definitions"),
            Field::resolved("breachs", lst(ty("Breach")), "empty-list"),
            Field::new("content", ty("String")),
            Field::resolved("contributors", nn_list(ty("Contributor")), "empty-list"),
        ],
    })]
}

//#endregion 🏗️SchemaFile

//#region 🏗️SchemaSection

/// 💗️ The section type and the abstract item a section contains.
fn schema_section() -> Vec<NamedType> {
    vec![
        NamedType::Interface(InterfaceType {
            name: "SectionItem".to_string(),
            fields: vec![Field::new("id", nn(ty("ID"))), Field::new("name", nn(ty("String"))), Field::new("range", ty("Range"))],
        }),
        NamedType::Object(ObjectType {
            name: "Section".to_string(),
            interfaces: vec!["SectionItem".to_string()],
            fields: vec![
                Field::new("id", nn(ty("ID"))),
                Field::new("name", nn(ty("String"))),
                Field::new("path", nn(ty("String"))),
                Field::resolved("file", ty("File"), "item.file"),
                Field::new("parent", ty("Section")),
                Field::resolved("children", lst(ty("SectionItem")), "section.children"),
                Field::new("definitions", lst(ty("Definition"))),
                Field::resolved("breachs", lst(ty("Breach")), "empty-list"),
                Field::new("range", nn(ty("Range"))),
            ],
        }),
    ]
}

//#endregion 🏗️SchemaSection

//#region 🏗️SchemaDefinition

/// 💕️ The definition type.
fn schema_definition() -> Vec<NamedType> {
    vec![NamedType::Object(ObjectType {
        name: "Definition".to_string(),
        interfaces: vec!["SectionItem".to_string()],
        fields: vec![
            Field::new("id", nn(ty("ID"))),
            Field::new("name", nn(ty("String"))),
            Field::new("kind", nn(ty("DefinitionKind"))),
            Field::resolved("file", nn(ty("File")), "item.file"),
            Field::resolved("section", ty("Section"), "definition.section"),
            Field::resolved("breachs", nn_list(ty("Breach")), "empty-list"),
            Field::new("range", nn(ty("Range"))),
        ],
    })]
}

//#endregion 🏗️SchemaDefinition

//#region 🏗️SchemaStatute

/// 📜️ The statute type.
fn schema_statute() -> Vec<NamedType> {
    vec![NamedType::Object(ObjectType {
        name: "Statute".to_string(),
        interfaces: Vec::new(),
        fields: vec![
            Field::new("id", nn(ty("ID"))),
            Field::resolved("policy", nn(ty("Policy")), "statute.policy"),
            Field::new("priority", nn(ty("BreachPriority"))),
            Field::new("autofixable", nn(ty("Boolean"))),
            Field::new("reason", nn(ty("String"))),
            Field::new("solution", nn(ty("String"))),
            Field::new("breachs", nn_list(ty("Breach"))).arg("scope", ty("String")),
        ],
    })]
}

//#endregion 🏗️SchemaStatute

//#region 🏗️SchemaBreach

/// 🔶️ The breach type.
fn schema_breach() -> Vec<NamedType> {
    vec![NamedType::Object(ObjectType {
        name: "Breach".to_string(),
        interfaces: Vec::new(),
        fields: vec![
            Field::new("id", nn(ty("ID"))),
            Field::new("kindId", nn(ty("ID"))),
            Field::new("kind", nn(ty("Statute"))),
            Field::new("scope", nn(ty("String"))),
            Field::new("file", ty("File")),
            Field::new("folder", ty("Folder")),
            Field::new("line", ty("Int")),
            Field::new("column", ty("Int")),
            Field::new("excerpt", ty("String")),
            Field::new("summary", nn(ty("String"))),
            Field::new("priority", nn(ty("BreachPriority"))),
            Field::new("autofixable", nn(ty("Boolean"))),
        ],
    })]
}

//#endregion 🏗️SchemaBreach

//#region 🏗️SchemaPolicy

/// 👮️ The policy and territory types.
fn schema_policy() -> Vec<NamedType> {
    vec![
        NamedType::Object(ObjectType {
            name: "Territory".to_string(),
            interfaces: Vec::new(),
            fields: vec![
                Field::new("name", nn(ty("String"))),
                Field::new("description", nn(ty("String"))),
                Field::new("scopes", nn_list(ty("String"))),
                Field::new("groups", nn_list(ty("Territory"))),
                Field::new("kinds", nn_list(ty("Statute"))),
            ],
        }),
        NamedType::Object(ObjectType {
            name: "Policy".to_string(),
            interfaces: Vec::new(),
            fields: vec![
                Field::new("id", nn(ty("ID"))),
                Field::new("name", nn(ty("String"))),
                Field::new("description", ty("String")),
                Field::new("scopes", nn_list(ty("String"))),
                Field::new("groups", nn_list(ty("Territory"))),
                Field::new("statutes", nn_list(ty("Statute"))),
            ],
        }),
    ]
}

//#endregion 🏗️SchemaPolicy

//#region 🏗️SchemaInteraction

/// 💬️ The interaction types.
fn schema_interaction() -> Vec<NamedType> {
    vec![
        NamedType::Object(ObjectType {
            name: "Interaction".to_string(),
            interfaces: Vec::new(),
            fields: vec![
                Field::new("kind", nn(ty("String"))),
                Field::new("prompt", nn(ty("String"))),
                Field::new("checkpoint", ty("String")),
                Field::new("llm", ty("String")),
                Field::new("effort", ty("String")),
                Field::new("date", nn(ty("String"))),
                Field::new("system", nn(ty("String"))),
                Field::new("client", nn(ty("String"))),
                Field::new("author", nn(ty("String"))),
            ],
        }),
        NamedType::Object(ObjectType {
            name: "InteractionResource".to_string(),
            interfaces: Vec::new(),
            fields: vec![
                Field::new("kind", nn(ty("String"))),
                Field::new("prompt", nn(ty("String"))),
                Field::new("checkpoint", ty("String")),
                Field::new("author", nn(ty("String"))),
                Field::new("sourceKind", nn(ty("String"))),
                Field::new("sourceId", nn(ty("String"))),
                Field::new("goalId", ty("String")),
                Field::new("ticketId", ty("String")),
                Field::new("llm", ty("String")),
                Field::new("effort", ty("String")),
                Field::new("date", nn(ty("String"))),
                Field::new("system", nn(ty("String"))),
                Field::new("client", nn(ty("String"))),
            ],
        }),
    ]
}

//#endregion 🏗️SchemaInteraction

//#region 🏗️SchemaGoal

/// 🎯️ The goal type.
fn schema_goal() -> Vec<NamedType> {
    vec![NamedType::Object(ObjectType {
        name: "Goal".to_string(),
        interfaces: Vec::new(),
        fields: vec![
            Field::new("id", nn(ty("ID"))),
            Field::new("title", nn(ty("String"))),
            Field::new("description", ty("String")),
            Field::new("prompt", ty("String")),
            Field::new("dueDate", ty("String")),
            Field::new("createdAt", ty("String")),
            Field::new("client", ty("String")),
            Field::new("llm", ty("String")),
            Field::new("effort", ty("String")),
            Field::new("status", nn(ty("String"))),
            Field::new("milestone", ty("Int")),
            Field::new("issue", ty("String")),
            Field::new("parent", ty("String")),
            Field::new("interactions", nn_list(ty("Interaction"))),
        ],
    })]
}

//#endregion 🏗️SchemaGoal

//#region 🏗️SchemaDraft

/// 📝️ The draft type.
fn schema_draft() -> Vec<NamedType> {
    vec![NamedType::Object(ObjectType { name: "Draft".to_string(), interfaces: Vec::new(), fields: vec![Field::new("id", nn(ty("ID")))] })]
}

//#endregion 🏗️SchemaDraft

//#region 🏗️SchemaTodo

/// ✅️ The todo type and the position it points at.
fn schema_todo() -> Vec<NamedType> {
    vec![
        NamedType::Object(ObjectType {
            name: "Location".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("filePath", ty("String")), Field::new("line", ty("Int")), Field::new("column", ty("Int"))],
        }),
        NamedType::Object(ObjectType {
            name: "Todo".to_string(),
            interfaces: Vec::new(),
            fields: vec![
                Field::new("id", nn(ty("ID"))),
                Field::new("name", nn(ty("String"))),
                Field::new("description", ty("String")),
                Field::new("parentId", nn(ty("ID"))),
                Field::new("location", ty("Location")),
            ],
        }),
    ]
}

//#endregion 🏗️SchemaTodo

//#region 🏗️SchemaTicket

/// 🎫️ The ticket type and the date pair it carries.
fn schema_ticket() -> Vec<NamedType> {
    vec![
        NamedType::Object(ObjectType {
            name: "TicketDate".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("started", nn(ty("DateTime"))), Field::new("finished", ty("DateTime"))],
        }),
        NamedType::Object(ObjectType {
            name: "Ticket".to_string(),
            interfaces: Vec::new(),
            fields: vec![
                Field::new("id", nn(ty("ID"))),
                Field::new("year", nn(ty("Int"))),
                Field::new("month", nn(ty("Int"))),
                Field::new("day", nn(ty("Int"))),
                Field::new("slug", nn(ty("String"))),
                Field::new("path", nn(ty("String"))),
                Field::new("llm", ty("String")),
                Field::new("effort", ty("String")),
                Field::new("client", ty("TicketClient")),
                Field::new("checkpoint", ty("String")),
                Field::new("uri", nn(ty("String"))),
                Field::new("title", nn(ty("String"))),
                Field::new("emoji", ty("String")),
                Field::new("prompt", nn(ty("String"))),
                Field::new("summary", ty("String")),
                Field::new("status", nn(ty("TicketStatus"))),
                Field::new("interactions", nn_list(ty("Interaction"))),
                Field::resolved("author", ty("Contributor"), "ticket.author"),
                Field::new("dates", nn(ty("TicketDate"))),
                Field::new("goal", ty("String")),
                Field::new("parent", ty("String")),
                Field::new("bundles", nn_list(ty("Bundle"))),
                Field::new("files", nn_list(ty("File"))),
            ],
        }),
        NamedType::Object(ObjectType {
            name: "TicketDay".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("day", nn(ty("Int"))), Field::new("tickets", nn_list(ty("Ticket")))],
        }),
        NamedType::Object(ObjectType {
            name: "TicketMonth".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("month", nn(ty("Int"))), Field::new("days", nn_list(ty("TicketDay")))],
        }),
        NamedType::Object(ObjectType {
            name: "TicketYear".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("year", nn(ty("Int"))), Field::new("months", nn_list(ty("TicketMonth")))],
        }),
    ]
}

//#endregion 🏗️SchemaTicket

//#region 🏗️SchemaCheckpoint

/// ✔️ The checkpoint type.
fn schema_checkpoint() -> Vec<NamedType> {
    vec![NamedType::Object(ObjectType {
        name: "Checkpoint".to_string(),
        interfaces: Vec::new(),
        fields: vec![
            Field::new("id", nn(ty("ID"))),
            Field::new("sha", nn(ty("String"))),
            Field::new("title", nn(ty("String"))),
            Field::new("date", nn(ty("DateTime"))),
        ],
    })]
}

//#endregion 🏗️SchemaCheckpoint

//#region 🏗️SchemaContributor

/// 🧑️‍💻️ The contributor type and its contribution tree.
fn schema_contributor() -> Vec<NamedType> {
    let named = |name: &str, child: &str, plural: &str| {
        NamedType::Object(ObjectType {
            name: name.to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("name", nn(ty("String"))), Field::new(plural, nn_list(ty(child)))],
        })
    };
    vec![
        NamedType::Object(ObjectType {
            name: "ContributorIcons".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("avatar", ty("String")), Field::new("avatarRound", ty("String")), Field::new("github", ty("String"))],
        }),
        NamedType::Object(ObjectType {
            name: "ContributorLink".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("name", nn(ty("String"))), Field::new("url", nn(ty("String")))],
        }),
        NamedType::Object(ObjectType {
            name: "ContributorDefinition".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("name", nn(ty("String")))],
        }),
        named("ContributorSection", "ContributorDefinition", "definitions"),
        named("ContributorFile", "ContributorSection", "sections"),
        named("ContributorFolder", "ContributorFile", "files"),
        named("ContributorBundle", "ContributorFolder", "folders"),
        NamedType::Object(ObjectType {
            name: "ContributorContributions".to_string(),
            interfaces: Vec::new(),
            fields: vec![
                Field::new("checkpoints", nn_list(ty("Checkpoint"))),
                Field::new("tickets", nn_list(ty("TicketYear"))),
                Field::new("bundles", nn_list(ty("ContributorBundle"))),
            ],
        }),
        NamedType::Object(ObjectType {
            name: "Contributor".to_string(),
            interfaces: Vec::new(),
            fields: vec![
                Field::new("id", nn(ty("ID"))),
                Field::new("github", nn(ty("String"))),
                Field::new("emoji", ty("String")),
                Field::new("name", nn(ty("String"))),
                Field::new("names", nn_list(ty("String"))),
                Field::new("email", nn(ty("String"))),
                Field::new("emails", nn_list(ty("String"))),
                Field::new("fingerprint", ty("String")),
                Field::new("fingerprints", nn_list(ty("String"))),
                Field::new("links", nn_list(ty("ContributorLink"))),
                Field::resolved("contributions", ty("ContributorContributions"), "contributor.contributions"),
                Field::new("icons", ty("ContributorIcons")),
                Field::new("bundles", nn_list(ty("Bundle"))),
                Field::new("files", nn_list(ty("File"))),
                Field::new("tickets", nn_list(ty("Ticket"))),
            ],
        }),
    ]
}

//#endregion 🏗️SchemaContributor

//#region 🏗️SchemaInputs

/// 📥️ Every input object the query and mutation roots accept.
fn schema_inputs() -> Vec<NamedType> {
    let input = |name: &str, fields: Vec<(&str, TypeRef)>| {
        NamedType::InputObject(InputObjectType { name: name.to_string(), fields: fields.into_iter().map(|(field, kind)| (field.to_string(), kind)).collect() })
    };
    vec![
        input(
            "FilterInput",
            vec![
                ("filter", ty("String")),
                ("regex", ty("Boolean")),
                ("matchCase", ty("Boolean")),
                ("matchWholeWord", ty("Boolean")),
                ("showIgnored", ty("Boolean")),
                ("showGenerated", ty("Boolean")),
                ("excludeKinds", lst(ty("FileKind"))),
                ("includeKinds", lst(ty("FileKind"))),
            ],
        ),
        input("DraftCreateInput", vec![("title", nn(ty("String"))), ("files", lst(nn(ty("String"))))]),
        input(
            "TicketOpenInput",
            vec![
                ("emoji", nn(ty("String"))),
                ("title", nn(ty("String"))),
                ("prompt", nn(ty("String"))),
                ("llm", ty("String")),
                ("effort", ty("String")),
                ("client", nn(ty("TicketClient"))),
                ("noIssue", ty("Boolean")),
                ("draft", ty("String")),
                ("goal", nn(ty("String"))),
                ("parent", ty("String")),
                ("noManagement", ty("Boolean")),
                ("issue", ty("String")),
                ("planId", ty("String")),
                ("specId", ty("String")),
            ],
        ),
        input(
            "TicketCloseInput",
            vec![
                ("year", ty("Int")),
                ("month", ty("Int")),
                ("day", ty("Int")),
                ("slug", ty("String")),
                ("summary", ty("String")),
                ("files", lst(nn(ty("String")))),
                ("title", ty("String")),
                ("noManagement", ty("Boolean")),
                ("all", ty("Boolean")),
            ],
        ),
        input(
            "TicketReopenInput",
            vec![
                ("year", nn(ty("Int"))),
                ("month", nn(ty("Int"))),
                ("day", nn(ty("Int"))),
                ("slug", nn(ty("String"))),
                ("prompt", nn(ty("String"))),
                ("client", nn(ty("TicketClient"))),
                ("llm", ty("String")),
                ("effort", ty("String")),
                ("title", ty("String")),
                ("draft", ty("String")),
                ("goal", ty("String")),
                ("parent", ty("String")),
                ("noManagement", ty("Boolean")),
                ("planId", ty("String")),
                ("specId", ty("String")),
            ],
        ),
        input(
            "TicketChangeInput",
            vec![
                ("year", nn(ty("Int"))),
                ("month", nn(ty("Int"))),
                ("day", nn(ty("Int"))),
                ("slug", nn(ty("String"))),
                ("title", ty("String")),
                ("prompt", ty("String")),
                ("llm", ty("String")),
                ("effort", ty("String")),
                ("client", ty("TicketClient")),
                ("goal", ty("String")),
                ("parent", ty("String")),
                ("noManagement", ty("Boolean")),
            ],
        ),
        input("TodoCreateInput", vec![("name", nn(ty("String"))), ("description", nn(ty("String"))), ("parentId", nn(ty("String")))]),
        input("TodoChangeInput", vec![("id", nn(ty("String"))), ("name", ty("String")), ("description", ty("String"))]),
        input(
            "GoalCreateInput",
            vec![
                ("title", nn(ty("String"))),
                ("description", nn(ty("String"))),
                ("prompt", nn(ty("String"))),
                ("dueDate", nn(ty("String"))),
                ("llm", nn(ty("String"))),
                ("effort", ty("String")),
                ("client", nn(ty("String"))),
                ("parent", ty("String")),
                ("noManagement", ty("Boolean")),
                ("milestone", ty("String")),
            ],
        ),
        input(
            "GoalChangeInput",
            vec![
                ("id", nn(ty("String"))),
                ("title", ty("String")),
                ("description", ty("String")),
                ("dueDate", ty("String")),
                ("llm", ty("String")),
                ("effort", ty("String")),
                ("parent", ty("String")),
                ("noManagement", ty("Boolean")),
            ],
        ),
        input("GoalCloseInput", vec![("id", nn(ty("String"))), ("summary", nn(ty("String"))), ("noManagement", ty("Boolean"))]),
        input(
            "GoalReopenInput",
            vec![
                ("id", nn(ty("String"))),
                ("prompt", nn(ty("String"))),
                ("client", nn(ty("String"))),
                ("llm", nn(ty("String"))),
                ("effort", ty("String")),
                ("title", ty("String")),
                ("description", ty("String")),
                ("dueDate", ty("String")),
                ("parent", ty("String")),
                ("noManagement", ty("Boolean")),
            ],
        ),
        input(
            "ContributorAddInput",
            vec![
                ("github", nn(ty("String"))),
                ("name", ty("String")),
                ("names", lst(nn(ty("String")))),
                ("email", ty("String")),
                ("emails", lst(nn(ty("String")))),
                ("fingerprint", ty("String")),
                ("fingerprints", lst(nn(ty("String")))),
            ],
        ),
    ]
}

//#endregion 🏗️SchemaInputs

//#region 🏗️SchemaRoots

/// 🔍️ The query root and the node union it discriminates.
fn schema_query() -> Vec<NamedType> {
    vec![
        NamedType::Union(UnionType {
            name: "Node".to_string(),
            types: ["Repo", "Bundle", "Folder", "File", "Section", "Definition", "Contributor", "Ticket", "Policy", "Statute", "Breach", "Draft"].iter().map(|name| (*name).to_string()).collect(),
        }),
        NamedType::Object(ObjectType {
            name: "Query".to_string(),
            interfaces: Vec::new(),
            fields: vec![
                Field::resolved("node", nn(ty("Node")), "query.node").arg("id", nn(ty("ID"))),
                Field::resolved("repo", nn(ty("Repo")), "query.repo"),
                Field::resolved("technologies", nn_list(ty("Technology")), "technologies").arg("filter", ty("FilterInput")),
                Field::resolved("bundles", nn_list(ty("Bundle")), "bundles").arg("filter", ty("FilterInput")),
                Field::resolved("folders", nn_list(ty("Folder")), "folders"),
                Field::resolved("files", nn_list(ty("File")), "files"),
                Field::resolved("sections", nn_list(ty("Section")), "sections"),
                Field::resolved("definitions", nn_list(ty("Definition")), "definitions"),
                Field::resolved("contributors", nn_list(ty("Contributor")), "contributors").arg("filter", ty("FilterInput")),
                Field::resolved("todos", nn_list(ty("Todo")), "query.todos").arg("filter", ty("FilterInput")),
                Field::resolved("tickets", nn_list(ty("Ticket")), "tickets")
                    .arg("year", ty("Int"))
                    .arg("month", ty("Int"))
                    .arg("day", ty("Int"))
                    .arg("status", ty("TicketStatus"))
                    .arg("filter", ty("FilterInput")),
                Field::resolved("interactions", nn_list(ty("InteractionResource")), "query.interactions"),
                Field::resolved("drafts", nn_list(ty("Draft")), "query.drafts"),
                Field::resolved("policies", nn_list(ty("Policy")), "policies").arg("filter", ty("FilterInput")),
                Field::resolved("statutes", nn_list(ty("Statute")), "statutes"),
                Field::resolved("breachs", nn_list(ty("Breach")), "breachs").arg("scope", ty("String")),
                Field::resolved("bundle", ty("Bundle"), "query.bundle").arg("name", nn(ty("String"))),
                Field::resolved("folder", ty("Folder"), "query.folder").arg("path", nn(ty("String"))),
                Field::resolved("file", ty("File"), "query.file").arg("path", nn(ty("String"))),
                Field::resolved("section", ty("Section"), "query.section").arg("path", nn(ty("String"))).arg("sectionPath", nn_list(ty("String"))),
                Field::resolved("definition", ty("Definition"), "query.definition").arg("path", nn(ty("String"))).arg("name", nn(ty("String"))),
                Field::resolved("contributor", ty("Contributor"), "query.contributor").arg("id", nn(ty("String"))),
                Field::resolved("ticket", ty("Ticket"), "query.ticket")
                    .arg("year", nn(ty("Int")))
                    .arg("month", nn(ty("Int")))
                    .arg("day", nn(ty("Int")))
                    .arg("slug", nn(ty("String"))),
                Field::resolved("policy", ty("Policy"), "query.policy").arg("id", nn(ty("String"))),
                Field::resolved("statute", ty("Statute"), "query.statute").arg("id", nn(ty("String"))),
                Field::resolved("analyze", nn(ty("AnalyzeResult")), "query.analyze").arg("scope", ty("String")),
            ],
        }),
    ]
}

/// ✍️ The mutation root.
fn schema_mutation() -> Vec<NamedType> {
    vec![NamedType::Object(ObjectType {
        name: "Mutation".to_string(),
        interfaces: Vec::new(),
        fields: vec![
            Field::resolved("syncManagement", nn(ty("Boolean")), "mutation.syncManagement"),
            Field::resolved("goalCreate", ty("Goal"), "mutation.goalCreate").arg("input", nn(ty("GoalCreateInput"))),
            Field::resolved("goalChange", ty("Goal"), "mutation.goalChange").arg("id", nn(ty("ID"))).arg("input", nn(ty("GoalChangeInput"))),
            Field::resolved("goalClose", ty("Goal"), "mutation.goalClose").arg("input", nn(ty("GoalCloseInput"))),
            Field::resolved("goalReopen", ty("Goal"), "mutation.goalReopen").arg("input", nn(ty("GoalReopenInput"))),
            Field::resolved("draftCreate", ty("Draft"), "mutation.draftCreate").arg("input", nn(ty("DraftCreateInput"))),
            Field::resolved("draftDelete", nn(ty("Boolean")), "mutation.draftDelete").arg("id", nn(ty("String"))),
            Field::resolved("todoCreate", ty("Todo"), "mutation.todoCreate").arg("input", nn(ty("TodoCreateInput"))),
            Field::resolved("todoChange", ty("Todo"), "mutation.todoChange").arg("input", nn(ty("TodoChangeInput"))),
            Field::resolved("todoDelete", ty("Boolean"), "mutation.todoDelete").arg("id", nn(ty("ID"))),
            Field::resolved("ticketOpen", ty("Ticket"), "mutation.ticketOpen").arg("input", nn(ty("TicketOpenInput"))),
            Field::resolved("ticketClose", ty("Ticket"), "mutation.ticketClose").arg("input", nn(ty("TicketCloseInput"))),
            Field::resolved("ticketReopen", ty("Ticket"), "mutation.ticketReopen").arg("input", nn(ty("TicketReopenInput"))),
            Field::resolved("ticketChange", ty("Ticket"), "mutation.ticketChange").arg("input", nn(ty("TicketChangeInput"))),
            Field::resolved("contributorAdd", ty("Contributor"), "mutation.contributorAdd").arg("input", nn(ty("ContributorAddInput"))),
            Field::resolved("contributorRemove", nn(ty("Boolean")), "mutation.contributorRemove").arg("github", nn(ty("String"))),
            Field::resolved("folderCreate", ty("Folder"), "mutation.folderCreate").arg("path", nn(ty("String"))),
            Field::resolved("folderMove", ty("Folder"), "mutation.folderMove").arg("src", nn(ty("String"))).arg("dst", nn(ty("String"))),
            Field::resolved("folderDelete", nn(ty("Boolean")), "mutation.folderDelete").arg("path", nn(ty("String"))),
            Field::resolved("fileCreate", ty("File"), "mutation.fileCreate").arg("path", nn(ty("String"))),
            Field::resolved("fileMove", ty("File"), "mutation.fileMove").arg("src", nn(ty("String"))).arg("dst", nn(ty("String"))),
            Field::resolved("fileDelete", nn(ty("Boolean")), "mutation.fileDelete").arg("path", nn(ty("String"))),
            Field::resolved("sectionCreate", ty("Section"), "mutation.sectionCreate").arg("file", nn(ty("String"))).arg("name", nn(ty("String"))).arg("parent", ty("String")),
            Field::resolved("sectionMove", ty("Section"), "mutation.sectionMove").arg("file", nn(ty("String"))).arg("oldName", nn(ty("String"))).arg("newName", nn(ty("String"))),
            Field::resolved("sectionDelete", nn(ty("Boolean")), "mutation.sectionDelete").arg("file", nn(ty("String"))).arg("name", nn(ty("String"))),
            Field::resolved("integrate", ty("File"), "mutation.integrate")
                .arg("source", nn(ty("String")))
                .arg("targetSection", nn(ty("String")))
                .arg("targetFile", nn(ty("String")))
                .arg("targetParent", ty("String")),
            Field::resolved("extract", ty("File"), "mutation.extract").arg("sourceFile", nn(ty("String"))).arg("sourceSection", nn(ty("String"))).arg("targetFile", nn(ty("String"))),
        ],
    })]
}

/// 🗺️ Assembles the executable schema from every per-aggregate fragment.
pub fn build_schema() -> Schema {
    let mut types: BTreeMap<String, NamedType> = BTreeMap::new();
    for scalar in SCALARS {
        types.insert((*scalar).to_string(), NamedType::Scalar((*scalar).to_string()));
    }
    let fragments = [
        schema_enums(),
        schema_metrics(),
        schema_repo(),
        schema_technology(),
        schema_bundle(),
        schema_folder(),
        schema_file(),
        schema_section(),
        schema_definition(),
        schema_statute(),
        schema_breach(),
        schema_policy(),
        schema_interaction(),
        schema_goal(),
        schema_draft(),
        schema_todo(),
        schema_ticket(),
        schema_checkpoint(),
        schema_contributor(),
        schema_inputs(),
        schema_query(),
        schema_mutation(),
    ];
    for fragment in fragments {
        for declared in fragment {
            types.insert(declared.name().to_string(), declared);
        }
    }
    Schema { types, query: "Query".to_string(), mutation: Some("Mutation".to_string()) }
}

//#endregion 🏗️SchemaRoots

//#region ⚡️Execution

/// ❌️ One execution failure, carrying the reference implementation's error text.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionError {
    /// 📝️ The rendered failure.
    pub message: String,
}

impl ExecutionError {
    /// ❌️ Builds an execution failure from any displayable value.
    pub fn new(message: impl Into<String>) -> ExecutionError {
        ExecutionError { message: message.into() }
    }

    /// 🔗️ Prefixes this failure with the selection it happened under, as `field: cause`.
    fn under(self, field: &str) -> ExecutionError {
        ExecutionError { message: format!("{field}: {}", self.message) }
    }
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for ExecutionError {}

impl From<ContextError> for ExecutionError {
    fn from(error: ContextError) -> ExecutionError {
        ExecutionError { message: error.message }
    }
}

/// ⚡️ Case- and separator-insensitive field name the default resolver matches on.
fn canonical_name(value: &str) -> String {
    value.chars().filter(|character| *character != '_' && *character != '-').flat_map(char::to_lowercase).collect()
}

/// ⚡️ Reads a field off a source object when no resolver was declared.
fn resolve_default(source: &Json, name: &str) -> Json {
    let Json::Object(fields) = source else { return Json::Null };
    if let Some(value) = fields.get(name) {
        return value.clone();
    }
    let canonical = canonical_name(name);
    fields.iter().find(|(key, _)| canonical_name(key) == canonical).map_or(Json::Null, |(_, value)| value.clone())
}

/// 📥️ Coerces one resolved argument value against its declared type.
///
/// The reference implementation hands the raw literal straight to the resolver, so a bare enum
/// member reaches it as its own spelling rather than as the value the enum declares. This
/// executor coerces instead — the enum member resolves to its wire value, an input object
/// resolves field by field — which is what a conforming executor does and what the resolvers
/// below are written against.
fn coerce_input(schema: &Schema, value: &Json, declared: &TypeRef) -> Json {
    match declared {
        TypeRef::NonNull(inner) => coerce_input(schema, value, inner),
        TypeRef::List(inner) => match value {
            Json::Array(items) => Json::Array(items.iter().map(|item| coerce_input(schema, item, inner)).collect()),
            Json::Null => Json::Null,
            other => Json::Array(vec![coerce_input(schema, other, inner)]),
        },
        TypeRef::Named(name) => match schema.types.get(name) {
            Some(NamedType::Enum(enumeration)) => match value.as_str() {
                Some(member) => enumeration.values.iter().find(|(declared_name, _)| declared_name == member).map_or_else(|| value.clone(), |(_, wire)| Json::String(wire.clone())),
                None => value.clone(),
            },
            Some(NamedType::InputObject(input)) => match value {
                Json::Object(fields) => {
                    let mut coerced = Map::new();
                    for (field, kind) in &input.fields {
                        if let Some(item) = fields.get(field) {
                            coerced.insert(field.clone(), coerce_input(schema, item, kind));
                        }
                    }
                    Json::Object(coerced)
                }
                other => other.clone(),
            },
            _ => value.clone(),
        },
    }
}

/// 🧹️ Whether a text survives one filter input.
fn matches_filter(text: &str, filter: Option<&FilterInput>) -> bool {
    let Some(filter) = filter else { return true };
    let Some(needle) = filter.filter.as_deref().filter(|value| !value.is_empty()) else { return true };
    let (haystack, needle) = if filter.match_case.unwrap_or(false) { (text.to_string(), needle.to_string()) } else { (text.to_lowercase(), needle.to_lowercase()) };
    if filter.match_whole_word.unwrap_or(false) {
        haystack.split(|character: char| !character.is_alphanumeric()).any(|word| word == needle)
    } else {
        haystack.contains(&needle)
    }
}

/// 📥️ Decodes one coerced input object into a domain input type, filling every absent field
/// with the type's own default rather than failing on it.
fn decode_input<T>(value: &Json) -> Result<T, ExecutionError>
where
    T: Default + Serialize + serde::de::DeserializeOwned,
{
    let mut merged = body(&T::default());
    if let Json::Object(fields) = value {
        for (name, item) in fields {
            merged.insert(name.clone(), item.clone());
        }
    }
    serde_json::from_value(Json::Object(merged)).map_err(|error| ExecutionError::new(error.to_string()))
}

/// ⚡️ One execution of one document against one schema and one context.
struct Execution<'a> {
    schema: &'a Schema,
    context: &'a dyn RepoContext,
    variables: Map<String, Json>,
    root_dir: String,
    statutes: Vec<StatuteMeta>,
    technologies: Vec<Technology>,
}

impl Execution<'_> {
    /// ⚡️ Resolves every selection of one object, honouring aliases, defaults and `__typename`.
    fn execute_selections(&self, source: &Json, object: &ObjectType, selections: &[Selection]) -> Result<Map<String, Json>, ExecutionError> {
        let mut result = Map::new();
        for selected in selections {
            if selected.name == "__typename" {
                result.insert(selected.key().to_string(), Json::String(object.name.clone()));
                continue;
            }
            let Some(field) = object.field(&selected.name) else {
                return Err(ExecutionError::new(format!("unknown field {} on {}", quote_string(&selected.name), object.name)));
            };
            let defaults: Map<String, Json> = field.args.iter().map(|argument| (argument.name.clone(), argument.default.clone())).collect();
            let raw = coerce_arguments(&selected.arguments, &defaults, &self.variables);
            let mut args = Map::new();
            for (name, value) in raw {
                match field.args.iter().find(|argument| argument.name == name) {
                    Some(argument) => args.insert(name, coerce_input(self.schema, &value, &argument.ty)),
                    None => args.insert(name, value),
                };
            }
            let value = match &field.resolver {
                Some(key) => self.dispatch(key, source, &args).map_err(|error| error.under(&selected.name))?,
                None => resolve_default(source, &selected.name),
            };
            let projected = self.project(value, &field.ty, &selected.fields).map_err(|error| error.under(&selected.name))?;
            result.insert(selected.key().to_string(), projected);
        }
        Ok(result)
    }

    /// ⚡️ Shapes one resolved value to its declared type and sub-selection.
    fn project(&self, value: Json, declared: &TypeRef, selections: &[Selection]) -> Result<Json, ExecutionError> {
        if value.is_null() {
            return Ok(Json::Null);
        }
        match declared {
            TypeRef::NonNull(inner) => self.project(value, inner, selections),
            TypeRef::List(inner) => {
                let Json::Array(items) = value else {
                    return Err(ExecutionError::new(format!("expected list, got {value}")));
                };
                items.into_iter().map(|item| self.project(item, inner, selections)).collect::<Result<Vec<_>, _>>().map(Json::Array)
            }
            TypeRef::Named(name) => match self.schema.types.get(name) {
                Some(NamedType::Object(object)) => {
                    if selections.is_empty() {
                        return Ok(value);
                    }
                    self.execute_selections(&value, object, selections).map(Json::Object)
                }
                Some(NamedType::Interface(interface)) => {
                    let object = self.concrete(&value).ok_or_else(|| ExecutionError::new(format!("cannot resolve interface {}", interface.name)))?;
                    self.execute_selections(&value, object, selections).map(Json::Object)
                }
                Some(NamedType::Union(union)) => {
                    let object = self.concrete(&value).ok_or_else(|| ExecutionError::new(format!("cannot resolve union {}", union.name)))?;
                    self.execute_selections(&value, object, selections).map(Json::Object)
                }
                Some(NamedType::Enum(enumeration)) => Ok(match value.as_str().and_then(|wire| enumeration.values.iter().find(|(_, declared_wire)| declared_wire == wire)) {
                    Some((member, _)) => Json::String(member.clone()),
                    None => value,
                }),
                _ => Ok(value),
            },
        }
    }

    /// 🎭️ The concrete object a source discriminates itself as.
    fn concrete(&self, value: &Json) -> Option<&ObjectType> {
        value.get("__typename").and_then(Json::as_str).and_then(|name| self.schema.object(name))
    }
}

//#endregion ⚡️Execution

//#region 🗂️QueryResolvers

impl Execution<'_> {
    /// 🧹️ The filter argument of one field, decoded.
    fn filter_of(&self, args: &Map<String, Json>) -> Option<FilterInput> {
        args.get("filter").filter(|value| !value.is_null()).and_then(|value| decode_input::<FilterInput>(value).ok())
    }

    /// 📜️ Every technology, filtered and sorted by name.
    fn technologies(&self, args: &Map<String, Json>) -> Json {
        let filter = self.filter_of(args);
        let mut technologies = self.technologies.clone();
        technologies.sort_by(|left, right| left.name.cmp(&right.name));
        let rows = technologies
            .iter()
            .filter(|technology| matches_filter(&technology.name, filter.as_ref()) || matches_filter(&technology_id(technology), filter.as_ref()))
            .map(|technology| technology_source(technology, &self.technologies, &self.root_dir))
            .collect();
        Json::Array(rows)
    }

    /// 🔵️ Every bundle, filtered.
    fn bundles(&self, args: &Map<String, Json>) -> Json {
        let filter = self.filter_of(args);
        Json::Array(
            self.context
                .bundles()
                .iter()
                .filter(|bundle| matches_filter(&bundle.name, filter.as_ref()))
                .map(|bundle| bundle_source(bundle, &self.technologies, &self.root_dir))
                .collect(),
        )
    }

    /// 🎫️ Every ticket matching the date, status and text filters.
    fn tickets(&self, args: &Map<String, Json>) -> Result<Json, ExecutionError> {
        let integer = |name: &str| args.get(name).and_then(Json::as_i64);
        let status = args.get("status").and_then(Json::as_str).and_then(|wire| match wire {
            "open" => Some(TicketStatus::Open),
            "closed" => Some(TicketStatus::Closed),
            _ => None,
        });
        let filter = self.filter_of(args);
        let tickets = self.context.tickets(integer("year"), integer("month"), integer("day"), status)?;
        Ok(Json::Array(tickets.iter().filter(|ticket| matches_filter(&ticket.slug, filter.as_ref())).map(|ticket| ticket_source(ticket, &self.root_dir)).collect()))
    }

    /// 🌿️ Routes one artifact identifier to the aggregate it names.
    fn node(&self, args: &Map<String, Json>) -> Result<Json, ExecutionError> {
        let raw = args.get("id").and_then(Json::as_str).unwrap_or_default().to_string();
        let clean = raw.replace(['\u{FE0E}', '\u{FE0F}'], "");
        if clean.is_empty() {
            return Ok(self.repo());
        }
        let strip = |value: &str, emoji: &str| -> Option<String> {
            let prefix = emoji.replace(['\u{FE0E}', '\u{FE0F}'], "");
            if prefix.is_empty() {
                return None;
            }
            value.strip_prefix(&prefix).map(ToString::to_string)
        };
        for kind in ["technology-user", "technology-infrastructure", "technology-research", "technology-mono"] {
            if let Some(rest) = strip(&clean, entity(kind)) {
                let matched = self.technologies.iter().find(|technology| flat(&technology.name) == rest || technology.name == rest);
                return Ok(matched.map_or(Json::Null, |technology| technology_source(technology, &self.technologies, &self.root_dir)));
            }
        }
        for kind in ["folder-organization", "folder-required", "folder-root"] {
            if let Some(rest) = strip(&clean, entity(kind)) {
                return Ok(self.folder_by_path(&rest));
            }
        }
        for kind in ["file-code", "file-lab", "file-script", "file-docs", "file-config", "file-resource", "file-template", "file-license"] {
            if let Some(rest) = strip(&clean, entity(kind)) {
                return Ok(self.file_by_path(&rest));
            }
        }
        for kind in ["definition-impl", "definition-interface", "definition-constant", "definition-test"] {
            if let Some(rest) = strip(&clean, entity(kind)) {
                let matched = self.context.definitions().into_iter().find(|definition| flat(&definition.name) == rest || definition.name == rest);
                return Ok(matched.as_ref().map_or(Json::Null, definition_source));
            }
        }
        if let Some(rest) = strip(&clean, entity("ticket")) {
            let slug = rest.split('?').next().unwrap_or_default().to_string();
            let matched = self.context.tickets(None, None, None, None)?.into_iter().find(|ticket| flat(&ticket.slug) == slug || ticket.slug == slug);
            return Ok(matched.map_or(Json::Null, |ticket| ticket_source(&ticket, &self.root_dir)));
        }
        if let Some(rest) = strip(&clean, entity("contributor")) {
            let matched = self.context.contributors()?.into_iter().find(|contributor| flat(&contributor.alias) == rest || contributor.github == rest);
            return Ok(matched.as_ref().map_or(Json::Null, contributor_source));
        }
        if let Some(rest) = strip(&clean, entity("policy")) {
            return Ok(self.policy_by_id(&rest));
        }
        for (prefix, resolve) in [("repo:", 0), ("bundle:", 1), ("folder:", 2), ("file:", 3), ("contributor:", 4), ("policy:", 5)] {
            if let Some(rest) = raw.strip_prefix(prefix) {
                return Ok(match resolve {
                    0 => self.repo(),
                    1 => self.bundle_by_name(rest),
                    2 => self.folder_by_path(rest),
                    3 => self.file_by_path(rest),
                    4 => self.contributor_by_id(rest)?,
                    _ => self.policy_by_id(rest),
                });
            }
        }
        Err(ExecutionError::new(format!("invalid node id format: {raw}")))
    }

    /// 💠️ The repository root object.
    fn repo(&self) -> Json {
        let mut fields = Map::new();
        fields.insert("id".to_string(), Json::String("repo:compose".to_string()));
        fields.insert("name".to_string(), Json::String("compose".to_string()));
        fields.insert("path".to_string(), Json::String(self.root_dir.clone()));
        tagged(fields, "Repo")
    }

    /// 🔵️ One bundle by name or identifier.
    fn bundle_by_name(&self, name: &str) -> Json {
        let bundles = self.context.bundles();
        match bundles.iter().find(|bundle| bundle.name == name || bundle_id(bundle, &self.technologies) == name) {
            Some(bundle) => bundle_source(bundle, &self.technologies, &self.root_dir),
            None => bundle_source(
                &Bundle {
                    name: name.to_string(),
                    root: String::new(),
                    source_root: String::new(),
                    technology_name: String::new(),
                    tags: Vec::new(),
                    kind: BundleKind::Library,
                    emoji: String::new(),
                    packages: Vec::new(),
                },
                &self.technologies,
                &self.root_dir,
            ),
        }
    }

    /// 🩷️ One folder by path.
    fn folder_by_path(&self, path: &str) -> Json {
        let normalized = path.replace('\\', "/");
        self.context.folders().iter().find(|folder| folder.path == normalized).map_or(Json::Null, folder_source)
    }

    /// 📄️ One file by path.
    fn file_by_path(&self, path: &str) -> Json {
        let normalized = path.replace('\\', "/");
        self.context.files().iter().find(|file| file.path == normalized).map_or(Json::Null, file_source)
    }

    /// 🧑️‍💻️ One contributor by github handle, falling back to a bare record.
    fn contributor_by_id(&self, id: &str) -> Result<Json, ExecutionError> {
        let contributors = self.context.contributors()?;
        Ok(match contributors.iter().find(|contributor| contributor.github == id) {
            Some(contributor) => contributor_source(contributor),
            None => contributor_source(&Contributor {
                alias: id.to_string(),
                aliases: Vec::new(),
                emoji: String::new(),
                github: id.to_string(),
                githubs: Vec::new(),
                name: String::new(),
                names: Vec::new(),
                email: String::new(),
                emails: Vec::new(),
                links: BTreeMap::new(),
                fingerprint: String::new(),
                fingerprints: Vec::new(),
                contributions: ContributorContributionsStorage::default(),
            }),
        })
    }

    /// 👮️ One policy by name or identifier, falling back to a bare record.
    fn policy_by_id(&self, id: &str) -> Json {
        let policies = self.context.policies();
        match policies.iter().find(|policy| policy.name == id || policy.id == id || policy.name.eq_ignore_ascii_case(id) || policy.id.eq_ignore_ascii_case(id)) {
            Some(policy) => policy_source(policy, &self.statutes),
            None => policy_source(
                &Policy { id: format!("repo/policy/{id}"), name: id.to_string(), description: None, scopes: Some(Vec::new()), groups: Some(Vec::new()), statutes: Some(Vec::new()) },
                &self.statutes,
            ),
        }
    }
}

//#endregion 🗂️QueryResolvers

//#region 🧪️EntityResolvers

/// 📅️ Tickets grouped by year, then month, then day — the shape a contribution tree reports.
type TicketCalendar = Vec<(i64, Vec<(i64, Vec<(i64, Vec<Json>)>)>)>;

impl Execution<'_> {
    /// 🧑️‍💻️ The contribution tree of one contributor: their checkpoints and their tickets by date.
    fn contributions(&self, source: &Json) -> Result<Json, ExecutionError> {
        let github = source.get("github").and_then(Json::as_str).unwrap_or_default().to_string();
        let mut tickets: Vec<Ticket> = self.context.tickets(None, None, None, None)?.into_iter().filter(|ticket| ticket_author(ticket).eq_ignore_ascii_case(&github)).collect();
        tickets.sort_by_key(|ticket| std::cmp::Reverse(ticket_started(ticket)));
        let mut checkpoints: Vec<Json> = Vec::new();
        let mut seen: Vec<String> = Vec::new();
        for ticket in &tickets {
            let sha = ticket_checkpoint(ticket);
            if sha.is_empty() || seen.contains(&sha) {
                continue;
            }
            seen.push(sha.clone());
            let mut fields = Map::new();
            fields.insert("id".to_string(), Json::String(format!("repo/checkpoint/{sha}")));
            fields.insert("sha".to_string(), Json::String(sha));
            fields.insert("title".to_string(), Json::String(ticket.title.clone()));
            fields.insert("date".to_string(), Json::String(ticket_started(ticket)));
            checkpoints.push(tagged(fields, "Checkpoint"));
        }
        let mut years: TicketCalendar = Vec::new();
        for ticket in &tickets {
            let year = years.iter_mut().find(|(value, _)| *value == ticket.year);
            let months = match year {
                Some((_, months)) => months,
                None => {
                    years.push((ticket.year, Vec::new()));
                    &mut years.last_mut().expect("year was just pushed").1
                }
            };
            let month = months.iter_mut().find(|(value, _)| *value == ticket.month);
            let days = match month {
                Some((_, days)) => days,
                None => {
                    months.push((ticket.month, Vec::new()));
                    &mut months.last_mut().expect("month was just pushed").1
                }
            };
            match days.iter_mut().find(|(value, _)| *value == ticket.day) {
                Some((_, rows)) => rows.push(ticket_source(ticket, &self.root_dir)),
                None => days.push((ticket.day, vec![ticket_source(ticket, &self.root_dir)])),
            }
        }
        let ticket_years = years
            .into_iter()
            .map(|(year, months)| {
                let month_rows = months
                    .into_iter()
                    .map(|(month, days)| {
                        let day_rows = days
                            .into_iter()
                            .map(|(day, rows)| {
                                let mut fields = Map::new();
                                fields.insert("day".to_string(), Json::from(day));
                                fields.insert("tickets".to_string(), Json::Array(rows));
                                tagged(fields, "TicketDay")
                            })
                            .collect();
                        let mut fields = Map::new();
                        fields.insert("month".to_string(), Json::from(month));
                        fields.insert("days".to_string(), Json::Array(day_rows));
                        tagged(fields, "TicketMonth")
                    })
                    .collect();
                let mut fields = Map::new();
                fields.insert("year".to_string(), Json::from(year));
                fields.insert("months".to_string(), Json::Array(month_rows));
                tagged(fields, "TicketYear")
            })
            .collect();
        let mut fields = Map::new();
        fields.insert("checkpoints".to_string(), Json::Array(checkpoints));
        fields.insert("tickets".to_string(), Json::Array(ticket_years));
        fields.insert("bundles".to_string(), Json::Array(Vec::new()));
        Ok(tagged(fields, "ContributorContributions"))
    }

    /// 🎫️ The contributor a ticket's last interaction names.
    fn ticket_author_of(&self, source: &Json) -> Result<Json, ExecutionError> {
        let Some(Json::Array(interactions)) = source.get("interactions") else { return Ok(Json::Null) };
        let Some(last) = interactions.last() else { return Ok(Json::Null) };
        let raw = last.get("author").and_then(Json::as_str).unwrap_or_default();
        let (name, email) = parse_git_author(raw);
        let author = if email.is_empty() { name } else { email };
        let contributors = self.context.contributors()?;
        let matched = contributors
            .iter()
            .find(|contributor| contributor.emails.iter().any(|candidate| candidate == &author || author.contains(candidate.as_str())) || contributor.name == author);
        Ok(match matched {
            Some(contributor) => contributor_source(contributor),
            None => contributor_source(&Contributor {
                alias: author.clone(),
                aliases: Vec::new(),
                emoji: String::new(),
                github: author.clone(),
                githubs: Vec::new(),
                name: author.clone(),
                names: Vec::new(),
                email: String::new(),
                emails: vec![author],
                links: BTreeMap::new(),
                fingerprint: String::new(),
                fingerprints: Vec::new(),
                contributions: ContributorContributionsStorage::default(),
            }),
        })
    }

    /// 💗️ The children of a section: its nested sections and its definitions, in source order.
    fn section_children(source: &Json) -> Json {
        let mut items: Vec<Json> = Vec::new();
        for key in ["children", "definitions"] {
            if let Some(Json::Array(rows)) = source.get(key) {
                items.extend(rows.iter().cloned());
            }
        }
        items.sort_by_key(|item| (item.get("startLine").and_then(Json::as_i64).unwrap_or_default(), item.get("startIndex").and_then(Json::as_i64).unwrap_or_default()));
        Json::Array(items)
    }
}

//#endregion 🧪️EntityResolvers

//#region 💻️MutationResolvers

impl Execution<'_> {
    /// ⚡️ Runs one named resolver.
    #[allow(clippy::too_many_lines)]
    fn dispatch(&self, key: &str, source: &Json, args: &Map<String, Json>) -> Result<Json, ExecutionError> {
        let text = |name: &str| args.get(name).and_then(Json::as_str).unwrap_or_default().to_string();
        let optional_text = |name: &str| args.get(name).and_then(Json::as_str).map(ToString::to_string);
        let input = || args.get("input").cloned().unwrap_or(Json::Null);
        match key {
            "empty-list" => Ok(Json::Array(Vec::new())),
            "technologies" => Ok(self.technologies(args)),
            "bundles" => Ok(self.bundles(args)),
            "folders" => Ok(Json::Array(self.context.folders().iter().map(folder_source).collect())),
            "files" => Ok(Json::Array(self.context.files().iter().map(file_source).collect())),
            "sections" => Ok(Json::Array(self.context.sections().iter().map(section_source).collect())),
            "definitions" => Ok(Json::Array(self.context.definitions().iter().map(definition_source).collect())),
            "contributors" => {
                let filter = self.filter_of(args);
                Ok(Json::Array(self.context.contributors()?.iter().filter(|contributor| matches_filter(&contributor.github, filter.as_ref())).map(contributor_source).collect()))
            }
            "goals" => Ok(Json::Array(self.context.goals()?.iter().map(goal_source).collect())),
            "tickets" => self.tickets(args),
            "policies" => {
                let filter = self.filter_of(args);
                Ok(Json::Array(self.context.policies().iter().filter(|policy| matches_filter(&policy.name, filter.as_ref())).map(|policy| policy_source(policy, &self.statutes)).collect()))
            }
            "statutes" => Ok(Json::Array(self.statutes.iter().map(statute_source).collect())),
            "breachs" => {
                let result = self.context.analyze(optional_text("scope").as_deref())?;
                Ok(Json::Array(result.breachs.unwrap_or_default().iter().map(|breach| breach_source(breach, &self.statutes)).collect()))
            }
            "checkpoints" => Ok(Json::Array(self.context.checkpoints(args.get("limit").and_then(Json::as_i64))?.iter().map(checkpoint_source).collect())),
            "folder.children" => {
                let id = source.get("id").and_then(Json::as_str).unwrap_or_default().to_string();
                Ok(Json::Array(self.context.folders().iter().filter(|folder| folder.parent_id.as_deref() == Some(id.as_str())).map(folder_source).collect()))
            }
            "folder.files" => {
                let id = source.get("id").and_then(Json::as_str).unwrap_or_default().to_string();
                Ok(Json::Array(self.context.files().iter().filter(|file| file.folder_id.as_deref() == Some(id.as_str())).map(file_source).collect()))
            }
            "file.sections" => {
                let path = source.get("path").and_then(Json::as_str).unwrap_or_default().to_string();
                Ok(Json::Array(self.context.sections().iter().filter(|section| section.file_path == path).map(section_source).collect()))
            }
            "file.definitions" => {
                let path = source.get("path").and_then(Json::as_str).unwrap_or_default().to_string();
                Ok(Json::Array(self.context.definitions().iter().filter(|definition| definition.file_path == path).map(definition_source).collect()))
            }
            "item.file" => {
                let path = source.get("filePath").and_then(Json::as_str).unwrap_or_default().to_string();
                Ok(if path.is_empty() { Json::Null } else { self.file_by_path(&path) })
            }
            "definition.section" => {
                let path = source.get("sectionPath").and_then(Json::as_str).unwrap_or_default().to_string();
                if path.is_empty() {
                    return Ok(Json::Null);
                }
                let file_path = source.get("filePath").and_then(Json::as_str).unwrap_or_default().to_string();
                Ok(section_source(&Section {
                    id: String::new(),
                    name: path.clone(),
                    path,
                    file_path,
                    emoji: String::new(),
                    start_line: 0,
                    end_line: 0,
                    start_index: 0,
                    end_index: 0,
                    children: Vec::new(),
                    definitions: Vec::new(),
                }))
            }
            "section.children" => Ok(Self::section_children(source)),
            "statute.policy" => Ok(policy_source(
                &Policy {
                    id: "/policies/lint-scripts".to_string(),
                    name: "Lint scripts".to_string(),
                    description: None,
                    scopes: Some(vec!["**/*".to_string()]),
                    groups: Some(Vec::new()),
                    statutes: Some(Vec::new()),
                },
                &self.statutes,
            )),
            "ticket.author" => self.ticket_author_of(source),
            "contributor.contributions" => self.contributions(source),
            "query.node" => self.node(args),
            "query.repo" => Ok(self.repo()),
            "query.todos" => {
                let filter = self.filter_of(args);
                Ok(Json::Array(self.context.todos(filter.as_ref())?.iter().map(todo_source).collect()))
            }
            "query.interactions" => Ok(Json::Array(self.context.interactions()?.iter().map(interaction_resource_source).collect())),
            "query.drafts" => Ok(Json::Array(self.context.drafts()?.iter().map(draft_source).collect())),
            "query.bundle" => Ok(self.bundle_by_name(&text("name"))),
            "query.folder" => Ok(self.folder_by_path(&text("path"))),
            "query.file" => Ok(self.file_by_path(&text("path"))),
            "query.section" => {
                let path = text("path");
                let Some(Json::Array(segments)) = args.get("sectionPath") else { return Ok(Json::Null) };
                let joined = segments.iter().filter_map(Json::as_str).collect::<Vec<_>>().join("#");
                Ok(self.context.sections().iter().find(|section| section.file_path == path && section.path == joined).map_or_else(
                    || {
                        section_source(&Section {
                            id: String::new(),
                            name: joined.clone(),
                            path: String::new(),
                            file_path: String::new(),
                            emoji: String::new(),
                            start_line: 0,
                            end_line: 0,
                            start_index: 0,
                            end_index: 0,
                            children: Vec::new(),
                            definitions: Vec::new(),
                        })
                    },
                    section_source,
                ))
            }
            "query.definition" => {
                let path = text("path");
                let name = text("name");
                Ok(self.context.definitions().iter().find(|definition| definition.file_path == path && definition.name == name).map_or_else(
                    || {
                        definition_source(&Definition {
                            id: String::new(),
                            name: name.clone(),
                            kind: DefinitionKind::Implementation,
                            file_path: String::new(),
                            section_path: String::new(),
                            emoji: String::new(),
                            start_line: 0,
                            end_line: 0,
                            start_index: 0,
                            end_index: 0,
                        })
                    },
                    definition_source,
                ))
            }
            "query.contributor" => self.contributor_by_id(&text("id")),
            "query.ticket" => {
                let integer = |name: &str| args.get(name).and_then(Json::as_i64);
                let slug = text("slug");
                let tickets = self.context.tickets(integer("year"), integer("month"), integer("day"), None)?;
                Ok(tickets.iter().find(|ticket| ticket.slug == slug).map_or(Json::Null, |ticket| ticket_source(ticket, &self.root_dir)))
            }
            "query.policy" => Ok(self.policy_by_id(&text("id"))),
            "query.statute" => {
                let id = text("id");
                Ok(statute_source(&statute_meta_of(&Statute(id), &self.statutes)))
            }
            "query.analyze" => Ok(analyze_result_source(&self.context.analyze(optional_text("scope").as_deref())?, &self.statutes)),
            "mutation.syncManagement" => Ok(Json::Bool(self.context.sync_management()?)),
            "mutation.goalCreate" => Ok(goal_source(&self.context.goal_create(decode_input(&input())?)?)),
            "mutation.goalChange" => {
                let mut decoded: GoalChangeInput = decode_input(&input())?;
                decoded.id = text("id");
                Ok(goal_source(&self.context.goal_change(decoded)?))
            }
            "mutation.goalClose" => Ok(goal_source(&self.context.goal_close(decode_input(&input())?)?)),
            "mutation.goalReopen" => Ok(goal_source(&self.context.goal_reopen(decode_input(&input())?)?)),
            "mutation.draftCreate" => Ok(draft_source(&self.context.draft_create(decode_input(&input())?)?)),
            "mutation.draftDelete" => Ok(Json::Bool(self.context.draft_delete(&text("id"))?)),
            "mutation.todoCreate" => Ok(todo_source(&self.context.todo_create(decode_input(&input())?)?)),
            "mutation.todoChange" => Ok(todo_source(&self.context.todo_change(decode_input(&input())?)?)),
            "mutation.todoDelete" => Ok(Json::Bool(self.context.todo_delete(&text("id"))?)),
            "mutation.ticketOpen" => Ok(ticket_source(&self.context.ticket_open(decode_input(&input())?)?, &self.root_dir)),
            "mutation.ticketClose" => Ok(ticket_source(&self.context.ticket_close(decode_input(&input())?)?, &self.root_dir)),
            "mutation.ticketReopen" => Ok(ticket_source(&self.context.ticket_reopen(decode_input(&input())?)?, &self.root_dir)),
            "mutation.ticketChange" => Ok(ticket_source(&self.context.ticket_change(decode_input(&input())?)?, &self.root_dir)),
            "mutation.contributorAdd" => Ok(contributor_source(&self.context.contributor_add(decode_input(&input())?)?)),
            "mutation.contributorRemove" => {
                self.context.contributor_remove(&text("github"))?;
                Ok(Json::Bool(true))
            }
            "mutation.folderCreate" => Ok(folder_source(&self.context.folder_create(&text("path"))?)),
            "mutation.folderMove" => Ok(folder_source(&self.context.folder_move(&text("src"), &text("dst"))?)),
            "mutation.folderDelete" => {
                self.context.folder_delete(&text("path"))?;
                Ok(Json::Bool(true))
            }
            "mutation.fileCreate" => Ok(file_source(&self.context.file_create(&text("path"))?)),
            "mutation.fileMove" => Ok(file_source(&self.context.file_move(&text("src"), &text("dst"))?)),
            "mutation.fileDelete" => {
                self.context.file_delete(&text("path"))?;
                Ok(Json::Bool(true))
            }
            "mutation.sectionCreate" => Ok(section_source(&self.context.section_create(&text("file"), &text("name"), optional_text("parent").as_deref())?)),
            "mutation.sectionMove" => Ok(section_source(&self.context.section_move(&text("file"), &text("oldName"), &text("newName"))?)),
            "mutation.sectionDelete" => {
                self.context.section_delete(&text("file"), &text("name"))?;
                Ok(Json::Bool(true))
            }
            "mutation.integrate" => Ok(file_source(&self.context.integrate(
                optional_text("source").as_deref(),
                optional_text("targetSection").as_deref(),
                optional_text("targetFile").as_deref(),
                optional_text("targetParent").as_deref(),
            )?)),
            "mutation.extract" => Ok(file_source(&self.context.extract(optional_text("sourceFile").as_deref(), optional_text("sourceSection").as_deref(), optional_text("targetFile").as_deref())?)),
            other => Err(ExecutionError::new(format!("no resolver registered for {other}"))),
        }
    }
}

//#endregion 💻️MutationResolvers

//#region 🧱️Executor

/// 🧱️ One executable schema bound to one repository context.
pub struct Executor<'a> {
    schema: Schema,
    context: &'a dyn RepoContext,
}

impl<'a> Executor<'a> {
    /// 🔷️ Binds the repo schema to a context.
    pub fn new(context: &'a dyn RepoContext) -> Executor<'a> {
        Executor { schema: build_schema(), context }
    }

    /// 🗺️ The schema this executor serves.
    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    /// ⚡️ Parses and executes one request, returning the `data` payload.
    pub fn execute(&self, query: &str, variables: &Map<String, Json>) -> Result<Json, ExecutionError> {
        let document = parse(query).map_err(|error| ExecutionError::new(format!("graphql errors: [{}]", error.message)))?;
        let root_name = if document.operation == "mutation" {
            match &self.schema.mutation {
                Some(name) => name.clone(),
                None => return Err(ExecutionError::new("graphql errors: [mutation root is not configured]".to_string())),
            }
        } else {
            self.schema.query.clone()
        };
        let Some(root) = self.schema.object(&root_name) else {
            return Err(ExecutionError::new(format!("graphql errors: [{} root is not configured]", document.operation)));
        };
        let execution = Execution {
            schema: &self.schema,
            context: self.context,
            variables: variables.clone(),
            root_dir: self.context.root_dir(),
            statutes: self.context.statutes(),
            technologies: self.context.technologies(),
        };
        execution
            .execute_selections(&Json::Null, root, &document.selections)
            .map(Json::Object)
            .map_err(|error| ExecutionError::new(format!("graphql errors: [{}]", error.message)))
    }

    /// 📋️ Executes one request and renders the `data` payload the way the CLI prints it.
    pub fn execute_json(&self, query: &str, variables: &Map<String, Json>) -> Result<String, ExecutionError> {
        let data = self.execute(query, variables)?;
        serde_json::to_string_pretty(&data).map_err(|error| ExecutionError::new(error.to_string()))
    }

    /// 🔍️ Reports whether a request string is a document this grammar accepts.
    pub fn validate_query(&self, query: &str) -> Result<(), ParseError> {
        validate(query)
    }

    /// 📨️ The operation kind of a request string, without executing it.
    pub fn operation_type(&self, query: &str) -> Result<String, ParseError> {
        operation_type(query)
    }
}

//#endregion 🧱️Executor

//#region 📜️Sdl

/// 📜️ Collects every type reachable from the operation roots.
fn reachable(schema: &Schema) -> Vec<String> {
    let mut pending: Vec<String> = std::iter::once(schema.query.clone()).chain(schema.mutation.clone()).collect();
    let mut seen: Vec<String> = Vec::new();
    while let Some(name) = pending.pop() {
        if seen.contains(&name) {
            continue;
        }
        seen.push(name.clone());
        let Some(declared) = schema.types.get(&name) else { continue };
        let push_field = |field: &Field, pending: &mut Vec<String>| {
            pending.push(field.ty.type_name().to_string());
            for argument in &field.args {
                pending.push(argument.ty.type_name().to_string());
            }
        };
        match declared {
            NamedType::Object(object) => {
                pending.extend(object.interfaces.clone());
                for field in &object.fields {
                    push_field(field, &mut pending);
                }
            }
            NamedType::Interface(interface) => {
                for field in &interface.fields {
                    push_field(field, &mut pending);
                }
            }
            NamedType::Union(union) => pending.extend(union.types.clone()),
            NamedType::InputObject(input) => pending.extend(input.fields.iter().map(|(_, kind)| kind.type_name().to_string())),
            NamedType::Scalar(_) | NamedType::Enum(_) => {}
        }
    }
    seen.sort();
    seen
}

/// 📜️ Renders one field in SDL.
fn render_field(field: &Field) -> String {
    let arguments = if field.args.is_empty() {
        String::new()
    } else {
        format!("({})", field.args.iter().map(|argument| format!("{}: {}", argument.name, argument.ty.render())).collect::<Vec<_>>().join(", "))
    };
    format!("  {}{}: {}", field.name, arguments, field.ty.render())
}

/// 📜️ Renders the schema as the SDL both implementations must serve.
///
/// Deterministic by construction: reachable types only, sorted by name, fields in declaration
/// order — so the dump is a comparable artifact rather than a map walk.
pub fn render_sdl(schema: &Schema) -> String {
    let mut out = String::new();
    out.push_str("schema {\n");
    out.push_str(&format!("  query: {}\n", schema.query));
    if let Some(mutation) = &schema.mutation {
        out.push_str(&format!("  mutation: {mutation}\n"));
    }
    out.push_str("}\n");
    for name in reachable(schema) {
        let Some(declared) = schema.types.get(&name) else { continue };
        out.push('\n');
        match declared {
            NamedType::Scalar(scalar) => {
                if !["String", "Int", "Boolean", "ID"].contains(&scalar.as_str()) {
                    out.push_str(&format!("scalar {scalar}\n"));
                }
            }
            NamedType::Object(object) => {
                let implements = if object.interfaces.is_empty() { String::new() } else { format!(" implements {}", object.interfaces.join(" & ")) };
                out.push_str(&format!("type {}{} {{\n", object.name, implements));
                for field in &object.fields {
                    out.push_str(&render_field(field));
                    out.push('\n');
                }
                out.push_str("}\n");
            }
            NamedType::Interface(interface) => {
                out.push_str(&format!("interface {} {{\n", interface.name));
                for field in &interface.fields {
                    out.push_str(&render_field(field));
                    out.push('\n');
                }
                out.push_str("}\n");
            }
            NamedType::Union(union) => out.push_str(&format!("union {} = {}\n", union.name, union.types.join(" | "))),
            NamedType::Enum(enumeration) => {
                out.push_str(&format!("enum {} {{\n", enumeration.name));
                for (member, _) in &enumeration.values {
                    out.push_str(&format!("  {member}\n"));
                }
                out.push_str("}\n");
            }
            NamedType::InputObject(input) => {
                out.push_str(&format!("input {} {{\n", input.name));
                for (field, kind) in &input.fields {
                    out.push_str(&format!("  {}: {}\n", field, kind.render()));
                }
                out.push_str("}\n");
            }
        }
    }
    let trimmed = out.trim_end();
    format!("{trimmed}\n")
}

/// 📜️ The language-neutral inventory of the served schema: what a conforming reader finds in the
/// SDL, in the one order both implementations can agree on.
pub fn schema_inventory(schema: &Schema) -> Json {
    let field_row = |field: &Field| {
        json!({
            "name": field.name,
            "type": field.ty.render(),
            "args": field.args.iter().map(|argument| json!({ "name": argument.name, "type": argument.ty.render() })).collect::<Vec<_>>(),
        })
    };
    let types = reachable(schema)
        .into_iter()
        .filter_map(|name| schema.types.get(&name))
        .filter(|declared| !matches!(declared, NamedType::Scalar(scalar) if ["String", "Int", "Boolean", "ID"].contains(&scalar.as_str())))
        .map(|declared| match declared {
            NamedType::Scalar(scalar) => json!({ "name": scalar, "kind": "scalar" }),
            NamedType::Object(object) => json!({ "name": object.name, "kind": "object", "interfaces": object.interfaces, "fields": object.fields.iter().map(&field_row).collect::<Vec<_>>() }),
            NamedType::Interface(interface) => json!({ "name": interface.name, "kind": "interface", "fields": interface.fields.iter().map(&field_row).collect::<Vec<_>>() }),
            NamedType::Union(union) => json!({ "name": union.name, "kind": "union", "possibleTypes": union.types }),
            NamedType::Enum(enumeration) => json!({ "name": enumeration.name, "kind": "enum", "values": enumeration.values.iter().map(|(member, _)| member.clone()).collect::<Vec<_>>() }),
            NamedType::InputObject(input) => json!({ "name": input.name, "kind": "input", "fields": input.fields.iter().map(|(field, kind)| json!({ "name": field, "type": kind.render() })).collect::<Vec<_>>() }),
        })
        .collect::<Vec<_>>();
    json!({ "query": schema.query, "mutation": schema.mutation, "types": types })
}

//#endregion 📜️Sdl

//#region 🩻️RecordingContext

/// 🗄️ The mutable record set a [`RecordingContext`] answers from.
#[derive(Clone, Debug, Default)]
struct Records {
    root_dir: String,
    now: String,
    technologies: Vec<Technology>,
    bundles: Vec<Bundle>,
    checkpoints: Vec<Checkpoint>,
    folders: Vec<Folder>,
    files: Vec<File>,
    sections: Vec<Section>,
    definitions: Vec<Definition>,
    contributors: Vec<Contributor>,
    goals: Vec<Goal>,
    tickets: Vec<Ticket>,
    policies: Vec<Policy>,
    drafts: Vec<Draft>,
    todos: Vec<Todo>,
    statutes: Vec<StatuteMeta>,
    interactions: Vec<InteractionResource>,
    analyze: Option<AnalyzeResult>,
    events: Vec<Json>,
}

/// 🎫️ Decodes one ticket record, restoring the fields the wire shape derives from its path.
fn ticket_from_json(value: &Json) -> Result<Ticket, ContextError> {
    let mut ticket: Ticket = serde_json::from_value(value.clone()).map_err(|error| ContextError::new(error.to_string()))?;
    ticket.year = value.get("year").and_then(Json::as_i64).unwrap_or_default();
    ticket.month = value.get("month").and_then(Json::as_i64).unwrap_or_default();
    ticket.day = value.get("day").and_then(Json::as_i64).unwrap_or_default();
    ticket.slug = value.get("slug").and_then(Json::as_str).unwrap_or_default().to_string();
    ticket.parent = value.get("parent").and_then(Json::as_str).unwrap_or_default().to_string();
    ticket.folder_path = value.get("folderPath").and_then(Json::as_str).unwrap_or_default().to_string();
    ticket.json_path = value.get("jsonPath").and_then(Json::as_str).unwrap_or_default().to_string();
    ticket.important_path = value.get("importantPath").and_then(Json::as_str).unwrap_or_default().to_string();
    ticket.interactions = value.get("interactions").map_or_else(|| Ok(Vec::new()), |rows| serde_json::from_value(rows.clone())).map_err(|error| ContextError::new(error.to_string()))?;
    ticket.agents = value.get("agents").map_or_else(|| Ok(Vec::new()), |rows| serde_json::from_value(rows.clone())).map_err(|error| ContextError::new(error.to_string()))?;
    Ok(ticket)
}

/// 🎯️ Decodes one goal record, restoring the identifier the wire shape derives from its path.
fn goal_from_json(value: &Json) -> Result<Goal, ContextError> {
    let mut goal: Goal = serde_json::from_value(value.clone()).map_err(|error| ContextError::new(error.to_string()))?;
    goal.id = value.get("id").and_then(Json::as_str).unwrap_or_default().to_string();
    goal.path = value.get("path").and_then(Json::as_str).unwrap_or_default().to_string();
    Ok(goal)
}

/// 📥️ Decodes one array member list, defaulting to empty when the key is absent.
fn rows<T: serde::de::DeserializeOwned>(source: &Json, key: &str) -> Result<Vec<T>, ContextError> {
    match source.get(key) {
        None | Some(Json::Null) => Ok(Vec::new()),
        Some(value) => serde_json::from_value(value.clone()).map_err(|error| ContextError::new(format!("{key}: {error}"))),
    }
}

/// 🩻️ A fixture-backed context that answers every read from a record set and records every write.
///
/// It is the executor's language-agnostic test double: the same record file drives the Rust
/// subject and the TypeScript oracle, so what the two disagree about is the executor, never the
/// filesystem. Every mutation appends an event to [`RecordingContext::events`] and updates the
/// records, so a mutation case asserts both the returned aggregate and the trail it left.
pub struct RecordingContext {
    records: std::cell::RefCell<Records>,
}

impl RecordingContext {
    /// 📥️ Builds a context from one record document.
    pub fn from_json(source: &Json) -> Result<RecordingContext, ContextError> {
        let tickets = match source.get("tickets") {
            None | Some(Json::Null) => Vec::new(),
            Some(Json::Array(items)) => items.iter().map(ticket_from_json).collect::<Result<Vec<_>, _>>()?,
            Some(_) => return Err(ContextError::new("tickets must be an array")),
        };
        let goals = match source.get("goals") {
            None | Some(Json::Null) => Vec::new(),
            Some(Json::Array(items)) => items.iter().map(goal_from_json).collect::<Result<Vec<_>, _>>()?,
            Some(_) => return Err(ContextError::new("goals must be an array")),
        };
        let records = Records {
            root_dir: source.get("rootDir").and_then(Json::as_str).unwrap_or_default().to_string(),
            now: source.get("now").and_then(Json::as_str).unwrap_or("2026-01-01 00:00:00").to_string(),
            technologies: rows(source, "technologies")?,
            bundles: rows(source, "bundles")?,
            checkpoints: rows(source, "checkpoints")?,
            folders: rows(source, "folders")?,
            files: rows(source, "files")?,
            sections: rows(source, "sections")?,
            definitions: rows(source, "definitions")?,
            contributors: rows(source, "contributors")?,
            goals,
            tickets,
            policies: rows(source, "policies")?,
            drafts: rows(source, "drafts")?,
            todos: rows(source, "todos")?,
            statutes: rows(source, "statutes")?,
            interactions: rows(source, "interactions")?,
            analyze: source.get("analyze").filter(|value| !value.is_null()).map(|value| serde_json::from_value(value.clone())).transpose().map_err(|error| ContextError::new(error.to_string()))?,
            events: Vec::new(),
        };
        Ok(RecordingContext { records: std::cell::RefCell::new(records) })
    }

    /// 📥️ Builds a context from one record document's text.
    pub fn from_text(source: &str) -> Result<RecordingContext, ContextError> {
        let value: Json = serde_json::from_str(source).map_err(|error| ContextError::new(error.to_string()))?;
        RecordingContext::from_json(&value)
    }

    /// 📜️ The events every mutation appended, in the order they happened.
    pub fn events(&self) -> Vec<Json> {
        self.records.borrow().events.clone()
    }

    /// 🗄️ The mutable record set as it stands now, for a mutation case to assert on.
    pub fn snapshot(&self) -> Json {
        let records = self.records.borrow();
        let mut fields = Map::new();
        fields.insert("goals".to_string(), to_json(&records.goals));
        fields.insert(
            "tickets".to_string(),
            Json::Array(
                records
                    .tickets
                    .iter()
                    .map(|ticket| {
                        let mut row = body(ticket);
                        row.insert("year".to_string(), Json::from(ticket.year));
                        row.insert("month".to_string(), Json::from(ticket.month));
                        row.insert("day".to_string(), Json::from(ticket.day));
                        row.insert("slug".to_string(), Json::String(ticket.slug.clone()));
                        row.insert("interactions".to_string(), to_json(&ticket.interactions));
                        Json::Object(row)
                    })
                    .collect(),
            ),
        );
        fields.insert("todos".to_string(), to_json(&records.todos));
        fields.insert("drafts".to_string(), to_json(&records.drafts));
        fields.insert("contributors".to_string(), to_json(&records.contributors));
        fields.insert("folders".to_string(), to_json(&records.folders));
        fields.insert("files".to_string(), to_json(&records.files));
        fields.insert("sections".to_string(), to_json(&records.sections));
        Json::Object(fields)
    }

    /// 📝️ Appends one event.
    fn record(&self, kind: &str, payload: Json) {
        let mut fields = Map::new();
        fields.insert("kind".to_string(), Json::String(kind.to_string()));
        fields.insert("payload".to_string(), payload);
        self.records.borrow_mut().events.push(Json::Object(fields));
    }

    /// 🔤️ The identifier a created aggregate derives from its title.
    fn derive_id(title: &str) -> String {
        title.to_lowercase().split(|character: char| !character.is_alphanumeric()).filter(|part| !part.is_empty()).collect::<Vec<_>>().join("-")
    }
}

impl RepoContext for RecordingContext {
    fn root_dir(&self) -> String {
        self.records.borrow().root_dir.clone()
    }

    fn technologies(&self) -> Vec<Technology> {
        self.records.borrow().technologies.clone()
    }

    fn bundles(&self) -> Vec<Bundle> {
        self.records.borrow().bundles.clone()
    }

    fn checkpoints(&self, limit: Option<i64>) -> Result<Vec<Checkpoint>, ContextError> {
        let checkpoints = self.records.borrow().checkpoints.clone();
        Ok(match limit {
            Some(limit) if limit >= 0 => checkpoints.into_iter().take(limit.unsigned_abs() as usize).collect(),
            _ => checkpoints,
        })
    }

    fn folders(&self) -> Vec<Folder> {
        self.records.borrow().folders.clone()
    }

    fn files(&self) -> Vec<File> {
        self.records.borrow().files.clone()
    }

    fn sections(&self) -> Vec<Section> {
        self.records.borrow().sections.clone()
    }

    fn definitions(&self) -> Vec<Definition> {
        self.records.borrow().definitions.clone()
    }

    fn contributors(&self) -> Result<Vec<Contributor>, ContextError> {
        Ok(self.records.borrow().contributors.clone())
    }

    fn goals(&self) -> Result<Vec<Goal>, ContextError> {
        Ok(self.records.borrow().goals.clone())
    }

    fn tickets(&self, year: Option<i64>, month: Option<i64>, day: Option<i64>, status: Option<TicketStatus>) -> Result<Vec<Ticket>, ContextError> {
        Ok(self
            .records
            .borrow()
            .tickets
            .iter()
            .filter(|ticket| year.is_none_or(|value| ticket.year == value))
            .filter(|ticket| month.is_none_or(|value| ticket.month == value))
            .filter(|ticket| day.is_none_or(|value| ticket.day == value))
            .filter(|ticket| status.is_none_or(|value| ticket.status == value))
            .cloned()
            .collect())
    }

    fn policies(&self) -> Vec<Policy> {
        self.records.borrow().policies.clone()
    }

    fn drafts(&self) -> Result<Vec<Draft>, ContextError> {
        Ok(self.records.borrow().drafts.clone())
    }

    fn todos(&self, filter: Option<&FilterInput>) -> Result<Vec<Todo>, ContextError> {
        Ok(self.records.borrow().todos.iter().filter(|todo| matches_filter(&todo.name, filter)).cloned().collect())
    }

    fn statutes(&self) -> Vec<StatuteMeta> {
        self.records.borrow().statutes.clone()
    }

    fn interactions(&self) -> Result<Vec<InteractionResource>, ContextError> {
        Ok(self.records.borrow().interactions.clone())
    }

    fn analyze(&self, scope: Option<&str>) -> Result<AnalyzeResult, ContextError> {
        let records = self.records.borrow();
        let result = records.analyze.clone().unwrap_or(AnalyzeResult { breachs: Some(Vec::new()), metrics: Some(AnalyzeMetrics { total: 0, by_priority: Some(PriorityCount::default()), autofixable: 0 }) });
        let Some(scope) = scope else { return Ok(result) };
        let breachs: Vec<Breach> = result.breachs.unwrap_or_default().into_iter().filter(|breach| breach.scope.starts_with(scope)).collect();
        Ok(AnalyzeResult { metrics: result.metrics, breachs: Some(breachs) })
    }

    fn goal_create(&self, input: GoalCreateInput) -> Result<Goal, ContextError> {
        let goal = Goal {
            title: input.title.clone(),
            description: input.description.clone(),
            prompt: input.prompt.clone(),
            status: GoalStatus::Open,
            summary: String::new(),
            due_date: input.due_date.clone(),
            dates: GoalDates { due: input.due_date.clone() },
            client: input.client.clone(),
            llm: input.llm.clone(),
            effort: input.effort.clone(),
            parent: input.parent.clone(),
            management: Some(GoalManagementData { milestone: input.milestone.clone(), issue: String::new() }),
            id: RecordingContext::derive_id(&input.title),
            path: String::new(),
        };
        self.records.borrow_mut().goals.push(goal.clone());
        self.record("goal.create", to_json(&input));
        Ok(goal)
    }

    fn goal_change(&self, input: GoalChangeInput) -> Result<Goal, ContextError> {
        let mut records = self.records.borrow_mut();
        let Some(goal) = records.goals.iter_mut().find(|goal| goal.id == input.id) else {
            return Err(ContextError::new(format!("goal {} not found", input.id)));
        };
        if let Some(title) = &input.title {
            goal.title.clone_from(title);
        }
        if let Some(description) = &input.description {
            goal.description.clone_from(description);
        }
        if let Some(due) = &input.due_date {
            goal.due_date.clone_from(due);
            goal.dates.due.clone_from(due);
        }
        if let Some(llm) = &input.llm {
            goal.llm.clone_from(llm);
        }
        if let Some(effort) = &input.effort {
            goal.effort.clone_from(effort);
        }
        if let Some(parent) = &input.parent {
            goal.parent.clone_from(parent);
        }
        let changed = goal.clone();
        drop(records);
        self.record("goal.change", to_json(&input));
        Ok(changed)
    }

    fn goal_close(&self, input: GoalCloseInput) -> Result<Goal, ContextError> {
        let mut records = self.records.borrow_mut();
        let Some(goal) = records.goals.iter_mut().find(|goal| goal.id == input.id) else {
            return Err(ContextError::new(format!("goal {} not found", input.id)));
        };
        goal.status = GoalStatus::Closed;
        goal.summary.clone_from(&input.summary);
        let changed = goal.clone();
        drop(records);
        self.record("goal.close", to_json(&input));
        Ok(changed)
    }

    fn goal_reopen(&self, input: GoalReopenInput) -> Result<Goal, ContextError> {
        let mut records = self.records.borrow_mut();
        let Some(goal) = records.goals.iter_mut().find(|goal| goal.id == input.id) else {
            return Err(ContextError::new(format!("goal {} not found", input.id)));
        };
        goal.status = GoalStatus::Open;
        goal.prompt.clone_from(&input.prompt);
        goal.client.clone_from(&input.client);
        goal.llm.clone_from(&input.llm);
        let changed = goal.clone();
        drop(records);
        self.record("goal.reopen", to_json(&input));
        Ok(changed)
    }

    fn goal_delete(&self, input: GoalDeleteInput) -> Result<bool, ContextError> {
        let before = self.records.borrow().goals.len();
        self.records.borrow_mut().goals.retain(|goal| goal.id != input.id);
        let removed = self.records.borrow().goals.len() < before;
        self.record("goal.delete", to_json(&input));
        Ok(removed)
    }

    fn todo_create(&self, input: TodoCreateInput) -> Result<Todo, ContextError> {
        let todo = Todo { id: RecordingContext::derive_id(&input.name), name: input.name.clone(), description: input.description.clone(), parent_id: input.parent_id.clone(), location: None };
        self.records.borrow_mut().todos.push(todo.clone());
        self.record("todo.create", to_json(&input));
        Ok(todo)
    }

    fn todo_change(&self, input: TodoChangeInput) -> Result<Todo, ContextError> {
        let mut records = self.records.borrow_mut();
        let Some(todo) = records.todos.iter_mut().find(|todo| todo.id == input.id) else {
            return Err(ContextError::new(format!("todo {} not found", input.id)));
        };
        if let Some(name) = &input.name {
            todo.name.clone_from(name);
        }
        if let Some(description) = &input.description {
            todo.description.clone_from(description);
        }
        let changed = todo.clone();
        drop(records);
        self.record("todo.change", to_json(&input));
        Ok(changed)
    }

    fn todo_delete(&self, id: &str) -> Result<bool, ContextError> {
        let before = self.records.borrow().todos.len();
        self.records.borrow_mut().todos.retain(|todo| todo.id != id);
        let removed = self.records.borrow().todos.len() < before;
        self.record("todo.delete", Json::String(id.to_string()));
        Ok(removed)
    }

    fn draft_create(&self, input: DraftCreateInput) -> Result<Draft, ContextError> {
        let draft = Draft { id: RecordingContext::derive_id(&input.title) };
        self.records.borrow_mut().drafts.push(draft.clone());
        self.record("draft.create", to_json(&input));
        Ok(draft)
    }

    fn draft_delete(&self, id: &str) -> Result<bool, ContextError> {
        let before = self.records.borrow().drafts.len();
        self.records.borrow_mut().drafts.retain(|draft| draft.id != id);
        let removed = self.records.borrow().drafts.len() < before;
        self.record("draft.delete", Json::String(id.to_string()));
        Ok(removed)
    }

    fn ticket_open(&self, input: TicketOpenInput) -> Result<Ticket, ContextError> {
        let now = self.records.borrow().now.clone();
        let (year, month, day) = (
            now.get(0..4).and_then(|value| value.parse().ok()).unwrap_or(0),
            now.get(5..7).and_then(|value| value.parse().ok()).unwrap_or(0),
            now.get(8..10).and_then(|value| value.parse().ok()).unwrap_or(0),
        );
        let slug = RecordingContext::derive_id(&input.title).to_uppercase();
        let ticket = Ticket {
            year,
            month,
            day,
            slug: slug.clone(),
            title: input.title.clone(),
            emoji: input.emoji.clone(),
            status: TicketStatus::Open,
            description: input.prompt.clone(),
            summary: String::new(),
            management: Some(TicketManagementData { issue: input.issue.clone() }),
            goal: input.goal.clone(),
            parent: input.parent.clone(),
            plan: None,
            sessions: Vec::new(),
            interactions: vec![Interaction {
                kind: "ticket.open".to_string(),
                date: now,
                author: input.client.clone(),
                system: String::new(),
                client: input.client.clone(),
                checkpoint: String::new(),
                prompt: input.prompt.clone(),
                summary: String::new(),
                llm: input.llm.clone(),
                effort: input.effort.clone(),
                files: Vec::new(),
            }],
            agents: Vec::new(),
            folder_path: format!("{year:04}/{month:02}/{day:02}/{slug}"),
            json_path: String::new(),
            important_path: String::new(),
        };
        self.records.borrow_mut().tickets.push(ticket.clone());
        self.record("ticket.open", to_json(&input));
        Ok(ticket)
    }

    fn ticket_close(&self, input: TicketCloseInput) -> Result<Ticket, ContextError> {
        let now = self.records.borrow().now.clone();
        let mut records = self.records.borrow_mut();
        let Some(ticket) = records.tickets.iter_mut().find(|ticket| ticket.year == input.year && ticket.month == input.month && ticket.day == input.day && ticket.slug == input.slug) else {
            return Err(ContextError::new(format!("ticket {}/{}/{}/{} not found", input.year, input.month, input.day, input.slug)));
        };
        ticket.status = TicketStatus::Closed;
        ticket.summary.clone_from(&input.summary);
        if let Some(title) = &input.title {
            ticket.title.clone_from(title);
        }
        ticket.interactions.push(Interaction {
            kind: "ticket.close".to_string(),
            date: now,
            author: String::new(),
            system: String::new(),
            client: String::new(),
            checkpoint: String::new(),
            prompt: String::new(),
            summary: input.summary.clone(),
            llm: String::new(),
            effort: String::new(),
            files: input.files.clone().unwrap_or_default().into_iter().map(|path| InteractionFile { path, id: String::new(), uri: String::new() }).collect(),
        });
        let changed = ticket.clone();
        drop(records);
        self.record("ticket.close", to_json(&input));
        Ok(changed)
    }

    fn ticket_reopen(&self, input: TicketReopenInput) -> Result<Ticket, ContextError> {
        let now = self.records.borrow().now.clone();
        let mut records = self.records.borrow_mut();
        let Some(ticket) = records.tickets.iter_mut().find(|ticket| ticket.year == input.year && ticket.month == input.month && ticket.day == input.day && ticket.slug == input.slug) else {
            return Err(ContextError::new(format!("ticket {}/{}/{}/{} not found", input.year, input.month, input.day, input.slug)));
        };
        ticket.status = TicketStatus::Open;
        if let Some(title) = &input.title {
            ticket.title.clone_from(title);
        }
        ticket.interactions.push(Interaction {
            kind: "ticket.open".to_string(),
            date: now,
            author: input.client.clone(),
            system: String::new(),
            client: input.client.clone(),
            checkpoint: String::new(),
            prompt: input.prompt.clone(),
            summary: String::new(),
            llm: input.llm.clone(),
            effort: input.effort.clone(),
            files: Vec::new(),
        });
        let changed = ticket.clone();
        drop(records);
        self.record("ticket.reopen", to_json(&input));
        Ok(changed)
    }

    fn ticket_change(&self, input: TicketChangeInput) -> Result<Ticket, ContextError> {
        let mut records = self.records.borrow_mut();
        let Some(ticket) = records.tickets.iter_mut().find(|ticket| ticket.year == input.year && ticket.month == input.month && ticket.day == input.day && ticket.slug == input.slug) else {
            return Err(ContextError::new(format!("ticket {}/{}/{}/{} not found", input.year, input.month, input.day, input.slug)));
        };
        if let Some(title) = &input.title {
            ticket.title.clone_from(title);
        }
        if let Some(prompt) = &input.prompt {
            ticket.description.clone_from(prompt);
        }
        if let Some(goal) = &input.goal {
            ticket.goal.clone_from(goal);
        }
        if let Some(parent) = &input.parent {
            ticket.parent.clone_from(parent);
        }
        let changed = ticket.clone();
        drop(records);
        self.record("ticket.change", to_json(&input));
        Ok(changed)
    }

    fn ticket_delete(&self, input: TicketDeleteInput) -> Result<bool, ContextError> {
        let before = self.records.borrow().tickets.len();
        self.records.borrow_mut().tickets.retain(|ticket| !(ticket.year == input.year && ticket.month == input.month && ticket.day == input.day && ticket.slug == input.slug));
        let removed = self.records.borrow().tickets.len() < before;
        self.record("ticket.delete", to_json(&input));
        Ok(removed)
    }

    fn contributor_add(&self, input: ContributorAddInput) -> Result<Contributor, ContextError> {
        let contributor = Contributor {
            alias: input.github.clone(),
            aliases: Vec::new(),
            emoji: String::new(),
            github: input.github.clone(),
            githubs: Vec::new(),
            name: input.name.clone().unwrap_or_default(),
            names: input.names.clone(),
            email: input.email.clone().unwrap_or_default(),
            emails: input.emails.clone(),
            links: BTreeMap::new(),
            fingerprint: input.fingerprint.clone().unwrap_or_default(),
            fingerprints: input.fingerprints.clone(),
            contributions: ContributorContributionsStorage::default(),
        };
        self.records.borrow_mut().contributors.push(contributor.clone());
        self.record("contributor.add", to_json(&input));
        Ok(contributor)
    }

    fn contributor_remove(&self, github: &str) -> Result<(), ContextError> {
        self.records.borrow_mut().contributors.retain(|contributor| contributor.github != github);
        self.record("contributor.remove", Json::String(github.to_string()));
        Ok(())
    }

    fn folder_create(&self, path: &str) -> Result<Folder, ContextError> {
        let folder = Folder {
            id: RecordingContext::derive_id(path),
            path: path.to_string(),
            uri: file_uri(&self.root_dir(), path),
            name: path.rsplit('/').next().unwrap_or(path).to_string(),
            parent_id: None,
            bundle_id: None,
            kind: FolderKind::Organization,
            emoji: String::new(),
            ignored: false,
            generated: false,
        };
        self.records.borrow_mut().folders.push(folder.clone());
        self.record("folder.create", Json::String(path.to_string()));
        Ok(folder)
    }

    fn folder_move(&self, source: &str, destination: &str) -> Result<Folder, ContextError> {
        let root = self.root_dir();
        let mut records = self.records.borrow_mut();
        let Some(folder) = records.folders.iter_mut().find(|folder| folder.path == source) else {
            return Err(ContextError::new(format!("folder {source} not found")));
        };
        folder.path = destination.to_string();
        folder.uri = file_uri(&root, destination);
        folder.name = destination.rsplit('/').next().unwrap_or(destination).to_string();
        let moved = folder.clone();
        drop(records);
        self.record("folder.move", Json::Array(vec![Json::String(source.to_string()), Json::String(destination.to_string())]));
        Ok(moved)
    }

    fn folder_delete(&self, path: &str) -> Result<(), ContextError> {
        self.records.borrow_mut().folders.retain(|folder| folder.path != path);
        self.record("folder.delete", Json::String(path.to_string()));
        Ok(())
    }

    fn file_create(&self, path: &str) -> Result<File, ContextError> {
        let name = path.rsplit('/').next().unwrap_or(path).to_string();
        let file = File {
            id: RecordingContext::derive_id(path),
            path: path.to_string(),
            uri: file_uri(&self.root_dir(), path),
            extension: name.rfind('.').map(|index| name[index..].to_string()).unwrap_or_default(),
            name,
            folder_id: None,
            bundle_id: None,
            kind: "code".to_string(),
            emoji: String::new(),
            ignored: false,
            generated: false,
        };
        self.records.borrow_mut().files.push(file.clone());
        self.record("file.create", Json::String(path.to_string()));
        Ok(file)
    }

    fn file_move(&self, source: &str, destination: &str) -> Result<File, ContextError> {
        let root = self.root_dir();
        let mut records = self.records.borrow_mut();
        let Some(file) = records.files.iter_mut().find(|file| file.path == source) else {
            return Err(ContextError::new(format!("file {source} not found")));
        };
        file.path = destination.to_string();
        file.uri = file_uri(&root, destination);
        file.name = destination.rsplit('/').next().unwrap_or(destination).to_string();
        let moved = file.clone();
        drop(records);
        self.record("file.move", Json::Array(vec![Json::String(source.to_string()), Json::String(destination.to_string())]));
        Ok(moved)
    }

    fn file_delete(&self, path: &str) -> Result<(), ContextError> {
        self.records.borrow_mut().files.retain(|file| file.path != path);
        self.record("file.delete", Json::String(path.to_string()));
        Ok(())
    }

    fn section_create(&self, file: &str, name: &str, parent: Option<&str>) -> Result<Section, ContextError> {
        let path = parent.map_or_else(|| name.to_string(), |parent| format!("{parent}#{name}"));
        let section = Section {
            id: String::new(),
            name: name.to_string(),
            path,
            file_path: file.to_string(),
            emoji: String::new(),
            start_line: 0,
            end_line: 0,
            start_index: 0,
            end_index: 0,
            children: Vec::new(),
            definitions: Vec::new(),
        };
        self.records.borrow_mut().sections.push(section.clone());
        self.record("section.create", Json::Array(vec![Json::String(file.to_string()), Json::String(name.to_string())]));
        Ok(section)
    }

    fn section_move(&self, file: &str, old_name: &str, new_name: &str) -> Result<Section, ContextError> {
        let mut records = self.records.borrow_mut();
        let Some(section) = records.sections.iter_mut().find(|section| section.file_path == file && section.name == old_name) else {
            return Err(ContextError::new(format!("section {old_name} not found in {file}")));
        };
        section.name = new_name.to_string();
        section.path = new_name.to_string();
        let moved = section.clone();
        drop(records);
        self.record("section.move", Json::Array(vec![Json::String(file.to_string()), Json::String(old_name.to_string()), Json::String(new_name.to_string())]));
        Ok(moved)
    }

    fn section_delete(&self, file: &str, name: &str) -> Result<(), ContextError> {
        self.records.borrow_mut().sections.retain(|section| !(section.file_path == file && section.name == name));
        self.record("section.delete", Json::Array(vec![Json::String(file.to_string()), Json::String(name.to_string())]));
        Ok(())
    }

    fn integrate(&self, source: Option<&str>, target_section: Option<&str>, target_file: Option<&str>, target_parent: Option<&str>) -> Result<File, ContextError> {
        let target = target_file.unwrap_or_default();
        self.record(
            "integrate",
            Json::Array(
                [source, target_section, target_file, target_parent]
                    .into_iter()
                    .map(|value| value.map_or(Json::Null, |text| Json::String(text.to_string())))
                    .collect(),
            ),
        );
        self.records.borrow().files.iter().find(|file| file.path == target).cloned().ok_or_else(|| ContextError::new(format!("file {target} not found")))
    }

    fn extract(&self, source_file: Option<&str>, source_section: Option<&str>, target_file: Option<&str>) -> Result<File, ContextError> {
        let target = target_file.unwrap_or_default();
        self.record(
            "extract",
            Json::Array([source_file, source_section, target_file].into_iter().map(|value| value.map_or(Json::Null, |text| Json::String(text.to_string()))).collect()),
        );
        let existing = self.records.borrow().files.iter().find(|file| file.path == target).cloned();
        match existing {
            Some(file) => Ok(file),
            None => self.file_create(target),
        }
    }

    fn sync_management(&self) -> Result<bool, ContextError> {
        self.record("sync.management", Json::Null);
        Ok(true)
    }
}

//#endregion 🩻️RecordingContext

//#region 🗄️FsRepoContext

use semio_framework_repo_codebase::{Codebase, CodebaseContext, Scope as CodebaseScope};
use semio_framework_repo_goals::{FsGoalStore, GoalError, GoalStore, Goals, NullManagement};
use semio_framework_repo_model::FixResult;
use semio_framework_repo_providers::{ManagementLabel, ManagementMilestone, ManagementProvider, ManagementProviders, McpClientKind, VersionControlProvider};
use semio_framework_repo_statutes::{SourceFile, SourceSet};
use semio_framework_repo_tickets::{CoordinatorEventSink, FileTicketStore, PlanRoots, SystemClock as TicketSystemClock, TicketChangeRequest, TicketCloseRequest, TicketId, TicketLayout, TicketOpenRequest, TicketReopenRequest, TicketService};
use semio_framework_repo_todos::{FsDraftStore, FsTodoTree, Todos};

/// 📡️ The emitter that forwards an aggregate event to the repository coordinator.
#[derive(Debug, Clone, Copy, Default)]
pub struct CoordinatorEmitter;

impl semio_framework_repo_events::Emitter for CoordinatorEmitter {
    fn emit(&self, kind: &str, source: &str, payload: &Json) {
        semio_framework_repo_events::emit(kind, source, payload);
    }
}

/// 🐙️ The goal management port answered by the repository's management provider.
pub struct ProviderManagement {
    provider: ManagementProviders,
    repo_url: String,
}

impl ProviderManagement {
    /// 🆕️ Binds the provider the machine is configured for and the repository URL milestones hang off.
    pub fn system() -> ProviderManagement {
        ProviderManagement { provider: semio_framework_repo_providers::system_management_provider(), repo_url: VersionControlProvider::repo_url(&semio_framework_repo_providers::system_version_control_provider()).unwrap_or_default() }
    }
}

impl semio_framework_repo_goals::ManagementPort for ProviderManagement {
    fn create_milestone(&self, title: &str, description: &str) -> Result<i64, GoalError> {
        ManagementProvider::create_milestone(&self.provider, title, description).map_err(|error| GoalError::Port(error.message))
    }
    fn update_milestone(&self, number: i64, title: &str, description: &str, state: &str, due_on: &str) -> Result<(), GoalError> {
        ManagementProvider::update_milestone(&self.provider, number, title, description, state, due_on).map_err(|error| GoalError::Port(error.message))
    }
    fn delete_milestone(&self, number: i64) -> Result<(), GoalError> {
        ManagementProvider::delete_milestone(&self.provider, number).map_err(|error| GoalError::Port(error.message))
    }
    fn create_goal_issue(&self, title: &str, description: &str, milestone: Option<i64>) -> Result<String, GoalError> {
        ManagementProvider::create_goal_issue(&self.provider, title, description, milestone).map_err(|error| GoalError::Port(error.message))
    }
    fn update_goal_issue(&self, issue_url: &str, title: &str, description: &str) -> Result<(), GoalError> {
        ManagementProvider::update_goal_issue(&self.provider, issue_url, title, description).map_err(|error| GoalError::Port(error.message))
    }
    fn close_issue(&self, issue_url: &str) -> Result<(), GoalError> {
        ManagementProvider::close_issue(&self.provider, issue_url).map_err(|error| GoalError::Port(error.message))
    }
    fn reopen_issue(&self, issue_url: &str) -> Result<(), GoalError> {
        ManagementProvider::reopen_issue(&self.provider, issue_url).map_err(|error| GoalError::Port(error.message))
    }
    fn delete_issue(&self, number: &str) -> Result<(), GoalError> {
        ManagementProvider::delete_issue(&self.provider, number).map_err(|error| GoalError::Port(error.message))
    }
    fn add_sub_issue(&self, parent_issue_url: &str, child_issue_url: &str) -> Result<(), GoalError> {
        ManagementProvider::add_sub_issue(&self.provider, parent_issue_url, child_issue_url).map_err(|error| GoalError::Port(error.message))
    }
    fn repo_url(&self) -> String {
        self.repo_url.clone()
    }
}

/// 🧭️ One analyze scope, parsed the way the reference `ParseScope` reads a raw scope string.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AnalyzeScope {
    Repo,
    Definition(String),
    Section(String),
    File(String),
    Technology(String),
    Folder(String),
}

impl AnalyzeScope {
    /// 🔤️ Parses a raw scope: `compose` and blank are the repository, `§` a definition, `#` a section.
    pub fn parse(raw: &str) -> AnalyzeScope {
        const CODE_EXTENSIONS: [&str; 13] = [".ts", ".tsx", ".js", ".jsx", ".py", ".cs", ".go", ".json", ".md", ".yaml", ".yml", ".sql", ".graphql"];
        if raw.is_empty() || raw == "compose" {
            return AnalyzeScope::Repo;
        }
        if let Some((file, _)) = raw.split_once('§') {
            return AnalyzeScope::Definition(file.to_string());
        }
        if let Some((file, _)) = raw.split_once('#') {
            return AnalyzeScope::Section(file.to_string());
        }
        let extension = semio_framework_repo_identity::ext_of(raw).to_lowercase();
        if CODE_EXTENSIONS.contains(&extension.as_str()) {
            return AnalyzeScope::File(raw.to_string());
        }
        if raw.starts_with("compose/") {
            return AnalyzeScope::Technology(raw.to_string());
        }
        AnalyzeScope::Folder(raw.to_string())
    }

    /// 🎯️ Whether a breach belongs to this scope, twin of `BreachMatchesScope`.
    pub fn matches(&self, breach: &Breach) -> bool {
        let scope = breach.scope.replace('\\', "/");
        let prefix = |base: &str| {
            let base = base.replace('\\', "/");
            let base = base.trim_end_matches('/');
            base.is_empty() || scope == base || scope.starts_with(&format!("{base}/"))
        };
        match self {
            AnalyzeScope::Repo => true,
            AnalyzeScope::File(path) => {
                let path = path.replace('\\', "/");
                scope == path || scope.starts_with(&format!("{path}#")) || scope.starts_with(&format!("{path}::"))
            }
            AnalyzeScope::Folder(path) | AnalyzeScope::Technology(path) => prefix(path),
            AnalyzeScope::Section(path) | AnalyzeScope::Definition(path) => scope.starts_with(&path.replace('\\', "/")),
        }
    }
}

/// 🗄️ The production context: every aggregate answered from the repository on disk through the
/// domain crate that owns it. Twin of `NewRepoContext` in `🔗️graphql/📦️packages/🐹️go/🐹️.go`.
pub struct FsRepoContext {
    root: std::path::PathBuf,
    codebase: Codebase,
}

impl FsRepoContext {
    /// 🆕️ Opens the context of one repository root.
    pub fn open(root: &std::path::Path) -> FsRepoContext {
        FsRepoContext { root: root.to_path_buf(), codebase: Codebase::new(root) }
    }

    /// 🏠️ The repository root the context reads.
    pub fn root(&self) -> &std::path::Path {
        &self.root
    }

    /// 🗂️ The codebase the context walks.
    pub fn codebase(&self) -> &Codebase {
        &self.codebase
    }

    /// ✍️ The alias the invoking git identity resolves to.
    pub fn author(&self) -> String {
        semio_framework_repo_contributors::git_author_alias(&self.root)
    }

    fn slash_root(&self) -> String {
        self.root.display().to_string().replace('\\', "/")
    }

    fn layout(&self) -> TicketLayout {
        TicketLayout::new(format!("{}/{}/{}", self.slash_root(), semio_framework_repo_workspace::SEMIO_DIR_NAME, semio_framework_repo_workspace::REPO_DIR_NAME))
    }

    /// 🎫️ Runs one operation against the ticket domain bound to this repository.
    pub fn with_tickets<R>(&self, kind: McpClientKind, body: impl FnOnce(&TicketService<'_, FileTicketStore, ManagementProviders, TicketSystemClock, CoordinatorEventSink>) -> R) -> R {
        let store = FileTicketStore::new();
        let tracker = semio_framework_repo_providers::system_management_provider();
        let clock = TicketSystemClock;
        let events = CoordinatorEventSink;
        let roots = PlanRoots { repo_root: self.slash_root(), home_dir: std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")).unwrap_or_default().replace('\\', "/") };
        let service = TicketService::new(self.layout(), &store, &tracker, &clock, &events).with_roots(roots).with_mcp_client(kind);
        body(&service)
    }

    /// 🎯️ Runs one operation against the goal aggregate bound to this repository.
    pub fn with_goals<R>(&self, no_management: bool, body: impl FnOnce(&Goals<'_>) -> R) -> R {
        let store = FsGoalStore::new(&self.root);
        let emitter = CoordinatorEmitter;
        let author = self.author();
        if no_management {
            body(&Goals::new(&store, &NullManagement, &emitter, &author))
        } else {
            body(&Goals::new(&store, &ProviderManagement::system(), &emitter, &author))
        }
    }

    fn with_todos<R>(&self, body: impl FnOnce(&Todos<'_>) -> R) -> R {
        let tree = FsTodoTree::new(&self.root);
        let emitter = CoordinatorEmitter;
        body(&Todos::new(&tree, &emitter, &self.author()))
    }

    fn considered(&self) -> CodebaseContext<'_> {
        let mut context = CodebaseContext::new(&self.codebase);
        context.load_bundles();
        context.load_files();
        context
    }

    fn read(&self, path: &str) -> Option<String> {
        std::fs::read_to_string(self.root.join(path)).ok()
    }

    fn ticket(&self, year: i64, month: i64, day: i64, slug: &str) -> Result<Ticket, ContextError> {
        self.with_tickets(McpClientKind::Generic, |service| service.read(&TicketId::new(year, month, day, slug))).map_err(|error| ContextError::new(error.message))
    }

    fn ticket_by_id(&self, id: &str) -> Result<Ticket, ContextError> {
        let id = TicketId::parse(id).map_err(|error| ContextError::new(error.message))?;
        self.ticket(id.year, id.month, id.day, &id.slug)
    }

    /// 🏷️ Retitles a ticket when a non-blank title is given, following the folder the new slug names.
    pub fn retitle(&self, id: &TicketId, title: Option<&String>, no_management: bool) -> Result<TicketId, ContextError> {
        let Some(title) = title.filter(|title| !title.trim().is_empty()) else { return Ok(id.clone()) };
        let before = self.ticket(id.year, id.month, id.day, &id.slug)?;
        let outcome = self
            .with_tickets(McpClientKind::Generic, |service| service.change(&TicketChangeRequest { id: id.id(), title: Some(title.clone()), no_management, ..Default::default() }))
            .map_err(|error| ContextError::new(error.message))?;
        if let Some(issue) = before.management.as_ref().map(|management| management.issue.clone()).filter(|issue| !issue.is_empty() && !no_management) {
            if let Err(error) = ManagementProvider::update_issue_title(&semio_framework_repo_providers::system_management_provider(), &issue, title) {
                eprintln!("Warning: Failed to update GitHub issue title: {}", error.message);
            }
        }
        TicketId::parse(&outcome.id).map_err(|error| ContextError::new(error.message))
    }

    fn ensure_goal_milestone(&self, provider: &ManagementProviders, goal: &mut Goal) -> Result<Option<ManagementMilestone>, ContextError> {
        if goal.title.trim().is_empty() || !semio_framework_repo_goals::is_root_goal(&goal.id) {
            return Ok(None);
        }
        let known = goal.management.as_ref().map(|management| management.milestone.clone()).unwrap_or_default();
        if let Some(number) = parse_milestone_number(&known).filter(|number| *number > 0) {
            if let Ok(Some(milestone)) = ManagementProvider::get_milestone(provider, number) {
                return Ok(Some(milestone));
            }
        }
        let found = ManagementProvider::find_milestone_by_title(provider, &goal.title).map_err(|error| ContextError::new(error.message))?;
        let milestone = match found {
            Some(milestone) => milestone,
            None => {
                let number = ManagementProvider::create_milestone(provider, &goal.title, &goal.description).map_err(|error| ContextError::new(error.message))?;
                if !goal.dates.due.is_empty() {
                    let _ = ManagementProvider::update_milestone(provider, number, "", "", "", &goal.dates.due);
                }
                ManagementProvider::get_milestone(provider, number).ok().flatten().unwrap_or(ManagementMilestone { number, title: goal.title.clone(), description: goal.description.clone(), ..Default::default() })
            }
        };
        let repo_url = VersionControlProvider::repo_url(&semio_framework_repo_providers::system_version_control_provider()).map_err(|error| ContextError::new(error.message))?;
        let link = format!("{repo_url}/milestone/{}", milestone.number);
        let management = goal.management.get_or_insert_with(GoalManagementData::default);
        if management.milestone != link {
            management.milestone = link;
            self.save_goal(goal)?;
        }
        Ok(Some(milestone))
    }

    fn save_goal(&self, goal: &Goal) -> Result<(), ContextError> {
        let document = semio_framework_repo_goals::encode_goal(goal).map_err(|error| ContextError::new(error.to_string()))?;
        GoalStore::write(&FsGoalStore::new(&self.root), &goal.id, &document).map_err(|error| ContextError::new(error.to_string()))
    }

    fn file_record(&self, path: &str) -> File {
        let name = semio_framework_repo_identity::base_of(path);
        File {
            id: self.codebase.build_file_id(path),
            path: path.to_string(),
            uri: self.codebase.file_uri(path),
            name: name.clone(),
            extension: semio_framework_repo_identity::ext_of(&name),
            folder_id: None,
            bundle_id: None,
            kind: semio_framework_repo_model::derive_file_kind(&name).as_str().to_string(),
            emoji: String::new(),
            ignored: false,
            generated: false,
        }
    }

    fn apply(&self, plan: Result<semio_framework_repo_move::Plan, semio_framework_repo_move::MoveError>) -> Result<Vec<String>, ContextError> {
        let plan = plan.map_err(|error| ContextError::new(error.0))?;
        let (starting, ended): (Vec<&semio_framework_repo_move::PlanEvent>, Vec<&semio_framework_repo_move::PlanEvent>) = plan.events.iter().partition(|event| event.kind.ends_with(".starting"));
        for event in starting {
            semio_framework_repo_events::emit(&event.kind, "repo-cli", &event.payload);
        }
        let mut executor = semio_framework_repo_move::FileSystemExecutor::new(&self.root);
        semio_framework_repo_move::execute(&plan, &mut executor).map_err(|error| ContextError::new(error.0))?;
        for event in ended {
            semio_framework_repo_events::emit(&event.kind, "repo-cli", &event.payload);
        }
        Ok(plan.lines())
    }

    /// 📄️ The header region a new file opens with, twin of `GenerateFileHeader`.
    pub fn file_header(&self, path: &str) -> String {
        let Some(language) = semio_framework_repo_languages::language_for_path(path) else { return String::new() };
        let year = semio_framework_repo_tickets::Clock::stamp(&TicketSystemClock).chars().take(4).collect::<String>();
        let header = semio_framework_repo_languages::Header {
            file_id: self.codebase.build_file_id(path),
            file_uri: self.codebase.file_uri(path),
            summary: String::new(),
            contributors: format!("{year} {}", semio_framework_repo_workspace::git_author(&self.root)),
            license: semio_framework_repo_workspace::AGPL_LICENSE_TEXT.to_string(),
            requirements: String::new(),
        };
        semio_framework_repo_languages::format_header(language, &header)
    }

    /// 🗂️ The in-memory workspace a move plan reads: the named files, and every considered file
    /// below the named directories — structurally excluded and ignored paths never enter it.
    pub fn workspace(&self, files: &[&str], directories: &[&str]) -> semio_framework_repo_move::Workspace {
        let mut workspace = semio_framework_repo_move::Workspace::new();
        for directory in directories.iter().filter(|directory| !directory.is_empty()) {
            let absolute = self.root.join(directory);
            if absolute.is_dir() {
                if *directory != "." {
                    workspace.insert_directory(directory);
                }
                self.load_directory(&mut workspace, if *directory == "." { "" } else { directory });
            } else if let Some(content) = self.read(directory) {
                workspace.insert_file(directory, &content);
            }
        }
        for file in files.iter().filter(|file| !file.is_empty()) {
            if let Some(content) = self.read(file) {
                workspace.insert_file(file, &content);
            }
        }
        if let Some(agents) = self.read("AGENTS.md") {
            workspace.insert_file("AGENTS.md", &agents);
        }
        workspace
    }

    fn load_directory(&self, workspace: &mut semio_framework_repo_move::Workspace, directory: &str) {
        let Ok(entries) = std::fs::read_dir(self.root.join(directory)) else { return };
        let mut entries: Vec<std::fs::DirEntry> = entries.flatten().collect();
        entries.sort_by_key(std::fs::DirEntry::file_name);
        for entry in entries {
            let name = entry.file_name().to_string_lossy().to_string();
            let relative = if directory.is_empty() { name } else { format!("{directory}/{name}") };
            if self.codebase.is_repo_excluded_path(&relative) || self.codebase.is_gitignored(&relative) {
                continue;
            }
            let Ok(kind) = entry.file_type() else { continue };
            if kind.is_dir() {
                workspace.insert_directory(&relative);
                self.load_directory(workspace, &relative);
            } else if kind.is_file() {
                if let Ok(content) = std::fs::read_to_string(entry.path()) {
                    workspace.insert_file(&relative, &content);
                }
            }
        }
    }

    /// 📑️ Renames a section of one file and reports the plan lines.
    pub fn move_section(&self, file: &str, old_name: &str, new_name: &str) -> Result<Vec<String>, ContextError> {
        let workspace = self.workspace(&[file], &[]);
        self.apply(semio_framework_repo_move::plan_section_move(&workspace, file, old_name, new_name))
    }

    /// 🧩️ Integrates a whole file into a named section of another file.
    pub fn integrate_file(&self, source: &str, target_section: &str, target_file: &str, target_parent: &str) -> Result<Vec<String>, ContextError> {
        let workspace = self.workspace(&[source, target_file], &[]);
        self.apply(semio_framework_repo_move::plan_integrate(&workspace, source, target_section, target_file, target_parent))
    }

    /// 🧲️ Extracts one section of a file into a file of its own.
    pub fn extract_section(&self, source_file: &str, source_section: &str, target_file: &str) -> Result<Vec<String>, ContextError> {
        let workspace = self.workspace(&[source_file, target_file], &[]);
        self.apply(semio_framework_repo_move::plan_extract(&workspace, source_file, source_section, target_file))
    }

    /// 🔤️ Rewrites every casing of one token across the contents and names below a scope.
    pub fn rename_token(&self, old_token: &str, new_token: &str, scope: &str) -> Result<Vec<String>, ContextError> {
        let workspace = self.workspace(&[], &[if scope.is_empty() { "." } else { scope }]);
        self.apply(semio_framework_repo_move::plan_rename(&workspace, old_token, new_token, scope))
    }

    /// 📂️ Every breach the repository's breach cache records.
    pub fn cached_breachs(&self) -> Result<Vec<Breach>, ContextError> {
        let directory = semio_framework_repo_workspace::repo_meta_path_for_root(&self.root, "⚡️cache/breaches");
        let Ok(entries) = std::fs::read_dir(&directory) else { return Ok(Vec::new()) };
        let mut names: Vec<std::path::PathBuf> = entries.flatten().map(|entry| entry.path()).filter(|path| path.is_file() && path.extension().is_some_and(|extension| extension == "json")).collect();
        names.sort();
        let documents = names.iter().map(|path| std::fs::read_to_string(path).map_err(|error| ContextError::new(format!("{}: {error}", path.display())))).collect::<Result<Vec<String>, ContextError>>()?;
        semio_framework_repo_statutes::breachs_from_cache(&documents).map_err(ContextError::new)
    }

    /// 🩹️ Applies every autofix to the files of one scope and reports how many breaches it cleared.
    pub fn autofix(&self, scope: Option<&str>) -> Result<FixResult, ContextError> {
        let scope = AnalyzeScope::parse(scope.unwrap_or("compose"));
        let files = self.codebase.scope_to_files(&match &scope {
            AnalyzeScope::Repo => CodebaseScope::Repo,
            AnalyzeScope::Technology(name) => CodebaseScope::Technology(name.clone()),
            AnalyzeScope::Folder(path) => CodebaseScope::Folder(if path.ends_with('/') { path.clone() } else { format!("{path}/") }),
            AnalyzeScope::File(path) | AnalyzeScope::Section(path) | AnalyzeScope::Definition(path) => CodebaseScope::File(path.clone()),
        });
        let sources: Vec<SourceFile> = files.iter().filter_map(|path| self.read(path).map(|content| SourceFile { path: path.clone(), content })).collect();
        let before = semio_framework_repo_statutes::filter_ignored(semio_framework_repo_statutes::analyze(&SourceSet::new(sources.clone())), &SourceSet::new(sources.clone()));
        let mut fixed_sources = Vec::with_capacity(sources.len());
        for source in &sources {
            let fixed = semio_framework_repo_statutes::autofix(source);
            if fixed.content != source.content {
                std::fs::write(self.root.join(&source.path), &fixed.content).map_err(|error| ContextError::new(format!("{}: {error}", source.path)))?;
            }
            fixed_sources.push(SourceFile { path: source.path.clone(), content: fixed.content });
        }
        let set = SourceSet::new(fixed_sources);
        let after = semio_framework_repo_statutes::filter_ignored(semio_framework_repo_statutes::analyze(&set), &set);
        Ok(FixResult { fixed: before.len().saturating_sub(after.len()) as i64, remaining: after.len() as i64, breachs: Some(after) })
    }
}

impl RepoContext for FsRepoContext {
    fn root_dir(&self) -> String {
        self.slash_root()
    }

    fn technologies(&self) -> Vec<Technology> {
        self.codebase.technologies()
    }

    fn bundles(&self) -> Vec<Bundle> {
        self.codebase.bundles()
    }

    fn checkpoints(&self, limit: Option<i64>) -> Result<Vec<Checkpoint>, ContextError> {
        Ok(semio_framework_repo_contributors::list_checkpoints(&semio_framework_repo_contributors::GitCheckpoints::new(&self.root), limit.map(|limit| limit.max(0) as usize)))
    }

    fn folders(&self) -> Vec<Folder> {
        self.considered()
            .build_folders()
            .into_iter()
            .map(|folder| Folder {
                kind: self.codebase.derive_folder_kind(&folder.path),
                generated: semio_framework_repo_codebase::is_generated_folder(&folder.path),
                id: folder.id,
                path: folder.path,
                uri: folder.uri,
                name: folder.name,
                parent_id: folder.parent_id,
                bundle_id: folder.bundle_id,
                emoji: String::new(),
                ignored: false,
            })
            .collect()
    }

    fn files(&self) -> Vec<File> {
        self.considered()
            .build_files()
            .into_iter()
            .map(|file| {
                let mut record = self.file_record(&file.path);
                record.id = file.id;
                record.uri = file.uri;
                record.folder_id = file.folder_id;
                record.bundle_id = file.bundle_id;
                record
            })
            .collect()
    }

    fn sections(&self) -> Vec<Section> {
        let context = self.considered();
        let mut result = Vec::new();
        for file in &context.files {
            let Some(content) = self.read(file) else { continue };
            let file_id = self.codebase.build_file_id(file);
            for mut section in semio_framework_repo_languages::parse_sections(&content, file) {
                section.file_path = file.clone();
                section.id = semio_framework_repo_model::build_section_id(&file_id, &semio_framework_repo_languages::normalize_section_path(&section.path));
                result.push(section);
            }
        }
        result
    }

    fn definitions(&self) -> Vec<Definition> {
        let context = self.considered();
        let mut result = Vec::new();
        for file in &context.files {
            let Some(content) = self.read(file) else { continue };
            let file_id = self.codebase.build_file_id(file);
            for mut definition in semio_framework_repo_codebase::file_definitions(&content, file) {
                definition.id = semio_framework_repo_codebase::definition_id(&file_id, &definition);
                result.push(definition);
            }
        }
        result
    }

    fn contributors(&self) -> Result<Vec<Contributor>, ContextError> {
        Ok(semio_framework_repo_contributors::list_contributors(&semio_framework_repo_contributors::FsContributorStore::new(&self.root)))
    }

    fn goals(&self) -> Result<Vec<Goal>, ContextError> {
        self.with_goals(true, |goals| goals.list()).map_err(|error| ContextError::new(error.to_string()))
    }

    fn tickets(&self, year: Option<i64>, month: Option<i64>, day: Option<i64>, status: Option<TicketStatus>) -> Result<Vec<Ticket>, ContextError> {
        let tickets = self.with_tickets(McpClientKind::Generic, |service| service.list(year, month, day)).map_err(|error| ContextError::new(error.message))?;
        Ok(tickets.into_iter().filter(|ticket| status.is_none_or(|status| ticket.status == status)).collect())
    }

    fn policies(&self) -> Vec<Policy> {
        semio_framework_repo_statutes::policies().to_vec()
    }

    fn drafts(&self) -> Result<Vec<Draft>, ContextError> {
        Ok(semio_framework_repo_todos::list_drafts(&FsDraftStore::new(&self.root)))
    }

    fn todos(&self, filter: Option<&FilterInput>) -> Result<Vec<Todo>, ContextError> {
        let all = self.with_todos(|todos| todos.list());
        let Some(search) = filter.and_then(|filter| filter.filter.as_ref()).map(|search| search.to_lowercase()) else { return Ok(all) };
        Ok(all.into_iter().filter(|todo| todo.name.to_lowercase().contains(&search) || todo.description.to_lowercase().contains(&search)).collect())
    }

    fn statutes(&self) -> Vec<StatuteMeta> {
        semio_framework_repo_statutes::statutes().to_vec()
    }

    fn interactions(&self) -> Result<Vec<InteractionResource>, ContextError> {
        let tickets = self.tickets(None, None, None, None)?;
        Ok(tickets
            .into_iter()
            .flat_map(|ticket| {
                let id = emoji_text(entity("ticket")) + &flat(&ticket.slug);
                let goal = ticket.goal.clone();
                ticket.interactions.into_iter().map(move |interaction| InteractionResource { interaction, source_kind: "ticket".to_string(), source_id: id.clone(), goal_id: goal.clone(), ticket_id: id.clone() })
            })
            .collect())
    }

    fn analyze(&self, scope: Option<&str>) -> Result<AnalyzeResult, ContextError> {
        let scope = AnalyzeScope::parse(scope.unwrap_or("compose"));
        let statutes = semio_framework_repo_statutes::statutes();
        let breachs: Vec<Breach> = self.cached_breachs()?.into_iter().filter(|breach| scope.matches(breach)).collect();
        let mut by_priority = PriorityCount::default();
        let mut autofixable = 0i64;
        for breach in &breachs {
            let meta = statutes.iter().find(|meta| meta.kind == breach.kind);
            match breach.lint_priority.or(meta.map(|meta| meta.priority)).unwrap_or(BreachPriority::Low) {
                BreachPriority::High => by_priority.high += 1,
                BreachPriority::Medium => by_priority.medium += 1,
                BreachPriority::Low => by_priority.low += 1,
            }
            if breach.lint_autofixable.or(meta.map(|meta| meta.autofixable)).unwrap_or(false) {
                autofixable += 1;
            }
        }
        Ok(AnalyzeResult { metrics: Some(AnalyzeMetrics { total: breachs.len() as i64, by_priority: Some(by_priority), autofixable }), breachs: Some(breachs) })
    }

    fn goal_create(&self, input: GoalCreateInput) -> Result<Goal, ContextError> {
        self.with_goals(input.no_management, |goals| goals.create(&input)).map_err(|error| ContextError::new(error.to_string()))
    }

    fn goal_change(&self, input: GoalChangeInput) -> Result<Goal, ContextError> {
        self.with_goals(input.no_management, |goals| goals.change(&input)).map_err(|error| ContextError::new(error.to_string()))
    }

    fn goal_close(&self, input: GoalCloseInput) -> Result<Goal, ContextError> {
        self.with_goals(input.no_management, |goals| goals.close(&input)).map_err(|error| ContextError::new(error.to_string()))
    }

    fn goal_reopen(&self, input: GoalReopenInput) -> Result<Goal, ContextError> {
        self.with_goals(input.no_management, |goals| goals.reopen(&input)).map_err(|error| ContextError::new(error.to_string()))
    }

    fn goal_delete(&self, input: GoalDeleteInput) -> Result<bool, ContextError> {
        self.with_goals(input.no_management, |goals| goals.delete(&input)).map_err(|error| ContextError::new(error.to_string()))
    }

    fn todo_create(&self, input: TodoCreateInput) -> Result<Todo, ContextError> {
        self.with_todos(|todos| todos.create(&input)).map_err(|error| ContextError::new(error.to_string()))
    }

    fn todo_change(&self, input: TodoChangeInput) -> Result<Todo, ContextError> {
        self.with_todos(|todos| todos.change(&input)).map_err(|error| ContextError::new(error.to_string()))
    }

    fn todo_delete(&self, id: &str) -> Result<bool, ContextError> {
        self.with_todos(|todos| todos.delete(id)).map_err(|error| ContextError::new(error.to_string()))
    }

    fn draft_create(&self, input: DraftCreateInput) -> Result<Draft, ContextError> {
        let files = input
            .files
            .iter()
            .map(|path| std::fs::read_to_string(self.root.join(path)).map(|content| (path.clone(), content)).map_err(|error| ContextError::new(format!("{path}: {error}"))))
            .collect::<Result<Vec<(String, String)>, ContextError>>()?;
        let draft = semio_framework_repo_todos::create_draft(&FsDraftStore::new(&self.root), &input.title, &files).map_err(|error| ContextError::new(error.to_string()))?;
        semio_framework_repo_events::emit(semio_framework_repo_events::DRAFT_CREATE_ENDED, "repo-cli", &json!({ "slug": draft.id, "title": input.title }));
        Ok(draft)
    }

    fn draft_delete(&self, id: &str) -> Result<bool, ContextError> {
        semio_framework_repo_todos::delete_draft(&FsDraftStore::new(&self.root), id).map_err(|error| ContextError::new(error.to_string()))?;
        semio_framework_repo_events::emit(semio_framework_repo_events::DRAFT_DELETE_ENDED, "repo-cli", &json!({ "slug": id }));
        Ok(true)
    }

    fn ticket_open(&self, input: TicketOpenInput) -> Result<Ticket, ContextError> {
        let client = semio_framework_repo_model::resolve_allowed_client(&input.client).map_err(|error| ContextError::new(error.to_string()))?;
        let kind = semio_framework_repo_providers::mcp_kind_from_resolved_client(&client);
        let request = TicketOpenRequest {
            emoji: input.emoji,
            title: input.title,
            prompt: input.prompt,
            llm: input.llm,
            effort: input.effort,
            client: input.client,
            goal: input.goal,
            parent: input.parent,
            no_issue: input.no_issue,
            no_management: input.no_management,
            issue: input.issue,
            session: String::new(),
            plan_id: input.plan_id,
            spec_id: input.spec_id,
        };
        let outcome = self.with_tickets(kind, |service| service.open(&request)).map_err(|error| ContextError::new(error.message))?;
        for warning in &outcome.warnings {
            eprintln!("Warning: {warning}");
        }
        self.ticket_by_id(&outcome.id)
    }

    fn ticket_close(&self, input: TicketCloseInput) -> Result<Ticket, ContextError> {
        if input.all {
            let mut last = None;
            for ticket in self.tickets(None, None, None, Some(TicketStatus::Open))? {
                let id = TicketId::new(ticket.year, ticket.month, ticket.day, ticket.slug.clone());
                println!("Closing ticket {}...", ticket.slug);
                let request = TicketCloseRequest { id: id.id(), summary: "Bulk close".to_string(), files: Vec::new(), no_management: input.no_management, bulk: true };
                match self.with_tickets(McpClientKind::Generic, |service| service.close(&request)) {
                    Ok(_) => last = self.ticket_by_id(&id.id()).ok(),
                    Err(error) => eprintln!("Warning: Failed to close ticket {}: {}", ticket.slug, error.message),
                }
            }
            if !input.no_management {
                let provider = semio_framework_repo_providers::system_management_provider();
                match ManagementProvider::list_open_issues_with_label(&provider, "ticket") {
                    Ok(issues) => {
                        for issue in issues {
                            println!("Closing GitHub issue {issue}...");
                            if let Err(error) = ManagementProvider::close_issue(&provider, &issue) {
                                eprintln!("Warning: Failed to close GitHub issue {issue}: {}", error.message);
                            }
                        }
                    }
                    Err(error) => eprintln!("Warning: Failed to list GitHub issues with 'ticket' label: {}", error.message),
                }
            }
            return last.ok_or_else(|| ContextError::new("no open tickets"));
        }
        let id = self.retitle(&TicketId::new(input.year, input.month, input.day, input.slug.clone()), input.title.as_ref(), input.no_management)?;
        let request = TicketCloseRequest { id: id.id(), summary: input.summary, files: input.files.unwrap_or_default(), no_management: input.no_management, bulk: false };
        let outcome = self.with_tickets(McpClientKind::Generic, |service| service.close(&request)).map_err(|error| ContextError::new(error.message))?;
        for warning in &outcome.warnings {
            eprintln!("Warning: {warning}");
        }
        self.ticket_by_id(&outcome.id)
    }

    fn ticket_reopen(&self, input: TicketReopenInput) -> Result<Ticket, ContextError> {
        let id = self.retitle(&TicketId::new(input.year, input.month, input.day, input.slug.clone()), input.title.as_ref(), input.no_management)?;
        let client = semio_framework_repo_model::resolve_allowed_client(&input.client).map_err(|error| ContextError::new(error.to_string()))?;
        let kind = semio_framework_repo_providers::mcp_kind_from_resolved_client(&client);
        let request = TicketReopenRequest {
            id: id.id(),
            prompt: input.prompt,
            llm: input.llm,
            effort: input.effort,
            client: input.client,
            goal: input.goal,
            parent: input.parent,
            no_management: input.no_management,
            session: String::new(),
            plan_id: input.plan_id,
            spec_id: input.spec_id,
        };
        let outcome = self.with_tickets(kind, |service| service.reopen(&request)).map_err(|error| ContextError::new(error.message))?;
        for warning in &outcome.warnings {
            eprintln!("Warning: {warning}");
        }
        self.ticket_by_id(&outcome.id)
    }

    fn ticket_change(&self, input: TicketChangeInput) -> Result<Ticket, ContextError> {
        let id = self.retitle(&TicketId::new(input.year, input.month, input.day, input.slug.clone()), input.title.as_ref(), input.no_management)?;
        let mut ticket = self.ticket(id.year, id.month, id.day, &id.slug)?;
        let mut changed = false;
        if let Some(prompt) = &input.prompt {
            ticket.description = prompt.clone();
            if let Some(last) = ticket.interactions.last_mut() {
                last.prompt = prompt.clone();
            }
            changed = true;
        }
        if let Some(last) = ticket.interactions.last_mut() {
            if let Some(llm) = &input.llm {
                last.llm = semio_framework_repo_model::resolve_allowed_llm(llm).map_err(|error| ContextError::new(error.to_string()))?;
                changed = true;
            }
            if let Some(effort) = &input.effort {
                last.effort = semio_framework_repo_model::resolve_allowed_effort(effort).map_err(|error| ContextError::new(error.to_string()))?;
                changed = true;
            }
            if let Some(client) = &input.client {
                last.client = semio_framework_repo_model::resolve_allowed_client(client).map_err(|error| ContextError::new(error.to_string()))?;
                changed = true;
            }
        }
        if let Some(goal) = &input.goal {
            ticket.goal = goal.clone();
            changed = true;
        }
        if let Some(parent) = &input.parent {
            ticket.parent = parent.clone();
            changed = true;
        }
        if changed {
            self.with_tickets(McpClientKind::Generic, |service| service.save(&ticket)).map_err(|error| ContextError::new(error.message))?;
            semio_framework_repo_events::emit(
                semio_framework_repo_events::TICKET_CHANGE_ENDED,
                "repo-cli",
                &json!({ "id": id.id(), "year": id.year, "month": id.month, "day": id.day, "slug": id.slug, "title": input.title, "prompt": input.prompt, "llm": input.llm, "effort": input.effort, "goal": input.goal, "parent": input.parent, "author": self.author() }),
            );
        }
        Ok(ticket)
    }

    fn ticket_delete(&self, input: TicketDeleteInput) -> Result<bool, ContextError> {
        let ticket = self.ticket(input.year, input.month, input.day, &input.slug)?;
        if !input.no_management {
            if let Some(issue) = ticket.management.as_ref().map(|management| management.issue.clone()).filter(|issue| !issue.is_empty()) {
                let number = issue.rsplit('/').next().unwrap_or_default().to_string();
                ManagementProvider::delete_issue(&semio_framework_repo_providers::system_management_provider(), &number).map_err(|error| ContextError::new(error.message))?;
            }
        }
        std::fs::remove_dir_all(&ticket.folder_path).map_err(|error| ContextError::new(format!("{}: {error}", ticket.folder_path)))?;
        Ok(true)
    }

    fn contributor_add(&self, input: ContributorAddInput) -> Result<Contributor, ContextError> {
        let mut contributor = match semio_framework_repo_contributors::load_contributor(&self.root, &input.github) {
            Ok(contributor) => contributor,
            Err(_) => semio_framework_repo_contributors::create_contributor(&self.root, &input.github).map_err(ContextError::new)?,
        };
        let mut changed = false;
        let mut fill = |slot: &mut String, value: &Option<String>| {
            if let Some(value) = value.as_ref().filter(|value| !value.is_empty() && slot.is_empty()) {
                *slot = value.clone();
                changed = true;
            }
        };
        fill(&mut contributor.name, &input.name);
        fill(&mut contributor.email, &input.email);
        fill(&mut contributor.fingerprint, &input.fingerprint);
        for (list, values) in [(&mut contributor.names, &input.names), (&mut contributor.emails, &input.emails), (&mut contributor.fingerprints, &input.fingerprints)] {
            for value in values {
                if !list.contains(value) {
                    list.push(value.clone());
                    changed = true;
                }
            }
        }
        if changed {
            semio_framework_repo_contributors::save_contributor(&self.root, &contributor).map_err(ContextError::new)?;
            semio_framework_repo_events::emit(semio_framework_repo_events::CONTRIBUTOR_ADD_ENDED, "repo-cli", &json!({ "github": contributor.github, "author": self.author() }));
        }
        Ok(contributor)
    }

    fn contributor_remove(&self, github: &str) -> Result<(), ContextError> {
        semio_framework_repo_contributors::remove_contributor(&self.root, github).map_err(ContextError::new)?;
        semio_framework_repo_events::emit(semio_framework_repo_events::CONTRIBUTOR_REMOVE_ENDED, "repo-cli", &json!({ "github": github, "author": self.author() }));
        Ok(())
    }

    fn folder_create(&self, path: &str) -> Result<Folder, ContextError> {
        let workspace = self.workspace(&[], &[]);
        self.apply(semio_framework_repo_move::plan_folder_create(&workspace, path))?;
        Ok(Folder {
            id: self.codebase.build_folder_id(path),
            path: path.to_string(),
            uri: self.codebase.folder_uri(path),
            name: semio_framework_repo_identity::base_of(path),
            parent_id: None,
            bundle_id: None,
            kind: self.codebase.derive_folder_kind(path),
            emoji: String::new(),
            ignored: false,
            generated: semio_framework_repo_codebase::is_generated_folder(path),
        })
    }

    fn folder_move(&self, source: &str, destination: &str) -> Result<Folder, ContextError> {
        let workspace = self.workspace(&[], &[source, destination]);
        self.apply(semio_framework_repo_move::plan_folder_move(&workspace, source, destination))?;
        let mut folder = self.folder_create_record(destination);
        folder.id = self.codebase.build_folder_id(destination);
        Ok(folder)
    }

    fn folder_delete(&self, path: &str) -> Result<(), ContextError> {
        let workspace = self.workspace(&[], &[path]);
        self.apply(semio_framework_repo_move::plan_folder_delete(&workspace, path))?;
        Ok(())
    }

    fn file_create(&self, path: &str) -> Result<File, ContextError> {
        let workspace = self.workspace(&[path], &[]);
        let header = self.file_header(path);
        self.apply(semio_framework_repo_move::plan_file_create(&workspace, path, &header))?;
        Ok(self.file_record(path))
    }

    fn file_move(&self, source: &str, destination: &str) -> Result<File, ContextError> {
        let workspace = self.workspace(&[source, destination], &[]);
        self.apply(semio_framework_repo_move::plan_file_move(&workspace, source, destination))?;
        Ok(self.file_record(destination))
    }

    fn file_delete(&self, path: &str) -> Result<(), ContextError> {
        let workspace = self.workspace(&[path], &[]);
        self.apply(semio_framework_repo_move::plan_file_delete(&workspace, path))?;
        Ok(())
    }

    fn section_create(&self, file: &str, name: &str, parent: Option<&str>) -> Result<Section, ContextError> {
        let path = match parent.filter(|parent| !parent.is_empty()) {
            Some(parent) => format!("{parent}/{name}"),
            None => name.to_string(),
        };
        let workspace = self.workspace(&[file], &[]);
        self.apply(semio_framework_repo_move::plan_section_create(&workspace, file, &path))?;
        let id = semio_framework_repo_model::build_section_id(&self.codebase.build_file_id(file), &path.split('/').map(str::to_string).collect::<Vec<String>>());
        Ok(section_record(id, name, &path, file))
    }

    fn section_move(&self, file: &str, old_name: &str, new_name: &str) -> Result<Section, ContextError> {
        self.move_section(file, old_name, new_name)?;
        let id = semio_framework_repo_model::build_section_id(&self.codebase.build_file_id(file), &new_name.split('/').map(str::to_string).collect::<Vec<String>>());
        Ok(section_record(id, new_name, new_name, file))
    }

    fn section_delete(&self, file: &str, name: &str) -> Result<(), ContextError> {
        let workspace = self.workspace(&[file], &[]);
        self.apply(semio_framework_repo_move::plan_section_delete(&workspace, file, name))?;
        Ok(())
    }

    fn integrate(&self, source: Option<&str>, target_section: Option<&str>, target_file: Option<&str>, target_parent: Option<&str>) -> Result<File, ContextError> {
        let target = target_file.unwrap_or_default();
        self.integrate_file(source.unwrap_or_default(), target_section.unwrap_or_default(), target, target_parent.unwrap_or_default())?;
        Ok(self.file_record(target))
    }

    fn extract(&self, source_file: Option<&str>, source_section: Option<&str>, target_file: Option<&str>) -> Result<File, ContextError> {
        let target = target_file.unwrap_or_default();
        self.extract_section(source_file.unwrap_or_default(), source_section.unwrap_or_default(), target)?;
        Ok(self.file_record(target))
    }

    fn sync_management(&self) -> Result<bool, ContextError> {
        println!("Syncing local tickets and goals with GitHub...");
        let provider = semio_framework_repo_providers::system_management_provider();
        let mut valid: Vec<String> = vec!["repo".to_string()];
        for technology in self.codebase.technologies() {
            valid.push(format!("@{}", technology.name.trim_start_matches('@')));
            for bundle in technology.bundles.unwrap_or_default() {
                valid.push(semio_framework_repo_codebase::normalize_bundle_label(&bundle.name));
            }
        }
        valid.sort();
        valid.dedup();
        if let Err(error) = ManagementProvider::sync_repo_label_catalog(&provider, &valid) {
            eprintln!("Warning: Failed to sync GitHub label catalog: {}", error.message);
        }
        match self.goals() {
            Err(error) => eprintln!("Warning: Failed to list goals: {error}"),
            Ok(mut goals) => {
                goals.sort_by_key(|goal| semio_framework_repo_goals::goal_depth(&goal.id));
                for mut goal in goals {
                    self.sync_goal(&provider, &mut goal);
                }
            }
        }
        for ticket in self.tickets(None, None, None, None)? {
            let Some(issue) = ticket.management.as_ref().map(|management| management.issue.clone()).filter(|issue| !issue.is_empty()) else { continue };
            let remote = match ManagementProvider::get_issue_details(&provider, &issue) {
                Ok(Some(remote)) => remote,
                Ok(None) => continue,
                Err(error) => {
                    eprintln!("Warning: Failed to get GitHub issue {issue}: {}", error.message);
                    continue;
                }
            };
            if ticket.status == TicketStatus::Closed && remote.state.eq_ignore_ascii_case("open") {
                println!("Closing GitHub issue {issue} (Ticket is closed locally)");
                if let Err(error) = ManagementProvider::close_issue(&provider, &issue) {
                    eprintln!("Warning: Failed to close GitHub issue {issue}: {}", error.message);
                }
            }
            if !ticket.goal.is_empty() {
                if let Ok(mut root_goal) = self.with_goals(true, |goals| goals.read(&semio_framework_repo_goals::root_goal_id(&ticket.goal))) {
                    match self.ensure_goal_milestone(&provider, &mut root_goal) {
                        Err(error) => eprintln!("Warning: Failed to resolve goal milestone for issue {issue}: {error}"),
                        Ok(Some(milestone)) if !milestone.title.is_empty() && remote.milestone.as_ref().is_none_or(|current| current.title != milestone.title) => {
                            println!("Updating milestone for issue {issue} to {}...", milestone.title);
                            if let Err(error) = ManagementProvider::update_issue_milestone(&provider, &issue, &milestone.title) {
                                eprintln!("Warning: Failed to update milestone for GitHub issue {issue}: {}", error.message);
                            }
                        }
                        Ok(_) => {}
                    }
                }
            }
            prune_labels(&provider, &issue, &remote.labels, &valid);
        }
        match ManagementProvider::list_issues_for_label_sync(&provider) {
            Err(error) => eprintln!("Warning: Failed to list GitHub issues for label sync: {}", error.message),
            Ok(issues) => {
                for issue in issues {
                    prune_labels(&provider, &issue.url, &issue.labels, &valid);
                }
            }
        }
        println!("GitHub sync completed.");
        Ok(true)
    }
}

/// 💗️ The section record a section mutation answers with.
fn section_record(id: String, name: &str, path: &str, file: &str) -> Section {
    Section { id, name: name.to_string(), path: path.to_string(), file_path: file.to_string(), emoji: String::new(), start_line: 0, end_line: 0, start_index: 0, end_index: 0, children: Vec::new(), definitions: Vec::new() }
}

/// 🏷️ Removes every technology label an issue carries that the catalog no longer declares.
fn prune_labels(provider: &ManagementProviders, issue: &str, labels: &[ManagementLabel], valid: &[String]) {
    let stale: Vec<String> = labels.iter().map(|label| label.name.clone()).filter(|name| name.starts_with('@') && !valid.contains(name)).collect();
    if stale.is_empty() {
        return;
    }
    println!("Removing invalid technology labels from issue {issue}: {stale:?}");
    if let Err(error) = ManagementProvider::remove_labels(provider, issue, &stale) {
        eprintln!("Warning: Failed to remove labels from GitHub issue {issue}: {}", error.message);
    }
}

impl FsRepoContext {
    fn folder_create_record(&self, path: &str) -> Folder {
        Folder {
            id: String::new(),
            path: path.to_string(),
            uri: self.codebase.folder_uri(path),
            name: semio_framework_repo_identity::base_of(path),
            parent_id: None,
            bundle_id: None,
            kind: self.codebase.derive_folder_kind(path),
            emoji: String::new(),
            ignored: false,
            generated: semio_framework_repo_codebase::is_generated_folder(path),
        }
    }

    fn sync_goal(&self, provider: &ManagementProviders, goal: &mut Goal) {
        if semio_framework_repo_goals::is_root_goal(&goal.id) {
            if let Some(management) = goal.management.as_mut().filter(|management| !management.issue.is_empty()) {
                println!("Migrating root goal {}: removing issue reference", goal.id);
                management.issue.clear();
            }
            match self.ensure_goal_milestone(provider, goal) {
                Err(error) => eprintln!("Warning: Failed to ensure milestone for root goal {}: {error}", goal.id),
                Ok(Some(milestone)) => {
                    let state = if goal.status == GoalStatus::Closed { "closed" } else { "open" };
                    if let Err(error) = ManagementProvider::update_milestone(provider, milestone.number, &goal.title, &goal.description, state, &goal.dates.due) {
                        eprintln!("Warning: Failed to update milestone for root goal {}: {}", goal.id, error.message);
                    }
                }
                Ok(None) => {}
            }
            return;
        }
        if let Some(management) = goal.management.as_mut().filter(|management| !management.milestone.is_empty()) {
            println!("Migrating child goal {}: removing milestone", goal.id);
            if let Some(number) = parse_milestone_number(&management.milestone) {
                if let Err(error) = ManagementProvider::delete_milestone(provider, number) {
                    eprintln!("Warning: Failed to delete milestone {number} for child goal {}: {}", goal.id, error.message);
                }
            }
            management.milestone.clear();
        }
        let issue = goal.management.as_ref().map(|management| management.issue.clone()).unwrap_or_default();
        if issue.is_empty() {
            let milestone = if semio_framework_repo_goals::is_first_gen_goal(&goal.id) {
                self.with_goals(true, |goals| goals.read(&semio_framework_repo_goals::root_goal_id(&goal.id))).ok().and_then(|root| root.management).and_then(|management| parse_milestone_number(&management.milestone))
            } else {
                None
            };
            let created = match ManagementProvider::create_goal_issue(provider, &goal.title, &goal.description, milestone) {
                Ok(created) => created,
                Err(error) => {
                    eprintln!("Warning: Failed to create issue for child goal {}: {}", goal.id, error.message);
                    return;
                }
            };
            goal.management.get_or_insert_with(GoalManagementData::default).issue = created.clone();
            println!("Created issue for child goal {}: {created}", goal.id);
            if goal.status == GoalStatus::Closed {
                let _ = ManagementProvider::close_issue(provider, &created);
            }
            if semio_framework_repo_goals::is_deeper_goal(&goal.id) {
                let parent_id = semio_framework_repo_goals::parent_goal_id(&goal.id);
                if let Some(parent_issue) = self.with_goals(true, |goals| goals.read(&parent_id)).ok().and_then(|parent| parent.management).map(|management| management.issue).filter(|issue| !issue.is_empty()) {
                    match ManagementProvider::add_sub_issue(provider, &parent_issue, &created) {
                        Ok(()) => println!("Linked sub-issue {} to parent {parent_id}", goal.id),
                        Err(error) => eprintln!("Warning: Failed to link sub-issue {} to parent {parent_id}: {}", goal.id, error.message),
                    }
                }
            }
            if let Err(error) = self.save_goal(goal) {
                eprintln!("Warning: Failed to save goal {}: {error}", goal.id);
            }
            return;
        }
        let remote = match ManagementProvider::get_issue_details(provider, &issue) {
            Ok(Some(remote)) => remote,
            Ok(None) => return,
            Err(error) => {
                eprintln!("Warning: Failed to get issue for goal {}: {}", goal.id, error.message);
                return;
            }
        };
        if goal.status == GoalStatus::Closed && remote.state.eq_ignore_ascii_case("open") {
            let _ = ManagementProvider::close_issue(provider, &issue);
        } else if goal.status == GoalStatus::Open && remote.state.eq_ignore_ascii_case("closed") {
            let _ = ManagementProvider::reopen_issue(provider, &issue);
        }
        if semio_framework_repo_goals::is_first_gen_goal(&goal.id) {
            if let Ok(root_goal) = self.with_goals(true, |goals| goals.read(&semio_framework_repo_goals::root_goal_id(&goal.id))) {
                let has_milestone = root_goal.management.as_ref().is_some_and(|management| parse_milestone_number(&management.milestone).is_some());
                if has_milestone && remote.milestone.as_ref().is_none_or(|current| current.title != root_goal.title) {
                    let _ = ManagementProvider::update_issue_milestone(provider, &issue, &root_goal.title);
                }
            }
        } else if semio_framework_repo_goals::is_deeper_goal(&goal.id) {
            if remote.milestone.is_some() {
                if let Err(error) = ManagementProvider::clear_issue_milestone(provider, &issue) {
                    eprintln!("Warning: Failed to clear milestone for goal {}: {}", goal.id, error.message);
                }
            }
            let parent_id = semio_framework_repo_goals::parent_goal_id(&goal.id);
            if let Some(parent_issue) = self.with_goals(true, |goals| goals.read(&parent_id)).ok().and_then(|parent| parent.management).map(|management| management.issue).filter(|issue| !issue.is_empty()) {
                match ManagementProvider::get_issue_parent_url(provider, &issue) {
                    Err(error) => eprintln!("Warning: Failed to resolve parent for goal {}: {}", goal.id, error.message),
                    Ok(current) if current != parent_issue => match ManagementProvider::add_sub_issue(provider, &parent_issue, &issue) {
                        Ok(()) => println!("Linked sub-issue {} to parent {parent_id}", goal.id),
                        Err(error) => eprintln!("Warning: Failed to link sub-issue {} to parent {parent_id}: {}", goal.id, error.message),
                    },
                    Ok(_) => {}
                }
            }
        }
        if !remote.labels.iter().any(|label| label.name == "goal") {
            if let Err(error) = ManagementProvider::add_labels(provider, &issue, &["goal".to_string()]) {
                eprintln!("Warning: Failed to add goal label for {}: {}", goal.id, error.message);
            }
        }
    }
}

//#endregion 🗄️FsRepoContext

//#region 🧪️Tests

#[cfg(test)]
#[path = "../../🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#endregion 🧪️Tests
