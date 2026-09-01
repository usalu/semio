//! 🧾️ `pack_json` — an owned, spec-correct streaming JSON reader and writer: the replacement for
//! `serde_json` inside framework crates (see `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/
//! INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-9-RUNTIME-DEPENDENCY-REMOVAL/📓️p9b-owned-serialization.md`).
//!
//! Scope, deliberately: a whole-document [`Value`] tree (parse/write), built on top of a
//! token-at-a-time [`Lexer`] so a future chunked-`Read` streaming API can be layered on without a
//! rewrite — no consumer needs multi-buffer incremental parsing today, so that layer itself is NOT
//! built (see the ticket doc's "deliberately not built" section). NOT built either: pretty-printing
//! (no consumer wants indented output), `$ref`/schema composition keywords (the sibling validator in
//! `semio-framework-schema` only needs the keyword subset it actually exercises), arbitrary-precision
//! integers (JSON numbers outside `[i64::MIN, u64::MAX]` fall back to `f64`, exactly like
//! `serde_json` without its `arbitrary_precision` feature — the only configuration this repo ever
//! built with).
//!
//! Number formatting follows the ECMA-262 `Number::toString` fixed/exponential split (`-6 <= e <
//! 21` stays fixed) so a float and an integer of the same magnitude are never spelled the same way
//! (`42.0` always keeps its `.0`) — this is the exact distinction a `serde_json`-based golden test in
//! this repo already polices. NaN/±Infinity — not representable in JSON — encode as `null`,
//! matching `serde_json`'s own behaviour (verified by the differential tests below).

use std::fmt;

use protocol::value::{DslValue, FromValue, ToValue, ValueError};

//#region 🔖️Errors
/// @emoji 🚨️ Every parse failure this crate can produce, with a byte offset into the input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsonError {
    UnexpectedEof,
    UnexpectedByte { found: u8, offset: usize },
    InvalidNumber(usize),
    InvalidEscape(usize),
    InvalidUnicodeEscape(usize),
    UnpairedSurrogate(usize),
    ControlCharacterInString { byte: u8, offset: usize },
    InvalidUtf8,
    TrailingData(usize),
    MaxDepthExceeded(u32),
}

impl fmt::Display for JsonError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedEof => formatter.write_str("unexpected end of input"),
            Self::UnexpectedByte { found, offset } => write!(formatter, "unexpected byte {found:?} at offset {offset}"),
            Self::InvalidNumber(offset) => write!(formatter, "invalid number literal at offset {offset}"),
            Self::InvalidEscape(offset) => write!(formatter, "invalid escape sequence at offset {offset}"),
            Self::InvalidUnicodeEscape(offset) => write!(formatter, "invalid \\u escape at offset {offset}"),
            Self::UnpairedSurrogate(offset) => write!(formatter, "unpaired UTF-16 surrogate at offset {offset}"),
            Self::ControlCharacterInString { byte, offset } => write!(formatter, "control character 0x{byte:02x} in string at offset {offset}"),
            Self::InvalidUtf8 => formatter.write_str("invalid UTF-8 in input"),
            Self::TrailingData(offset) => write!(formatter, "trailing data at offset {offset}"),
            Self::MaxDepthExceeded(depth) => write!(formatter, "maximum nesting depth {depth} exceeded"),
        }
    }
}

impl std::error::Error for JsonError {}

/// @emoji 🛡️ Recursion ceiling for nested arrays/objects — matches `serde_json`'s own default
/// (128), the value this repo's fixtures were authored against.
pub const MAX_DEPTH: u32 = 128;
//#endregion 🔖️Errors

//#region 🔖️Number
/// @emoji 🔢️ A JSON number, keeping the writer's-eye distinction JSON itself does not: an integer
/// literal (`UInt`/`Int`) round-trips without a decimal point, a `Float` always carries one (or an
/// exponent) so `42` and `42.0` are never confused on the wire.
#[derive(Clone, Copy, Debug)]
pub enum Number {
    UInt(u64),
    Int(i64),
    Float(f64),
}

impl Number {
    /// 🔎️ Widens to `f64` regardless of variant — lossy for `u64`/`i64` magnitudes beyond 2^53.
    pub fn as_f64(&self) -> f64 {
        match *self {
            Number::UInt(v) => v as f64,
            Number::Int(v) => v as f64,
            Number::Float(v) => v,
        }
    }

    /// 🔎️ Exact `i64`, only for the `Int` variant and `UInt` values that fit.
    pub fn as_i64(&self) -> Option<i64> {
        match *self {
            Number::Int(v) => Some(v),
            Number::UInt(v) => i64::try_from(v).ok(),
            Number::Float(_) => None,
        }
    }

    /// 🔎️ Exact `u64`, only for the `UInt` variant and non-negative `Int` values.
    pub fn as_u64(&self) -> Option<u64> {
        match *self {
            Number::UInt(v) => Some(v),
            Number::Int(v) => u64::try_from(v).ok(),
            Number::Float(_) => None,
        }
    }

    /// 🔎️ Whether this literal was written without a decimal point or exponent.
    pub fn is_integer(&self) -> bool {
        !matches!(self, Number::Float(_))
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (Number::UInt(a), Number::UInt(b)) => a == b,
            (Number::Int(a), Number::Int(b)) => a == b,
            (Number::Float(a), Number::Float(b)) => a == b,
            (Number::UInt(a), Number::Int(b)) | (Number::Int(b), Number::UInt(a)) => i64::try_from(a).is_ok_and(|a| a == b),
            _ => false,
        }
    }
}

impl From<u64> for Number {
    fn from(v: u64) -> Self {
        Number::UInt(v)
    }
}
impl From<i64> for Number {
    fn from(v: i64) -> Self {
        Number::Int(v)
    }
}
impl From<f64> for Number {
    fn from(v: f64) -> Self {
        Number::Float(v)
    }
}
impl From<u32> for Number {
    fn from(v: u32) -> Self {
        Number::UInt(v as u64)
    }
}
impl From<i32> for Number {
    fn from(v: i32) -> Self {
        Number::Int(v as i64)
    }
}
impl From<usize> for Number {
    fn from(v: usize) -> Self {
        Number::UInt(v as u64)
    }
}
impl From<i8> for Number {
    fn from(v: i8) -> Self {
        Number::Int(v as i64)
    }
}
//#endregion 🔖️Number

//#region 🔖️Value
/// @emoji 🗂️ An insertion-order-preserving JSON object. Re-inserting an existing key overwrites
/// its value in place (last-value-wins) rather than moving it to the end — the same externally
/// observable behaviour as `serde_json::Map`'s default `BTreeMap` backing, just order-preserving
/// for the common no-duplicate case instead of key-sorted.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Object(Vec<(String, Value)>);

impl Object {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.0.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.0.iter_mut().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    pub fn insert(&mut self, key: impl Into<String>, value: Value) -> Option<Value> {
        let key = key.into();
        if let Some(slot) = self.0.iter_mut().find(|(k, _)| *k == key) {
            return Some(std::mem::replace(&mut slot.1, value));
        }
        self.0.push((key, value));
        None
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &Value)> {
        self.0.iter().map(|(k, v)| (k.as_str(), v))
    }
}

impl<'a> IntoIterator for &'a Object {
    type Item = (&'a str, &'a Value);
    type IntoIter = Box<dyn Iterator<Item = (&'a str, &'a Value)> + 'a>;
    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.iter())
    }
}

impl FromIterator<(String, Value)> for Object {
    fn from_iter<T: IntoIterator<Item = (String, Value)>>(iter: T) -> Self {
        let mut object = Object::new();
        for (key, value) in iter {
            object.insert(key, value);
        }
        object
    }
}

/// @emoji 🌳️ An owned JSON value tree — the `serde_json::Value` replacement. Every framework
/// consumer of dynamically-shaped JSON (schema leaves, protocol probes) reads/writes this type.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(Object),
}

