//! 📐 Part21 — shared ISO 10303-21 (STEP physical file) tokenizer + generic instance graph.
//! Public and importable cross-artifact: any Part-21-syntax format builds a typed view on top
//! of this same generic graph (this crate's `step` AP214 and `ifc` IFC4 both do — IFC is
//! literally "STEP syntax + a different EXPRESS schema"). https://www.iso.org/standard/63141.html
//! Escape/matrix logic below was pre-verified via a standalone scratch binary per this
//! session's own convention (ticket `26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION`).

use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Write as _;

//#region 🔖️Value
/// 🔢️ Exact logical STEP real: decimal coefficient/scale plus an optional base-10 exponent.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Part21Decimal {
    pub negative: bool,
    pub coefficient: String,
    pub scale: u32,
    pub exponent: Option<i32>,
}

impl Part21Decimal {
    pub async fn parse(text: &str) -> Result<Self, String> {
        let (negative, unsigned) = match text.as_bytes().first() {
            Some(b'-') => (true, &text[1..]),
            Some(b'+') => (false, &text[1..]),
            _ => (false, text),
        };
        let (mantissa, exponent) = match unsigned.find(['E', 'e']) {
            Some(index) => (&unsigned[..index], Some(unsigned[index + 1..].parse::<i32>().map_err(|e| e.to_string())?)),
            None => (unsigned, None),
        };
        let (integer, fraction) = mantissa.split_once('.').ok_or_else(|| format!("STEP real requires decimal point: {text:?}"))?;
        if integer.is_empty() || !integer.bytes().all(|b| b.is_ascii_digit()) || !fraction.bytes().all(|b| b.is_ascii_digit()) {
            return Err(format!("invalid STEP real: {text:?}"));
        }
        Ok(Self { negative, coefficient: format!("{integer}{fraction}"), scale: fraction.len() as u32, exponent })
    }

    pub async fn from_f64(value: f64) -> Self {
        let text = format!("{value}");
        let normalized = if text.contains('.') || text.contains('e') || text.contains('E') { text } else { format!("{text}.") };
        Self::parse(&normalized).await.expect("finite f64 has valid STEP decimal form")
    }

    pub async fn to_f64(&self) -> Option<f64> {
        self.to_string().parse().ok()
    }
}

impl From<f64> for Part21Decimal {
    fn from(value: f64) -> Self {
        Self::from_f64(value)
    }
}

impl fmt::Display for Part21Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.negative {
            f.write_char('-')?;
        }
        let scale = self.scale as usize;
        if scale >= self.coefficient.len() {
            f.write_str("0.")?;
            for _ in 0..scale.saturating_sub(self.coefficient.len()) {
                f.write_char('0')?;
            }
            f.write_str(&self.coefficient)?;
        } else {
            let split = self.coefficient.len() - scale;
            f.write_str(&self.coefficient[..split])?;
            f.write_char('.')?;
            f.write_str(&self.coefficient[split..])?;
        }
        if let Some(exponent) = self.exponent {
            write!(f, "E{exponent}")?;
        }
        Ok(())
    }
}

/// 🔤️ A single typed value in Part-21 argument-list syntax.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Part21Value {
    Ref(u64),
    Str(String),
    Enum(String),
    Int(i64),
    Real(Part21Decimal),
    List(Vec<Part21Value>),
    /// 🏷️ A "defined type" wrapper appearing as an argument, e.g. `IFCLENGTHMEASURE(3000.)`.
    Typed(String, Vec<Part21Value>),
    Unset,
    Derived,
}

impl Part21Value {
    pub async fn as_ref_id(&self) -> Option<u64> {
        if let Part21Value::Ref(id) = self {
            Some(*id)
        } else {
            None
        }
    }
    pub async fn as_str(&self) -> Option<&str> {
        if let Part21Value::Str(s) = self {
            Some(s.as_str())
        } else {
            None
        }
    }
    pub async fn as_enum(&self) -> Option<&str> {
        if let Part21Value::Enum(s) = self {
            Some(s.as_str())
        } else {
            None
        }
    }
    pub async fn as_real(&self) -> Option<f64> {
        match self {
            Part21Value::Real(r) => r.to_f64().await,
            Part21Value::Int(i) => Some(*i as f64),
            _ => None,
        }
    }
    pub async fn as_list(&self) -> Option<&[Part21Value]> {
        if let Part21Value::List(items) = self {
            Some(items.as_slice())
        } else {
            None
        }
    }
    pub async fn as_typed(&self) -> Option<(&str, &[Part21Value])> {
        if let Part21Value::Typed(name, items) = self {
            Some((name.as_str(), items.as_slice()))
        } else {
            None
        }
    }
    pub async fn is_unset(&self) -> bool {
        matches!(self, Part21Value::Unset)
    }
}
//#endregion 🔖️Value

