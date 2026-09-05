//! 📐 Part21 — shared ISO 10303-21 (STEP physical file) tokenizer + generic instance graph.
//! Public and importable cross-artifact: any Part-21-syntax format builds a typed view on top
//! of this same generic graph (this crate's `step` AP214 and `ifc` IFC4 both do — IFC is
//! literally "STEP syntax + a different EXPRESS schema"). https://www.iso.org/standard/63141.html
//! Escape/matrix logic below was pre-verified via a standalone scratch binary per this
//! session's own convention (ticket `26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION`).

use std::fmt;
use std::fmt::Write as _;

//#region 🔖️Value
/// 🔢️ Exact logical STEP real: decimal coefficient/scale plus an optional base-10 exponent.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Part21Decimal {
    pub negative: bool,
    pub coefficient: String,
    pub scale: u32,
    pub exponent: Option<i32>,
}

impl Part21Decimal {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn parse(text: &str) -> Result<Self, String> {
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_f64(value: f64) -> Self {
        let text = format!("{value}");
        let normalized = if text.contains('.') || text.contains('e') || text.contains('E') { text } else { format!("{text}.") };
        Self::parse(&normalized).expect("finite f64 has valid STEP decimal form")
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_f64(&self) -> Option<f64> {
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum Part21Value {
    Ref(u64),
    Str(String),
    Enum(String),
    Int(i64),
    Real(Part21Decimal),
    List(Vec<Part21Value>),
    /// 🏷️ A "defined type" wrapper appearing as an argument, e.g. `IFCLENGTHMEASURE(3000.)`.
    Typed { name: String, items: Vec<Part21Value> },
    Unset,
    Derived,
}

impl Part21Value {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_ref_id(&self) -> Option<u64> {
        if let Part21Value::Ref(id) = self {
            Some(*id)
        } else {
            None
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_str(&self) -> Option<&str> {
        if let Part21Value::Str(s) = self {
            Some(s.as_str())
        } else {
            None
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_enum(&self) -> Option<&str> {
        if let Part21Value::Enum(s) = self {
            Some(s.as_str())
        } else {
            None
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_real(&self) -> Option<f64> {
        match self {
            Part21Value::Real(r) => r.to_f64(),
            Part21Value::Int(i) => Some(*i as f64),
            _ => None,
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_list(&self) -> Option<&[Part21Value]> {
        if let Part21Value::List(items) = self {
            Some(items.as_slice())
        } else {
            None
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_typed(&self) -> Option<(&str, &[Part21Value])> {
        if let Part21Value::Typed { name, items } = self {
            Some((name.as_str(), items.as_slice()))
        } else {
            None
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_unset(&self) -> bool {
        matches!(self, Part21Value::Unset)
    }
}
//#endregion 🔖️Value

//#region 🔖️Instance
/// 🧩️ One `#N = TYPE(args...)` line, or `#N = (TYPE1(...) TYPE2(...))` for a complex instance —
/// every `(type_name, args)` pair is kept, nothing about a multi-type instance is dropped.
#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, value_derive::FromValue)]
pub struct Part21Instance {
    pub id: u64,
    pub entities: Vec<(String, Vec<Part21Value>)>,
}

impl Part21Instance {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn entity(&self, type_name: &str) -> Option<&Vec<Part21Value>> {
        self.entities.iter().find(|(name, _)| name.eq_ignore_ascii_case(type_name)).map(|(_, args)| args)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn primary(&self) -> Option<(&str, &Vec<Part21Value>)> {
        self.entities.first().map(|(name, args)| (name.as_str(), args))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_type(&self, type_name: &str) -> bool {
        self.entities.iter().any(|(name, _)| name.eq_ignore_ascii_case(type_name))
    }
}
//#endregion 🔖️Instance

//#region 🔖️Header
/// 📇️ The three standard `HEADER;` records (`FILE_DESCRIPTION`/`FILE_NAME`/`FILE_SCHEMA`),
/// each a parenthesized tuple of typed values — kept verbatim, not schema-interpreted.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct Part21Header {
    pub file_description: Vec<Part21Value>,
    pub file_name: Vec<Part21Value>,
    pub file_schema: Vec<Part21Value>,
}

/// 📜️ ISO 10303-21 §8.2.2's population constraint for one attribute of a mandatory `HEADER`
/// record: either a plain `STRING` or a `LIST[1:?] OF STRING`, which the standard forbids from
/// ever being empty. This is the schema of the exchange structure itself — fixed by the standard,
/// identical for every EXPRESS schema carried in it — so it lives with the syntax rather than with
/// any one artifact's typed view of it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum HeaderAttribute {
    Text,
    NonEmptyTextList,
}

/// 📜️ `FILE_DESCRIPTION(description, implementation_level)` — ISO 10303-21 §8.2.2.
const FILE_DESCRIPTION_ATTRIBUTES: &[HeaderAttribute] = &[HeaderAttribute::NonEmptyTextList, HeaderAttribute::Text];
/// 📜️ `FILE_NAME(name, time_stamp, author, organization, preprocessor_version,
/// originating_system, authorization)` — ISO 10303-21 §8.2.3. `author` and `organization` are
/// `LIST[1:?]`, exactly like `FILE_DESCRIPTION.description`.
const FILE_NAME_ATTRIBUTES: &[HeaderAttribute] = &[
    HeaderAttribute::Text,
    HeaderAttribute::Text,
    HeaderAttribute::NonEmptyTextList,
    HeaderAttribute::NonEmptyTextList,
    HeaderAttribute::Text,
    HeaderAttribute::Text,
    HeaderAttribute::Text,
];
/// 📜️ `FILE_SCHEMA(schema_identifiers)` — ISO 10303-21 §8.2.4, also `LIST[1:?]`.
const FILE_SCHEMA_ATTRIBUTES: &[HeaderAttribute] = &[HeaderAttribute::NonEmptyTextList];

impl HeaderAttribute {
    /// 🈳️ The conformant spelling of "nothing to say here" for this attribute — `''` for a
    /// `STRING`, `('')` for a `LIST[1:?] OF STRING`. `()` is NOT that spelling: the standard's
    /// lower bound of one is a population constraint, and every conformant producer writes the
    /// one-empty-string list instead (the ruststep reference reader refuses `()` outright).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn unpopulated(self) -> Part21Value {
        match self {
            HeaderAttribute::Text => Part21Value::Str(String::new()),
            HeaderAttribute::NonEmptyTextList => Part21Value::List(vec![Part21Value::Str(String::new())]),
        }
    }
}

impl Part21Header {
    /// 🌱 ISO 10303-21 §8.2's conformant minimum `HEADER` — all three mandatory records present,
    /// each with its full attribute list and every `LIST[1:?]` populated. This is what `default()`
    /// means for a header: the previous all-empty derive produced `FILE_DESCRIPTION();`, which is
    /// neither the right arity nor a readable exchange structure.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn iso_10303_21_minimum() -> Self {
        Self {
            file_description: FILE_DESCRIPTION_ATTRIBUTES.iter().map(|attribute| attribute.unpopulated()).collect(),
            file_name: FILE_NAME_ATTRIBUTES.iter().map(|attribute| attribute.unpopulated()).collect(),
            file_schema: FILE_SCHEMA_ATTRIBUTES.iter().map(|attribute| attribute.unpopulated()).collect(),
        }
    }
}

impl Default for Part21Header {
    fn default() -> Self {
        Self::iso_10303_21_minimum()
    }
}
//#endregion 🔖️Header

//#region 🔖️Document
/// 📦️ The full, lossless generic Part-21 graph: header + every `DATA;` instance.
#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, value_derive::FromValue)]
pub struct Part21Document {
    pub header: Part21Header,
    pub instances: Vec<Part21Instance>,
}

impl Part21Document {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn instance(&self, id: u64) -> Option<&Part21Instance> {
        self.instances.iter().find(|i| i.id == id)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn resolve(&self, value: &Part21Value) -> Option<&Part21Instance> {
        value.as_ref_id().and_then(|id| self.instance(id))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn by_type<'a>(&'a self, type_name: &'a str) -> impl Iterator<Item = &'a Part21Instance> + 'a {
        self.instances.iter().filter(move |i| i.is_type(type_name))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn next_id(&self) -> u64 {
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new() -> Self {
        Self { instances: Vec::new(), next_id: 1 }
    }
    /// ➕️ Allocates the next `#id` and appends a simple `TYPE(args)` instance.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn alloc(&mut self, type_name: &str, args: Vec<Part21Value>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.instances.push(Part21Instance { id, entities: vec![(type_name.to_string(), args)] });
        id
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn build(self, header: Part21Header) -> Part21Document {
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn new(text: &str) -> Self {
        Self { chars: text.chars().collect(), pos: 0 }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn peek_at(&self, offset: usize) -> Option<char> {
        self.chars.get(self.pos + offset).copied()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn bump(&mut self) -> Option<char> {
        let c = self.peek();
        if c.is_some() {
            self.pos += 1;
        }
        c
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn skip_ws_and_comments(&mut self) {
        loop {
            match self.peek() {
                Some(c) if c.is_whitespace() => self.pos += 1,
                Some('/') if self.peek_at(1) == Some('*') => {
                    self.pos += 2;
                    while let Some(c) = self.peek() {
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn expect_literal(&mut self, lit: &'static str) -> Result<(), Part21Error> {
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn try_literal(&mut self, lit: &str) -> bool {
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn read_keyword(&mut self) -> Result<String, Part21Error> {
        self.skip_ws_and_comments();
        let start = self.pos;
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_' {
                s.push(c);
                self.pos += 1;
            } else {
                break;
            }
        }
        if s.is_empty() {
            return Err(Part21Error::UnexpectedChar { at: start, found: self.peek().unwrap_or('\0'), expected: "keyword" });
        }
        Ok(s)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn read_string(&mut self) -> Result<String, Part21Error> {
        self.skip_ws_and_comments();
        if self.peek() != Some('\'') {
            return Err(Part21Error::UnexpectedChar { at: self.pos, found: self.peek().unwrap_or('\0'), expected: "'" });
        }
        self.pos += 1;
        let mut out = String::new();
        // 📖 §6.4.2's `alphabet` directive holds until the end of THIS string literal, so the
        // selected ISO 8859 part is per-literal state rather than per-document.
        let mut alphabet = 'A';
        loop {
            match self.bump() {
                None => return Err(Part21Error::UnexpectedEof { at: self.pos, expected: "closing '" }),
                Some('\'') => {
                    if self.peek() == Some('\'') {
                        self.pos += 1;
                        out.push('\'');
                    } else {
                        break;
                    }
                }
                Some('\\') => self.read_escape(&mut out, &mut alphabet)?,
                Some(c) => out.push(c),
            }
        }
        Ok(out)
    }

    /// 🔤️ ISO 10303-21 §6.4.2's COMPLETE `control_directive` set, plus the STRING production's own
    /// doubled reverse solidus — called with the opening `\` already consumed.
    ///
    /// * `\\` — one literal REVERSE SOLIDUS. The string production escapes it by doubling exactly
    ///   as it escapes the apostrophe by doubling, and real exporters emit it: IfcOpenShell writes
    ///   `'\\'` for a one-character backslash name at byte 138718 of the committed
    ///   `🏗️nakagin-capsule-tower.ifc`. Rejecting it failed EVERY `🏗️mutate-ifc-4` subject scenario
    ///   (22 of 22 executable rows) until this wave, while `ruststep` — the registered independent
    ///   reader for that same case — read the file without complaint.
    /// * `\X\HH` — `arbitrary`: EXACTLY two hex digits and no terminator (`arbitrary = "\X\"
    ///   hex_one`). This lexer used to demand a trailing `\`, which would mis-parse a conformant
    ///   `\X\41\S\A` by eating the next directive's own opener.
    /// * `\X2\HHHH…\X0\` / `\X4\HHHHHHHH…\X0\` — `extended2`/`extended4` UCS-2 / UCS-4 runs.
    /// * `\S\c` — `page`: the character at `code(c) + 128` in the selected alphabet.
    /// * `\PA\`…`\PI\` — `alphabet`: which ISO 8859 part `\S\` shifts into. Only `A` (ISO 8859-1,
    ///   the one part where `code + 128` IS the Unicode codepoint) is decoded; `B`..`I` need
    ///   per-part mapping tables this codec does not carry, and a typed error naming the page is
    ///   the honest answer rather than a silently wrong character.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn read_escape(&mut self, out: &mut String, alphabet: &mut char) -> Result<(), Part21Error> {
        let start = self.pos - 1;
        match self.bump() {
            Some('\\') => {
                out.push('\\');
                Ok(())
            }
            Some('P') => {
                let selected = match self.bump() {
                    Some(c @ 'A'..='I') => c,
                    other => return Err(Part21Error::UnsupportedEscape { at: start, detail: format!("bad \\P alphabet {other:?}") }),
                };
                if self.bump() != Some('\\') {
                    return Err(Part21Error::UnsupportedEscape { at: start, detail: "expected \\ after \\P".into() });
                }
                *alphabet = selected;
                Ok(())
            }
            Some('S') => {
                if self.bump() != Some('\\') {
                    return Err(Part21Error::UnsupportedEscape { at: start, detail: "expected \\ after \\S".into() });
                }
                if *alphabet != 'A' {
                    return Err(Part21Error::UnsupportedEscape { at: start, detail: format!("\\S\\ on ISO 8859 page {alphabet} needs a mapping table this codec does not carry") });
                }
                match self.bump() {
                    Some(c) if (c as u32) < 0x80 => {
                        out.push(char::from_u32(c as u32 + 128).ok_or_else(|| Part21Error::UnsupportedEscape { at: start, detail: "bad \\S\\ character".into() })?);
                        Ok(())
                    }
                    other => Err(Part21Error::UnsupportedEscape { at: start, detail: format!("bad \\S\\ character {other:?}") }),
                }
            }
            Some('X') => match self.peek() {
                Some(width) if width == '2' || width == '4' => {
                    let group = if width == '2' { 4 } else { 8 };
                    self.pos += 1;
                    if self.bump() != Some('\\') {
                        return Err(Part21Error::UnsupportedEscape { at: start, detail: format!("expected \\ after \\X{width}") });
                    }
                    loop {
                        let mut hex = String::new();
                        for _ in 0..group {
                            match self.bump() {
                                Some(c) if c.is_ascii_hexdigit() => hex.push(c),
                                _ => return Err(Part21Error::UnsupportedEscape { at: start, detail: format!("bad \\X{width}\\ hex group") }),
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
                        match self.bump() {
                            Some(c) if c.is_ascii_hexdigit() => hex.push(c),
                            _ => return Err(Part21Error::UnsupportedEscape { at: start, detail: "bad \\X\\ hex".into() }),
                        }
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn read_number(&mut self) -> Result<Part21Value, Part21Error> {
        self.skip_ws_and_comments();
        let start = self.pos;
        let mut s = String::new();
        if matches!(self.peek(), Some('-') | Some('+')) {
            s.push(self.bump().unwrap());
        }
        while let Some(c) = self.peek() {
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
            while let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    s.push(c);
                    self.pos += 1;
                } else {
                    break;
                }
            }
        }
        if matches!(self.peek(), Some('E') | Some('e')) {
            is_real = true;
            s.push('E');
            self.pos += 1;
            if matches!(self.peek(), Some('+') | Some('-')) {
                s.push(self.bump().unwrap());
            }
            while let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    s.push(c);
                    self.pos += 1;
                } else {
                    break;
                }
            }
        }
        if is_real {
            Part21Decimal::parse(&s).map(Part21Value::Real).map_err(|_| Part21Error::InvalidNumber { at: start, text: s })
        } else {
            s.parse::<i64>().map(Part21Value::Int).map_err(|_| Part21Error::InvalidNumber { at: start, text: s })
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn read_enum(&mut self) -> Result<String, Part21Error> {
        self.skip_ws_and_comments();
        if self.peek() != Some('.') {
            return Err(Part21Error::UnexpectedChar { at: self.pos, found: self.peek().unwrap_or('\0'), expected: "." });
        }
        self.pos += 1;
        let mut s = String::new();
        while let Some(c) = self.peek() {
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn read_value(&mut self) -> Result<Part21Value, Part21Error> {
        self.skip_ws_and_comments();
        match self.peek() {
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
                while let Some(c) = self.peek() {
                    if c.is_ascii_digit() {
                        s.push(c);
                        self.pos += 1;
                    } else {
                        break;
                    }
                }
                s.parse::<u64>().map(Part21Value::Ref).map_err(|_| Part21Error::InvalidNumber { at: start, text: s })
            }
            Some('\'') => self.read_string().map(Part21Value::Str),
            Some('.') => self.read_enum().map(Part21Value::Enum),
            Some('(') => {
                self.pos += 1;
                let items = self.read_value_list()?;
                self.expect_literal(")")?;
                Ok(Part21Value::List(items))
            }
            Some(c) if c.is_ascii_digit() || c == '-' || c == '+' => self.read_number(),
            Some(c) if c.is_ascii_uppercase() => {
                let kw = self.read_keyword()?;
                self.skip_ws_and_comments();
                if self.peek() == Some('(') {
                    self.pos += 1;
                    let items = self.read_value_list()?;
                    self.expect_literal(")")?;
                    Ok(Part21Value::Typed { name: kw, items })
                } else {
                    Err(Part21Error::UnexpectedChar { at: self.pos, found: self.peek().unwrap_or('\0'), expected: "( after typed value keyword" })
                }
            }
            Some(c) => Err(Part21Error::UnexpectedChar { at: self.pos, found: c, expected: "value" }),
            None => Err(Part21Error::UnexpectedEof { at: self.pos, expected: "value" }),
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn read_value_list(&mut self) -> Result<Vec<Part21Value>, Part21Error> {
        self.skip_ws_and_comments();
        let mut out = Vec::new();
        if self.peek() == Some(')') {
            return Ok(out);
        }
        loop {
            out.push(self.read_value()?);
            self.skip_ws_and_comments();
            if self.peek() == Some(',') {
                self.pos += 1;
                continue;
            }
            break;
        }
        Ok(out)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn read_record(&mut self) -> Result<(String, Vec<Part21Value>), Part21Error> {
        let name = self.read_keyword()?;
        self.expect_literal("(")?;
        let args = self.read_value_list()?;
        self.expect_literal(")")?;
        Ok((name, args))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn read_instance(&mut self) -> Result<Part21Instance, Part21Error> {
        self.expect_literal("#")?;
        let start = self.pos;
        let mut id_s = String::new();
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                id_s.push(c);
                self.pos += 1;
            } else {
                break;
            }
        }
        let id = id_s.parse::<u64>().map_err(|_| Part21Error::InvalidNumber { at: start, text: id_s })?;
        self.expect_literal("=")?;
        self.skip_ws_and_comments();
        let mut entities = Vec::new();
        if self.peek() == Some('(') {
            self.pos += 1;
            loop {
                self.skip_ws_and_comments();
                if self.peek() == Some(')') {
                    break;
                }
                entities.push(self.read_record()?);
                self.skip_ws_and_comments();
            }
            self.expect_literal(")")?;
        } else {
            entities.push(self.read_record()?);
        }
        self.expect_literal(";")?;
        Ok(Part21Instance { id, entities })
    }
}
//#endregion 🔖️Lexer

//#region 🔖️Parse
/// 📥️ Parses a full ISO 10303-21 physical file into the generic graph. Real tokenizer,
/// not a scraper — every header record and every data instance/argument round-trips.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_part21(text: &str) -> Result<Part21Document, Part21Error> {
    let mut lex = Lexer::new(text);
    lex.expect_literal("ISO-10303-21;")?;
    lex.expect_literal("HEADER;")?;
    let mut header = Part21Header::default();
    loop {
        lex.skip_ws_and_comments();
        if lex.try_literal("ENDSEC;") {
            break;
        }
        let (name, args) = lex.read_record()?;
        lex.expect_literal(";")?;
        match name.as_str() {
            "FILE_DESCRIPTION" => header.file_description = args,
            "FILE_NAME" => header.file_name = args,
            "FILE_SCHEMA" => header.file_schema = args,
            _ => {}
        }
    }
    lex.expect_literal("DATA;")?;
    let mut instances = Vec::new();
    loop {
        lex.skip_ws_and_comments();
        if lex.try_literal("ENDSEC;") {
            break;
        }
        instances.push(lex.read_instance()?);
    }
    let _ = lex.try_literal("END-ISO-10303-21;");
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
    fn write_preamble(&self, out: &mut String, line_ending: &str);
}

/// 🈳️ Zero-sized stand-in preamble type (O1 — R11(a): `write_part21_with`'s preamble is a
/// borrowed-reference parameter, trivially generic; `write_part21` still needs SOME concrete type to
/// instantiate that generic with when it passes `None`, and picking a real implementor — e.g. ifc's
/// `Ifc2x3EdmPreamble` — would make this generic `step` module depend downward on a specific format
/// built on top of it). Never constructed; `write_preamble` is unreachable by construction.
struct NoPreamble;

impl Part21Preamble for NoPreamble {
    fn write_preamble(&self, _out: &mut String, _line_ending: &str) {}
}

/// 📤️ Regenerates valid Part-21 text from the generic graph — round-trip losslessness is
/// the writer's job; it never re-derives STEP/IFC semantics.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_part21(doc: &Part21Document) -> String {
    write_part21_with::<NoPreamble>(doc, Part21WriteOptions::default(), None)
}

/// 📤️ Regenerates Part-21 with a standard-selected deterministic layout and typed preamble. Generic
/// over the preamble implementor (O1 — R11(a): borrowed-reference parameter, trivially generic).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_part21_with<P: Part21Preamble>(doc: &Part21Document, options: Part21WriteOptions, preamble: Option<&P>) -> String {
    let eol = options.line_ending;
    let mut out = format!("ISO-10303-21;{eol}HEADER;{eol}");
    if options.blank_after_header {
        out.push_str(eol);
    }
    if let Some(preamble) = preamble {
        preamble.write_preamble(&mut out, eol);
    }
    write_header_record(&mut out, "FILE_DESCRIPTION", FILE_DESCRIPTION_ATTRIBUTES, &doc.header.file_description, eol);
    write_header_record(&mut out, "FILE_NAME", FILE_NAME_ATTRIBUTES, &doc.header.file_name, eol);
    write_header_record(&mut out, "FILE_SCHEMA", FILE_SCHEMA_ATTRIBUTES, &doc.header.file_schema, eol);
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_record(out: &mut String, name: &str, args: &[Part21Value], line_ending: &str) {
    out.push_str(name);
    out.push('(');
    write_value_list(out, args);
    out.push_str(");");
    out.push_str(line_ending);
}

/// 📜️ One mandatory `HEADER` record, written against ISO 10303-21 §8.2's fixed attribute list so
/// the emitted exchange structure is always one a conformant reader accepts.
///
/// Two spec obligations the generic [`write_record`] cannot know about, both of which this
/// codebase was breaching until wave 15's differential run caught it (`ruststep` refusing
/// `FILE_DESCRIPTION((),'')` with "expected ')', found ("):
///
/// * every attribute the standard declares is present, so a header carrying fewer values than its
///   record's arity is padded with that position's unpopulated spelling rather than emitted short;
/// * a `LIST[1:?] OF STRING` is never emitted empty — `()` violates the lower bound, and the
///   conformant spelling of an empty description/author/organization/schema list is `('')`.
///
/// Values the caller DID populate are written verbatim; this only ever fills in what is missing,
/// so nothing a real document carried is normalized away.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_header_record(out: &mut String, name: &str, attributes: &[HeaderAttribute], args: &[Part21Value], line_ending: &str) {
    let conformant: Vec<Part21Value> = attributes
        .iter()
        .enumerate()
        .map(|(position, attribute)| match args.get(position) {
            Some(Part21Value::List(items)) if items.is_empty() && *attribute == HeaderAttribute::NonEmptyTextList => attribute.unpopulated(),
            Some(value) => value.clone(),
            None => attribute.unpopulated(),
        })
        .chain(args.iter().skip(attributes.len()).cloned())
        .collect();
    write_record(out, name, &conformant, line_ending);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_instance(out: &mut String, inst: &Part21Instance, line_ending: &str, space_after_equals: bool) {
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_value_list(out: &mut String, items: &[Part21Value]) {
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        write_value(out, item);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_value(out: &mut String, v: &Part21Value) {
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
        Part21Value::Typed { name, items } => {
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn escape_part21_string(s: &str) -> String {
    let mut out = String::new();
    let mut run: Vec<char> = Vec::new();
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn flush(run: &mut Vec<char>, out: &mut String) {
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

    /// 🧪️ ISO 10303-21 §6.4.2's remaining control directives, each read back from a literal
    /// spelled the way the standard spells it. The `\\` row is the one the real committed
    /// IfcOpenShell export carries (`'\\'` at byte 138718 of `🏗️nakagin-capsule-tower.ifc`) and the
    /// one this lexer used to reject outright, failing all 22 executable `🏗️mutate-ifc-4` subject
    /// scenarios while `ruststep` read the same file without complaint. `\X\` carries NO
    /// terminator per the grammar's own `arbitrary = "\X\" hex_one`, which is why the `\X\41\S\A`
    /// row matters: demanding one would swallow the next directive's opener.
    #[test]
    fn every_iso_10303_21_string_directive_is_read() {
        let label = |literal: &str| {
            let text = format!("ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('X'));\nENDSEC;\nDATA;\n#1=LABEL('{literal}');\nENDSEC;\nEND-ISO-10303-21;\n");
            parse_part21(&text).unwrap_or_else(|e| panic!("parse {literal:?}: {e}")).instance(1).unwrap().entity("LABEL").unwrap()[0].as_str().unwrap().to_string()
        };
        assert_eq!(label(r"\\"), "\\", "the doubled reverse solidus is ONE literal backslash");
        assert_eq!(label(r"a\\b"), "a\\b");
        assert_eq!(label(r"\X\41"), "A", "\\X\\ takes exactly two hex digits and no terminator");
        assert_eq!(label(r"\X\41\S\A"), "A\u{00C1}", "\\X\\ must not swallow the next directive's opener");
        assert_eq!(label(r"\S\A"), "\u{00C1}", "\\S\\ shifts the character by 128 on the default alphabet");
        assert_eq!(label(r"\PA\\S\A"), "\u{00C1}", "\\PA\\ selects ISO 8859-1, which is the default");
        assert_eq!(label(r"\X4\0001F600\X0\"), "\u{1F600}", "\\X4\\ reads UCS-4 groups");
        assert_eq!(label(r"\X2\4E2D6587\X0\"), "中文");
    }

    /// 🧪️ A page this codec cannot map is a TYPED ERROR naming the page, never a wrong character:
    /// ISO 8859-2..-9 do not place `code + 128` at the Unicode codepoint the way ISO 8859-1 does,
    /// and guessing would corrupt a real name silently.
    #[test]
    fn an_unmappable_iso_8859_page_is_refused_rather_than_guessed() {
        let text = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('X'));\nENDSEC;\nDATA;\n#1=LABEL('\\PB\\\\S\\A');\nENDSEC;\nEND-ISO-10303-21;\n";
        let error = parse_part21(text).expect_err("page B must not be decoded");
        assert!(error.to_string().contains("ISO 8859 page B"), "the error must name the page it refused: {error}");
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
