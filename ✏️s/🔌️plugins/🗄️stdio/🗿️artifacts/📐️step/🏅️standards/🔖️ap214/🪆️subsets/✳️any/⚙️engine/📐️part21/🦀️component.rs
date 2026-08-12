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
/// 🔤️ A single typed value in Part-21 argument-list syntax.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Part21Value {
    Ref(u64),
    Str(String),
    Enum(String),
    Int(i64),
    Real(f64),
    List(Vec<Part21Value>),
    /// 🏷️ A "defined type" wrapper appearing as an argument, e.g. `IFCLENGTHMEASURE(3000.)`.
    Typed(String, Vec<Part21Value>),
    Unset,
    Derived,
}

impl Part21Value {
    pub fn as_ref_id(&self) -> Option<u64> {
        if let Part21Value::Ref(id) = self { Some(*id) } else { None }
    }
    pub fn as_str(&self) -> Option<&str> {
        if let Part21Value::Str(s) = self { Some(s.as_str()) } else { None }
    }
    pub fn as_enum(&self) -> Option<&str> {
        if let Part21Value::Enum(s) = self { Some(s.as_str()) } else { None }
    }
    pub fn as_real(&self) -> Option<f64> {
        match self {
            Part21Value::Real(r) => Some(*r),
            Part21Value::Int(i) => Some(*i as f64),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[Part21Value]> {
        if let Part21Value::List(items) = self { Some(items.as_slice()) } else { None }
    }
    pub fn as_typed(&self) -> Option<(&str, &[Part21Value])> {
        if let Part21Value::Typed(name, items) = self { Some((name.as_str(), items.as_slice())) } else { None }
    }
    pub fn is_unset(&self) -> bool {
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
    pub fn entity(&self, type_name: &str) -> Option<&Vec<Part21Value>> {
        self.entities.iter().find(|(name, _)| name.eq_ignore_ascii_case(type_name)).map(|(_, args)| args)
    }
    pub fn primary(&self) -> Option<(&str, &Vec<Part21Value>)> {
        self.entities.first().map(|(name, args)| (name.as_str(), args))
    }
    pub fn is_type(&self, type_name: &str) -> bool {
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
    pub fn instance(&self, id: u64) -> Option<&Part21Instance> {
        self.instances.iter().find(|i| i.id == id)
    }
    pub fn resolve(&self, value: &Part21Value) -> Option<&Part21Instance> {
        value.as_ref_id().and_then(|id| self.instance(id))
    }
    pub fn by_type<'a>(&'a self, type_name: &'a str) -> impl Iterator<Item = &'a Part21Instance> + 'a {
        self.instances.iter().filter(move |i| i.is_type(type_name))
    }
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
    pub fn new() -> Self {
        Self { instances: Vec::new(), next_id: 1 }
    }
    /// ➕️ Allocates the next `#id` and appends a simple `TYPE(args)` instance.
    pub fn alloc(&mut self, type_name: &str, args: Vec<Part21Value>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.instances.push(Part21Instance { id, entities: vec![(type_name.to_string(), args)] });
        id
    }
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
    fn new(text: &str) -> Self {
        Self { chars: text.chars().collect(), pos: 0 }
    }
    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }
    fn peek_at(&self, offset: usize) -> Option<char> {
        self.chars.get(self.pos + offset).copied()
    }
    fn bump(&mut self) -> Option<char> {
        let c = self.peek();
        if c.is_some() {
            self.pos += 1;
        }
        c
    }

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

    fn read_string(&mut self) -> Result<String, Part21Error> {
        self.skip_ws_and_comments();
        if self.peek() != Some('\'') {
            return Err(Part21Error::UnexpectedChar { at: self.pos, found: self.peek().unwrap_or('\0'), expected: "'" });
        }
        self.pos += 1;
        let mut out = String::new();
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
                Some('\\') => self.read_escape(&mut out)?,
                Some(c) => out.push(c),
            }
        }
        Ok(out)
    }