//#region 🔖️Instance
/// 🧩️ One `#N = TYPE(args...)` line, or `#N = (TYPE1(...) TYPE2(...))` for a complex instance —
/// every `(type_name, args)` pair is kept, nothing about a multi-type instance is dropped.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Part21Instance {
    pub id: u64,
    pub entities: Vec<(String, Vec<Part21Value>)>,
}

impl Part21Instance {
    pub async fn entity(&self, type_name: &str) -> Option<&Vec<Part21Value>> {
        self.entities.iter().find(|(name, _)| name.eq_ignore_ascii_case(type_name)).map(|(_, args)| args)
    }
    pub async fn primary(&self) -> Option<(&str, &Vec<Part21Value>)> {
        self.entities.first().map(|(name, args)| (name.as_str(), args))
    }
    pub async fn is_type(&self, type_name: &str) -> bool {
        self.entities.iter().any(|(name, _)| name.eq_ignore_ascii_case(type_name))
    }
}
//#endregion 🔖️Instance

//#region 🔖️Header
/// 📇️ The three standard `HEADER;` records (`FILE_DESCRIPTION`/`FILE_NAME`/`FILE_SCHEMA`),
/// each a parenthesized tuple of typed values — kept verbatim, not schema-interpreted.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Part21Header {
    pub file_description: Vec<Part21Value>,
    pub file_name: Vec<Part21Value>,
    pub file_schema: Vec<Part21Value>,
}
//#endregion 🔖️Header

//#region 🔖️Document
/// 📦️ The full, lossless generic Part-21 graph: header + every `DATA;` instance.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Part21Document {
    pub header: Part21Header,
    pub instances: Vec<Part21Instance>,
}

impl Part21Document {
    pub async fn instance(&self, id: u64) -> Option<&Part21Instance> {
        self.instances.iter().find(|i| i.id == id)
    }
    pub async fn resolve(&self, value: &Part21Value) -> Option<&Part21Instance> {
        value.as_ref_id().await.and_then(|id| self.instance(id))
    }
    pub async fn by_type<'a>(&'a self, type_name: &'a str) -> impl Iterator<Item = &'a Part21Instance> + 'a {
        self.instances.iter().filter(move |i| i.is_type(type_name))
    }
    pub async fn next_id(&self) -> u64 {
        self.instances.iter().map(|i| i.id).max().unwrap_or(0) + 1
    }
}
//#endregion 🔖️Document

//#region 🔖️Builder
/// 🏗️ Shared incremental-id instance allocator — every Part-21 writer (step's BrepMesh writer,
/// ifc's future typed writers) builds its generated graph through this, so id allocation and
/// header shape stay consistent across artifacts instead of each hand-rolling a counter.
#[derive(Default)]
pub struct Part21Builder {
    pub instances: Vec<Part21Instance>,
    next_id: u64,
}

impl Part21Builder {
    pub async fn new() -> Self {
        Self { instances: Vec::new(), next_id: 1 }
    }
    /// ➕️ Allocates the next `#id` and appends a simple `TYPE(args)` instance.
    pub async fn alloc(&mut self, type_name: &str, args: Vec<Part21Value>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.instances.push(Part21Instance { id, entities: vec![(type_name.to_string(), args)] });
        id
    }
    pub async fn build(self, header: Part21Header) -> Part21Document {
        Part21Document { header, instances: self.instances }
    }
}
//#endregion 🔖️Builder

//#region 🔖️Error
/// 🚫️ Real parse failures — malformed input always surfaces here, never silently fabricated.
#[derive(Clone, Debug, PartialEq)]
pub enum Part21Error {
    UnexpectedEof { at: usize, expected: &'static str },
    UnexpectedChar { at: usize, found: char, expected: &'static str },
    UnsupportedEscape { at: usize, detail: String },
    InvalidNumber { at: usize, text: String },
    MissingLiteral { at: usize, literal: &'static str },
}

impl fmt::Display for Part21Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Part21Error::UnexpectedEof { at, expected } => write!(f, "part21: unexpected end of input at char {at}, expected {expected}"),
            Part21Error::UnexpectedChar { at, found, expected } => write!(f, "part21: unexpected char {found:?} at {at}, expected {expected}"),
            Part21Error::UnsupportedEscape { at, detail } => write!(f, "part21: unsupported string escape at {at}: {detail}"),
            Part21Error::InvalidNumber { at, text } => write!(f, "part21: invalid number {text:?} at {at}"),
            Part21Error::MissingLiteral { at, literal } => write!(f, "part21: expected literal {literal:?} at {at}"),
        }
    }
}
impl std::error::Error for Part21Error {}
//#endregion 🔖️Error

