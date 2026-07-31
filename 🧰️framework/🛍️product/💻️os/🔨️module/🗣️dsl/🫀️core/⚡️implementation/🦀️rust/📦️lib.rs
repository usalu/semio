//! 🧬️ `dsl_core` — foundation of the token-native declarative DSL engine: spans, errors,
//! diagnostics, resource limits, symbol interning, the fixed token alphabet + lexer, the one
//! canonical escape scheme, canonical number formatting, and trust-ladder branded types.
//! Sits BELOW `vcs` (which re-exports `TextSpan`/`TextError` from here) so every future DSL
//! surface — including `vcs`'s own textual op-log — shares one vocabulary.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};
use thiserror::Error;

//#region 🔖️Span
/// @emoji 📍️ 1-based line/column position with a length, covering a run of source text.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextSpan {
    pub line: u32,
    pub column: u32,
    pub length: u32,
}

impl TextSpan {
    pub fn at(line: u32, column: u32) -> Self {
        Self { line, column, length: 0 }
    }

    pub fn with_length(line: u32, column: u32, length: u32) -> Self {
        Self { line, column, length }
    }
}
//#endregion 🔖️Span

//#region 🔖️Errors
/// @emoji 🚧️ Span-carrying parse/print failure — the one error type every DSL surface returns.
#[derive(Clone, Debug, PartialEq, Error, Serialize, Deserialize)]
#[error("{message} at {}:{}", span.line, span.column)]
pub struct TextError {
    pub message: String,
    pub span: TextSpan,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected: Option<String>,
}

impl TextError {
    pub fn new(message: impl Into<String>, span: TextSpan) -> Self {
        Self { message: message.into(), span, expected: None }
    }

    pub fn expected(message: impl Into<String>, span: TextSpan, expected: impl Into<String>) -> Self {
        Self { message: message.into(), span, expected: Some(expected.into()) }
    }

    pub fn from_diagnostic(diagnostic: Diagnostic) -> Self {
        diagnostic.into_text_error()
    }
}

/// @emoji 🏷️ Stable, greppable diagnostic identifier, e.g. `"DSL0001"`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DiagnosticCode(pub &'static str);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Error,
    Warning,
    Hint,
}

/// @emoji 🧭️ What the parser would have accepted at the failure point — the raw material for
/// completions and for `TextError.expected`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ExpectedSet {
    pub tokens: Vec<String>,
    pub keywords: Vec<String>,
    pub keys: Vec<String>,
}

impl ExpectedSet {
    pub fn describe(&self) -> String {
        let mut parts = Vec::new();
        if !self.keywords.is_empty() {
            parts.push(self.keywords.join("|"));
        }
        if !self.keys.is_empty() {
            parts.push(self.keys.iter().map(|k| format!("{k}=")).collect::<Vec<_>>().join("|"));
        }
        if !self.tokens.is_empty() {
            parts.push(self.tokens.join("|"));
        }
        parts.join(" or ")
    }
}

/// @emoji 🩺️ A structured diagnostic anchored to a span, with an optional `ExpectedSet` for
/// completions/fixes. Lowers into `TextError` at API boundaries that predate diagnostics.
#[derive(Clone, Debug, PartialEq)]
pub struct Diagnostic {
    pub code: DiagnosticCode,
    pub severity: Severity,
    pub span: TextSpan,
    pub message: String,
    pub expected: Option<ExpectedSet>,
}

impl Diagnostic {
    pub fn error(code: &'static str, span: TextSpan, message: impl Into<String>) -> Self {
        Self { code: DiagnosticCode(code), severity: Severity::Error, span, message: message.into(), expected: None }
    }

    pub fn with_expected(mut self, expected: ExpectedSet) -> Self {
        self.expected = Some(expected);
        self
    }

    pub fn into_text_error(self) -> TextError {
        let expected = self.expected.as_ref().map(ExpectedSet::describe);
        match expected {
            Some(expected) => TextError::expected(self.message, self.span, expected),
            None => TextError::new(self.message, self.span),
        }
    }
}
//#endregion 🔖️Errors

//#region 🔖️Limits
/// @emoji 🛡️ Resource budgets threaded through every parse — exceeding one yields a budget
/// diagnostic (`DSL0100`), never a panic or unbounded recursion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Limits {
    pub max_bytes: usize,
    pub max_tokens: usize,
    pub max_depth: usize,
    pub max_nodes: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self { max_bytes: 16 * 1024 * 1024, max_tokens: 1_000_000, max_depth: 64, max_nodes: 1_000_000 }
    }
}

pub const BUDGET_EXCEEDED_CODE: &str = "DSL0100";

impl Limits {
    pub fn check_bytes(&self, len: usize) -> Result<(), TextError> {
        if len > self.max_bytes {
            return Err(TextError::new(format!("input exceeds max_bytes limit ({} > {})", len, self.max_bytes), TextSpan::at(1, 1)));
        }
        Ok(())
    }

    pub fn check_depth(&self, depth: usize, span: TextSpan) -> Result<(), TextError> {
        if depth > self.max_depth {
            return Err(TextError::new(format!("nesting exceeds max_depth limit ({} > {})", depth, self.max_depth), span));
        }
        Ok(())
    }

    pub fn check_tokens(&self, count: usize, span: TextSpan) -> Result<(), TextError> {
        if count > self.max_tokens {
            return Err(TextError::new(format!("token count exceeds max_tokens limit ({} > {})", count, self.max_tokens), span));
        }
        Ok(())
    }

    pub fn check_nodes(&self, count: usize, span: TextSpan) -> Result<(), TextError> {
        if count > self.max_nodes {
            return Err(TextError::new(format!("node count exceeds max_nodes limit ({} > {})", count, self.max_nodes), span));
        }
        Ok(())
    }
}
//#endregion 🔖️Limits