impl Value {
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(v) => Some(v.as_str()),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<&Number> {
        match self {
            Value::Number(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        self.as_number().map(Number::as_f64)
    }

    pub fn as_i64(&self) -> Option<i64> {
        self.as_number().and_then(Number::as_i64)
    }

    pub fn as_u64(&self) -> Option<u64> {
        self.as_number().and_then(Number::as_u64)
    }

    /// 🔎️ `serde_json::Value::as_array`'s own signature (`Option<&Vec<Value>>`, not a bare slice) —
    /// on purpose: `Vec<Value>: Clone` lets `.and_then(Value::as_array).cloned()` call sites that
    /// used to target `serde_json::Value` keep compiling unchanged (`[Value]` alone is unsized and
    /// has no `Clone`).
    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Array(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Value>> {
        match self {
            Value::Array(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&Object> {
        match self {
            Value::Object(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_object_mut(&mut self) -> Option<&mut Object> {
        match self {
            Value::Object(v) => Some(v),
            _ => None,
        }
    }

    /// 🔎️ Object-field lookup, mirroring `serde_json::Value::get` — `None` on any non-object.
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.as_object().and_then(|object| object.get(key))
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.as_object_mut().and_then(|object| object.get_mut(key))
    }

    /// 🔎️ Array-element lookup by position, mirroring `serde_json::Value::get(usize)` —
    /// `None` on any non-array or out-of-bounds index.
    pub fn get_index(&self, index: usize) -> Option<&Value> {
        self.as_array().and_then(|array| array.get(index))
    }

    /// 🧭️ RFC 6901 JSON Pointer lookup, mirroring `serde_json::Value::pointer` — the empty string
    /// resolves to `self`; a non-empty pointer must start with `/`, and each `/`-separated segment
    /// is unescaped (`~1` -> `/`, `~0` -> `~`) before being tried as an object key, then as an
    /// array index. `None` on a malformed pointer, a missing key, or an out-of-range index.
    pub fn pointer(&self, pointer: &str) -> Option<&Value> {
        if pointer.is_empty() {
            return Some(self);
        }
        if !pointer.starts_with('/') {
            return None;
        }
        pointer.split('/').skip(1).try_fold(self, |current, raw_segment| {
            let segment = raw_segment.replace("~1", "/").replace("~0", "~");
            match current {
                Value::Object(_) => current.get(&segment),
                Value::Array(_) => segment.parse::<usize>().ok().and_then(|index| current.get_index(index)),
                _ => None,
            }
        })
    }
}

/// 🗝️ `value["key"]`, mirroring `serde_json::Value`'s own `Index<&str>` — panics if `self` is
/// not an object, returns [`Value::Null`] for a missing key (never panics on a missing key,
/// matching `serde_json`'s own permissive lookup semantics for assertions/fixtures).
impl std::ops::Index<&str> for Value {
    type Output = Value;
    fn index(&self, key: &str) -> &Value {
        static NULL: Value = Value::Null;
        match self.get(key) {
            Some(value) => value,
            None => &NULL,
        }
    }
}

/// 🗝️ `value[index]`, mirroring `serde_json::Value`'s own `Index<usize>` — panics if `self` is
/// not an array, returns [`Value::Null`] for an out-of-bounds index.
impl std::ops::Index<usize> for Value {
    type Output = Value;
    fn index(&self, index: usize) -> &Value {
        static NULL: Value = Value::Null;
        match self.get_index(index) {
            Some(value) => value,
            None => &NULL,
        }
    }
}

/// 🪞️ Cross-type equality against Rust primitives, mirroring `serde_json::Value`'s own
/// `impl_value_eq!` family — lets `assert_eq!(value["key"], "literal")`/`value == 3.0` read
/// exactly like the `serde_json` call sites they replace, with no `.as_str()`/`.as_f64()` unwrap
/// noise at the assertion site.
macro_rules! impl_value_partial_eq {
    ($($ty:ty => $variant_check:expr),+ $(,)?) => {
        $(
            impl PartialEq<$ty> for Value {
                fn eq(&self, other: &$ty) -> bool {
                    ($variant_check)(self, other)
                }
            }
            impl PartialEq<Value> for $ty {
                fn eq(&self, other: &Value) -> bool {
                    ($variant_check)(other, self)
                }
            }
        )+
    };
}

impl_value_partial_eq! {
    str => |value: &Value, other: &str| value.as_str() == Some(other),
    String => |value: &Value, other: &String| value.as_str() == Some(other.as_str()),
    bool => |value: &Value, other: &bool| value.as_bool() == Some(*other),
    f64 => |value: &Value, other: &f64| value.as_f64() == Some(*other),
    i64 => |value: &Value, other: &i64| value.as_i64() == Some(*other),
    u64 => |value: &Value, other: &u64| value.as_u64() == Some(*other),
    i32 => |value: &Value, other: &i32| value.as_i64() == Some(*other as i64),
    u32 => |value: &Value, other: &u32| value.as_u64() == Some(*other as u64),
    usize => |value: &Value, other: &usize| value.as_u64() == Some(*other as u64),
}

impl PartialEq<&str> for Value {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == Some(*other)
    }
}
impl PartialEq<Value> for &str {
    fn eq(&self, other: &Value) -> bool {
        other.as_str() == Some(*self)
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Value::Bool(v)
    }
}
impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value::String(v.to_string())
    }
}
impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::String(v)
    }
}
impl From<u64> for Value {
    fn from(v: u64) -> Self {
        Value::Number(Number::UInt(v))
    }
}
impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Value::Number(Number::Int(v))
    }
}
impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::Number(Number::Float(v))
    }
}
impl From<u32> for Value {
    fn from(v: u32) -> Self {
        Value::Number(Number::UInt(v as u64))
    }
}
impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Value::Number(Number::Int(v as i64))
    }
}
impl From<usize> for Value {
    fn from(v: usize) -> Self {
        Value::Number(Number::UInt(v as u64))
    }
}
impl From<i8> for Value {
    fn from(v: i8) -> Self {
        Value::Number(Number::Int(v as i64))
    }
}
impl From<Vec<Value>> for Value {
    fn from(v: Vec<Value>) -> Self {
        Value::Array(v)
    }
}
impl From<Object> for Value {
    fn from(v: Object) -> Self {
        Value::Object(v)
    }
}
impl<T: Into<Value>> From<Option<T>> for Value {
    fn from(v: Option<T>) -> Self {
        match v {
            Some(inner) => inner.into(),
            None => Value::Null,
        }
    }
}

/// 🏗️ Builds a [`Value::Array`] from any iterator of values.
pub fn array(items: impl IntoIterator<Item = Value>) -> Value {
    Value::Array(items.into_iter().collect())
}

/// 🏗️ Builds a [`Value::Object`] from any iterator of `(key, value)` pairs.
pub fn object(pairs: impl IntoIterator<Item = (String, Value)>) -> Value {
    Value::Object(pairs.into_iter().collect())
}

/// ⚖️ Structural equality that ignores object key order — [`Object`]'s derived `PartialEq`
/// (insertion-order `Vec`-backed) is order-sensitive, unlike `serde_json::Map`'s default
/// key-sorted `BTreeMap` backing; a decode→re-encode round trip through [`crate::json::to_json_string`]
/// naturally reorders fields to Rust struct declaration order, so a "committed JSON is already
/// canonical" style assertion needs this, not `==`, to match the old serde-era test semantics.
pub fn value_eq_ignoring_object_order(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Object(x), Value::Object(y)) => {
            x.len() == y.len() && x.iter().all(|(key, value)| y.get(key).is_some_and(|other| value_eq_ignoring_object_order(value, other)))
        }
        (Value::Array(x), Value::Array(y)) => x.len() == y.len() && x.iter().zip(y.iter()).all(|(l, r)| value_eq_ignoring_object_order(l, r)),
        _ => a == b,
    }
}
//#endregion 🔖️Value