//#region 🔖️Lexer
struct Lexer {
    chars: Vec<char>,
    pos: usize,
}

impl Lexer {
    async fn new(text: &str) -> Self {
        Self { chars: text.chars().collect(), pos: 0 }
    }
    async fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }
    async fn peek_at(&self, offset: usize) -> Option<char> {
        self.chars.get(self.pos + offset).copied()
    }
    async fn bump(&mut self) -> Option<char> {
        let c = self.peek().await;
        if c.is_some() {
            self.pos += 1;
        }
        c
    }

    async fn skip_ws_and_comments(&mut self) {
        loop {
            match self.peek().await {
                Some(c) if c.is_whitespace() => self.pos += 1,
                Some('/') if self.peek_at(1) == Some('*') => {
                    self.pos += 2;
                    while let Some(c) = self.peek().await {
                        if c == '*' && self.peek_at(1) == Some('/') {
                            self.pos += 2;
                            break;
                        }
                        self.pos += 1;
                    }
                }
                _ => break,
            }
        }
    }

    async fn expect_literal(&mut self, lit: &'static str) -> Result<(), Part21Error> {
        self.skip_ws_and_comments();
        for c in lit.chars() {
            if self.peek() == Some(c) {
                self.pos += 1;
            } else {
                return Err(Part21Error::MissingLiteral { at: self.pos, literal: lit });
            }
        }
        Ok(())
    }

    async fn try_literal(&mut self, lit: &str) -> bool {
        let save = self.pos;
        self.skip_ws_and_comments();
        for c in lit.chars() {
            if self.peek() == Some(c) {
                self.pos += 1;
            } else {
                self.pos = save;
                return false;
            }
        }
        true
    }

    async fn read_keyword(&mut self) -> Result<String, Part21Error> {
        self.skip_ws_and_comments();
        let start = self.pos;
        let mut s = String::new();
        while let Some(c) = self.peek().await {
            if c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_' {
                s.push(c);
                self.pos += 1;
            } else {
                break;
            }
        }
        if s.is_empty() {
            return Err(Part21Error::UnexpectedChar { at: start, found: self.peek().await.unwrap_or('\0'), expected: "keyword" });
        }
        Ok(s)
    }

    async fn read_string(&mut self) -> Result<String, Part21Error> {
        self.skip_ws_and_comments();
        if self.peek() != Some('\'') {
            return Err(Part21Error::UnexpectedChar { at: self.pos, found: self.peek().await.unwrap_or('\0'), expected: "'" });
        }
        self.pos += 1;
        let mut out = String::new();
        loop {
            match self.bump().await {
                None => return Err(Part21Error::UnexpectedEof { at: self.pos, expected: "closing '" }),
                Some('\'') => {
                    if self.peek() == Some('\'') {
                        self.pos += 1;
                        out.push('\'');
                    } else {
                        break;
                    }
                }
                Some('\\') => self.read_escape(&mut out).await?,
                Some(c) => out.push(c),
            }
        }
        Ok(out)
    }

    /// 🔤️ `\X\HH\` single byte, `\X2\HHHH(HHHH)*\X0\` UCS-2 run — the two Part-21 escape
    /// forms this codebase's fixtures/writer actually emit; anything else is a typed error.
    async fn read_escape(&mut self, out: &mut String) -> Result<(), Part21Error> {
        let start = self.pos - 1;
        match self.bump().await {
            Some('X') => match self.peek().await {
                Some('2') => {
                    self.pos += 1;
                    if self.bump() != Some('\\') {
                        return Err(Part21Error::UnsupportedEscape { at: start, detail: "expected \\ after \\X2".into() });
                    }
                    loop {
                        let mut hex = String::new();
                        for _ in 0..4 {
                            match self.bump().await {
                                Some(c) if c.is_ascii_hexdigit() => hex.push(c),
                                _ => return Err(Part21Error::UnsupportedEscape { at: start, detail: "bad \\X2\\ hex group".into() }),
                            }
                        }
                        let code = u32::from_str_radix(&hex, 16).map_err(|_| Part21Error::UnsupportedEscape { at: start, detail: "bad hex".into() })?;
                        let ch = char::from_u32(code).ok_or_else(|| Part21Error::UnsupportedEscape { at: start, detail: format!("bad codepoint {code}") })?;
                        out.push(ch);
                        if self.peek() == Some('\\') && self.peek_at(1) == Some('X') && self.peek_at(2) == Some('0') {
                            self.pos += 3;
                            if self.bump() != Some('\\') {
                                return Err(Part21Error::UnsupportedEscape { at: start, detail: "expected trailing \\ after \\X0".into() });
                            }
                            break;
                        }
                    }
                    Ok(())
                }
                _ => {
                    if self.bump() != Some('\\') {
                        return Err(Part21Error::UnsupportedEscape { at: start, detail: "expected \\ after \\X".into() });
                    }
                    let mut hex = String::new();
                    for _ in 0..2 {
                        match self.bump().await {
                            Some(c) if c.is_ascii_hexdigit() => hex.push(c),
                            _ => return Err(Part21Error::UnsupportedEscape { at: start, detail: "bad \\X\\ hex".into() }),
                        }
                    }
                    if self.bump() != Some('\\') {
                        return Err(Part21Error::UnsupportedEscape { at: start, detail: "expected trailing \\ after \\X..".into() });
                    }
                    let code = u32::from_str_radix(&hex, 16).map_err(|_| Part21Error::UnsupportedEscape { at: start, detail: "bad hex".into() })?;
                    let ch = char::from_u32(code).ok_or_else(|| Part21Error::UnsupportedEscape { at: start, detail: format!("bad byte {code}") })?;
                    out.push(ch);
                    Ok(())
                }
            },
            other => Err(Part21Error::UnsupportedEscape { at: start, detail: format!("unsupported escape start {other:?}") }),
        }
    }

    async fn read_number(&mut self) -> Result<Part21Value, Part21Error> {
        self.skip_ws_and_comments();
        let start = self.pos;
        let mut s = String::new();
        if matches!(self.peek().await, Some('-') | Some('+')) {
            s.push(self.bump().await.unwrap());
        }
        while let Some(c) = self.peek().await {
            if c.is_ascii_digit() {
                s.push(c);
                self.pos += 1;
            } else {
                break;
            }
        }
        let mut is_real = false;
        if self.peek() == Some('.') {
            is_real = true;
            s.push('.');
            self.pos += 1;
            while let Some(c) = self.peek().await {
                if c.is_ascii_digit() {
                    s.push(c);
                    self.pos += 1;
                } else {
                    break;
                }
            }
        }
        if matches!(self.peek().await, Some('E') | Some('e')) {
            is_real = true;
            s.push('E');
            self.pos += 1;
            if matches!(self.peek().await, Some('+') | Some('-')) {
                s.push(self.bump().await.unwrap());
            }
            while let Some(c) = self.peek().await {
                if c.is_ascii_digit() {
                    s.push(c);
                    self.pos += 1;
                } else {
                    break;
                }
            }
        }
        if is_real {
            Part21Decimal::parse(&s).await.map(Part21Value::Real).map_err(|_| Part21Error::InvalidNumber { at: start, text: s })
        } else {
            s.parse::<i64>().map(Part21Value::Int).map_err(|_| Part21Error::InvalidNumber { at: start, text: s })
        }
    }

    async fn read_enum(&mut self) -> Result<String, Part21Error> {
        self.skip_ws_and_comments();
        if self.peek() != Some('.') {
            return Err(Part21Error::UnexpectedChar { at: self.pos, found: self.peek().await.unwrap_or('\0'), expected: "." });
        }
        self.pos += 1;
        let mut s = String::new();
        while let Some(c) = self.peek().await {
            if c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_' {
                s.push(c);
                self.pos += 1;
            } else {
                break;
            }
        }
        if self.bump() != Some('.') {
            return Err(Part21Error::MissingLiteral { at: self.pos, literal: "." });
        }
        Ok(s)
    }

    async fn read_value(&mut self) -> Result<Part21Value, Part21Error> {
        self.skip_ws_and_comments();
        match self.peek().await {
            Some('$') => {
                self.pos += 1;
                Ok(Part21Value::Unset)
            }
            Some('*') => {
                self.pos += 1;
                Ok(Part21Value::Derived)
            }
            Some('#') => {
                self.pos += 1;
                let start = self.pos;
                let mut s = String::new();
                while let Some(c) = self.peek().await {
                    if c.is_ascii_digit() {
                        s.push(c);
                        self.pos += 1;
                    } else {
                        break;
                    }
                }
                s.parse::<u64>().map(Part21Value::Ref).map_err(|_| Part21Error::InvalidNumber { at: start, text: s })
            }
            Some('\'') => self.read_string().await.map(Part21Value::Str),
            Some('.') => self.read_enum().await.map(Part21Value::Enum),
            Some('(') => {
                self.pos += 1;
                let items = self.read_value_list().await?;
                self.expect_literal(")").await?;
                Ok(Part21Value::List(items))
            }
            Some(c) if c.is_ascii_digit() || c == '-' || c == '+' => self.read_number().await,
            Some(c) if c.is_ascii_uppercase() => {
                let kw = self.read_keyword().await?;
                self.skip_ws_and_comments();
                if self.peek() == Some('(') {
                    self.pos += 1;
                    let items = self.read_value_list().await?;
                    self.expect_literal(")").await?;
                    Ok(Part21Value::Typed(kw, items))
                } else {
                    Err(Part21Error::UnexpectedChar { at: self.pos, found: self.peek().await.unwrap_or('\0'), expected: "( after typed value keyword" })
                }
            }
            Some(c) => Err(Part21Error::UnexpectedChar { at: self.pos, found: c, expected: "value" }),
            None => Err(Part21Error::UnexpectedEof { at: self.pos, expected: "value" }),
        }
    }

    async fn read_value_list(&mut self) -> Result<Vec<Part21Value>, Part21Error> {
        self.skip_ws_and_comments();
        let mut out = Vec::new();
        if self.peek() == Some(')') {
            return Ok(out);
        }
        loop {
            out.push(self.read_value().await?);
            self.skip_ws_and_comments();
            if self.peek() == Some(',') {
                self.pos += 1;
                continue;
            }
            break;
        }
        Ok(out)
    }

    async fn read_record(&mut self) -> Result<(String, Vec<Part21Value>), Part21Error> {
        let name = self.read_keyword().await?;
        self.expect_literal("(").await?;
        let args = self.read_value_list().await?;
        self.expect_literal(")").await?;
        Ok((name, args))
    }

    async fn read_instance(&mut self) -> Result<Part21Instance, Part21Error> {
        self.expect_literal("#").await?;
        let start = self.pos;
        let mut id_s = String::new();
        while let Some(c) = self.peek().await {
            if c.is_ascii_digit() {
                id_s.push(c);
                self.pos += 1;
            } else {
                break;
            }
        }
        let id = id_s.parse::<u64>().map_err(|_| Part21Error::InvalidNumber { at: start, text: id_s })?;
        self.expect_literal("=").await?;
        self.skip_ws_and_comments();
        let mut entities = Vec::new();
        if self.peek() == Some('(') {
            self.pos += 1;
            loop {
                self.skip_ws_and_comments();
                if self.peek() == Some(')') {
                    break;
                }
                entities.push(self.read_record().await?);
                self.skip_ws_and_comments();
            }
            self.expect_literal(")").await?;
        } else {
            entities.push(self.read_record().await?);
        }
        self.expect_literal(";").await?;
        Ok(Part21Instance { id, entities })
    }
}
//#endregion 🔖️Lexer