//#region 🔖️Intern
/// @emoji 🔖️ An interned string handle — cheap to copy/compare, the payload type for `Ident`
/// tokens and keyword/key lookups.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Symbol(u32);

struct InternerState {
    strings: Vec<Arc<str>>,
    lookup: HashMap<Arc<str>, u32>,
}

static INTERNER: OnceLock<Mutex<InternerState>> = OnceLock::new();

fn interner() -> &'static Mutex<InternerState> {
    INTERNER.get_or_init(|| Mutex::new(InternerState { strings: Vec::new(), lookup: HashMap::new() }))
}

impl Symbol {
    pub fn intern(text: &str) -> Self {
        let mut state = interner().lock().unwrap_or_else(|poison| poison.into_inner());
        if let Some(id) = state.lookup.get(text) {
            return Symbol(*id);
        }
        let arc: Arc<str> = Arc::from(text);
        let id = state.strings.len() as u32;
        state.strings.push(arc.clone());
        state.lookup.insert(arc, id);
        Symbol(id)
    }

    pub fn as_str(&self) -> Arc<str> {
        let state = interner().lock().unwrap_or_else(|poison| poison.into_inner());
        state.strings[self.0 as usize].clone()
    }
}
//#endregion 🔖️Intern

//#region 🔖️Tokens
/// @emoji 🪙️ Stable token identity WITHIN one lex pass — an index into that pass's token vector,
/// never a byte offset. `dsl_token` gives tokens identity that survives edits; this crate's
/// `TokenId` is the snapshot-scoped building block that layer builds on.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TokenId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Ident,
    Int,
    Float,
    Text,
    Equals,
    Comma,
    Colon,
    At,
    Arrow,
    DashArrow,
    BackArrow,
    Placeholder,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Comment,
    Whitespace,
    Newline,
    Error,
    Eof,
}

impl TokenKind {
    pub fn is_trivia(&self) -> bool {
        matches!(self, TokenKind::Whitespace | TokenKind::Newline | TokenKind::Comment)
    }
}

/// @emoji 🎨️ Editor-facing classification of a token — the highlighting/completion vocabulary,
/// generalizing `mathematical_graph_dsl`'s `TokenClass`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenClass {
    Keyword,
    Ident,
    Number,
    String,
    Operator,
    Punctuation,
    Comment,
    Error,
}

/// @emoji 🧾️ One lexed token: kind, interned text, and a real span (never `(1,1)` placeholder).
#[derive(Clone, Debug, PartialEq)]
pub struct SpannedToken {
    pub id: TokenId,
    pub kind: TokenKind,
    pub text: Symbol,
    pub span: TextSpan,
    pub byte_range: (u32, u32),
}
//#endregion 🔖️Tokens

//#region 🔖️Escape
/// @emoji 🔐️ The ONE canonical escape scheme for quoted `Text` tokens: `\\ \" \n \r \t` plus
/// `\u{XXXX}` for any other control character. Nesting-sound because quoting is a token
/// boundary — re-escaping an already-printed line is exactly invertible, no percent-encoding
/// or per-technology scheme needed. Strict superset of every hand-rolled scheme it replaces.
pub fn escape_text(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{{{:x}}}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

/// @emoji 🔓️ Inverse of [`escape_text`]. Unknown escapes in strict mode are an error; `forgiving`
/// keeps the backslash and following character literal instead (editor/recovery mode).
pub fn unescape_text(value: &str, forgiving: bool) -> Result<String, String> {
    let mut out = String::with_capacity(value.len());
    let mut chars = value.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch != '\\' {
            out.push(ch);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            Some('t') => out.push('\t'),
            Some('"') => out.push('"'),
            Some('\\') => out.push('\\'),
            Some('u') if chars.peek() == Some(&'{') => {
                chars.next();
                let mut hex = String::new();
                loop {
                    match chars.next() {
                        Some('}') => break,
                        Some(c) => hex.push(c),
                        None => return Err("unterminated \\u{...} escape".into()),
                    }
                }
                let code = u32::from_str_radix(&hex, 16).map_err(|_| format!("invalid unicode escape \\u{{{hex}}}"))?;
                let c = char::from_u32(code).ok_or_else(|| format!("invalid unicode scalar \\u{{{hex}}}"))?;
                out.push(c);
            }
            Some(other) => {
                if forgiving {
                    out.push('\\');
                    out.push(other);
                } else {
                    return Err(format!("unknown escape \\{other}"));
                }
            }
            None => {
                if forgiving {
                    out.push('\\');
                } else {
                    return Err("dangling escape at end of text".into());
                }
            }
        }
    }
    Ok(out)
}
//#endregion 🔖️Escape

//#region 🔖️Numbers
/// @emoji 🔢️ Canonical float printing: Rust's `Display` (shortest round-trip repr), with
/// explicit `nan`/`inf`/`-inf` idents so the grammar never emits ambiguous bit patterns.
pub fn format_f64(value: f64) -> String {
    if value.is_nan() {
        "nan".to_string()
    } else if value.is_infinite() {
        if value > 0.0 { "inf".to_string() } else { "-inf".to_string() }
    } else {
        format!("{value}")
    }
}

pub fn format_f32(value: f32) -> String {
    if value.is_nan() {
        "nan".to_string()
    } else if value.is_infinite() {
        if value > 0.0 { "inf".to_string() } else { "-inf".to_string() }
    } else {
        format!("{value}")
    }
}

pub fn parse_f64(text: &str) -> Result<f64, String> {
    match text {
        "nan" => Ok(f64::NAN),
        "inf" => Ok(f64::INFINITY),
        "-inf" => Ok(f64::NEG_INFINITY),
        other => other.parse::<f64>().map_err(|_| format!("invalid float literal '{other}'")),
    }
}