//#region 🔖️DslValueBridge
/// 🌉️ Structural conversion from `protocol::value::DslValue` (the in-memory tree
/// `ToValue`/`FromValue` target onto — see `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/
/// RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/🔍️research/
/// 📓️serde-replacement-surface.md` §"pack::json::Value ↔ DslValue conversion") into this crate's
/// own JSON-text-oriented [`Value`] — the two are sibling shapes with no shared type, so a
/// `Mutation`/`MutationDiff` payload that needs literal JSON **text** (a wire byte string, not
/// just an in-memory value) walks through here on the way to [`to_string`]/[`parse`].
/// `DslValue::Number` is always `f64`; this always produces [`Number::Float`] (never
/// `UInt`/`Int`) — exact per `DslValue`'s own single-`f64` contract, matching what a
/// `serde_json`-via-`f64` round trip already did.
pub fn from_dsl_value(value: &protocol::value::DslValue) -> Value {
    match value {
        protocol::value::DslValue::Null => Value::Null,
        protocol::value::DslValue::Bool(b) => Value::Bool(*b),
        protocol::value::DslValue::Number(n) => Value::Number(Number::Float(*n)),
        protocol::value::DslValue::String(s) => Value::String(s.clone()),
        protocol::value::DslValue::Array(items) => Value::Array(items.iter().map(from_dsl_value).collect()),
        protocol::value::DslValue::Object(entries) => Value::Object(entries.iter().map(|(key, value)| (key.clone(), from_dsl_value(value))).collect()),
    }
}

/// 🌉️ The reverse of [`from_dsl_value`] — widens every JSON number to `f64` on the way in,
/// matching `DslValue::Number`'s own single-`f64` contract.
pub fn to_dsl_value(value: &Value) -> protocol::value::DslValue {
    match value {
        Value::Null => protocol::value::DslValue::Null,
        Value::Bool(b) => protocol::value::DslValue::Bool(*b),
        Value::Number(n) => protocol::value::DslValue::Number(n.as_f64()),
        Value::String(s) => protocol::value::DslValue::String(s.clone()),
        Value::Array(items) => protocol::value::DslValue::Array(items.iter().map(to_dsl_value).collect()),
        Value::Object(entries) => protocol::value::DslValue::object(entries.iter().map(|(key, value)| (key.to_string(), to_dsl_value(value)))),
    }
}
//#endregion 🔖️DslValueBridge

//#region 🔖️Lexer
/// @emoji 🪙️ One structural token — the streaming layer everything else is built on. A future
/// chunked-`Read` streaming API would produce these incrementally across buffer refills; today's
/// `Lexer` already tokenizes without materializing the whole document as a DOM first, it just still
/// requires the whole input as one contiguous `&str` (every framework consumer today hands over a
/// short handcrafted schema leaf, never something worth reading in chunks).
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Null,
    True,
    False,
    Number(Number),
    String(String),
    ArrayStart,
    ArrayEnd,
    ObjectStart,
    ObjectEnd,
    Comma,
    Colon,
}

/// @emoji 🔤️ Token-at-a-time reader over a `&str` — zero-copy for structural bytes, allocating
/// only for string/number token payloads.
pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    /// 🔎️ Current byte offset into the input.
    pub fn position(&self) -> usize {
        self.pos
    }

    fn peek_byte(&self) -> Option<u8> {
        self.input.as_bytes().get(self.pos).copied()
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek_byte(), Some(b' ' | b'\t' | b'\n' | b'\r')) {
            self.pos += 1;
        }
    }

    fn read_literal(&mut self, literal: &'static str, token: Token) -> Result<Option<Token>, JsonError> {
        if self.input[self.pos..].starts_with(literal) {
            self.pos += literal.len();
            Ok(Some(token))
        } else {
            Err(JsonError::UnexpectedByte { found: self.peek_byte().unwrap_or(0), offset: self.pos })
        }
    }

    /// 🔢️ Reads one JSON number literal per RFC 8259 §6: `-? int frac? exp?`, no leading zeros.
    /// Falls back to `Number::Float` whenever a fractional part, exponent, or `u64`/`i64` overflow
    /// is present — identical to `serde_json` with its `float_roundtrip` oracle configuration and
    /// without `arbitrary_precision` (the only configuration this repo ever builds with).
    fn read_number(&mut self) -> Result<Token, JsonError> {
        let start = self.pos;
        let negative = self.peek_byte() == Some(b'-');
        if negative {
            self.pos += 1;
        }
        match self.peek_byte() {
            Some(b'0') => self.pos += 1,
            Some(b'1'..=b'9') => {
                while matches!(self.peek_byte(), Some(b'0'..=b'9')) {
                    self.pos += 1;
                }
            }
            _ => return Err(JsonError::InvalidNumber(start)),
        }
        let mut is_float = false;
        if self.peek_byte() == Some(b'.') {
            is_float = true;
            self.pos += 1;
            let frac_start = self.pos;
            while matches!(self.peek_byte(), Some(b'0'..=b'9')) {
                self.pos += 1;
            }
            if self.pos == frac_start {
                return Err(JsonError::InvalidNumber(start));
            }
        }
        if matches!(self.peek_byte(), Some(b'e' | b'E')) {
            is_float = true;
            self.pos += 1;
            if matches!(self.peek_byte(), Some(b'+' | b'-')) {
                self.pos += 1;
            }
            let exp_start = self.pos;
            while matches!(self.peek_byte(), Some(b'0'..=b'9')) {
                self.pos += 1;
            }
            if self.pos == exp_start {
                return Err(JsonError::InvalidNumber(start));
            }
        }
        let text = &self.input[start..self.pos];
        if is_float {
            let value: f64 = text.parse().map_err(|_| JsonError::InvalidNumber(start))?;
            if !value.is_finite() {
                return Err(JsonError::InvalidNumber(start));
            }
            return Ok(Token::Number(Number::Float(value)));
        }
        if !negative {
            if let Ok(value) = text.parse::<u64>() {
                return Ok(Token::Number(Number::UInt(value)));
            }
        } else if let Ok(value) = text.parse::<i64>() {
            return Ok(Token::Number(Number::Int(value)));
        }
        let value: f64 = text.parse().map_err(|_| JsonError::InvalidNumber(start))?;
        if !value.is_finite() {
            return Err(JsonError::InvalidNumber(start));
        }
        Ok(Token::Number(Number::Float(value)))
    }

    /// 🧵️ Reads the 4 hex digits of one `\uXXXX` escape (already past the `u`).
    fn read_hex4(&mut self, escape_start: usize) -> Result<u32, JsonError> {
        let bytes = self.input.as_bytes();
        if self.pos + 4 > bytes.len() {
            return Err(JsonError::InvalidUnicodeEscape(escape_start));
        }
        let mut value: u32 = 0;
        for &byte in &bytes[self.pos..self.pos + 4] {
            let digit = match byte {
                b'0'..=b'9' => u32::from(byte - b'0'),
                b'a'..=b'f' => u32::from(byte - b'a') + 10,
                b'A'..=b'F' => u32::from(byte - b'A') + 10,
                _ => return Err(JsonError::InvalidUnicodeEscape(escape_start)),
            };
            value = value * 16 + digit;
        }
        self.pos += 4;
        Ok(value)
    }

    /// 🧵️ Reads a full JSON string literal (opening quote must be at `self.pos`). Handles every
    /// short escape, `\uXXXX`, and UTF-16 surrogate pairs for supplementary-plane characters —
    /// rejects a lone (unpaired) surrogate rather than silently producing an invalid `char`.
    fn read_string(&mut self) -> Result<String, JsonError> {
        self.pos += 1; // opening quote
        let mut out = String::new();
        loop {
            let rest = &self.input[self.pos..];
            let ch = rest.chars().next().ok_or(JsonError::UnexpectedEof)?;
            match ch {
                '"' => {
                    self.pos += 1;
                    return Ok(out);
                }
                '\\' => {
                    let escape_start = self.pos;
                    self.pos += 1;
                    let escape_char = self.input[self.pos..].chars().next().ok_or(JsonError::UnexpectedEof)?;
                    self.pos += escape_char.len_utf8();
                    match escape_char {
                        '"' => out.push('"'),
                        '\\' => out.push('\\'),
                        '/' => out.push('/'),
                        'b' => out.push('\u{0008}'),
                        'f' => out.push('\u{000C}'),
                        'n' => out.push('\n'),
                        'r' => out.push('\r'),
                        't' => out.push('\t'),
                        'u' => {
                            let unit = self.read_hex4(escape_start)?;
                            if (0xD800..=0xDBFF).contains(&unit) {
                                if self.peek_byte() != Some(b'\\') {
                                    return Err(JsonError::UnpairedSurrogate(escape_start));
                                }
                                self.pos += 1;
                                if self.peek_byte() != Some(b'u') {
                                    return Err(JsonError::UnpairedSurrogate(escape_start));
                                }
                                self.pos += 1;
                                let low = self.read_hex4(escape_start)?;
                                if !(0xDC00..=0xDFFF).contains(&low) {
                                    return Err(JsonError::UnpairedSurrogate(escape_start));
                                }
                                let scalar = 0x10000 + ((unit - 0xD800) << 10) + (low - 0xDC00);
                                out.push(char::from_u32(scalar).ok_or(JsonError::UnpairedSurrogate(escape_start))?);
                            } else if (0xDC00..=0xDFFF).contains(&unit) {
                                return Err(JsonError::UnpairedSurrogate(escape_start));
                            } else {
                                out.push(char::from_u32(unit).ok_or(JsonError::InvalidUnicodeEscape(escape_start))?);
                            }
                        }
                        _ => return Err(JsonError::InvalidEscape(escape_start)),
                    }
                }
                control if (control as u32) < 0x20 => {
                    return Err(JsonError::ControlCharacterInString { byte: control as u8, offset: self.pos });
                }
                other => {
                    out.push(other);
                    self.pos += other.len_utf8();
                }
            }
        }
    }

    /// 📤️ Reads the next structural or scalar token, or `None` at end of input.
    pub fn next_token(&mut self) -> Result<Option<Token>, JsonError> {
        self.skip_ws();
        let Some(byte) = self.peek_byte() else { return Ok(None) };
        match byte {
            b'{' => {
                self.pos += 1;
                Ok(Some(Token::ObjectStart))
            }
            b'}' => {
                self.pos += 1;
                Ok(Some(Token::ObjectEnd))
            }
            b'[' => {
                self.pos += 1;
                Ok(Some(Token::ArrayStart))
            }
            b']' => {
                self.pos += 1;
                Ok(Some(Token::ArrayEnd))
            }
            b',' => {
                self.pos += 1;
                Ok(Some(Token::Comma))
            }
            b':' => {
                self.pos += 1;
                Ok(Some(Token::Colon))
            }
            b'"' => self.read_string().map(|s| Some(Token::String(s))),
            b't' => self.read_literal("true", Token::True),
            b'f' => self.read_literal("false", Token::False),
            b'n' => self.read_literal("null", Token::Null),
            b'-' | b'0'..=b'9' => self.read_number().map(Some),
            _ => Err(JsonError::UnexpectedByte { found: byte, offset: self.pos }),
        }
    }
}
//#endregion 🔖️Lexer