//#region 🔖️Parse
/// 📥️ Parses a full ISO 10303-21 physical file into the generic graph. Real tokenizer,
/// not a scraper — every header record and every data instance/argument round-trips.
pub async fn parse_part21(text: &str) -> Result<Part21Document, Part21Error> {
    let mut lex = Lexer::new(text).await;
    lex.expect_literal("ISO-10303-21;").await?;
    lex.expect_literal("HEADER;").await?;
    let mut header = Part21Header::default();
    loop {
        lex.skip_ws_and_comments().await;
        if lex.try_literal("ENDSEC;").await {
            break;
        }
        let (name, args) = lex.read_record().await?;
        lex.expect_literal(";").await?;
        match name.as_str() {
            "FILE_DESCRIPTION" => header.file_description = args,
            "FILE_NAME" => header.file_name = args,
            "FILE_SCHEMA" => header.file_schema = args,
            _ => {}
        }
    }
    lex.expect_literal("DATA;").await?;
    let mut instances = Vec::new();
    loop {
        lex.skip_ws_and_comments().await;
        if lex.try_literal("ENDSEC;").await {
            break;
        }
        instances.push(lex.read_instance().await?);
    }
    let _ = lex.try_literal("END-ISO-10303-21;").await;
    Ok(Part21Document { header, instances })
}
//#endregion 🔖️Parse