pub fn parse_f32(text: &str) -> Result<f32, String> {
    match text {
        "nan" => Ok(f32::NAN),
        "inf" => Ok(f32::INFINITY),
        "-inf" => Ok(f32::NEG_INFINITY),
        other => other.parse::<f32>().map_err(|_| format!("invalid float literal '{other}'")),
    }
}
//#endregion 🔖️Numbers

//#region 🔖️Units
/// @emoji 📐️ SI base-unit exponents (metre, kilogram, second, kelvin, ampere, radian) identifying
/// a physical dimension. Two units convert into each other only when their `Dimension`s match —
/// this is what stops `Shape::Quantity`/`Shape::Angle` from accepting an incompatible suffix.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Dimension {
    pub m: i8,
    pub kg: i8,
    pub s: i8,
    pub k: i8,
    pub a: i8,
    pub rad: i8,
}

/// @emoji 📏️ One named unit: its printed symbol, physical dimension, and linear factor to the
/// dimension's SI base unit (`base_value = value * factor`). Offset units (`°C`) are out of scope
/// for v1 — every unit here is linear through the origin.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UnitSpec {
    pub symbol: &'static str,
    pub dimension: Dimension,
    pub factor: f64,
}

const DIM_LENGTH: Dimension = Dimension { m: 1, kg: 0, s: 0, k: 0, a: 0, rad: 0 };
const DIM_MASS: Dimension = Dimension { m: 0, kg: 1, s: 0, k: 0, a: 0, rad: 0 };
const DIM_TIME: Dimension = Dimension { m: 0, kg: 0, s: 1, k: 0, a: 0, rad: 0 };
const DIM_TEMPERATURE: Dimension = Dimension { m: 0, kg: 0, s: 0, k: 1, a: 0, rad: 0 };
const DIM_ANGLE: Dimension = Dimension { m: 0, kg: 0, s: 0, k: 0, a: 0, rad: 1 };
const DIM_DIMENSIONLESS: Dimension = Dimension { m: 0, kg: 0, s: 0, k: 0, a: 0, rad: 0 };
const DIM_FORCE: Dimension = Dimension { m: 1, kg: 1, s: -2, k: 0, a: 0, rad: 0 };
const DIM_PRESSURE: Dimension = Dimension { m: -1, kg: 1, s: -2, k: 0, a: 0, rad: 0 };
const DIM_ENERGY: Dimension = Dimension { m: 2, kg: 1, s: -2, k: 0, a: 0, rad: 0 };
const DIM_POWER: Dimension = Dimension { m: 2, kg: 1, s: -3, k: 0, a: 0, rad: 0 };
const DIM_AREA: Dimension = Dimension { m: 2, kg: 0, s: 0, k: 0, a: 0, rad: 0 };
const DIM_VOLUME: Dimension = Dimension { m: 3, kg: 0, s: 0, k: 0, a: 0, rad: 0 };
const DIM_DENSITY: Dimension = Dimension { m: -3, kg: 1, s: 0, k: 0, a: 0, rad: 0 };
const DIM_VELOCITY: Dimension = Dimension { m: 1, kg: 0, s: -1, k: 0, a: 0, rad: 0 };
const DIM_ACCELERATION: Dimension = Dimension { m: 1, kg: 0, s: -2, k: 0, a: 0, rad: 0 };
const DIM_AREAL_LOAD: Dimension = Dimension { m: -2, kg: 1, s: -2, k: 0, a: 0, rad: 0 };
const DIM_HEAT_TRANSFER: Dimension = Dimension { m: 0, kg: 1, s: -3, k: -1, a: 0, rad: 0 };