    /// 🔤️ `\X\HH\` single byte, `\X2\HHHH(HHHH)*\X0\` UCS-2 run — the two Part-21 escape
    /// forms this codebase's fixtures/writer actually emit; anything else is a typed error.
    fn read_escape(&mut self, out: &mut String) -> Result<(), Part21Error> {
        let start = self.pos - 1;
        match self.bump() {
            Some('X') => match self.peek() {
                Some('2') => {
                    self.pos += 1;
                    if self.bump() != Some('\\') {
                        return Err(Part21Error::UnsupportedEscape { at: start, detail: "expected \\ after \\X2".into() });
                    }
                    loop {
                        let mut hex = String::new();
                        for _ in 0..4 {
                            match self.bump() {
                                Some(c) if c.is_ascii_hexdigit() => hex.push(c),
                                _ => return Err(Part21Error::UnsupportedEscape { at: start, detail: "bad \\X2\\ hex group".into() }),
                            }
                        }
                        let code = u32::from_str_radix(&hex, 16)
                            .map_err(|_| Part21Error::UnsupportedEscape { at: start, detail: "bad hex".into() })?;
                        let ch = char::from_u32(code)
                            .ok_or_else(|| Part21Error::UnsupportedEscape { at: start, detail: format!("bad codepoint {code}") })?;
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
                    if self.bump() != Some('\\') {
                        return Err(Part21Error::UnsupportedEscape { at: start, detail: "expected trailing \\ after \\X..".into() });
                    }
                    let code = u32::from_str_radix(&hex, 16)
                        .map_err(|_| Part21Error::UnsupportedEscape { at: start, detail: "bad hex".into() })?;
                    let ch = char::from_u32(code)
                        .ok_or_else(|| Part21Error::UnsupportedEscape { at: start, detail: format!("bad byte {code}") })?;
                    out.push(ch);
                    Ok(())
                }
            },
            other => Err(Part21Error::UnsupportedEscape { at: start, detail: format!("unsupported escape start {other:?}") }),
        }
    }

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
            s.parse::<f64>().map(Part21Value::Real).map_err(|_| Part21Error::InvalidNumber { at: start, text: s })
        } else {
            s.parse::<i64>().map(Part21Value::Int).map_err(|_| Part21Error::InvalidNumber { at: start, text: s })
        }
    }

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
                    Ok(Part21Value::Typed(kw, items))
                } else {
                    Err(Part21Error::UnexpectedChar { at: self.pos, found: self.peek().unwrap_or('\0'), expected: "( after typed value keyword" })
                }
            }
            Some(c) => Err(Part21Error::UnexpectedChar { at: self.pos, found: c, expected: "value" }),
            None => Err(Part21Error::UnexpectedEof { at: self.pos, expected: "value" }),
        }
    }

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

    fn read_record(&mut self) -> Result<(String, Vec<Part21Value>), Part21Error> {
        let name = self.read_keyword()?;
        self.expect_literal("(")?;
        let args = self.read_value_list()?;
        self.expect_literal(")")?;
        Ok((name, args))
    }

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
/// 📤️ Regenerates valid Part-21 text from the generic graph — round-trip losslessness is
/// the writer's job; it never re-derives STEP/IFC semantics.
pub fn write_part21(doc: &Part21Document) -> String {
    let mut out = String::from("ISO-10303-21;\nHEADER;\n");
    write_record(&mut out, "FILE_DESCRIPTION", &doc.header.file_description);
    write_record(&mut out, "FILE_NAME", &doc.header.file_name);
    write_record(&mut out, "FILE_SCHEMA", &doc.header.file_schema);
    out.push_str("ENDSEC;\nDATA;\n");
    for inst in &doc.instances {
        write_instance(&mut out, inst);
    }
    out.push_str("ENDSEC;\nEND-ISO-10303-21;\n");
    out
}

fn write_record(out: &mut String, name: &str, args: &[Part21Value]) {
    out.push_str(name);
    out.push('(');
    write_value_list(out, args);
    out.push_str(");\n");
}

fn write_instance(out: &mut String, inst: &Part21Instance) {
    let _ = write!(out, "#{}=", inst.id);
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
    out.push_str(";\n");
}

fn write_value_list(out: &mut String, items: &[Part21Value]) {
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        write_value(out, item);
    }
}

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
        Part21Value::Real(r) => out.push_str(&format_real(*r)),
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

/// 🔢️ STEP reals always carry a decimal point (`digit+ '.' digit*`); Rust's own f64 `Display`
/// never emits one for whole numbers and never uses exponent notation, so only that case needs help.
fn format_real(r: f64) -> String {
    let s = format!("{r}");
    if s.contains('.') { s } else { format!("{s}.") }
}