//#region 🔖️Write
/// 🧭️ Deterministic physical layout selected by a standard-specific serializer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Part21WriteOptions {
    pub line_ending: &'static str,
    pub blank_after_header: bool,
    pub blank_before_data: bool,
    pub blank_before_terminator: bool,
    pub space_after_instance_equals: bool,
}

impl Default for Part21WriteOptions {
    fn default() -> Self {
        Self { line_ending: "\n", blank_after_header: false, blank_before_data: false, blank_before_terminator: false, space_after_instance_equals: false }
    }
}

/// 🏭️ Logical producer metadata capable of deterministic Part-21 header materialization.
pub trait Part21Preamble {
    async fn write_preamble(&self, out: &mut String, line_ending: &str);
}

/// 🈳️ Zero-sized stand-in preamble type (O1 — R11(a): `write_part21_with`'s preamble is a
/// borrowed-reference parameter, trivially generic; `write_part21` still needs SOME concrete type to
/// instantiate that generic with when it passes `None`, and picking a real implementor — e.g. ifc's
/// `Ifc2x3EdmPreamble` — would make this generic `step` module depend downward on a specific format
/// built on top of it). Never constructed; `write_preamble` is unreachable by construction.
struct NoPreamble;

impl Part21Preamble for NoPreamble {
    async fn write_preamble(&self, _out: &mut String, _line_ending: &str) {}
}