/// @emoji 📚️ The static unit vocabulary every `Shape::Quantity`/`Shape::Angle` field draws from.
/// Symbols are matched verbatim against the ident glued onto a numeric literal
/// (`210GPa` -> number `210`, suffix `GPa`) — see `dsl_schema::parse_scalar`'s `Quantity`/`Angle`
/// arms. Grows as adopter DSLs need new units; never remove a symbol once a fixture uses it.
const UNITS: &[UnitSpec] = &[
    UnitSpec { symbol: "m", dimension: DIM_LENGTH, factor: 1.0 },
    UnitSpec { symbol: "mm", dimension: DIM_LENGTH, factor: 0.001 },
    UnitSpec { symbol: "cm", dimension: DIM_LENGTH, factor: 0.01 },
    UnitSpec { symbol: "km", dimension: DIM_LENGTH, factor: 1000.0 },
    UnitSpec { symbol: "kg", dimension: DIM_MASS, factor: 1.0 },
    UnitSpec { symbol: "g", dimension: DIM_MASS, factor: 0.001 },
    UnitSpec { symbol: "t", dimension: DIM_MASS, factor: 1000.0 },
    UnitSpec { symbol: "s", dimension: DIM_TIME, factor: 1.0 },
    UnitSpec { symbol: "h", dimension: DIM_TIME, factor: 3600.0 },
    UnitSpec { symbol: "K", dimension: DIM_TEMPERATURE, factor: 1.0 },
    UnitSpec { symbol: "deg", dimension: DIM_ANGLE, factor: std::f64::consts::PI / 180.0 },
    UnitSpec { symbol: "°", dimension: DIM_ANGLE, factor: std::f64::consts::PI / 180.0 },
    UnitSpec { symbol: "rad", dimension: DIM_ANGLE, factor: 1.0 },
    UnitSpec { symbol: "turn", dimension: DIM_ANGLE, factor: std::f64::consts::TAU },
    UnitSpec { symbol: "pct", dimension: DIM_DIMENSIONLESS, factor: 0.01 },
    UnitSpec { symbol: "%", dimension: DIM_DIMENSIONLESS, factor: 0.01 },
    UnitSpec { symbol: "N", dimension: DIM_FORCE, factor: 1.0 },
    UnitSpec { symbol: "kN", dimension: DIM_FORCE, factor: 1000.0 },
    UnitSpec { symbol: "MN", dimension: DIM_FORCE, factor: 1_000_000.0 },
    UnitSpec { symbol: "Pa", dimension: DIM_PRESSURE, factor: 1.0 },
    UnitSpec { symbol: "kPa", dimension: DIM_PRESSURE, factor: 1_000.0 },
    UnitSpec { symbol: "MPa", dimension: DIM_PRESSURE, factor: 1_000_000.0 },
    UnitSpec { symbol: "GPa", dimension: DIM_PRESSURE, factor: 1_000_000_000.0 },
    UnitSpec { symbol: "J", dimension: DIM_ENERGY, factor: 1.0 },
    UnitSpec { symbol: "kJ", dimension: DIM_ENERGY, factor: 1_000.0 },
    UnitSpec { symbol: "W", dimension: DIM_POWER, factor: 1.0 },
    UnitSpec { symbol: "kW", dimension: DIM_POWER, factor: 1_000.0 },
    UnitSpec { symbol: "m2", dimension: DIM_AREA, factor: 1.0 },
    UnitSpec { symbol: "cm2", dimension: DIM_AREA, factor: 0.0001 },
    UnitSpec { symbol: "mm2", dimension: DIM_AREA, factor: 0.000001 },
    UnitSpec { symbol: "m3", dimension: DIM_VOLUME, factor: 1.0 },
    UnitSpec { symbol: "cm3", dimension: DIM_VOLUME, factor: 0.000001 },
    UnitSpec { symbol: "mm4", dimension: Dimension { m: 4, kg: 0, s: 0, k: 0, a: 0, rad: 0 }, factor: 1e-12 },
    UnitSpec { symbol: "cm4", dimension: Dimension { m: 4, kg: 0, s: 0, k: 0, a: 0, rad: 0 }, factor: 1e-8 },
    UnitSpec { symbol: "kg/m3", dimension: DIM_DENSITY, factor: 1.0 },
    UnitSpec { symbol: "m/s", dimension: DIM_VELOCITY, factor: 1.0 },
    UnitSpec { symbol: "m/s2", dimension: DIM_ACCELERATION, factor: 1.0 },
    UnitSpec { symbol: "kN/m2", dimension: DIM_AREAL_LOAD, factor: 1000.0 },
    UnitSpec { symbol: "kPa/m2", dimension: DIM_AREAL_LOAD, factor: 1000.0 },
    UnitSpec { symbol: "W/m2K", dimension: DIM_HEAT_TRANSFER, factor: 1.0 },
];

/// @emoji 🔍️ Looks up a unit by its exact printed symbol (e.g. `"GPa"`).
pub fn unit_by_symbol(symbol: &str) -> Option<&'static UnitSpec> {
    UNITS.iter().find(|u| u.symbol == symbol)
}

/// @emoji 🔁️ Converts `value` (expressed in `from`) into the equivalent value expressed in `to`.
/// `None` if the two units don't share a dimension — never silently reinterprets across
/// incompatible units (e.g. a length suffix on an angle field).
pub fn convert(value: f64, from: &UnitSpec, to: &UnitSpec) -> Option<f64> {
    if from.dimension != to.dimension {
        return None;
    }
    Some(value * from.factor / to.factor)
}
//#endregion 🔖️Units

//#region 🔖️Lexer
fn is_ident_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_ident_continue(c: char) -> bool {
    c.is_alphanumeric() || matches!(c, '_' | '-' | '.' | '/')
}

