//! 🔤 Symbol interning, token alphabet, escapes, numbers, and units.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};
use crate::os_dsl::diagnostic::{Limits, TextError, TextSpan};

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

// 🚫️async: E1 pure accessor — plain `OnceLock`/`Mutex` interner, no suspension point, and
// overwhelmingly consumed sync across the tokenizer/lexer/parser (112 sync call sites vs. 8 that
// had been wrongly `.await`ed by the blind codemod) — see R9.
fn interner() -> &'static Mutex<InternerState> {
    INTERNER.get_or_init(|| Mutex::new(InternerState { strings: Vec::new(), lookup: HashMap::new() }))
}

impl Symbol {
    // 🚫️async: E1 pure accessor — see `interner` above.
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

    // 🚫️async: E1 pure accessor — see `interner` above.
    pub fn as_str(&self) -> Arc<str> {
        let state = interner().lock().unwrap_or_else(|poison| poison.into_inner());
        state.strings[self.0 as usize].clone()
    }
}
//#endregion 🔖️Intern

//#region 🔖️Tokens
/// @emoji 🪙️ Stable token identity WITHIN one lex pass — an index into that pass's token vector,
/// never a byte offset. Snapshot-scoped: a fresh lex pass assigns fresh ids, so a `TokenId` is
/// only meaningful against the exact token vector it came from.
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
    /// Fused labeled edge operator: `-e1:Connection>` or `-e1-` (payload in `text`, includes leading `-`).
    EdgeArrow,
    Caret,
    DotDot,
    /// Arithmetic operators, only ever produced in a position where they were previously an
    /// "unknown character" lex error (a leading `+`/`*`/`/`, or a `-` that isn't digit-adjacent
    /// and isn't the start of `->`/`--`) — see `Shape::Expr`'s parser, the only consumer. Purely
    /// additive: no existing valid document could contain one of these in the old error position.
    Plus,
    Minus,
    Star,
    Slash,
    /// A fenced verbatim block (see the lexer's own doc comment on the `` ` `` branch) — the ONE
    /// non-token-by-token construct in this alphabet, and `Shape::Embed`'s only consumer.
    Fence,
    Placeholder,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    LParen,
    RParen,
    /// @emoji 🅰️ Promoted single-char tokens (P2-M1): bare `<`/`>` (XML/SVG tags, never fused into
    /// `<-`/`->`/edge-arrow forms — those are checked first and `continue` before this token can be
    /// produced), `&` (XML entity refs), `$` (STEP unset sigil / DXF header var names), `;` (STEP
    /// statement terminators). Previously every one of these was an "unknown character" `Error`.
    Lt,
    Gt,
    Amp,
    Dollar,
    Semicolon,
    /// @emoji 🔵️ STEP Part 21 dot-delimited enum literal (`.T.` / `.UNSPECIFIED.`) — a leading dot,
    /// an ident-shaped run, a closing dot, captured as one token (text includes both dots).
    DotEnum,
    Comment,
    Whitespace,
    Newline,
    Error,
    Eof,
}

impl TokenKind {
    // 🚫️async: E1 pure classifier consumed by `Iterator::filter` sync closures across the dsl module — see R9
    pub fn is_trivia(&self) -> bool {
        matches!(self, TokenKind::Whitespace | TokenKind::Newline | TokenKind::Comment)
    }
}