/// 🔡️ Inverse of the lexer's `read_escape`: `'` doubles, backslash is escaped (to stay
/// unambiguous with `\X..` on reparse), any other non-printable-ASCII goes through `\X2\..\X0\`.
fn escape_part21_string(s: &str) -> String {
    let mut out = String::new();
    let mut run: Vec<char> = Vec::new();
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

    #[test]
    fn round_trip_parse_serialize_reparse() {
        let doc = parse_part21(FIXTURE).expect("parse fixture");
        assert!(!doc.instances.is_empty());
        assert_eq!(doc.header.file_schema, vec![Part21Value::List(vec![Part21Value::Str("AUTOMOTIVE_DESIGN".into())])]);
        let text = write_part21(&doc);
        let reparsed = parse_part21(&text).expect("reparse generated text");
        assert_eq!(doc, reparsed, "round trip must be lossless at the graph level");
    }

    #[test]
    fn instance_count_and_types_preserved() {
        let doc = parse_part21(FIXTURE).expect("parse");
        assert_eq!(doc.instances.len(), 26);
        assert_eq!(doc.by_type("CARTESIAN_POINT").count(), 3);
        assert_eq!(doc.by_type("ADVANCED_FACE").count(), 1);
        let derived_placement = doc.instance(40).expect("axis2placement");
        let args = derived_placement.entity("AXIS2_PLACEMENT_3D").expect("typed");
        assert!(matches!(args[3], Part21Value::Unset));
    }

    #[test]
    fn oriented_edge_derived_attrs_are_star() {
        let doc = parse_part21(FIXTURE).expect("parse");
        let oe = doc.instance(11).expect("oriented edge");
        let args = oe.entity("ORIENTED_EDGE").expect("typed");
        assert_eq!(args[1], Part21Value::Derived);
        assert_eq!(args[2], Part21Value::Derived);
    }

    #[test]
    fn complex_instance_keeps_every_type() {
        let text = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('IFC4'));\nENDSEC;\nDATA;\n#1=(IFCQUANTITYAREA($,$,$,10.5,$)IFCPHYSICALSIMPLEQUANTITY($,$,$,$));\nENDSEC;\nEND-ISO-10303-21;\n";
        let doc = parse_part21(text).expect("parse complex instance");
        let inst = doc.instance(1).expect("instance 1");
        assert_eq!(inst.entities.len(), 2);
        assert_eq!(inst.entities[0].0, "IFCQUANTITYAREA");
        assert_eq!(inst.entities[1].0, "IFCPHYSICALSIMPLEQUANTITY");
        let round = write_part21(&doc);
        assert_eq!(parse_part21(&round).expect("reparse"), doc);
    }

    #[test]
    fn typed_value_wrapper_round_trips() {
        let text = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('IFC4'));\nENDSEC;\nDATA;\n#1=IFCPROPERTYSINGLEVALUE('Height',$,IFCLENGTHMEASURE(3000.),$);\nENDSEC;\nEND-ISO-10303-21;\n";
        let doc = parse_part21(text).expect("parse");
        let args = doc.instance(1).unwrap().entity("IFCPROPERTYSINGLEVALUE").unwrap();
        let (name, inner) = args[2].as_typed().expect("typed value");
        assert_eq!(name, "IFCLENGTHMEASURE");
        assert_eq!(inner[0].as_real(), Some(3000.0));
        assert_eq!(parse_part21(&write_part21(&doc)).unwrap(), doc);
    }

    #[test]
    fn string_escapes_round_trip() {
        for raw in ["it's a test", "unicode: \u{20AC} \u{4E2D}\u{6587}", "back\\slash", "", "plain"] {
            let text = format!(
                "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('X'));\nENDSEC;\nDATA;\n#1=LABEL('{}');\nENDSEC;\nEND-ISO-10303-21;\n",
                escape_part21_string(raw)
            );
            let doc = parse_part21(&text).unwrap_or_else(|e| panic!("parse {raw:?}: {e}"));
            let got = doc.instance(1).unwrap().entity("LABEL").unwrap()[0].as_str().unwrap();
            assert_eq!(got, raw, "escape round trip for {raw:?}");
        }
    }

    #[test]
    fn doubled_quote_escape() {
        let text = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('X'));\nENDSEC;\nDATA;\n#1=LABEL('it''s here');\nENDSEC;\nEND-ISO-10303-21;\n";
        let doc = parse_part21(text).expect("parse");
        assert_eq!(doc.instance(1).unwrap().entity("LABEL").unwrap()[0].as_str(), Some("it's here"));
    }

    #[test]
    fn unicode_x2_escape() {
        let text = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('X'));\nENDSEC;\nDATA;\n#1=LABEL('\\X2\\4E2D6587\\X0\\');\nENDSEC;\nEND-ISO-10303-21;\n";
        let doc = parse_part21(text).expect("parse");
        assert_eq!(doc.instance(1).unwrap().entity("LABEL").unwrap()[0].as_str(), Some("中文"));
    }

    #[test]
    fn unset_and_derived_values() {
        let text = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('X'));\nENDSEC;\nDATA;\n#1=THING($,*,1);\nENDSEC;\nEND-ISO-10303-21;\n";
        let doc = parse_part21(text).expect("parse");
        let args = doc.instance(1).unwrap().entity("THING").unwrap();
        assert_eq!(args[0], Part21Value::Unset);
        assert_eq!(args[1], Part21Value::Derived);
        assert_eq!(args[2], Part21Value::Int(1));
    }

    #[test]
    fn nested_lists_round_trip() {
        let text = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('X'));\nENDSEC;\nDATA;\n#1=MATRIX(((1.,0.),(0.,1.)));\nENDSEC;\nEND-ISO-10303-21;\n";
        let doc = parse_part21(text).expect("parse");
        let args = doc.instance(1).unwrap().entity("MATRIX").unwrap();
        let outer = args[0].as_list().expect("outer list");
        assert_eq!(outer.len(), 2);
        assert_eq!(outer[0].as_list().unwrap()[0].as_real(), Some(1.0));
        assert_eq!(parse_part21(&write_part21(&doc)).unwrap(), doc);
    }

    #[test]
    fn malformed_input_is_typed_error_not_fabrication() {
        let bad = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('X'));\nENDSEC;\nDATA;\n#1=THING(;\nENDSEC;\nEND-ISO-10303-21;\n";
        assert!(parse_part21(bad).is_err());
    }
}
//#endregion 🧪️Tests