/// @emoji 🔬️ Grammar-independent lexer for the fixed token alphabet shared by every DSL grammar
/// declared on this engine. `forgiving = true` never fails (malformed regions become `Error`
/// tokens instead), which is what editor/completion mode needs; `forgiving = false` is strict
/// parse mode and returns the first lexical error.
pub fn lex(text: &str, limits: &Limits, forgiving: bool) -> Result<Vec<SpannedToken>, TextError> {
    limits.check_bytes(text.len())?;
    let chars: Vec<char> = text.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0usize;
    let mut line = 1u32;
    let mut column = 1u32;
    let mut byte_offset = 0u32;
    let mut next_id = 0u32;

    macro_rules! push {
        ($kind:expr, $start_line:expr, $start_col:expr, $start_byte:expr, $text:expr) => {{
            let text_str: String = $text;
            let len = text_str.chars().count() as u32;
            tokens.push(SpannedToken {
                id: TokenId(next_id),
                kind: $kind,
                text: Symbol::intern(&text_str),
                span: TextSpan::with_length($start_line, $start_col, len),
                byte_range: ($start_byte, byte_offset),
            });
            next_id += 1;
        }};
    }

    while i < chars.len() {
        limits.check_tokens(tokens.len(), TextSpan::at(line, column))?;
        let c = chars[i];
        let start_line = line;
        let start_col = column;
        let start_byte = byte_offset;

        if c == '\n' {
            i += 1;
            byte_offset += c.len_utf8() as u32;
            push!(TokenKind::Newline, start_line, start_col, start_byte, "\n".to_string());
            line += 1;
            column = 1;
            continue;
        }
        if c.is_whitespace() {
            let mut j = i;
            let mut buf = String::new();
            while j < chars.len() && chars[j].is_whitespace() && chars[j] != '\n' {
                buf.push(chars[j]);
                byte_offset += chars[j].len_utf8() as u32;
                column += 1;
                j += 1;
            }
            i = j;
            push!(TokenKind::Whitespace, start_line, start_col, start_byte, buf);
            continue;
        }
        if c == '#' {
            let mut j = i;
            let mut buf = String::new();
            while j < chars.len() && chars[j] != '\n' {
                buf.push(chars[j]);
                byte_offset += chars[j].len_utf8() as u32;
                column += 1;
                j += 1;
            }
            i = j;
            push!(TokenKind::Comment, start_line, start_col, start_byte, buf);
            continue;
        }
        if c == '"' {
            let mut j = i + 1;
            let mut buf = String::new();
            let mut closed = false;
            byte_offset += c.len_utf8() as u32;
            column += 1;
            while j < chars.len() {
                let cj = chars[j];
                if cj == '\\' && j + 1 < chars.len() {
                    buf.push(cj);
                    buf.push(chars[j + 1]);
                    byte_offset += cj.len_utf8() as u32 + chars[j + 1].len_utf8() as u32;
                    column += 2;
                    j += 2;
                    continue;
                }
                if cj == '"' {
                    byte_offset += cj.len_utf8() as u32;
                    column += 1;
                    j += 1;
                    closed = true;
                    break;
                }
                if cj == '\n' {
                    if forgiving {
                        break;
                    }
                    return Err(TextError::new("unterminated string literal (newline before closing quote)", TextSpan::at(start_line, start_col)));
                }
                buf.push(cj);
                byte_offset += cj.len_utf8() as u32;
                column += 1;
                j += 1;
            }
            i = j;
            if !closed && !forgiving {
                return Err(TextError::new("unterminated string literal", TextSpan::at(start_line, start_col)));
            }
            push!(if closed { TokenKind::Text } else { TokenKind::Error }, start_line, start_col, start_byte, buf);
            continue;
        }
        // `-inf` is its own special float literal (the negative-infinity half of the "nan/inf/-inf"
        // ident convention `format_f64`/`parse_f64` round-trip) — unlike ordinary numbers, `-` isn't
        // followed by a digit here, and `-` isn't a valid ident-start character either, so without
        // this it falls through every branch below to "unknown character".
        if c == '-' && i + 4 <= chars.len() && chars[i + 1] == 'i' && chars[i + 2] == 'n' && chars[i + 3] == 'f' && !chars.get(i + 4).is_some_and(|next| is_ident_continue(*next)) {
            i += 4;
            byte_offset += 4;
            column += 4;
            push!(TokenKind::Float, start_line, start_col, start_byte, "-inf".to_string());
            continue;
        }
        if c.is_ascii_digit() || (c == '-' && i + 1 < chars.len() && chars[i + 1].is_ascii_digit()) {
            let mut j = i;
            let mut buf = String::new();
            let mut is_float = false;
            if chars[j] == '-' {
                buf.push('-');
                byte_offset += 1;
                column += 1;
                j += 1;
            }
            while j < chars.len() && chars[j].is_ascii_digit() {
                buf.push(chars[j]);
                byte_offset += 1;
                column += 1;
                j += 1;
            }
            if j < chars.len() && chars[j] == '.' && j + 1 < chars.len() && chars[j + 1].is_ascii_digit() {
                is_float = true;
                buf.push('.');
                byte_offset += 1;
                column += 1;
                j += 1;
                while j < chars.len() && chars[j].is_ascii_digit() {
                    buf.push(chars[j]);
                    byte_offset += 1;
                    column += 1;
                    j += 1;
                }
            }
            if j < chars.len() && (chars[j] == 'e' || chars[j] == 'E') {
                let mut k = j + 1;
                if k < chars.len() && (chars[k] == '+' || chars[k] == '-') {
                    k += 1;
                }
                if k < chars.len() && chars[k].is_ascii_digit() {
                    is_float = true;
                    while j < k {
                        buf.push(chars[j]);
                        byte_offset += 1;
                        column += 1;
                        j += 1;
                    }
                    while j < chars.len() && chars[j].is_ascii_digit() {
                        buf.push(chars[j]);
                        byte_offset += 1;
                        column += 1;
                        j += 1;
                    }
                }
            }
            i = j;
            push!(if is_float { TokenKind::Float } else { TokenKind::Int }, start_line, start_col, start_byte, buf);
            continue;
        }
        if is_ident_start(c) {
            let mut j = i;
            let mut buf = String::new();
            while j < chars.len() && is_ident_continue(chars[j]) {
                // A '-' that starts an `->`/`--` operator terminates the ident here instead of
                // being swallowed into it, so kebab-case idents ("hexagonal-mushroom-column")
                // and the Arrow/DashArrow operators coexist without ambiguity.
                if chars[j] == '-' && j + 1 < chars.len() && matches!(chars[j + 1], '>' | '-') {
                    break;
                }
                buf.push(chars[j]);
                byte_offset += chars[j].len_utf8() as u32;
                column += 1;
                j += 1;
            }
            i = j;
            // A lone `_` is the placeholder sigil (positional "absent" marker), never an ident —
            // `_foo`/`foo_bar` still lex as ordinary idents since the buffer differs from "_".
            let kind = if buf == "_" { TokenKind::Placeholder } else { TokenKind::Ident };
            push!(kind, start_line, start_col, start_byte, buf);
            continue;
        }
        if c == '<' && i + 1 < chars.len() && chars[i + 1] == '-' {
            i += 2;
            byte_offset += 2;
            column += 2;
            push!(TokenKind::BackArrow, start_line, start_col, start_byte, "<-".to_string());
            continue;
        }
        if c == '-' && i + 1 < chars.len() && chars[i + 1] == '>' {
            i += 2;
            byte_offset += 2;
            column += 2;
            push!(TokenKind::Arrow, start_line, start_col, start_byte, "->".to_string());
            continue;
        }
        if c == '-' && i + 1 < chars.len() && chars[i + 1] == '-' {
            i += 2;
            byte_offset += 2;
            column += 2;
            push!(TokenKind::DashArrow, start_line, start_col, start_byte, "--".to_string());
            continue;
        }
        let single = match c {
            '=' => Some(TokenKind::Equals),
            ',' => Some(TokenKind::Comma),
            ':' => Some(TokenKind::Colon),
            '@' => Some(TokenKind::At),
            '{' => Some(TokenKind::LBrace),
            '}' => Some(TokenKind::RBrace),
            '[' => Some(TokenKind::LBracket),
            ']' => Some(TokenKind::RBracket),
            '(' => Some(TokenKind::LParen),
            ')' => Some(TokenKind::RParen),
            _ => None,
        };
        if let Some(kind) = single {
            i += 1;
            byte_offset += c.len_utf8() as u32;
            column += 1;
            push!(kind, start_line, start_col, start_byte, c.to_string());
            continue;
        }
        // Unknown character.
        i += 1;
        byte_offset += c.len_utf8() as u32;
        column += 1;
        if !forgiving {
            return Err(TextError::new(format!("unexpected character '{c}'"), TextSpan::at(start_line, start_col)));
        }
        push!(TokenKind::Error, start_line, start_col, start_byte, c.to_string());
    }
    let eof_line = line;
    let eof_col = column;
    tokens.push(SpannedToken {
        id: TokenId(next_id),
        kind: TokenKind::Eof,
        text: Symbol::intern(""),
        span: TextSpan::at(eof_line, eof_col),
        byte_range: (byte_offset, byte_offset),
    });
    // Strict-mode success is exactly the invariant `Sanitized` documents — brand it here so the
    // type isn't just a paper promise, then unwrap since callers still want plain tokens.
    if forgiving { Ok(tokens) } else { Ok(Sanitized::new_trusted(tokens).into_inner()) }
}

