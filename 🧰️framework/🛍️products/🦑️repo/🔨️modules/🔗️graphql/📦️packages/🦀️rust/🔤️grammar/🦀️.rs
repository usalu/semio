//! 🔤️ The repo GraphQL document grammar: lexer, AST, recursive-descent parser, validation and variable coercion, twin of the Go package.

use super::*;

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
    pub(crate) source: Vec<u8>,
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
pub(crate) fn quote_string(value: &str) -> String {
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
    pub(crate) error: Option<ParseError>,
}

impl Parser {
    pub(crate) fn new(source: &str) -> Self {
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

    pub(crate) fn take(&mut self, text: &str) -> Result<(), ParseError> {
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

    pub(crate) fn value(&mut self) -> Result<Value, ParseError> {
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