//#region 🔖️Parser
/// 🌳️ Parses one whole JSON document from `input`, rejecting trailing non-whitespace bytes.
// 🚫️async: R9 pure in-memory parse, no I/O.
pub fn parse(input: &str) -> Result<Value, JsonError> {
    let mut lexer = Lexer::new(input);
    let value = parse_value(&mut lexer, 0)?;
    lexer.skip_ws();
    if lexer.pos < lexer.input.len() {
        return Err(JsonError::TrailingData(lexer.pos));
    }
    Ok(value)
}

/// 🌳️ [`parse`] over raw bytes — errors with [`JsonError::InvalidUtf8`] if `input` is not UTF-8.
pub fn parse_bytes(input: &[u8]) -> Result<Value, JsonError> {
    let text = std::str::from_utf8(input).map_err(|_| JsonError::InvalidUtf8)?;
    parse(text)
}

fn parse_value(lexer: &mut Lexer<'_>, depth: u32) -> Result<Value, JsonError> {
    if depth > MAX_DEPTH {
        return Err(JsonError::MaxDepthExceeded(MAX_DEPTH));
    }
    lexer.skip_ws();
    let offset = lexer.pos;
    let token = lexer.next_token()?.ok_or(JsonError::UnexpectedEof)?;
    match token {
        Token::Null => Ok(Value::Null),
        Token::True => Ok(Value::Bool(true)),
        Token::False => Ok(Value::Bool(false)),
        Token::Number(number) => Ok(Value::Number(number)),
        Token::String(text) => Ok(Value::String(text)),
        Token::ArrayStart => parse_array(lexer, depth + 1),
        Token::ObjectStart => parse_object(lexer, depth + 1),
        Token::ObjectEnd | Token::ArrayEnd | Token::Comma | Token::Colon => Err(JsonError::UnexpectedByte { found: lexer.input.as_bytes()[offset], offset }),
    }
}

fn parse_array(lexer: &mut Lexer<'_>, depth: u32) -> Result<Value, JsonError> {
    let mut items = Vec::new();
    lexer.skip_ws();
    if lexer.peek_byte() == Some(b']') {
        lexer.pos += 1;
        return Ok(Value::Array(items));
    }
    loop {
        items.push(parse_value(lexer, depth)?);
        lexer.skip_ws();
        match lexer.peek_byte() {
            Some(b',') => lexer.pos += 1,
            Some(b']') => {
                lexer.pos += 1;
                return Ok(Value::Array(items));
            }
            Some(other) => return Err(JsonError::UnexpectedByte { found: other, offset: lexer.pos }),
            None => return Err(JsonError::UnexpectedEof),
        }
    }
}

fn parse_object(lexer: &mut Lexer<'_>, depth: u32) -> Result<Value, JsonError> {
    let mut object = Object::new();
    lexer.skip_ws();
    if lexer.peek_byte() == Some(b'}') {
        lexer.pos += 1;
        return Ok(Value::Object(object));
    }
    loop {
        lexer.skip_ws();
        if lexer.peek_byte() != Some(b'"') {
            return Err(JsonError::UnexpectedByte { found: lexer.peek_byte().unwrap_or(0), offset: lexer.pos });
        }
        let key = lexer.read_string()?;
        lexer.skip_ws();
        if lexer.peek_byte() != Some(b':') {
            return Err(JsonError::UnexpectedByte { found: lexer.peek_byte().unwrap_or(0), offset: lexer.pos });
        }
        lexer.pos += 1;
        let value = parse_value(lexer, depth)?;
        object.insert(key, value);
        lexer.skip_ws();
        match lexer.peek_byte() {
            Some(b',') => lexer.pos += 1,
            Some(b'}') => {
                lexer.pos += 1;
                return Ok(Value::Object(object));
            }
            Some(other) => return Err(JsonError::UnexpectedByte { found: other, offset: lexer.pos }),
            None => return Err(JsonError::UnexpectedEof),
        }
    }
}
//#endregion 🔖️Parser

//#region 🔖️Writer
/// ✍️ Writes `value` as compact JSON.
// 🚫️async: R9 pure in-memory format, no I/O.
pub fn to_string(value: &Value) -> String {
    let mut out = String::new();
    write_value(value, &mut out);
    out
}

/// ✍️ Writes `value` as 2-space-indented JSON, matching `serde_json::to_string_pretty`'s own
/// layout (`": "` after object keys, one array/object member per line, no trailing newline) — the
/// `serde_json::to_string_pretty` replacement every human-facing JSON view (an example-document
/// viewer, an exported fixture, a rule inspector) still needs even after its call site stops
/// depending on `serde_json` for everything else.
// 🚫️async: R9 pure in-memory format, no I/O.
pub fn to_string_pretty(value: &Value) -> String {
    let mut out = String::new();
    write_value_pretty(value, &mut out, 0);
    out
}

fn write_indent(out: &mut String, depth: usize) {
    for _ in 0..depth {
        out.push_str("  ");
    }
}