/// @emoji 🎨️ Maps lexed tokens to editor highlighting classes. `keywords` is the live set of
/// idents that are structural keywords in the current grammar context (schema-declared).
pub fn token_classes(tokens: &[SpannedToken], keywords: &[&str]) -> Vec<(TokenClass, TextSpan)> {
    tokens
        .iter()
        .filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof)
        .map(|t| {
            let class = match t.kind {
                TokenKind::Ident => {
                    let text = t.text.as_str();
                    if keywords.contains(&text.as_ref()) { TokenClass::Keyword } else { TokenClass::Ident }
                }
                TokenKind::Int | TokenKind::Float => TokenClass::Number,
                TokenKind::Text => TokenClass::String,
                TokenKind::Placeholder => TokenClass::Ident,
                TokenKind::Equals | TokenKind::Arrow | TokenKind::DashArrow | TokenKind::BackArrow | TokenKind::At | TokenKind::Colon => TokenClass::Operator,
                TokenKind::Comma | TokenKind::LBrace | TokenKind::RBrace | TokenKind::LBracket | TokenKind::RBracket | TokenKind::LParen | TokenKind::RParen => TokenClass::Punctuation,
                TokenKind::Comment => TokenClass::Comment,
                TokenKind::Error => TokenClass::Error,
                TokenKind::Whitespace | TokenKind::Newline | TokenKind::Eof => unreachable!("filtered above"),
            };
            (class, t.span)
        })
        .collect()
}
/// @emoji 🪪️ True iff `s` lexes (strict) as exactly one `Ident` token whose text equals `s` —
/// i.e. `s` is safe to print bare (unquoted) wherever `Shape::Text` is expected. Excludes the
/// reserved literal idents (`_`/`true`/`false`/`null`/`nan`/`inf`) and anything number-shaped
/// (those lex as `Int`/`Float`/`Placeholder`, not `Ident`, so they're already excluded by
/// construction — the reserved-word list catches the ones that would otherwise lex as `Ident`).
/// Implemented defensively by actually calling the lexer rather than hand-rolling a second
/// notion of "identifier-shaped" that could drift from the real grammar.
pub fn is_bare_ident(s: &str) -> bool {
    if matches!(s, "_" | "true" | "false" | "null" | "nan" | "inf") {
        return false;
    }
    match lex(s, &Limits::default(), false) {
        Ok(tokens) => {
            let significant: Vec<&SpannedToken> = tokens.iter().filter(|t| t.kind != TokenKind::Eof).collect();
            matches!(significant.as_slice(), [only] if only.kind == TokenKind::Ident && only.text.as_str().as_ref() == s)
        }
        Err(_) => false,
    }
}
//#endregion 🔖️Lexer

//#region 🔖️Trust
/// @emoji 🛂️ A value that has passed [`crate::lex`] in strict mode. Constructible only within
/// this crate/its trusted callers — public API never lets a caller wrap arbitrary text as
/// `Sanitized` without going through the real check.
#[derive(Clone, Debug)]
pub struct Sanitized<T>(T);

impl<T> Sanitized<T> {
    pub(crate) fn new_trusted(value: T) -> Self {
        Self(value)
    }

    pub fn into_inner(self) -> T {
        self.0
    }

    pub fn get(&self) -> &T {
        &self.0
    }
}

/// @emoji 🛂️ A value that has additionally passed schema validation. Reserved for the
/// `dsl_schema` layer to construct.
#[derive(Clone, Debug)]
pub struct SchemaValid<T>(T);

impl<T> SchemaValid<T> {
    pub fn new_trusted(value: T) -> Self {
        Self(value)
    }

    pub fn into_inner(self) -> T {
        self.0
    }