/// @emoji 🎨️ Editor-facing classification of a token — the highlighting/completion vocabulary,
/// generalizing `math::graph::dsl`'s `TokenClass`.
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
// 🚫️async: E1 pure, inlined directly into `format!` args at every call site (Display, not Future) — see R9
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
pub async fn unescape_text(value: &str, forgiving: bool) -> Result<String, String> {
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
// 🚫️async: E1 pure Display formatting consumed by `dsl_schema::print_expr_prec`, itself forced sync by an
// `Iterator::map(...).join(...)` sync-closure consumer (`Call` arm) — see R9
pub fn format_f64(value: f64) -> String {
    if value.is_nan() {
        "nan".to_string()
    } else if value.is_infinite() {
        if value > 0.0 {
            "inf".to_string()
        } else {
            "-inf".to_string()
        }
    } else {
        format!("{value}")
    }
}

pub async fn format_f32(value: f32) -> String {
    if value.is_nan() {
        "nan".to_string()
    } else if value.is_infinite() {
        if value > 0.0 {
            "inf".to_string()
        } else {
            "-inf".to_string()
        }
    } else {
        format!("{value}")
    }
}

pub async fn parse_f64(text: &str) -> Result<f64, String> {
    match text {
        "nan" => Ok(f64::NAN),
        "inf" => Ok(f64::INFINITY),
        "-inf" => Ok(f64::NEG_INFINITY),
        other => other.parse::<f64>().map_err(|_| format!("invalid float literal '{other}'")),
    }
}

pub async fn parse_f32(text: &str) -> Result<f32, String> {
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

pub(crate) const DIM_LENGTH: Dimension = Dimension { m: 1, kg: 0, s: 0, k: 0, a: 0, rad: 0 };
pub(crate) const DIM_MASS: Dimension = Dimension { m: 0, kg: 1, s: 0, k: 0, a: 0, rad: 0 };
pub(crate) const DIM_TIME: Dimension = Dimension { m: 0, kg: 0, s: 1, k: 0, a: 0, rad: 0 };
pub(crate) const DIM_TEMPERATURE: Dimension = Dimension { m: 0, kg: 0, s: 0, k: 1, a: 0, rad: 0 };
pub(crate) const DIM_ANGLE: Dimension = Dimension { m: 0, kg: 0, s: 0, k: 0, a: 0, rad: 1 };
pub(crate) const DIM_DIMENSIONLESS: Dimension = Dimension { m: 0, kg: 0, s: 0, k: 0, a: 0, rad: 0 };
pub(crate) const DIM_FORCE: Dimension = Dimension { m: 1, kg: 1, s: -2, k: 0, a: 0, rad: 0 };
pub(crate) const DIM_PRESSURE: Dimension = Dimension { m: -1, kg: 1, s: -2, k: 0, a: 0, rad: 0 };
pub(crate) const DIM_ENERGY: Dimension = Dimension { m: 2, kg: 1, s: -2, k: 0, a: 0, rad: 0 };
pub(crate) const DIM_POWER: Dimension = Dimension { m: 2, kg: 1, s: -3, k: 0, a: 0, rad: 0 };
pub(crate) const DIM_AREA: Dimension = Dimension { m: 2, kg: 0, s: 0, k: 0, a: 0, rad: 0 };
pub(crate) const DIM_VOLUME: Dimension = Dimension { m: 3, kg: 0, s: 0, k: 0, a: 0, rad: 0 };
pub(crate) const DIM_DENSITY: Dimension = Dimension { m: -3, kg: 1, s: 0, k: 0, a: 0, rad: 0 };
pub(crate) const DIM_VELOCITY: Dimension = Dimension { m: 1, kg: 0, s: -1, k: 0, a: 0, rad: 0 };
pub(crate) const DIM_ACCELERATION: Dimension = Dimension { m: 1, kg: 0, s: -2, k: 0, a: 0, rad: 0 };
pub(crate) const DIM_AREAL_LOAD: Dimension = Dimension { m: -2, kg: 1, s: -2, k: 0, a: 0, rad: 0 };
pub(crate) const DIM_HEAT_TRANSFER: Dimension = Dimension { m: 0, kg: 1, s: -3, k: -1, a: 0, rad: 0 };

/// @emoji 📚️ The static unit vocabulary every `Shape::Quantity`/`Shape::Angle` field draws from.
/// Symbols are matched verbatim against the ident glued onto a numeric literal
/// (`210GPa` -> number `210`, suffix `GPa`) — see `crate::os_dsl::schema::parse_scalar`'s `Quantity`/`Angle`
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
    UnitSpec { symbol: "m4", dimension: Dimension { m: 4, kg: 0, s: 0, k: 0, a: 0, rad: 0 }, factor: 1.0 },
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
pub async fn unit_by_symbol(symbol: &str) -> Option<&'static UnitSpec> {
    UNITS.iter().find(|u| u.symbol == symbol)
}

/// @emoji 🔁️ Converts `value` (expressed in `from`) into the equivalent value expressed in `to`.
/// `None` if the two units don't share a dimension — never silently reinterprets across
/// incompatible units (e.g. a length suffix on an angle field).
pub async fn convert(value: f64, from: &UnitSpec, to: &UnitSpec) -> Option<f64> {
    if from.dimension != to.dimension {
        return None;
    }
    if from.symbol == to.symbol {
        return Some(value);
    }
    Some(value * from.factor / to.factor)
}
//#endregion 🔖️Units