fn write_value_pretty(value: &Value, out: &mut String, depth: usize) {
    match value {
        Value::Array(items) if !items.is_empty() => {
            out.push_str("[\n");
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    out.push_str(",\n");
                }
                write_indent(out, depth + 1);
                write_value_pretty(item, out, depth + 1);
            }
            out.push('\n');
            write_indent(out, depth);
            out.push(']');
        }
        Value::Object(object) if !object.is_empty() => {
            out.push_str("{\n");
            for (index, (key, value)) in object.iter().enumerate() {
                if index > 0 {
                    out.push_str(",\n");
                }
                write_indent(out, depth + 1);
                write_string(key, out);
                out.push_str(": ");
                write_value_pretty(value, out, depth + 1);
            }
            out.push('\n');
            write_indent(out, depth);
            out.push('}');
        }
        other => write_value(other, out),
    }
}

/// 🪞️ `value.to_string()`/`format!("{value}")`, mirroring `serde_json::Value`'s own `Display` —
/// every plugin call site that used to write `serde_json::json!({...}).to_string()` keeps
/// compiling unchanged against `pack::json!({...}).to_string()`.
impl fmt::Display for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&to_string(self))
    }
}

fn write_value(value: &Value, out: &mut String) {
    match value {
        Value::Null => out.push_str("null"),
        Value::Bool(true) => out.push_str("true"),
        Value::Bool(false) => out.push_str("false"),
        Value::Number(number) => write_number(*number, out),
        Value::String(text) => write_string(text, out),
        Value::Array(items) => {
            out.push('[');
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                write_value(item, out);
            }
            out.push(']');
        }
        Value::Object(object) => {
            out.push('{');
            for (index, (key, value)) in object.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                write_string(key, out);
                out.push(':');
                write_value(value, out);
            }
            out.push('}');
        }
    }
}

/// ✍️ Escapes `"`, `\`, and control characters (`\b \f \n \r \t` shorthands, `\u00XX` otherwise);
/// everything else — including non-ASCII — passes through unescaped, matching `serde_json`'s
/// default (`ensure_ascii`-off) writer.
fn write_string(text: &str, out: &mut String) {
    out.push('"');
    for ch in text.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{0008}' => out.push_str("\\b"),
            '\u{000C}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            control if (control as u32) < 0x20 => {
                use fmt::Write as _;
                let _ = write!(out, "\\u{:04x}", control as u32);
            }
            other => out.push(other),
        }
    }
    out.push('"');
}

fn write_number(number: Number, out: &mut String) {
    match number {
        Number::UInt(value) => {
            use fmt::Write as _;
            let _ = write!(out, "{value}");
        }
        Number::Int(value) => {
            use fmt::Write as _;
            let _ = write!(out, "{value}");
        }
        Number::Float(value) => write_float(value, out),
    }
}

/// ✍️ ECMA-262 `Number::toString`-style fixed/exponential split: fixed notation for
/// `-6 <= exponent < 21`, exponential otherwise. A whole-number float always gets an explicit
/// `.0` in fixed notation so it never collapses onto its integer twin on the wire.
fn write_float(value: f64, out: &mut String) {
    if !value.is_finite() {
        out.push_str("null");
        return;
    }
    if value == 0.0 {
        out.push_str(if value.is_sign_negative() { "-0.0" } else { "0.0" });
        return;
    }
    let negative = value.is_sign_negative();
    let magnitude = value.abs();
    let scientific = format!("{magnitude:e}");
    let (mantissa, exponent_text) = scientific.split_once('e').expect("LowerExp always emits an exponent");
    let exponent: i32 = exponent_text.parse().expect("LowerExp exponent is always a plain integer");
    let digits: String = mantissa.chars().filter(|c| *c != '.').collect();
    let digit_count = digits.len() as i32;
    if negative {
        out.push('-');
    }
    if (-6..21).contains(&exponent) {
        if exponent >= digit_count - 1 {
            out.push_str(&digits);
            for _ in 0..(exponent - (digit_count - 1)) {
                out.push('0');
            }
            out.push_str(".0");
        } else if exponent >= 0 {
            let integer_len = (exponent + 1) as usize;
            out.push_str(&digits[..integer_len]);
            out.push('.');
            out.push_str(&digits[integer_len..]);
        } else {
            out.push_str("0.");
            for _ in 0..(-exponent - 1) {
                out.push('0');
            }
            out.push_str(&digits);
        }
    } else {
        out.push(digits.as_bytes()[0] as char);
        if digits.len() > 1 {
            out.push('.');
            out.push_str(&digits[1..]);
        }
        out.push('e');
        if exponent >= 0 {
            out.push('+');
        }
        let _ = fmt::Write::write_fmt(out, format_args!("{exponent}"));
    }
}
//#endregion 🔖️Writer

//#region 🔖️ToFromValueBridge
/// 🔤️ `serde_json::to_string`/`from_str` analogs over [`ToValue`]/[`FromValue`] instead of
/// `Serialize`/`DeserializeOwned` — layered on the structural [`from_dsl_value`]/[`to_dsl_value`]
/// walk above (`//#region 🔖️DslValueBridge`) rather than a second one: a concurrent session
/// landed that walk in this same file while this one was in flight, so this region only adds the
/// generic string convenience pair it didn't have, instead of a duplicate `DslValue <-> Value`
/// conversion. Ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`: the
/// pair every plugin routes JSON text through once it stops deriving
/// `serde::Serialize`/`Deserialize` in favor of `ToValue`/`FromValue`.
pub fn to_json_string<T: ToValue>(value: &T) -> String {
    to_string(&from_dsl_value(&value.to_value()))
}

/// 🔤️ `serde_json::from_str` analog over [`FromValue`] instead of `DeserializeOwned` — a parse
/// failure and a decode failure both collapse onto [`ValueError`], matching `from_value`'s own.
pub fn from_json_str<T: FromValue>(text: &str) -> Result<T, ValueError> {
    let value = parse(text).map_err(|error| ValueError::new(error.to_string()))?;
    T::from_value(to_dsl_value(&value))
}
//#endregion 🔖️ToFromValueBridge