/// 📤️ Regenerates valid Part-21 text from the generic graph — round-trip losslessness is
/// the writer's job; it never re-derives STEP/IFC semantics.
pub async fn write_part21(doc: &Part21Document) -> String {
    write_part21_with::<NoPreamble>(doc, Part21WriteOptions::default(), None).await
}

/// 📤️ Regenerates Part-21 with a standard-selected deterministic layout and typed preamble. Generic
/// over the preamble implementor (O1 — R11(a): borrowed-reference parameter, trivially generic).
pub async fn write_part21_with<P: Part21Preamble>(doc: &Part21Document, options: Part21WriteOptions, preamble: Option<&P>) -> String {
    let eol = options.line_ending;
    let mut out = format!("ISO-10303-21;{eol}HEADER;{eol}");
    if options.blank_after_header {
        out.push_str(eol);
    }
    if let Some(preamble) = preamble {
        preamble.write_preamble(&mut out, eol);
    }
    write_record(&mut out, "FILE_DESCRIPTION", &doc.header.file_description, eol);
    write_record(&mut out, "FILE_NAME", &doc.header.file_name, eol);
    write_record(&mut out, "FILE_SCHEMA", &doc.header.file_schema, eol);
    out.push_str("ENDSEC;");
    out.push_str(eol);
    if options.blank_before_data {
        out.push_str(eol);
    }
    out.push_str("DATA;");
    out.push_str(eol);
    for inst in &doc.instances {
        write_instance(&mut out, inst, eol, options.space_after_instance_equals);
    }
    out.push_str("ENDSEC;");
    out.push_str(eol);
    if options.blank_before_terminator {
        out.push_str(eol);
    }
    out.push_str("END-ISO-10303-21;");
    out.push_str(eol);
    out
}

async fn write_record(out: &mut String, name: &str, args: &[Part21Value], line_ending: &str) {
    out.push_str(name);
    out.push('(');
    write_value_list(out, args);
    out.push_str(");");
    out.push_str(line_ending);
}

async fn write_instance(out: &mut String, inst: &Part21Instance, line_ending: &str, space_after_equals: bool) {
    let _ = write!(out, "#{}=", inst.id);
    if space_after_equals {
        out.push(' ');
    }
    if inst.entities.len() == 1 {
        let (name, args) = &inst.entities[0];
        out.push_str(name);
        out.push('(');
        write_value_list(out, args);
        out.push(')');
    } else {
        out.push('(');
        for (name, args) in &inst.entities {
            out.push_str(name);
            out.push('(');
            write_value_list(out, args);
            out.push(')');
        }
        out.push(')');
    }
    out.push(';');
    out.push_str(line_ending);
}

async fn write_value_list(out: &mut String, items: &[Part21Value]) {
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        write_value(out, item);
    }
}

async fn write_value(out: &mut String, v: &Part21Value) {
    match v {
        Part21Value::Ref(id) => {
            let _ = write!(out, "#{id}");
        }
        Part21Value::Str(s) => {
            out.push('\'');
            out.push_str(&escape_part21_string(s));
            out.push('\'');
        }
        Part21Value::Enum(name) => {
            out.push('.');
            out.push_str(name);
            out.push('.');
        }
        Part21Value::Int(i) => {
            let _ = write!(out, "{i}");
        }
        Part21Value::Real(r) => write!(out, "{r}").expect("String write"),
        Part21Value::List(items) => {
            out.push('(');
            write_value_list(out, items);
            out.push(')');
        }
        Part21Value::Typed(name, items) => {
            out.push_str(name);
            out.push('(');
            write_value_list(out, items);
            out.push(')');
        }
        Part21Value::Unset => out.push('$'),
        Part21Value::Derived => out.push('*'),
    }
}