    pub fn get(&self) -> &T {
        &self.0
    }
}
//#endregion 🔖️Trust

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_round_trips_every_control_case() {
        let cases = [
            "plain text",
            "with \"quotes\" and \\backslash\\",
            "line1\nline2\ttabbed\r\n",
            "unicode: 🔖️ café naïve",
            "\u{0007}bell and \u{001b}escape",
        ];
        for case in cases {
            let escaped = escape_text(case);
            assert!(!escaped.contains('\n'), "escaped text must not contain a raw newline: {escaped:?}");
            let restored = unescape_text(&escaped, false).expect("unescape");
            assert_eq!(restored, case, "round trip failed for {case:?}");
        }
    }

    #[test]
    fn unescape_forgiving_mode_keeps_unknown_escapes_literal() {
        assert_eq!(unescape_text("\\q", true).unwrap(), "\\q");
        assert!(unescape_text("\\q", false).is_err());
    }

    #[test]
    fn float_format_round_trips_including_specials() {
        for value in [0.0_f64, -0.0, 1.5, -42.125, 1e300, 1e-300, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let printed = format_f64(value);
            let parsed = parse_f64(&printed).expect("parse");
            if value.is_nan() {
                assert!(parsed.is_nan());
            } else {
                assert_eq!(parsed, value, "float round trip failed for {value} -> {printed}");
            }
        }
    }

    #[test]
    fn lexer_tokenizes_a_representative_record_line() {
        let tokens = lex(r#"camera x=1.5 y=-2 zoom=1 label="a \"b\" c""#, &Limits::default(), false).expect("lex");
        let kinds: Vec<TokenKind> = tokens.iter().map(|t| t.kind).filter(|k| !k.is_trivia()).collect();
        assert_eq!(
            kinds,
            vec![
                TokenKind::Ident, // camera
                TokenKind::Ident, // x
                TokenKind::Equals,
                TokenKind::Float, // 1.5
                TokenKind::Ident, // y
                TokenKind::Equals,
                TokenKind::Int, // -2
                TokenKind::Ident, // zoom
                TokenKind::Equals,
                TokenKind::Int, // 1
                TokenKind::Ident, // label
                TokenKind::Equals,
                TokenKind::Text,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn lexer_spans_are_real_not_placeholder() {
        let tokens = lex("a\nb c", &Limits::default(), false).expect("lex");
        let b = tokens.iter().find(|t| t.text.as_str().as_ref() == "b").expect("b token");
        assert_eq!(b.span.line, 2);
        assert_eq!(b.span.column, 1);
        let c = tokens.iter().find(|t| t.text.as_str().as_ref() == "c").expect("c token");
        assert_eq!(c.span.line, 2);
        assert_eq!(c.span.column, 3);
    }

    #[test]
    fn lexer_wire_literal_alphabet_tokenizes() {
        let tokens = lex("a:Kind@out->b:Kind2@in", &Limits::default(), false).expect("lex");
        let kinds: Vec<TokenKind> = tokens.iter().map(|t| t.kind).filter(|k| !k.is_trivia() && *k != TokenKind::Eof).collect();
        assert_eq!(
            kinds,
            vec![
                TokenKind::Ident,
                TokenKind::Colon,
                TokenKind::Ident,
                TokenKind::At,
                TokenKind::Ident,
                TokenKind::Arrow,
                TokenKind::Ident,
                TokenKind::Colon,
                TokenKind::Ident,
                TokenKind::At,
                TokenKind::Ident,
            ]
        );
    }

    #[test]
    fn lexer_kebab_case_ident_and_arrow_coexist() {
        let tokens = lex("hexagonal-mushroom-column->target", &Limits::default(), false).expect("lex");
        let significant: Vec<(TokenKind, String)> = tokens
            .iter()
            .filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof)
            .map(|t| (t.kind, t.text.as_str().to_string()))
            .collect();
        assert_eq!(
            significant,
            vec![
                (TokenKind::Ident, "hexagonal-mushroom-column".to_string()),
                (TokenKind::Arrow, "->".to_string()),
                (TokenKind::Ident, "target".to_string()),
            ]
        );
    }

    #[test]
    fn lexer_recognizes_negative_infinity_as_one_float_token() {
        let tokens = lex("x=-inf y=-influence z=5", &Limits::default(), true).expect("lex");
        let significant: Vec<(TokenKind, String)> =
            tokens.iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).map(|t| (t.kind, t.text.as_str().to_string())).collect();
        assert_eq!(
            significant,
            vec![
                (TokenKind::Ident, "x".to_string()),
                (TokenKind::Equals, "=".to_string()),
                (TokenKind::Float, "-inf".to_string()),
                (TokenKind::Ident, "y".to_string()),
                (TokenKind::Equals, "=".to_string()),
                // "-influence" must NOT be split into a "-inf" float token plus a stray "luence"
                // ident — the lookahead requires the char right after "-inf" to not itself
                // continue an identifier, so this falls through to ordinary "unknown character"
                // handling for the leading '-' instead (forgiving mode turns it into an Error
                // token), then lexes "influence" as its own ident.
                (TokenKind::Error, "-".to_string()),
                (TokenKind::Ident, "influence".to_string()),
                (TokenKind::Ident, "z".to_string()),
                (TokenKind::Equals, "=".to_string()),
                (TokenKind::Int, "5".to_string()),
            ]
        );
        assert_eq!(parse_f64("-inf").unwrap(), f64::NEG_INFINITY);
        assert_eq!(format_f64(f64::NEG_INFINITY), "-inf");
    }

    #[test]
    fn lexer_strict_mode_errors_on_unterminated_string_with_real_span() {
        let error = lex("key=\"unterminated", &Limits::default(), false).unwrap_err();
        assert_eq!(error.span.line, 1);
        assert_eq!(error.span.column, 5);
    }

    #[test]
    fn lexer_forgiving_mode_never_fails_on_malformed_input() {
        let result = lex("key=\"unterminated\n$$$", &Limits::default(), true);
        assert!(result.is_ok(), "forgiving lexer must not error");
    }

    #[test]
    fn limits_reject_oversized_input_with_a_diagnostic_not_a_panic() {
        let tiny = Limits { max_bytes: 4, ..Limits::default() };
        let error = lex("way too long", &tiny, false).unwrap_err();
        assert!(error.message.contains("max_bytes"));
    }

    #[test]
    fn symbol_interning_is_stable_and_deduplicates() {
        let a = Symbol::intern("hello");
        let b = Symbol::intern("hello");
        let c = Symbol::intern("world");
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_eq!(a.as_str().as_ref(), "hello");
    }

    #[test]
    fn token_classes_distinguish_keywords_from_idents() {
        let tokens = lex("camera x=1", &Limits::default(), false).expect("lex");
        let classes = token_classes(&tokens, &["camera"]);
        assert_eq!(classes[0].0, TokenClass::Keyword);
        assert_eq!(classes[1].0, TokenClass::Ident);
    }

    #[test]
    fn diagnostic_lowers_to_text_error_with_expected_description() {
        let diagnostic = Diagnostic::error("DSL0001", TextSpan::at(2, 3), "unexpected token")
            .with_expected(ExpectedSet { tokens: vec![], keywords: vec!["camera".into(), "layer".into()], keys: vec![] });
        let error = diagnostic.into_text_error();
        assert_eq!(error.span, TextSpan::at(2, 3));
        assert_eq!(error.expected.as_deref(), Some("camera|layer"));
    }

    #[test]
    fn lexer_back_arrow_tokenizes_distinctly_from_dash_and_arrow() {
        let tokens = lex("a<-b a->b a--b a<-hexagonal-column", &Limits::default(), false).expect("lex");
        let significant: Vec<(TokenKind, String)> =
            tokens.iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).map(|t| (t.kind, t.text.as_str().to_string())).collect();
        assert_eq!(
            significant,
            vec![
                (TokenKind::Ident, "a".to_string()),
                (TokenKind::BackArrow, "<-".to_string()),
                (TokenKind::Ident, "b".to_string()),
                (TokenKind::Ident, "a".to_string()),
                (TokenKind::Arrow, "->".to_string()),
                (TokenKind::Ident, "b".to_string()),
                (TokenKind::Ident, "a".to_string()),
                (TokenKind::DashArrow, "--".to_string()),
                (TokenKind::Ident, "b".to_string()),
                (TokenKind::Ident, "a".to_string()),
                (TokenKind::BackArrow, "<-".to_string()),
                // `<` isn't ident-continue, so the '<' of "<-" can never be swallowed into the
                // preceding kebab ident, and the following ident lexes untouched.
                (TokenKind::Ident, "hexagonal-column".to_string()),
            ]
        );
    }

    #[test]
    fn lexer_lone_underscore_is_placeholder_but_underscore_words_are_ident() {
        let tokens = lex("_ _foo foo_bar _", &Limits::default(), false).expect("lex");
        let significant: Vec<(TokenKind, String)> =
            tokens.iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).map(|t| (t.kind, t.text.as_str().to_string())).collect();
        assert_eq!(
            significant,
            vec![
                (TokenKind::Placeholder, "_".to_string()),
                (TokenKind::Ident, "_foo".to_string()),
                (TokenKind::Ident, "foo_bar".to_string()),
                (TokenKind::Placeholder, "_".to_string()),
            ]
        );
    }

    #[test]
    fn is_bare_ident_accepts_normal_idents_and_rejects_reserved_and_number_shaped() {
        assert!(is_bare_ident("alpha"));
        assert!(is_bare_ident("hexagonal-mushroom-column"));
        assert!(is_bare_ident("airtightness_n50"));
        assert!(!is_bare_ident("_"));
        assert!(!is_bare_ident("true"));
        assert!(!is_bare_ident("false"));
        assert!(!is_bare_ident("null"));
        assert!(!is_bare_ident("nan"));
        assert!(!is_bare_ident("inf"));
        assert!(!is_bare_ident("3"));
        assert!(!is_bare_ident("1.5"));
        assert!(!is_bare_ident("-inf"));
        assert!(!is_bare_ident("-2"));
        assert!(!is_bare_ident("two words"));
        assert!(!is_bare_ident(""));
        assert!(!is_bare_ident("\"quoted\""));
    }

    #[test]
    fn unit_lookup_finds_known_symbols_and_rejects_unknown_ones() {
        assert_eq!(unit_by_symbol("GPa").unwrap().symbol, "GPa");
        assert_eq!(unit_by_symbol("deg").unwrap().dimension, DIM_ANGLE);
        assert!(unit_by_symbol("frobnicate").is_none());
    }

    #[test]
    fn unit_conversion_scales_within_a_dimension_and_rejects_across_dimensions() {
        let gpa = unit_by_symbol("GPa").unwrap();
        let mpa = unit_by_symbol("MPa").unwrap();
        assert_eq!(convert(210.0, gpa, mpa), Some(210_000.0));
        let deg = unit_by_symbol("deg").unwrap();
        let rad = unit_by_symbol("rad").unwrap();
        let converted = convert(180.0, deg, rad).unwrap();
        assert!((converted - std::f64::consts::PI).abs() < 1e-9);
        let kg = unit_by_symbol("kg").unwrap();
        assert_eq!(convert(1.0, gpa, kg), None, "pressure must not convert into mass");
    }

    #[test]
    fn unit_conversion_round_trips_back_to_the_original_value() {
        let kn = unit_by_symbol("kN").unwrap();
        let n = unit_by_symbol("N").unwrap();
        let forward = convert(1.5, kn, n).unwrap();
        let back = convert(forward, n, kn).unwrap();
        assert!((back - 1.5).abs() < 1e-9);
    }

    #[test]
    fn ten_thousand_iteration_generative_escape_round_trip() {
        // Hand-rolled xorshift — no proptest/quickcheck dependency in this workspace.
        let mut state: u64 = 0x9E3779B97F4A7C15;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        let alphabet: Vec<char> = "abc\"\\\n\t\r🔖️café".chars().collect();
        for _ in 0..10_000 {
            let len = (next() % 12) as usize;
            let s: String = (0..len).map(|_| alphabet[(next() as usize) % alphabet.len()]).collect();
            let escaped = escape_text(&s);
            assert!(!escaped.contains('\n'));
            let restored = unescape_text(&escaped, false).unwrap_or_else(|e| panic!("seed-reproducible failure for {s:?}: {e}"));
            assert_eq!(restored, s, "generative round trip failed for {s:?}");
        }
    }
}
//#endregion 🧪️Tests