//#region 🔖️Macro
/// 🧩️ `serde_json::json!` replacement — an object/array literal builder over [`Value`], expanded
/// via the standard TT-muncher recursion (see `json_object_internal!`/`json_array_internal!`,
/// `#[doc(hidden)]`, exported only so this macro's own expansion can call them from any crate).
/// Object keys are string literals (`"key": value`); every leaf value goes through
/// [`Value::from`] (or a nested `json!` for a bracketed/braced leaf), so any expression whose type
/// already has a `From<_> for Value` impl — including `Option<T>` — works as a leaf.
#[macro_export]
macro_rules! json {
    (null) => { $crate::json::Value::Null };
    (true) => { $crate::json::Value::Bool(true) };
    (false) => { $crate::json::Value::Bool(false) };
    ([]) => { $crate::json::Value::Array(::std::vec::Vec::new()) };
    ([ $($tt:tt)+ ]) => {
        $crate::json::Value::Array($crate::json_array_internal!(@collect [] $($tt)+))
    };
    ({}) => { $crate::json::Value::Object($crate::json::Object::new()) };
    ({ $($tt:tt)+ }) => {
        $crate::json::Value::Object({
            let mut __object = $crate::json::Object::new();
            $crate::json_object_internal!(__object $($tt)+);
            __object
        })
    };
    ($other:expr) => {
        $crate::json::Value::from($other)
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! json_array_internal {
    (@collect [$($elems:expr,)*]) => {
        ::std::vec![$($elems),*]
    };
    (@collect [$($elems:expr,)*] null $(, $($rest:tt)*)?) => {
        $crate::json_array_internal!(@collect [$($elems,)* $crate::json!(null),] $($($rest)*)?)
    };
    (@collect [$($elems:expr,)*] [$($array:tt)*] $(, $($rest:tt)*)?) => {
        $crate::json_array_internal!(@collect [$($elems,)* $crate::json!([$($array)*]),] $($($rest)*)?)
    };
    (@collect [$($elems:expr,)*] {$($object:tt)*} $(, $($rest:tt)*)?) => {
        $crate::json_array_internal!(@collect [$($elems,)* $crate::json!({$($object)*}),] $($($rest)*)?)
    };
    (@collect [$($elems:expr,)*] $next:expr $(, $($rest:tt)*)?) => {
        $crate::json_array_internal!(@collect [$($elems,)* $crate::json!($next),] $($($rest)*)?)
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! json_object_internal {
    ($object:ident) => {};
    ($object:ident $key:literal : null $(, $($rest:tt)*)?) => {
        $object.insert($key, $crate::json!(null));
        $crate::json_object_internal!($object $($($rest)*)?);
    };
    ($object:ident $key:literal : [$($array:tt)*] $(, $($rest:tt)*)?) => {
        $object.insert($key, $crate::json!([$($array)*]));
        $crate::json_object_internal!($object $($($rest)*)?);
    };
    ($object:ident $key:literal : {$($inner:tt)*} $(, $($rest:tt)*)?) => {
        $object.insert($key, $crate::json!({$($inner)*}));
        $crate::json_object_internal!($object $($($rest)*)?);
    };
    ($object:ident $key:literal : $value:expr $(, $($rest:tt)*)?) => {
        $object.insert($key, $crate::json!($value));
        $crate::json_object_internal!($object $($($rest)*)?);
    };
}
//#endregion 🔖️Macro

#[cfg(test)]
//#region 🔖️Tests
mod tests {
    use super::*;

    //#region 🔖️Literals
    #[test]
    fn parses_literals() {
        assert_eq!(parse("null").unwrap(), Value::Null);
        assert_eq!(parse("true").unwrap(), Value::Bool(true));
        assert_eq!(parse("false").unwrap(), Value::Bool(false));
        assert_eq!(parse("  null  ").unwrap(), Value::Null);
    }

    #[test]
    fn rejects_trailing_data() {
        assert_eq!(parse("null null"), Err(JsonError::TrailingData(5)));
    }

    #[test]
    fn rejects_empty_input() {
        assert_eq!(parse(""), Err(JsonError::UnexpectedEof));
        assert_eq!(parse("   "), Err(JsonError::UnexpectedEof));
    }
    //#endregion 🔖️Literals

    //#region 🔖️Numbers
    #[test]
    fn integer_and_float_are_never_confused() {
        assert_eq!(to_string(&Value::Number(Number::UInt(42))), "42");
        assert_eq!(to_string(&Value::Number(Number::Float(42.0))), "42.0");
        assert_eq!(parse("42").unwrap(), Value::Number(Number::UInt(42)));
        assert_eq!(parse("42.0").unwrap(), Value::Number(Number::Float(42.0)));
        assert_ne!(parse("42").unwrap(), parse("42.0").unwrap());
    }

    #[test]
    fn parses_number_grammar() {
        assert_eq!(parse("0").unwrap(), Value::Number(Number::UInt(0)));
        assert_eq!(parse("-0").unwrap(), Value::Number(Number::Int(0)));
        assert_eq!(parse("-17").unwrap(), Value::Number(Number::Int(-17)));
        assert_eq!(parse("3.125").unwrap(), Value::Number(Number::Float(3.125)));
        assert_eq!(parse("1e10").unwrap(), Value::Number(Number::Float(1e10)));
        assert_eq!(parse("1.5e-3").unwrap(), Value::Number(Number::Float(1.5e-3)));
        assert_eq!(parse("-2E+2").unwrap(), Value::Number(Number::Float(-200.0)));
    }

    #[test]
    fn parses_exact_fractional_zero_below_f64_mantissa_boundary() {
        let text = "8322951083873004.0";
        let expected = 8_322_951_083_873_004.0;
        assert_eq!(parse(text).unwrap(), Value::Number(Number::Float(expected)));
        assert_eq!(serde_json::from_str::<serde_json::Value>(text).unwrap().as_f64(), Some(expected));
    }

    #[test]
    fn parses_exact_decimal_exponent_below_f64_mantissa_boundary() {
        let text = "83229510838730040e-1";
        let expected = 8_322_951_083_873_004.0;
        assert_eq!(parse(text).unwrap(), Value::Number(Number::Float(expected)));
        assert_eq!(serde_json::from_str::<serde_json::Value>(text).unwrap().as_f64(), Some(expected));
    }

    #[test]
    fn rejects_leading_zeros() {
        assert!(parse("01").is_err());
        assert!(parse("[01]").is_err());
        assert!(parse("-01").is_err());
    }

    #[test]
    fn huge_integer_falls_back_to_float() {
        let text = "99999999999999999999999999999999";
        match parse(text).unwrap() {
            Value::Number(Number::Float(_)) => {}
            other => panic!("expected float fallback, got {other:?}"),
        }
    }

    #[test]
    fn rejects_numbers_outside_f64_range() {
        assert!(matches!(parse("1e999"), Err(JsonError::InvalidNumber(0))));
        assert!(matches!(parse("-1e999"), Err(JsonError::InvalidNumber(0))));
        assert!(serde_json::from_str::<serde_json::Value>("1e999").is_err());
        assert!(serde_json::from_str::<serde_json::Value>("-1e999").is_err());
    }

    #[test]
    fn non_finite_floats_encode_as_null() {
        assert_eq!(to_string(&Value::Number(Number::Float(f64::NAN))), "null");
        assert_eq!(to_string(&Value::Number(Number::Float(f64::INFINITY))), "null");
        assert_eq!(to_string(&Value::Number(Number::Float(f64::NEG_INFINITY))), "null");
    }

    #[test]
    fn large_and_small_magnitudes_use_exponential_notation() {
        let text = to_string(&Value::Number(Number::Float(1.5e300)));
        assert!(text.contains('e'), "expected exponential form, got {text}");
        assert_eq!(parse(&text).unwrap().as_f64().unwrap(), 1.5e300);

        let text = to_string(&Value::Number(Number::Float(5e-300)));
        assert!(text.contains('e'), "expected exponential form, got {text}");
        assert_eq!(parse(&text).unwrap().as_f64().unwrap(), 5e-300);
    }
    //#endregion 🔖️Numbers

    //#region 🔖️Strings
    #[test]
    fn parses_escapes_and_unicode() {
        assert_eq!(parse(r#""hi\nthere""#).unwrap().as_str().unwrap(), "hi\nthere");
        assert_eq!(parse(r#""café""#).unwrap().as_str().unwrap(), "café");
        assert_eq!(parse(r#""😀""#).unwrap().as_str().unwrap(), "😀");
        assert_eq!(parse("\"café\"").unwrap().as_str().unwrap(), "café"); // raw UTF-8 passthrough
    }

    #[test]
    fn rejects_lone_surrogate() {
        assert!(matches!(parse(r#""\ud83d""#), Err(JsonError::UnpairedSurrogate(_))));
        assert!(matches!(parse(r#""\ud83dX""#), Err(JsonError::UnpairedSurrogate(_))));
    }

    #[test]
    fn rejects_raw_control_character_in_string() {
        let text = "\"a\u{0001}b\"";
        assert!(matches!(parse(text), Err(JsonError::ControlCharacterInString { .. })));
    }

    #[test]
    fn writer_round_trips_supplementary_plane_and_control_chars() {
        let value = Value::String("😀\u{0001}\t\"\\".to_string());
        let text = to_string(&value);
        assert_eq!(parse(&text).unwrap(), value);
    }
    //#endregion 🔖️Strings

    //#region 🔖️Containers
    #[test]
    fn parses_arrays_and_objects() {
        let value = parse(r#"{"a":1,"b":[1,2,3],"c":{"nested":true}}"#).unwrap();
        assert_eq!(value.get("a").unwrap().as_u64(), Some(1));
        assert_eq!(value.get("b").unwrap().as_array().unwrap().len(), 3);
        assert_eq!(value.get("c").unwrap().get("nested").unwrap().as_bool(), Some(true));
    }

    #[test]
    fn duplicate_object_keys_keep_first_position_last_value() {
        let value = parse(r#"{"a":1,"b":2,"a":3}"#).unwrap();
        let object = value.as_object().unwrap();
        assert_eq!(object.len(), 2);
        assert_eq!(object.get("a").unwrap().as_u64(), Some(3));
        assert_eq!(object.iter().next().unwrap().0, "a"); // first occurrence's position kept
    }

    #[test]
    fn empty_array_and_object() {
        assert_eq!(parse("[]").unwrap(), Value::Array(vec![]));
        assert_eq!(parse("{}").unwrap(), Value::Object(Object::new()));
        assert_eq!(to_string(&Value::Array(vec![])), "[]");
        assert_eq!(to_string(&Value::Object(Object::new())), "{}");
    }

    #[test]
    fn max_depth_is_enforced() {
        let mut text = String::new();
        for _ in 0..(MAX_DEPTH + 10) {
            text.push('[');
        }
        assert!(matches!(parse(&text), Err(JsonError::MaxDepthExceeded(_))));
    }
    //#endregion 🔖️Containers

    //#region 🔖️ToFromValueBridge
    /// 🌉️ `from_dsl_value`/`to_dsl_value` (`//#region 🔖️DslValueBridge` above) had no direct test
    /// yet — this exercises the structural walk this region's `to_json_string`/`from_json_str`
    /// are built on.
    #[test]
    fn from_dsl_value_and_to_dsl_value_round_trip_every_shape() {
        let value = DslValue::object([("a".to_string(), DslValue::Number(1.0)), ("b".to_string(), DslValue::Array(vec![DslValue::Bool(true), DslValue::Null, DslValue::String("x".to_string())]))]);
        assert_eq!(to_dsl_value(&from_dsl_value(&value)), value);
    }

    #[test]
    fn to_json_string_and_from_json_str_round_trip_a_dsl_value() {
        let value = DslValue::object([("count".to_string(), DslValue::Number(3.0)), ("label".to_string(), DslValue::String("ok".to_string()))]);
        let text = to_json_string(&value);
        let parsed: DslValue = from_json_str(&text).unwrap();
        assert_eq!(parsed, value);
    }

    #[test]
    fn from_json_str_reports_a_value_error_on_malformed_text() {
        assert!(from_json_str::<DslValue>("not json").is_err());
    }

    //#region 🔖️JsonMacro
    #[test]
    fn json_macro_builds_scalars_and_null() {
        assert_eq!(crate::json!(null), Value::Null);
        assert_eq!(crate::json!(true), Value::Bool(true));
        assert_eq!(crate::json!(false), Value::Bool(false));
        assert_eq!(crate::json!(1), Value::Number(Number::Int(1)));
        assert_eq!(crate::json!(1.5), Value::Number(Number::Float(1.5)));
        assert_eq!(crate::json!("hi"), Value::String("hi".to_string()));
    }

    #[test]
    fn json_macro_builds_arrays_incl_empty_and_nested() {
        assert_eq!(crate::json!([]), Value::Array(vec![]));
        assert_eq!(crate::json!([1, 2, 3]), Value::Array(vec![Value::Number(Number::Int(1)), Value::Number(Number::Int(2)), Value::Number(Number::Int(3))]));
        assert_eq!(crate::json!([[1], [2, 3]]), Value::Array(vec![Value::Array(vec![Value::Number(Number::Int(1))]), Value::Array(vec![Value::Number(Number::Int(2)), Value::Number(Number::Int(3))])]));
    }

    #[test]
    fn json_macro_builds_objects_incl_empty_and_trailing_commas() {
        assert_eq!(crate::json!({}), Value::Object(Object::new()));
        let value = crate::json!({
            "a": 1,
            "b": [1, 2],
        });
        assert_eq!(value.get("a").unwrap().as_i64(), Some(1));
        assert_eq!(value.get("b").unwrap().as_array().unwrap().len(), 2);
    }

    #[test]
    fn json_macro_evaluates_arbitrary_expressions_and_options() {
        let index = 3;
        let value = crate::json!({
            "id": format!("semio_text-{index}"),
            "meshId": "box",
            "position": [index as f64 * 2.0, 0.0, 0.0],
            "rotation": [0.0, 0.0, 0.0, 1.0],
            "scale": [1.0, 1.0, 1.0],
            "label": format!("Semio Text {index}"),
            "smoothShading": false,
            "nested": { "deep": { "deeper": [1, 2, 3] } },
            "present": Some(5),
            "absent": Option::<i32>::None,
        });
        assert_eq!(value.get("id").unwrap().as_str(), Some("semio_text-3"));
        assert_eq!(value.get("position").unwrap().as_array().unwrap()[0].as_f64(), Some(6.0));
        assert_eq!(value.get("nested").unwrap().get("deep").unwrap().get("deeper").unwrap().as_array().unwrap().len(), 3);
        assert_eq!(value.get("present").unwrap().as_i64(), Some(5));
        assert!(value.get("absent").unwrap().is_null());
    }

    #[test]
    fn json_macro_matches_to_string_of_equivalent_hand_built_value() {
        let via_macro = crate::json!({"a": 1, "b": [true, null, "x"]});
        let hand_built = Value::Object(Object::from_iter([("a".to_string(), Value::Number(Number::Int(1))), ("b".to_string(), Value::Array(vec![Value::Bool(true), Value::Null, Value::String("x".to_string())]))]));
        assert_eq!(to_string(&via_macro), to_string(&hand_built));
    }
    #[test]
    fn value_eq_ignoring_object_order_is_order_insensitive_but_still_structural() {
        let a = crate::json!({"x": 1, "y": [1, 2, {"p": true, "q": "s"}]});
        let b = crate::json!({"y": [1, 2, {"q": "s", "p": true}], "x": 1});
        assert!(value_eq_ignoring_object_order(&a, &b));
        let c = crate::json!({"x": 1, "y": [1, 2, {"p": true, "q": "different"}]});
        assert!(!value_eq_ignoring_object_order(&a, &c));
        let d = crate::json!({"x": 1});
        assert!(!value_eq_ignoring_object_order(&a, &d));
    }

    #[test]
    fn mutable_accessors_update_nested_members() {
        let mut value = crate::json!({ "items": [{ "state": "before" }] });
        value
            .get_mut("items")
            .and_then(Value::as_array_mut)
            .and_then(|items| items.first_mut())
            .and_then(Value::as_object_mut)
            .and_then(|item| item.get_mut("state"))
            .expect("nested state")
            .clone_from(&Value::String("after".to_string()));
        assert_eq!(to_string(&value), r#"{"items":[{"state":"after"}]}"#);
    }
    //#endregion 🔖️JsonMacro

    /// 🔬️ Differential (single-key object — see the module's own note above on key-order
    /// ambiguity): our bridge's bytes agree with the framework's existing `DslValue ->
    /// serde_json::Value` path (`🌱️value/🦀️component.rs`'s `impl From<DslValue> for
    /// serde_json::Value`), the oracle every framework-internal caller still speaks.
    #[test]
    fn to_json_string_bytes_match_the_serde_json_bridge() {
        let value = DslValue::object([("nested".to_string(), DslValue::Array(vec![DslValue::Number(1.0), DslValue::Number(2.5)]))]);
        let mine = to_json_string(&value);
        let theirs = serde_json::to_string(&serde_json::Value::from(value)).unwrap();
        assert_eq!(mine, theirs);
    }
    //#endregion 🔖️ToFromValueBridge

    //#region 🔖️PropertyTesting
    /// 🎲️ A tiny deterministic PRNG (SplitMix64) — property/differential tests need arbitrary
    /// `Value` trees, but adding a `rand`/`proptest`/`arbitrary` crate would itself be a NEW
    /// third-party dependency the freeze ratchet forbids; this is small enough to own outright.
    struct Rng(u64);

    impl Rng {
        fn new(seed: u64) -> Self {
            Self(seed)
        }

        fn next_u64(&mut self) -> u64 {
            self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = self.0;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        }

        fn range(&mut self, bound: u64) -> u64 {
            self.next_u64() % bound.max(1)
        }

        fn unit_f64(&mut self) -> f64 {
            (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
        }

        fn bool(&mut self) -> bool {
            self.next_u64() & 1 == 0
        }
    }

    fn arbitrary_finite_float(rng: &mut Rng) -> f64 {
        loop {
            let value = match rng.range(4) {
                0 => 0.0,
                1 => (rng.next_u64() as i64) as f64 / 1e3,
                2 => {
                    let exponent = rng.range(600) as i32 - 300;
                    let mantissa = 1.0 + rng.unit_f64();
                    mantissa * 10f64.powi(exponent)
                }
                _ => f64::from_bits(rng.next_u64()),
            };
            if value.is_finite() {
                return value;
            }
        }
    }

    fn arbitrary_string(rng: &mut Rng) -> String {
        let len = rng.range(6);
        let mut out = String::new();
        for _ in 0..len {
            let ch = match rng.range(11) {
                0 => '"',
                1 => '\\',
                2 => '\n',
                3 => '\t',
                4 => '\u{0000}',
                5 => '\u{001F}',
                6 => '€',
                7 => '😀',
                8 => '\u{0008}',
                9 => '\u{000C}',
                _ => char::from_u32(0x20 + rng.range(0x5E) as u32).unwrap_or('x'),
            };
            out.push(ch);
        }
        out
    }

    fn arbitrary_value(rng: &mut Rng, depth: u32) -> Value {
        let kind_bound = if depth >= 4 { 5 } else { 7 };
        match rng.range(kind_bound) {
            0 => Value::Null,
            1 => Value::Bool(rng.bool()),
            2 => Value::Number(Number::UInt(rng.range(1_000_000))),
            3 => Value::Number(Number::Int(rng.range(2_000_000) as i64 - 1_000_000)),
            4 => Value::Number(Number::Float(arbitrary_finite_float(rng))),
            5 => Value::String(arbitrary_string(rng)),
            6 => {
                let len = rng.range(4);
                Value::Array((0..len).map(|_| arbitrary_value(rng, depth + 1)).collect())
            }
            _ => {
                let len = rng.range(4);
                let mut object = Object::new();
                for index in 0..len {
                    object.insert(format!("k{index}_{}", rng.range(1000)), arbitrary_value(rng, depth + 1));
                }
                Value::Object(object)
            }
        }
    }

    const PROPERTY_TEST_ITERATIONS: u32 = 3000;

    #[test]
    fn round_trips_arbitrary_values() {
        let mut rng = Rng::new(0xC0FF_EE00_1234_5678);
        for case in 0..PROPERTY_TEST_ITERATIONS {
            let value = arbitrary_value(&mut rng, 0);
            let text = to_string(&value);
            let parsed = parse(&text).unwrap_or_else(|error| panic!("case {case}: parse failed: {error}; text={text}"));
            assert_eq!(value, parsed, "case {case}: round-trip mismatch; text={text}");
        }
    }
    //#endregion 🔖️PropertyTesting

    //#region 🔖️DifferentialTesting
    /// 🔬️ Structural equality between our `Value` and `serde_json::Value` — deliberately NOT a
    /// byte-for-byte text comparison: object key order and float notation (fixed vs exponential;
    /// this writer never emits scientific notation the way `serde_json`'s ryu-based writer does
    /// for extreme magnitudes) are both allowed to differ as long as the two trees denote the same
    /// value. Byte-for-byte agreement is checked separately, only where both writers are known to
    /// agree deterministically (`canonical_bytes_match_serde_json_for_typical_documents`).
    fn values_match(mine: &Value, theirs: &serde_json::Value) -> bool {
        match (mine, theirs) {
            (Value::Null, serde_json::Value::Null) => true,
            (Value::Bool(a), serde_json::Value::Bool(b)) => a == b,
            (Value::String(a), serde_json::Value::String(b)) => a == b,
            (Value::Number(a), serde_json::Value::Number(b)) => number_matches(a, b),
            (Value::Array(a), serde_json::Value::Array(b)) => a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| values_match(x, y)),
            (Value::Object(a), serde_json::Value::Object(b)) => a.len() == b.len() && a.iter().all(|(k, v)| b.get(k).is_some_and(|bv| values_match(v, bv))),
            _ => false,
        }
    }

    fn number_matches(mine: &Number, theirs: &serde_json::Number) -> bool {
        match *mine {
            Number::UInt(v) => theirs.as_u64() == Some(v) || theirs.as_f64() == Some(v as f64),
            Number::Int(v) => theirs.as_i64() == Some(v) || theirs.as_f64() == Some(v as f64),
            Number::Float(v) => theirs.as_f64().is_some_and(|t| t == v),
        }
    }

    #[test]
    fn differential_parse_matches_serde_json_on_arbitrary_values() {
        let mut rng = Rng::new(0xD1FF_0000_BEEF_CAFE);
        let mut checked = 0usize;
        for case in 0..PROPERTY_TEST_ITERATIONS {
            let value = arbitrary_value(&mut rng, 0);
            let text = to_string(&value);
            let mine = parse(&text).unwrap();
            let theirs: serde_json::Value = serde_json::from_str(&text).unwrap_or_else(|error| panic!("case {case}: serde_json rejected our own writer output: {error}; text={text}"));
            assert!(values_match(&mine, &theirs), "case {case}: structural mismatch; text={text}\nmine={mine:?}\ntheirs={theirs:?}");
            checked += 1;
        }
        eprintln!("[DEBUG] [differential] parse: {checked} generated documents matched serde_json");
    }

    #[test]
    fn differential_cross_parse_serde_json_writer_output() {
        let mut rng = Rng::new(0x5EED_1357_2468_ACE0);
        let mut checked = 0usize;
        for case in 0..PROPERTY_TEST_ITERATIONS {
            let value = arbitrary_value(&mut rng, 0);
            let theirs = to_serde_json(&value);
            let text = serde_json::to_string(&theirs).unwrap();
            let mine = parse(&text).unwrap_or_else(|error| panic!("case {case}: our parser rejected serde_json's writer output: {error}; text={text}"));
            assert!(values_match(&mine, &theirs), "case {case}: structural mismatch; text={text}");
            checked += 1;
        }
        eprintln!("[DEBUG] [differential] cross-parse: {checked} serde_json-written documents matched");
    }

    fn to_serde_json(value: &Value) -> serde_json::Value {
        match value {
            Value::Null => serde_json::Value::Null,
            Value::Bool(v) => serde_json::Value::Bool(*v),
            Value::String(v) => serde_json::Value::String(v.clone()),
            Value::Number(Number::UInt(v)) => serde_json::Value::Number((*v).into()),
            Value::Number(Number::Int(v)) => serde_json::Value::Number((*v).into()),
            Value::Number(Number::Float(v)) => serde_json::Number::from_f64(*v).map_or(serde_json::Value::Null, serde_json::Value::Number),
            Value::Array(items) => serde_json::Value::Array(items.iter().map(to_serde_json).collect()),
            Value::Object(object) => serde_json::Value::Object(object.iter().map(|(k, v)| (k.to_string(), to_serde_json(v))).collect()),
        }
    }

    /// 🔬️ On documents with no object-key-order ambiguity (scalars, arrays, single-key objects) and
    /// no extreme float magnitudes, our writer's bytes agree with `serde_json`'s exactly.
    #[test]
    fn canonical_bytes_match_serde_json_for_typical_documents() {
        let cases: &[&str] = &[r#"null"#, r#"true"#, r#"false"#, r#"0"#, r#"-17"#, r#"3.5"#, r#""hello""#, r#""café""#, r#"[]"#, r#"{}"#, r#"[1,2,3]"#, r#"{"a":1}"#, r#"{"only":{"one":"key"}}"#];
        for text in cases {
            let mine = parse(text).unwrap();
            let theirs: serde_json::Value = serde_json::from_str(text).unwrap();
            let mine_bytes = to_string(&mine);
            let their_bytes = serde_json::to_string(&theirs).unwrap();
            assert_eq!(mine_bytes, their_bytes, "byte mismatch for {text}");
        }
    }
    //#endregion 🔖️DifferentialTesting
}
//#endregion 🔖️Tests