/// 🔡️ Inverse of the lexer's `read_escape`: `'` doubles, backslash is escaped (to stay
/// unambiguous with `\X..` on reparse), any other non-printable-ASCII goes through `\X2\..\X0\`.
async fn escape_part21_string(s: &str) -> String {
    let mut out = String::new();
    let mut run: Vec<char> = Vec::new();
    async fn flush(run: &mut Vec<char>, out: &mut String) {
        if run.is_empty() {
            return;
        }
        out.push_str("\\X2\\");
        for c in run.drain(..) {
            let _ = write!(out, "{:04X}", c as u32);
        }
        out.push_str("\\X0\\");
    }
    for c in s.chars() {
        if c == '\'' {
            flush(&mut run, &mut out);
            out.push_str("''");
        } else if c == '\\' {
            flush(&mut run, &mut out);
            run.push(c);
            flush(&mut run, &mut out);
        } else if (0x20..=0x7E).contains(&(c as u32)) {
            flush(&mut run, &mut out);
            out.push(c);
        } else {
            run.push(c);
        }
    }
    flush(&mut run, &mut out);
    out
}
//#endregion 🔖️Write

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('semio.step','2026-08-10T00:00:00',('Ueli'),('semio'),'semio','','');\nFILE_SCHEMA(('AUTOMOTIVE_DESIGN'));\nENDSEC;\nDATA;\n#1=CARTESIAN_POINT('',(0.,0.,0.));\n#2=CARTESIAN_POINT('',(10.,0.,0.));\n#3=CARTESIAN_POINT('',(10.,10.,0.));\n#4=DIRECTION('',(0.,0.,1.));\n#5=VERTEX_POINT('',#1);\n#6=VERTEX_POINT('',#2);\n#7=VERTEX_POINT('',#3);\n#8=EDGE_CURVE('',#5,#6,#20,.T.);\n#9=EDGE_CURVE('',#6,#7,#21,.T.);\n#10=EDGE_CURVE('',#7,#5,#22,.T.);\n#20=LINE('',#1,#30);\n#21=LINE('',#2,#31);\n#22=LINE('',#3,#32);\n#30=VECTOR('',#4,1.);\n#31=VECTOR('',#4,1.);\n#32=VECTOR('',#4,1.);\n#11=ORIENTED_EDGE('',*,*,#8,.T.);\n#12=ORIENTED_EDGE('',*,*,#9,.T.);\n#13=ORIENTED_EDGE('',*,*,#10,.T.);\n#14=EDGE_LOOP('',(#11,#12,#13));\n#15=FACE_OUTER_BOUND('',#14,.T.);\n#16=PLANE('',#40);\n#40=AXIS2_PLACEMENT_3D('',#1,#4,$);\n#17=ADVANCED_FACE('',(#15),#16,.T.);\n#18=CLOSED_SHELL('',(#17));\n#19=MANIFOLD_SOLID_BREP('',#18);\nENDSEC;\nEND-ISO-10303-21;\n";

    #[semio_framework_async_macros::async_test]
    async fn round_trip_parse_serialize_reparse() {
        let doc = parse_part21(FIXTURE).expect("parse fixture");
        assert!(!doc.instances.is_empty());
        assert_eq!(doc.header.file_schema, vec![Part21Value::List(vec![Part21Value::Str("AUTOMOTIVE_DESIGN".into())])]);
        let text = write_part21(&doc);
        let reparsed = parse_part21(&text).expect("reparse generated text");
        assert_eq!(doc, reparsed, "round trip must be lossless at the graph level");
    }

    #[semio_framework_async_macros::async_test]
    async fn instance_count_and_types_preserved() {
        let doc = parse_part21(FIXTURE).expect("parse");
        assert_eq!(doc.instances.len(), 26);
        assert_eq!(doc.by_type("CARTESIAN_POINT").count(), 3);
        assert_eq!(doc.by_type("ADVANCED_FACE").count(), 1);
        let derived_placement = doc.instance(40).expect("axis2placement");
        let args = derived_placement.entity("AXIS2_PLACEMENT_3D").expect("typed");
        assert!(matches!(args[3], Part21Value::Unset));
    }

    #[semio_framework_async_macros::async_test]
    async fn oriented_edge_derived_attrs_are_star() {
        let doc = parse_part21(FIXTURE).expect("parse");
        let oe = doc.instance(11).expect("oriented edge");
        let args = oe.entity("ORIENTED_EDGE").expect("typed");
        assert_eq!(args[1], Part21Value::Derived);
        assert_eq!(args[2], Part21Value::Derived);
    }

    #[semio_framework_async_macros::async_test]
    async fn complex_instance_keeps_every_type() {
        let text =
            "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('IFC4'));\nENDSEC;\nDATA;\n#1=(IFCQUANTITYAREA($,$,$,10.5,$)IFCPHYSICALSIMPLEQUANTITY($,$,$,$));\nENDSEC;\nEND-ISO-10303-21;\n";
        let doc = parse_part21(text).expect("parse complex instance");
        let inst = doc.instance(1).expect("instance 1");
        assert_eq!(inst.entities.len(), 2);
        assert_eq!(inst.entities[0].0, "IFCQUANTITYAREA");
        assert_eq!(inst.entities[1].0, "IFCPHYSICALSIMPLEQUANTITY");
        let round = write_part21(&doc);
        assert_eq!(parse_part21(&round).expect("reparse"), doc);
    }

    #[semio_framework_async_macros::async_test]
    async fn typed_value_wrapper_round_trips() {
        let text = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('IFC4'));\nENDSEC;\nDATA;\n#1=IFCPROPERTYSINGLEVALUE('Height',$,IFCLENGTHMEASURE(3000.),$);\nENDSEC;\nEND-ISO-10303-21;\n";
        let doc = parse_part21(text).expect("parse");
        let args = doc.instance(1).unwrap().entity("IFCPROPERTYSINGLEVALUE").unwrap();
        let (name, inner) = args[2].as_typed().expect("typed value");
        assert_eq!(name, "IFCLENGTHMEASURE");
        assert_eq!(inner[0].as_real(), Some(3000.0));
        assert_eq!(parse_part21(&write_part21(&doc)).unwrap(), doc);
    }

    #[semio_framework_async_macros::async_test]
    async fn string_escapes_round_trip() {
        for raw in ["it's a test", "unicode: \u{20AC} \u{4E2D}\u{6587}", "back\\slash", "", "plain"] {
            let text = format!("ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('X'));\nENDSEC;\nDATA;\n#1=LABEL('{}');\nENDSEC;\nEND-ISO-10303-21;\n", escape_part21_string(raw));
            let doc = parse_part21(&text).unwrap_or_else(|e| panic!("parse {raw:?}: {e}"));
            let got = doc.instance(1).unwrap().entity("LABEL").unwrap()[0].as_str().unwrap();
            assert_eq!(got, raw, "escape round trip for {raw:?}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn doubled_quote_escape() {
        let text = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('X'));\nENDSEC;\nDATA;\n#1=LABEL('it''s here');\nENDSEC;\nEND-ISO-10303-21;\n";
        let doc = parse_part21(text).expect("parse");
        assert_eq!(doc.instance(1).unwrap().entity("LABEL").unwrap()[0].as_str(), Some("it's here"));
    }

    #[semio_framework_async_macros::async_test]
    async fn unicode_x2_escape() {
        let text = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('X'));\nENDSEC;\nDATA;\n#1=LABEL('\\X2\\4E2D6587\\X0\\');\nENDSEC;\nEND-ISO-10303-21;\n";
        let doc = parse_part21(text).expect("parse");
        assert_eq!(doc.instance(1).unwrap().entity("LABEL").unwrap()[0].as_str(), Some("中文"));
    }

    #[semio_framework_async_macros::async_test]
    async fn unset_and_derived_values() {
        let text = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('X'));\nENDSEC;\nDATA;\n#1=THING($,*,1);\nENDSEC;\nEND-ISO-10303-21;\n";
        let doc = parse_part21(text).expect("parse");
        let args = doc.instance(1).unwrap().entity("THING").unwrap();
        assert_eq!(args[0], Part21Value::Unset);
        assert_eq!(args[1], Part21Value::Derived);
        assert_eq!(args[2], Part21Value::Int(1));
    }

    #[semio_framework_async_macros::async_test]
    async fn nested_lists_round_trip() {
        let text = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('X'));\nENDSEC;\nDATA;\n#1=MATRIX(((1.,0.),(0.,1.)));\nENDSEC;\nEND-ISO-10303-21;\n";
        let doc = parse_part21(text).expect("parse");
        let args = doc.instance(1).unwrap().entity("MATRIX").unwrap();
        let outer = args[0].as_list().expect("outer list");
        assert_eq!(outer.len(), 2);
        assert_eq!(outer[0].as_list().unwrap()[0].as_real(), Some(1.0));
        assert_eq!(parse_part21(&write_part21(&doc)).unwrap(), doc);
    }

    #[semio_framework_async_macros::async_test]
    async fn malformed_input_is_typed_error_not_fabrication() {
        let bad = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('X'));\nENDSEC;\nDATA;\n#1=THING(;\nENDSEC;\nEND-ISO-10303-21;\n";
        assert!(parse_part21(bad).is_err());
    }
}
//#endregion 🧪️Tests
