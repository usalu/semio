//! 🧾️ `pack_json` — an owned, spec-correct streaming JSON reader and writer: the replacement for
//! `serde_json` inside framework crates (see `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/
//! INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-9-RUNTIME-DEPENDENCY-REMOVAL/📓️p9b-owned-serialization.md`).
//!
//! Whole-document parsing drains the same retained [`JsonParseCursor`] used by interactive owners.
//! The cursor borrows unchanged source text on each grant and keeps partial candidate ownership;
//! [`JsonValueProjection`] moves that candidate into domain-neutral values without payload clones.
//! The token-at-a-time [`Lexer`] shares the same string and number grammar. Arbitrary-precision
//! integers are excluded (JSON numbers outside `[i64::MIN, u64::MAX]` fall back to `f64`, exactly like
//! `serde_json` without its `arbitrary_precision` feature — the only configuration this repo ever
//! built with).
//!
//! Number formatting is byte-identical to `serde_json`'s own (`zmij`-based) writer for every
//! `f64`: the fixed/exponential split follows `zmij`'s rule (`-5 <= exponent <= 15` stays fixed),
//! not ECMA-262's `Number::toString` (`-6 <= e < 21`) — the two differ, and only `zmij`'s matches
//! this repo's actual `serde_json` dependency — and the digits themselves are recomputed by exact
//! round-half-to-even arithmetic so a genuine last-digit tie always resolves the same way `zmij`
//! resolves it (proof: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/
//! RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/🔍️research/📓️float-format-parity.md`).
//! A float and an integer of the same magnitude are still never spelled the same way (`42.0`
//! always keeps its `.0`). NaN/±Infinity — not representable in JSON — encode as `null`, matching
//! `serde_json`'s own behaviour (verified by the differential tests below).

use std::fmt;

use semio_framework_value::{DslValue, FromValue, ValueError, ValueRefusalKind};
pub use semio_framework_value::ToValue;

#[path = "🧩️members/🦀️.rs"]
mod members;
pub use members::JsonMemberPolicy;

//#region 🔖️Errors
/// 🚨️ Every parse failure this crate can produce, with a byte offset into the input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsonError {
    Native(ValueError),
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
    DuplicateMember { name: String, offset: usize },
}

impl fmt::Display for JsonError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Native(error) => error.fmt(formatter),
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
            Self::DuplicateMember { name, offset } => write!(formatter, "duplicate member {name:?} at offset {offset}"),
        }
    }
}

impl std::error::Error for JsonError {}
impl From<ValueError> for JsonError {fn from(error:ValueError)->Self {Self::Native(error)}}

impl JsonError {
    /// 🧭️ Retains parse semantics at the owned value refusal boundary.
    pub const fn kind(&self) -> ValueRefusalKind { match self { Self::Native(error) => error.kind, Self::MaxDepthExceeded(_) => ValueRefusalKind::DepthLimit, _ => ValueRefusalKind::InvalidValue } }
    /// 🌱️ Carries parse refusal into typed native construction.
    pub fn into_value_error(self) -> ValueError { match self { Self::Native(error)=>error, error=>ValueError::new(error.kind(),error.to_string()) } }
}

/// 🛡️ Recursion ceiling for nested arrays/objects — matches `serde_json`'s own default
/// (128), the value this repo's fixtures were authored against.
pub const MAX_DEPTH: u32 = 128;
//#endregion 🔖️Errors

//#region 🔖️Number
/// 🔢️ A JSON number, keeping the writer's-eye distinction JSON itself does not: an integer
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
/// 🗂️ An insertion-order-preserving JSON object. Re-inserting an existing key overwrites
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

    /// 🗑️ Removes `key` and returns its value, preserving the insertion order of the rest.
    ///
    /// Mirrors `serde_json::Map::remove` so a recursive key-at-a-time retirement walk can be
    /// expressed against this type without reaching for serde_json.
    pub fn remove(&mut self, key: &str) -> Option<Value> {
        let index = self.0.iter().position(|(name, _)| name == key)?;
        Some(self.0.remove(index).1)
    }

    /// ✏️ Mutable iteration in insertion order, for in-place rewrites of nested values.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&str, &mut Value)> {
        self.0.iter_mut().map(|(name, value)| (name.as_str(), value))
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &Value)> {
        self.0.iter().map(|(k, v)| (k.as_str(), v))
    }

    /// 🫴️ Transfers the actual owned member storage in insertion order without copying keys or descendants.
    pub fn into_entries(self) -> Vec<(String, Value)> {
        self.0
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

/// 🌳️ An owned JSON value tree — the `serde_json::Value` replacement. Every framework
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
/// 🌉️ Structural conversion from `semio_framework_value::DslValue` (the in-memory tree
/// `ToValue`/`FromValue` target onto — see `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/
/// RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/🔍️research/
/// 📓️serde-replacement-surface.md` §"pack::json::Value ↔ DslValue conversion") into this crate's
/// own JSON-text-oriented [`Value`] — the two are sibling shapes with no shared type, so a
/// `Mutation`/`MutationDiff` payload that needs literal JSON **text** (a wire byte string, not
/// just an in-memory value) walks through here on the way to [`to_string`]/[`parse`].
/// Maps `semio_framework_value::Number`'s `UInt`/`Int`/`Float` variants onto this crate's own
/// identically-shaped [`Number`] one-for-one — an integer stays an integer across the bridge
/// instead of being widened to `f64` and printed back with a spurious `.0`.
pub fn from_dsl_value(value: &DslValue) -> Value {
    match value {
        DslValue::Null => Value::Null,
        DslValue::Bool(b) => Value::Bool(*b),
        DslValue::Number(n) => Value::Number(match n {
            semio_framework_value::Number::UInt(value) => Number::UInt(*value),
            semio_framework_value::Number::Int(value) => Number::Int(*value),
            semio_framework_value::Number::Float(value) => Number::Float(*value),
        }),
        DslValue::String(s) => Value::String(s.clone()),
        DslValue::Bytes(bytes) => Value::Array(bytes.iter().map(|byte|Value::Number(Number::UInt(u64::from(*byte)))).collect()),
        DslValue::Array(items) => Value::Array(items.iter().map(from_dsl_value).collect()),
        DslValue::Object(entries) => Value::Object(entries.iter().map(|(key, value)| (key.clone(), from_dsl_value(value))).collect()),
    }
}

/// 🌉️ The reverse of [`from_dsl_value`] — maps numbers variant-for-variant, so an integer stays an
/// integer across the bridge instead of being widened to `f64` and printed back as `1.0`.
pub fn to_dsl_value(value: &Value) -> DslValue {
    match value {
        Value::Null => DslValue::Null,
        Value::Bool(b) => DslValue::Bool(*b),
        Value::Number(n) => DslValue::Number(match n {
            Number::UInt(value) => semio_framework_value::Number::UInt(*value),
            Number::Int(value) => semio_framework_value::Number::Int(*value),
            Number::Float(value) => semio_framework_value::Number::Float(*value),
        }),
        Value::String(s) => DslValue::String(s.clone()),
        Value::Array(items) => DslValue::Array(items.iter().map(to_dsl_value).collect()),
        Value::Object(entries) => DslValue::object(entries.iter().map(|(key, value)| (key.to_string(), to_dsl_value(value)))),
    }
}
impl ToValue for Value {
    fn to_value(&self) -> DslValue {
        to_dsl_value(self)
    }
}

impl FromValue for Value {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        Ok(from_dsl_value(&value))
    }
}

impl ToValue for Object {
    fn to_value(&self) -> DslValue {
        DslValue::object(self.iter().map(|(key, value)| (key.to_string(), value.to_value())))
    }
}
//#endregion 🔖️DslValueBridge

#[path = "📥️decode/🫳️borrowed/🦀️.rs"]
mod borrowed_read_source;
pub use borrowed_read_source::{JsonReadSource,JsonBorrowedParseCursor,JsonBorrowedDslCursor,JsonParsedValue,JsonReadLimits,JsonSourceCursor};

//#region 🔖️Lexer
/// 🪙️ One structural token — the streaming layer everything else is built on. A future
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

/// 🔤️ Token-at-a-time reader over a `&str` — zero-copy for structural bytes, allocating
/// only for string/number token payloads.
pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

#[derive(Clone,Copy)]
struct NumberScan {
    start:usize,position:usize,state:u8,negative:bool,floating:bool,unsigned:Option<u64>,
    integer_digits:i64,leading:i64,significant:usize,significant_start:usize,significant_end:usize,sticky:bool,
    exponent:i64,exponent_negative:bool,
}
impl NumberScan {
    fn new(start:usize)->Self {Self {start,position:start,state:0,negative:false,floating:false,unsigned:Some(0),integer_digits:0,leading:0,significant:0,significant_start:start,significant_end:start,sticky:false,exponent:0,exponent_negative:false}}
    fn digit(&mut self,byte:u8,integer:bool) {
        if integer {self.integer_digits+=1;self.unsigned=self.unsigned.and_then(|value|value.checked_mul(10)?.checked_add(u64::from(byte-b'0')));}
        if self.significant==0 && byte==b'0' {self.leading+=1;}
        else {if self.significant==0 {self.significant_start=self.position;}self.significant+=1;if self.significant<=1152 {self.significant_end=self.position+1;}else {self.sticky|=byte!=b'0';}}
        self.position+=1;
    }
    fn finish<S:JsonReadSource+?Sized>(&self,input:&S)->Result<Number,JsonError> {
        if !self.floating {if let Some(value)=self.unsigned {if !self.negative {return Ok(Number::UInt(value));}if value<=i64::MAX as u64 {return Ok(Number::Int(-(value as i64)));}if value==i64::MAX as u64+1 {return Ok(Number::Int(i64::MIN));}}}
        let length=self.position-self.start;
        let value=if length<=1200{
            let mut text=borrowed_read_source::NumberText::new();
            for position in self.start..self.position{text.push(input.byte_at(position).ok_or(JsonError::InvalidNumber(self.start))?)?;}
            text.text()?.parse::<f64>()
        }else if self.significant==0{Ok(if self.negative{-0.0}else{0.0})}
        else{
            let mut normalized=borrowed_read_source::NumberText::new();if self.negative{normalized.push(b'-')?;}
            let mut first=true;
            for position in self.significant_start..self.significant_end{
                let digit=input.byte_at(position).ok_or(JsonError::InvalidNumber(self.start))?;if digit==b'.'{continue;}
                normalized.push(digit)?;if first{normalized.push(b'.')?;first=false;}
            }
            if self.sticky{normalized.push(b'1')?;}normalized.push(b'e')?;
            let exponent=if self.exponent_negative{-self.exponent}else{self.exponent};
            std::fmt::Write::write_fmt(&mut normalized,format_args!("{}",self.integer_digits.saturating_sub(self.leading).saturating_sub(1).saturating_add(exponent))).map_err(|_|JsonError::InvalidNumber(self.start))?;
            normalized.text()?.parse::<f64>()
        }.map_err(|_|JsonError::InvalidNumber(self.start))?;
        if !value.is_finite() {return Err(JsonError::InvalidNumber(self.start));}Ok(Number::Float(value))
    }
    fn step<S:JsonReadSource+?Sized>(&mut self,input:&S)->Result<Option<Number>,JsonError> {
        let byte=input.byte_at(self.position);
        match self.state {
            0=>{self.state=1;if byte==Some(b'-') {self.negative=true;self.position+=1;return Ok(None);}},
            1=>{},
            2=>{if let Some(byte @ b'0'..=b'9')=byte {self.digit(byte,true);return Ok(None);}},
            3=>{},
            4=>{let Some(byte @ b'0'..=b'9')=byte else {return Err(JsonError::InvalidNumber(self.start))};self.digit(byte,false);self.state=5;return Ok(None);},
            5=>{if let Some(byte @ b'0'..=b'9')=byte {self.digit(byte,false);return Ok(None);}},
            6=>{self.state=7;if matches!(byte,Some(b'+'|b'-')) {self.exponent_negative=byte==Some(b'-');self.position+=1;return Ok(None);}if !matches!(byte,Some(b'0'..=b'9')) {return Err(JsonError::InvalidNumber(self.start));}self.state=8;},
            7=>{if !matches!(byte,Some(b'0'..=b'9')) {return Err(JsonError::InvalidNumber(self.start));}self.state=8;},
            8=>{},
            _=>unreachable!(),
        }
        if self.state==1 {let Some(byte @ b'0'..=b'9')=byte else {return Err(JsonError::InvalidNumber(self.start))};self.state=if byte==b'0' {3}else {2};self.digit(byte,true);return Ok(None);}
        if self.state==8 {if let Some(byte @ b'0'..=b'9')=byte {self.exponent=(self.exponent.saturating_mul(10)+i64::from(byte-b'0')).min(i64::try_from(input.byte_len()).unwrap_or(i64::MAX-400).saturating_add(400));self.position+=1;return Ok(None);}return self.finish(input).map(Some);}
        if matches!(self.state,2|3) && byte==Some(b'.') {self.floating=true;self.state=4;self.position+=1;return Ok(None);}
        if matches!(self.state,2|3|5) && matches!(byte,Some(b'e'|b'E')) {self.floating=true;self.state=6;self.position+=1;return Ok(None);}
        self.finish(input).map(Some)
    }
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
        let mut number = NumberScan::new(self.pos);
        loop { if let Some(value) = number.step(self.input)? { self.pos = number.position; return Ok(Token::Number(value)); } }
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
        self.pos += 1; let mut output = String::new();
        while let Some(character) = json_character(self)? { output.push(character); }
        Ok(output)
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
pub fn parse(input: &str, policy: JsonMemberPolicy) -> Result<Value, JsonError> {
    let mut cursor = JsonParseCursor::new(policy);
    let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(usize::MAX,&mut accepted);
    loop { if let Some(value) = cursor.step(input, 4096,&mut control)? { return Ok(value); } }
}

/// 🌳️ [`parse`] over raw bytes — errors with [`JsonError::InvalidUtf8`] if `input` is not UTF-8.
pub fn parse_bytes(input: &[u8], policy: JsonMemberPolicy) -> Result<Value, JsonError> {
    let text = std::str::from_utf8(input).map_err(|_| JsonError::InvalidUtf8)?;
    parse(text, policy)
}

type JsonCandidates<T> = semio_framework_value::list::PagedList<T,{usize::MAX}>;
struct JsonFrame<V:JsonParsedValue> {
    object: bool, state: u8, item_count:u64, values: JsonCandidates<V>, entries: JsonCandidates<(String,V)>,
    key: Option<String>, key_offset: usize, probe: usize, compare: usize, duplicate: Option<usize>,
    array:Vec<V>, members:Vec<(String,V)>, admitted:bool, reverse:usize,
}
struct JsonStringScan {start:usize,position:usize,bytes:usize,output:String,writing:bool,admitted:bool}
enum JsonLexeme { String(JsonStringScan), Number(NumberScan) }

fn json_candidate_slot<T>(owner:&mut JsonCandidates<T>,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<bool,JsonError> {
    if owner.has_reserved_slot() {return Ok(true);}
    let bytes=owner.next_allocation_bytes().map_err(ValueError::from)?;control.charge(bytes)?;
    owner.reserve_one(bytes).map_err(|error|ValueError::from(error.refusal()))?;Ok(false)
}

/// 🧵️ Retains the canonical JSON grammar and admitted candidates while borrowing unchanged source.
pub type JsonParseCursor=JsonGrammarCursor<Value>;

/// 🌳️ One retained grammar moves admitted semantic cells directly into its declared first-party output.
pub struct JsonGrammarCursor<V:JsonParsedValue> {
    position: usize, validated_position:usize, limits:JsonReadLimits, policy: JsonMemberPolicy, frames: Vec<JsonFrame<V>>, lexeme: Option<JsonLexeme>,
    pending: Option<V>, result: Option<V>, retired: JsonCandidates<V>, obsolete:Option<V>, complete: bool,
}
impl<V:JsonParsedValue> JsonGrammarCursor<V> {
    /// 🌱️ Starts parsing without copying, scanning, or allocating for the source.
    pub fn new(policy: JsonMemberPolicy) -> Self { Self { position:0, validated_position:0, limits:JsonReadLimits{maximum_bytes:u64::MAX,maximum_allocation_bytes:usize::MAX,maximum_depth:MAX_DEPTH as usize,maximum_items:u64::MAX}, policy, frames:Vec::new(), lexeme:None, pending:None, result:None, retired:Default::default(),obsolete:None,complete:false } }
    /// 📍️ Returns the measured source byte offset.
    pub fn position(&self) -> usize { self.position }
    /// 🧭️ Identifies the existing grammar or physical candidate frontier.
    pub fn phase(&self)->&'static str {match &self.lexeme {Some(JsonLexeme::String(scan))=>if scan.writing {"materialize-string"}else {"measure-string"},Some(JsonLexeme::Number(_))=>"number",None=>match self.frames.last(){Some(frame) if frame.state==8=>if frame.object {"materialize-object"}else {"materialize-array"},Some(frame)=>if frame.object {"collect-object"}else {"collect-array"},None=>"grammar"}}}
    /// ⏱️ Advances admitted grammar transitions under this operation's cumulative decode authority.
    pub fn step(&mut self,input:&str,maximum_units:usize,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Option<V>,JsonError>{
        self.validated_position=input.len();
        self.step_source(input,maximum_units,control)
    }
    fn step_source<S:JsonReadSource+?Sized>(&mut self,input:&S,maximum_units:usize,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Option<V>,JsonError>{
        control.scoped_maximum(self.limits.maximum_allocation_bytes,|control|{
        for _ in 0..maximum_units{
            control.checkpoint()?;
            if self.validated_position<input.byte_len(){
                let character=borrowed_read_source::character(input,self.validated_position)?;
                self.validated_position+=character.len_utf8();control.step()?;continue;
            }
            if self.complete{return Ok(self.result.take());}
            self.advance(input,control)?;control.step()?;
            if self.complete{return Ok(self.result.take());}
        }
        Ok(None)
        })
    }
    fn advance<S:JsonReadSource+?Sized>(&mut self,input:&S,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),JsonError> {
        if self.obsolete.is_some() {
            if !json_candidate_slot(&mut self.retired,control)? {return Ok(());}
            self.retired.push_reserved(self.obsolete.take().unwrap()).unwrap_or_else(|_|unreachable!());return Ok(());
        }
        if self.frames.last().is_some_and(|frame|frame.state==8) {return self.materialize_frame(control);}
        if self.pending.is_some() {
            if let Some(frame)=self.frames.last_mut() {
                if frame.object {
                    if let Some(index)=frame.duplicate {
                        if !json_candidate_slot(&mut self.retired,control)? {return Ok(());}
                        let old=std::mem::replace(&mut frame.entries.get_mut(index).unwrap().1,self.pending.take().unwrap());self.retired.push_reserved(old).unwrap_or_else(|_|unreachable!());self.obsolete=frame.key.take().map(V::json_string);frame.duplicate=None;
                    } else {
                        if !json_candidate_slot(&mut frame.entries,control)? {return Ok(());}
                        frame.entries.push_reserved((frame.key.take().unwrap(),self.pending.take().unwrap())).unwrap_or_else(|_|unreachable!());
                    }
                    frame.state=5;
                } else {
                    if !json_candidate_slot(&mut frame.values,control)? {return Ok(());}
                    frame.values.push_reserved(self.pending.take().unwrap()).unwrap_or_else(|_|unreachable!());frame.state=1;
                }
            } else {self.result=self.pending.take();}
            return Ok(());
        }
        if let Some(lexeme)=&mut self.lexeme {
            match lexeme {
                JsonLexeme::String(scan)=>{
                    if scan.writing && !scan.admitted {control.charge(scan.bytes)?;scan.output.try_reserve_exact(scan.bytes).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"JSON string allocation failed"))?;scan.admitted=true;return Ok(());}
                    let mut position=if scan.writing{scan.position}else{self.position};
                    let character=borrowed_read_source::json_character(input,&mut position)?;
                    if scan.writing{scan.position=position;}else{self.position=position;}
                    if let Some(character)=character {
                        if scan.writing {if scan.output.len()+character.len_utf8()>scan.bytes{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"JSON string changed after admission").into());}scan.output.push(character);}else {scan.bytes=scan.bytes.checked_add(character.len_utf8()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"JSON string size overflow"))?;}
                        return Ok(());
                    }
                    if !scan.writing {scan.writing=true;scan.position=scan.start;return Ok(());}
                    if scan.output.len()!=scan.bytes {return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"JSON string changed after measurement").into());}
                    let Some(JsonLexeme::String(scan))=self.lexeme.take() else {unreachable!()};let text=scan.output;
                    if let Some(frame)=self.frames.last_mut().filter(|frame|frame.object && matches!(frame.state,0|2)) {frame.key=Some(text);frame.state=6;frame.probe=0;frame.compare=0;frame.duplicate=None;}
                    else {self.pending=Some(V::json_string(text));}
                },
                JsonLexeme::Number(number)=>{
                    if let Some(value)=number.step(input)? {self.position=number.position;self.lexeme=None;self.pending=Some(V::json_number(value));}
                    else {self.position=number.position;}
                },
            }
            return Ok(());
        }
        if let Some(frame)=self.frames.last_mut().filter(|frame|frame.state==6) {
            if frame.probe==frame.entries.len() {frame.state=3;return Ok(());}
            let name=&frame.entries.get(frame.probe).unwrap().0;let key=frame.key.as_ref().unwrap();
            if name.len()!=key.len() || name.as_bytes().get(frame.compare)!=key.as_bytes().get(frame.compare) {frame.probe+=1;frame.compare=0;return Ok(());}
            frame.compare+=1;
            if frame.compare>key.len() {
                if self.policy==JsonMemberPolicy::Reject {return Err(JsonError::DuplicateMember {name:frame.key.take().unwrap(),offset:frame.key_offset});}
                frame.duplicate=Some(frame.probe);frame.state=3;
            }
            return Ok(());
        }
        let byte=input.byte_at(self.position);
        if matches!(byte,Some(b' '|b'\t'|b'\n'|b'\r')) {self.position+=1;return Ok(());}
        if self.result.is_some() {if byte.is_some() {return Err(JsonError::TrailingData(self.position));}self.complete=true;return Ok(());}
        let error=||JsonError::UnexpectedByte {found:byte.unwrap_or(0),offset:self.position};
        let Some(byte)=byte else {return Err(JsonError::UnexpectedEof)};
        if let Some(frame)=self.frames.last_mut() {
            if frame.object {
                match frame.state {
                    0|2=>{if byte==b'}' && frame.state==0 {self.close_frame();return Ok(());}if byte!=b'"' {return Err(error());}frame.key_offset=self.position;},
                    3=>{if byte!=b':' {return Err(error());}frame.state=4;self.position+=1;return Ok(());},
                    5=>{if byte==b'}' {self.close_frame();return Ok(());}if byte!=b',' {return Err(error());}frame.state=2;self.position+=1;return Ok(());},
                    _=>{},
                }
            } else {
                if frame.state==0 && byte==b']' {self.close_frame();return Ok(());}
                if frame.state==1 {if byte==b']' {self.close_frame();return Ok(());}if byte!=b',' {return Err(error());}frame.state=2;self.position+=1;return Ok(());}
            }
        }
        if self.frames.len()>self.limits.maximum_depth.min(MAX_DEPTH as usize){return Err(JsonError::MaxDepthExceeded(self.limits.maximum_depth.min(MAX_DEPTH as usize)as u32));}
        let declared_item=self.frames.len().checked_sub(1).filter(|index|matches!(self.frames[*index].state,0|2));
        if let Some(index)=declared_item{if self.frames[index].item_count>=self.limits.maximum_items{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"JSON declared collection extent exceeds caller limit").into());}}
        match byte {
            b'"'=>{self.position+=1;self.lexeme=Some(JsonLexeme::String(JsonStringScan {start:self.position,position:self.position,bytes:0,output:String::new(),writing:false,admitted:false}));},
            b'-'|b'0'..=b'9'=>self.lexeme=Some(JsonLexeme::Number(NumberScan::new(self.position))),
            b'{'|b'['=>{
                if self.frames.capacity()==0 {self.frames=control.allocate_vec(self.limits.maximum_depth.min(MAX_DEPTH as usize)+2)?;return Ok(());}
                self.frames.push(JsonFrame {object:byte==b'{',state:0,item_count:0,values:Default::default(),entries:Default::default(),key:None,key_offset:0,probe:0,compare:0,duplicate:None,array:Vec::new(),members:Vec::new(),admitted:false,reverse:0});self.position+=1;
            },
            b't'|b'f'|b'n'=>{let (text,value)=match byte {b't'=>("true",V::json_bool(true)),b'f'=>("false",V::json_bool(false)),_=>("null",V::json_null())};if !borrowed_read_source::starts_with(input,self.position,text) {return Err(error());}self.position+=text.len();self.pending=Some(value);},
            _=>return Err(error()),
        }
        if let Some(index)=declared_item{self.frames[index].item_count+=1;}
        Ok(())
    }
    fn close_frame(&mut self) {self.frames.last_mut().unwrap().state=8;self.position+=1;}
    fn materialize_frame(&mut self,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),JsonError> {
        let frame=self.frames.last_mut().unwrap();
        if !frame.admitted {if frame.object {frame.members=control.allocate_vec(frame.entries.len())?;}else {frame.array=control.allocate_vec(frame.values.len())?;}frame.admitted=true;return Ok(());}
        if frame.object {if let Some(entry)=frame.entries.pop(){frame.members.push(entry);return Ok(());}}else if let Some(value)=frame.values.pop(){frame.array.push(value);return Ok(());}
        if !frame.entries.terminal_is_empty() {frame.entries.release_empty_page(usize::MAX).map_err(ValueError::from)?;return Ok(());}
        if !frame.values.terminal_is_empty() {frame.values.release_empty_page(usize::MAX).map_err(ValueError::from)?;return Ok(());}
        let length=if frame.object {frame.members.len()}else {frame.array.len()};
        if frame.reverse<length/2 {let opposite=length-1-frame.reverse;if frame.object {frame.members.swap(frame.reverse,opposite);}else {frame.array.swap(frame.reverse,opposite);}frame.reverse+=1;return Ok(());}
        let frame=self.frames.pop().unwrap();self.pending=Some(if frame.object {V::json_object(frame.members)}else {V::json_array(frame.array)});Ok(())
    }
}

impl semio_framework_value::retirement::RetireOwned for Number {fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::retirement::leaf(self)}}
impl semio_framework_value::retirement::RetireOwned for Object {fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::retirement::RetireOwned::retirement(self.0)}}
impl semio_framework_value::retirement::RetireOwned for Value {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{use semio_framework_value::retirement::*;match self {Self::String(value)=>value.retirement(),Self::Array(value)=>value.retirement(),Self::Object(value)=>value.retirement(),Self::Number(value)=>leaf(value),Self::Bool(value)=>leaf(value),Self::Null=>leaf(())}}
}
impl<V:JsonParsedValue> semio_framework_value::retirement::RetireOwned for JsonFrame<V> {fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::artifact_retirement_sequence!(self.values,self.entries,self.key,self.array,self.members)}}
impl semio_framework_value::retirement::RetireOwned for JsonLexeme {fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{use semio_framework_value::retirement::*;match self {Self::String(value)=>value.output.retirement(),Self::Number(value)=>leaf(value)}}}
impl<V:JsonParsedValue> semio_framework_value::retirement::RetireOwned for JsonGrammarCursor<V> {fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::artifact_retirement_sequence!(self.frames,self.lexeme,self.pending,self.result,self.retired,self.obsolete)}}

struct JsonMemberOrder {phase:u8,build:usize,end:usize,root:usize,child:usize,offset:usize,continuation:u8}
impl JsonMemberOrder {
    fn new(length:usize)->Self {Self {phase:0,build:length/2,end:length,root:0,child:0,offset:0,continuation:1}}
    fn compare(&mut self,entries:&[(String,DslValue)],left:usize,right:usize)->Option<std::cmp::Ordering> {let left=entries[left].0.as_bytes().get(self.offset);let right=entries[right].0.as_bytes().get(self.offset);if left!=right || left.is_none() {self.offset=0;Some(left.cmp(&right))}else {self.offset+=1;None}}
    fn step(&mut self,entries:&mut [(String,DslValue)])->bool {
        match self.phase {
            0=>{if self.end<2 {return true;}self.phase=1;},
            1=>{if self.build>0 {self.build-=1;self.root=self.build;self.continuation=1;self.phase=2;}else {self.phase=5;}},
            2=>{self.child=self.root.saturating_mul(2).saturating_add(1);if self.child>=self.end {self.phase=self.continuation;}else {self.offset=0;self.phase=if self.child+1<self.end {3}else {4};}},
            3=>{if let Some(order)=self.compare(entries,self.child+1,self.child) {if order==std::cmp::Ordering::Greater {self.child+=1;}self.phase=4;}},
            4=>{if let Some(order)=self.compare(entries,self.child,self.root) {if order==std::cmp::Ordering::Greater {entries.swap(self.child,self.root);self.root=self.child;self.phase=2;}else {self.phase=self.continuation;}}},
            _=>{if self.end<=1 {return true;}self.end-=1;entries.swap(0,self.end);self.root=0;self.continuation=5;self.phase=2;},
        }
        false
    }
}
struct JsonProjectionIterator<T> {values:std::vec::IntoIter<T>,allocation:usize}
impl<T> JsonProjectionIterator<T> {fn new(values:Vec<T>)->Self {let allocation=values.capacity().saturating_mul(std::mem::size_of::<T>());Self {values:values.into_iter(),allocation}}fn next(&mut self)->Option<T>{self.values.next()}fn len(&self)->usize{self.values.len()}}
struct JsonProjectionIteratorRetirement<T:semio_framework_value::retirement::RetireOwned> {values:std::mem::ManuallyDrop<std::vec::IntoIter<T>>,remaining:usize,released:bool}
impl<T:semio_framework_value::retirement::RetireOwned> semio_framework_value::retirement::RetirementCursor for JsonProjectionIteratorRetirement<T> {
    fn close_step(&mut self,maximum_bytes:usize)->semio_framework_value::retirement::RetirementStep {
        use semio_framework_value::retirement::RetirementStep;
        if self.released{return RetirementStep::Complete;}if maximum_bytes==0{return RetirementStep::BudgetExhausted;}
        if let Some(value)=self.values.next_back(){return RetirementStep::Child(value.retirement());}
        if self.remaining>0{let bytes=maximum_bytes.min(self.remaining);self.remaining-=bytes;return RetirementStep::Bytes(bytes);}
        unsafe{std::mem::ManuallyDrop::drop(&mut self.values)};self.released=true;RetirementStep::Complete
    }
    fn terminal_is_empty(&self)->bool{self.released}
}
impl<T:semio_framework_value::retirement::RetireOwned> Drop for JsonProjectionIteratorRetirement<T> {fn drop(&mut self){assert!(std::thread::panicking()||self.released,"JSON projection backing retired before terminal-empty");}}
impl<T:semio_framework_value::retirement::RetireOwned> semio_framework_value::retirement::RetireOwned for JsonProjectionIterator<T> {fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{Box::new(JsonProjectionIteratorRetirement {values:std::mem::ManuallyDrop::new(self.values),remaining:self.allocation,released:false})}}
struct JsonProjectionFrame {values:JsonProjectionIterator<Value>,entries:JsonProjectionIterator<(String,Value)>,array:Vec<DslValue>,object:Vec<(String,DslValue)>,key:Option<String>,is_object:bool,order:Option<JsonMemberOrder>,admitted:bool}
/// 🎒️ Moves a parsed JSON candidate into admitted canonical values without payload clones.
pub struct JsonValueProjection {pending:Option<Value>,output:Option<DslValue>,frames:Vec<JsonProjectionFrame>,ordered:bool,retirement:Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>}
impl JsonValueProjection {
    /// 🌱️ Takes ownership of the existing parsed candidate.
    pub fn new(value:Value)->Self {Self {pending:Some(value),output:None,frames:Vec::new(),ordered:false,retirement:None}}
    /// 🧬️ Projects the same candidate with canonical member order for dependency identity.
    pub fn new_ordered(value:Value)->Self {let mut cursor=Self::new(value);cursor.ordered=true;cursor}
    /// 🧭️ Exposes the current admitted value or consumed input frontier.
    pub fn phase(&self)->&'static str {if self.retirement.is_some(){"project-retire"}else if self.frames.last().is_some_and(|frame|!frame.admitted)||self.pending.as_ref().is_some_and(|value|matches!(value,Value::Array(_)|Value::Object(_))){"project-admit"}else{"project"}}
    /// ⏱️ Moves admitted values and drains consumed input under the same work and decode controls.
    pub fn step(&mut self,maximum_units:usize,maximum_bytes:usize,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Option<DslValue>,ValueError> {
        if maximum_bytes==0{return Ok(None);}
        for _ in 0..maximum_units {
            control.checkpoint()?;control.step()?;
            if let Some(retirement)=&mut self.retirement {retirement.close_step(1,maximum_bytes)?;if retirement.terminal_is_empty(){self.retirement=None;}continue;}
            if let Some(frame)=self.frames.last_mut().filter(|frame|!frame.admitted){if frame.is_object{frame.object=control.allocate_vec(frame.entries.len())?;}else{frame.array=control.allocate_vec(frame.values.len())?;}frame.admitted=true;continue;}
            if self.pending.as_ref().is_some_and(|value|matches!(value,Value::Array(_)|Value::Object(_)))&&self.frames.capacity()==0{self.frames=control.allocate_vec(MAX_DEPTH as usize+2)?;continue;}
            if let Some(value)=self.output.take() {if let Some(frame)=self.frames.last_mut() {if frame.is_object {frame.object.push((frame.key.take().unwrap(),value));}else {frame.array.push(value);}}else {return Ok(Some(value));}}
            else if self.pending.is_some() {
                if self.frames.len()>MAX_DEPTH as usize{return Err(ValueError::new(ValueRefusalKind::DepthLimit,"JSON projection exceeds nesting limit"));}
                let value=self.pending.take().unwrap();self.output=match value {
                    Value::Null=>Some(DslValue::Null),Value::Bool(value)=>Some(DslValue::Bool(value)),Value::String(value)=>Some(DslValue::String(value)),
                    Value::Number(value)=>Some(DslValue::Number(match value {Number::UInt(value)=>semio_framework_value::Number::UInt(value),Number::Int(value)=>semio_framework_value::Number::Int(value),Number::Float(value)=>semio_framework_value::Number::Float(value)})),
                    Value::Array(values)=>{self.frames.push(JsonProjectionFrame {values:JsonProjectionIterator::new(values),entries:JsonProjectionIterator::new(Vec::new()),array:Vec::new(),object:Vec::new(),key:None,is_object:false,order:None,admitted:false});None},
                    Value::Object(object)=>{self.frames.push(JsonProjectionFrame {values:JsonProjectionIterator::new(Vec::new()),entries:JsonProjectionIterator::new(object.0),array:Vec::new(),object:Vec::new(),key:None,is_object:true,order:None,admitted:false});None},
                };
            } else if let Some(frame)=self.frames.last_mut() {
                if frame.is_object {if let Some((key,value))=frame.entries.next() {frame.key=Some(key);self.pending=Some(value);continue;}if self.ordered {let order=frame.order.get_or_insert_with(||JsonMemberOrder::new(frame.object.len()));if !order.step(&mut frame.object) {continue;}}}
                else if let Some(value)=frame.values.next() {self.pending=Some(value);continue;}
                let mut frame=self.frames.pop().unwrap();self.output=Some(if frame.is_object {DslValue::Object(std::mem::take(&mut frame.object))}else {DslValue::Array(std::mem::take(&mut frame.array))});self.retirement=Some(semio_framework_value::retirement::owned_retirement(frame));
            } else {return Ok(None);}
        }
        Ok(None)
    }
}
impl semio_framework_value::retirement::RetireOwned for JsonProjectionFrame {fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::artifact_retirement_sequence!(self.values,self.entries,self.array,self.object,self.key)}}
impl semio_framework_value::retirement::RetireOwned for JsonValueProjection {fn retirement(mut self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{use semio_framework_value::retirement::*;let residual=self.retirement.take().map(erased_cursor);let children=semio_framework_value::artifact_retirement_sequence!(self.pending,self.output,self.frames);match residual {Some(residual)=>sequence(vec![residual,children]),None=>children}}}
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

/// ✍️ `serde_json`'s own (`zmij`-based) fixed/exponential split for `f64`: fixed notation for
/// `-5 <= exponent <= 15` on the leading-digit decimal exponent, exponential otherwise — traced
/// by hand from `zmij 1.0.21`'s `write<Float>` (`~/.cargo/registry/…/zmij-1.0.21/src/lib.rs`,
/// the `(-5..=15).contains(&dec_exp)` guard) and confirmed against the pinned resolved version in
/// this workspace's own `Cargo.lock`. Not ECMA-262's `-6 <= e < 21` (that was this writer's prior,
/// wrong rule — ties in the last significant digit alone hid the boundary-case fanout it caused).
/// A whole-number float always gets an explicit `.0` in fixed notation so it never collapses onto
/// its integer twin on the wire. See `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/
/// RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/🔍️research/📓️float-format-parity.md`.
fn write_float(value: f64, out: &mut String) {
    write_float_to(value, out).expect("String accepts formatted JSON floats");
}

struct ScalarText { bytes: [u8; 128], length: usize }
impl ScalarText {
    fn new() -> Self { Self { bytes: [0; 128], length: 0 } }
    fn text(&self) -> &str { std::str::from_utf8(&self.bytes[..self.length]).expect("formatted scalar UTF-8") }
}
impl fmt::Write for ScalarText {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        let end = self.length.checked_add(text.len()).filter(|end| *end <= self.bytes.len()).ok_or(fmt::Error)?;
        self.bytes[self.length..end].copy_from_slice(text.as_bytes()); self.length = end; Ok(())
    }
}

fn write_float_to(value: f64, out: &mut impl fmt::Write) -> fmt::Result {
    use fmt::Write as _;
    if !value.is_finite() {
        return out.write_str("null");
    }
    if value == 0.0 {
        return out.write_str(if value.is_sign_negative() { "-0.0" } else { "0.0" });
    }
    let negative = value.is_sign_negative();
    let magnitude = value.abs();
    let mut scientific = ScalarText::new();
    write!(scientific, "{magnitude:e}")?;
    let (mantissa_text, exponent_text) = scientific.text().split_once('e').expect("LowerExp always emits an exponent");
    let mut exponent: i32 = exponent_text.parse().expect("LowerExp exponent is always a plain integer");
    let digit_count = mantissa_text.bytes().filter(|byte| *byte != b'.').count();
    let (mut digits, exponent_adjust) = float_format::correctly_rounded_digits(magnitude, exponent, digit_count);
    exponent += exponent_adjust;
    if exponent_adjust != 0 {
        digits.truncate(digit_count);
    }
    let digit_count = digits.len() as i32;
    if negative {
        out.write_char('-')?;
    }
    if (-5..=15).contains(&exponent) {
        if exponent >= digit_count - 1 {
            out.write_str(std::str::from_utf8(&digits).expect("decimal digits are ASCII"))?;
            for _ in 0..(exponent - (digit_count - 1)) {
                out.write_char('0')?;
            }
            out.write_str(".0")?;
        } else if exponent >= 0 {
            let integer_len = (exponent + 1) as usize;
            out.write_str(std::str::from_utf8(&digits[..integer_len]).expect("decimal digits are ASCII"))?;
            out.write_char('.')?;
            out.write_str(std::str::from_utf8(&digits[integer_len..]).expect("decimal digits are ASCII"))?;
        } else {
            out.write_str("0.")?;
            for _ in 0..(-exponent - 1) {
                out.write_char('0')?;
            }
            out.write_str(std::str::from_utf8(&digits).expect("decimal digits are ASCII"))?;
        }
    } else {
        out.write_char(digits[0] as char)?;
        if digits.len() > 1 {
            out.write_char('.')?;
            out.write_str(std::str::from_utf8(&digits[1..]).expect("decimal digits are ASCII"))?;
        }
        out.write_char('e')?;
        if exponent >= 0 {
            out.write_char('+')?;
        }
        fmt::Write::write_fmt(out, format_args!("{exponent}"))?;
    }
    Ok(())
}

/// ✍️ [`write_float`] as a standalone string, for callers that write one bare JSON number
/// directly (no [`Value`] tree) — `🧵️canonical-edit::ScalarBytes` is exactly this shape:
/// a fixed-size scalar buffer, not a document.
pub fn format_f64(value: f64) -> String {
    let mut out = String::new();
    write_float(value, &mut out);
    out
}
//#endregion 🔖️Writer

//#region 🔖️FloatFormat
/// 🎯️ Exact-arithmetic correctly-rounded decimal digit generation for `f64`, used to reconcile
/// [`write_float`]'s digit string with `serde_json`'s (`zmij`'s) round-half-to-even tie-break at
/// the last significant digit — Rust's own shortest-round-trip `{:e}` formatter picks the
/// mathematically-correct-length, correctly-magnituded digit string, but at an exact
/// halfway-point tie (the true binary value sits precisely between two adjacent same-length
/// decimal strings, both of which round-trip) it does not consistently pick the even one the way
/// IEEE 754 and `zmij`'s Schubfach-based writer do. A "does a neighboring digit also round-trip"
/// probe was tried first and found unsound for large-magnitude values, where the round-trip basin
/// can hold more than two adjacent minimal-length candidates — this module instead recomputes the
/// digits directly from the value's exact rational form, `mantissa * 2^binary_exponent`, using a
/// small first-party big unsigned integer (no external bignum crate).
mod float_format {
    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Limbs { values: [u32; 80], length: usize }
    impl Limbs {
        fn zeroes(length: usize) -> Self { assert!(length <= 80); Self { values: [0; 80], length } }
        fn push(&mut self, value: u32) { assert!(self.length < self.values.len()); self.values[self.length] = value; self.length += 1; }
        fn pop(&mut self) -> Option<u32> { if self.length == 0 { None } else { self.length -= 1; Some(self.values[self.length]) } }
    }
    impl std::ops::Deref for Limbs { type Target = [u32]; fn deref(&self) -> &[u32] { &self.values[..self.length] } }
    impl std::ops::DerefMut for Limbs { fn deref_mut(&mut self) -> &mut [u32] { &mut self.values[..self.length] } }
    pub(super) struct Digits { values: [u8; 32], length: usize }
    impl Digits {
        fn new() -> Self { Self { values: [0; 32], length: 0 } }
        fn push(&mut self, value: u8) { assert!(self.length < self.values.len()); self.values[self.length] = value; self.length += 1; }
        fn pop(&mut self) { assert!(self.length > 0); self.length -= 1; }
        fn prepend_zero(&mut self) { assert!(self.length < self.values.len()); self.values.copy_within(0..self.length, 1); self.values[0] = b'0'; self.length += 1; }
        pub(super) fn truncate(&mut self, length: usize) { self.length = self.length.min(length); }
    }
    impl std::ops::Deref for Digits { type Target = [u8]; fn deref(&self) -> &[u8] { &self.values[..self.length] } }
    impl std::ops::DerefMut for Digits { fn deref_mut(&mut self) -> &mut [u8] { &mut self.values[..self.length] } }
    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Big(Limbs);

    impl Big {
        /// 🌱️ A single 64-bit value as a two-limb (or trimmed one-limb) big unsigned integer.
        fn from_u64(value: u64) -> Self {
            let mut limbs = Limbs::zeroes(2); limbs[0] = value as u32; limbs[1] = (value >> 32) as u32;
            Self::trim(&mut limbs);
            Big(limbs)
        }

        fn trim(limbs: &mut Limbs) {
            while limbs.len() > 1 && *limbs.last().expect("non-empty by loop condition") == 0 {
                limbs.pop();
            }
        }

        fn is_zero(&self) -> bool {
            self.0.len() == 1 && self.0[0] == 0
        }

        /// ✳️ In-place multiply by a small (`u32`-sized) factor.
        fn mul_small(&mut self, factor: u32) {
            let mut carry: u64 = 0;
            for limb in self.0.iter_mut() {
                let product = (*limb as u64) * (factor as u64) + carry;
                *limb = product as u32;
                carry = product >> 32;
            }
            if carry > 0 {
                self.0.push(carry as u32);
            }
            Self::trim(&mut self.0);
        }

        /// ✳️ In-place multiply by `5^exponent`, chunked so every factor still fits a `u32`.
        fn mul_pow5(&mut self, mut exponent: u32) {
            while exponent > 0 {
                let chunk = exponent.min(13);
                self.mul_small(5u32.pow(chunk));
                exponent -= chunk;
            }
        }

        /// ⬅️️ In-place multiply by `2^bits`.
        fn shl(&mut self, bits: u32) {
            if bits == 0 {
                return;
            }
            let limb_shift = (bits / 32) as usize;
            let bit_shift = bits % 32;
            let mut result = Limbs::zeroes(self.0.len() + limb_shift + 1);
            for (index, &limb) in self.0.iter().enumerate() {
                let value = limb as u64;
                if bit_shift == 0 {
                    result[index + limb_shift] |= value as u32;
                } else {
                    let shifted = value << bit_shift;
                    result[index + limb_shift] |= shifted as u32;
                    result[index + limb_shift + 1] |= (shifted >> 32) as u32;
                }
            }
            Self::trim(&mut result);
            self.0 = result;
        }

        fn bit_length(&self) -> u32 {
            let top = *self.0.last().expect("non-empty by construction");
            if top == 0 {
                0
            } else {
                (self.0.len() as u32 - 1) * 32 + (32 - top.leading_zeros())
            }
        }

        fn get_bit(&self, index: u32) -> bool {
            let limb = (index / 32) as usize;
            let bit = index % 32;
            if limb >= self.0.len() {
                false
            } else {
                (self.0[limb] >> bit) & 1 == 1
            }
        }

        fn cmp(&self, other: &Big) -> std::cmp::Ordering {
            if self.0.len() != other.0.len() {
                return self.0.len().cmp(&other.0.len());
            }
            for index in (0..self.0.len()).rev() {
                if self.0[index] != other.0[index] {
                    return self.0[index].cmp(&other.0[index]);
                }
            }
            std::cmp::Ordering::Equal
        }

        /// ➖️ In-place `self -= other`, requiring `self >= other`.
        fn sub_assign(&mut self, other: &Big) {
            let mut borrow: i64 = 0;
            for index in 0..self.0.len() {
                let lhs = self.0[index] as i64;
                let rhs = if index < other.0.len() { other.0[index] as i64 } else { 0 };
                let mut diff = lhs - rhs - borrow;
                if diff < 0 {
                    diff += 1 << 32;
                    borrow = 1;
                } else {
                    borrow = 0;
                }
                self.0[index] = diff as u32;
            }
            Self::trim(&mut self.0);
        }

        fn shl1(&mut self) {
            let mut carry = 0u32;
            for limb in self.0.iter_mut() {
                let next_carry = *limb >> 31;
                *limb = (*limb << 1) | carry;
                carry = next_carry;
            }
            if carry != 0 {
                self.0.push(carry);
            }
        }

        fn add_one(&mut self) {
            let mut carry = 1u64;
            for limb in self.0.iter_mut() {
                let sum = *limb as u64 + carry;
                *limb = sum as u32;
                carry = sum >> 32;
                if carry == 0 {
                    return;
                }
            }
            if carry > 0 {
                self.0.push(carry as u32);
            }
        }

        /// ➗️ Schoolbook binary long division: `self / other`, returning `(quotient, remainder)`.
        fn div_rem(&self, other: &Big) -> (Big, Big) {
            let bits = self.bit_length();
            let mut quotient = Big(Limbs::zeroes((bits / 32) as usize + 1));
            let mut remainder = Big(Limbs::zeroes(1));
            for index in (0..bits).rev() {
                remainder.shl1();
                if self.get_bit(index) {
                    remainder.0[0] |= 1;
                }
                if remainder.cmp(other) != std::cmp::Ordering::Less {
                    remainder.sub_assign(other);
                    let limb = (index / 32) as usize;
                    quotient.0[limb] |= 1 << (index % 32);
                }
            }
            Self::trim(&mut quotient.0);
            Self::trim(&mut remainder.0);
            (quotient, remainder)
        }

        fn double(&self) -> Big {
            let mut doubled = self.clone();
            doubled.shl1();
            doubled
        }

        /// 🔟️ Base-10 textual expansion, most-significant digit first, no leading zero (unless
        /// the value itself is zero).
        fn to_decimal_digits(&self) -> Digits {
            if self.is_zero() {
                let mut digits = Digits::new(); digits.push(b'0'); return digits;
            }
            let mut little_endian_digits = Digits::new();
            let mut remaining = self.clone();
            let billion = Big::from_u64(1_000_000_000);
            while !remaining.is_zero() {
                let (quotient, remainder) = remaining.div_rem(&billion);
                let mut chunk = remainder.0.iter().rev().fold(0u64, |accumulator, &limb| (accumulator << 32) | limb as u64);
                for _ in 0..9 {
                    little_endian_digits.push(b'0' + (chunk % 10) as u8);
                    chunk /= 10;
                }
                remaining = quotient;
            }
            while little_endian_digits.len() > 1 && *little_endian_digits.last().expect("non-empty by loop condition") == b'0' {
                little_endian_digits.pop();
            }
            little_endian_digits.reverse(); little_endian_digits
        }
    }

    /// 🎯️ Decomposes a finite, non-zero, non-negative `f64` into its exact
    /// `mantissa * 2^binary_exponent` form (the mantissa carries the implicit leading bit for
    /// normal numbers; subnormals keep the raw 52-bit significand at the fixed minimum exponent).
    fn decompose(magnitude: f64) -> (u64, i32) {
        let bits = magnitude.to_bits();
        let raw_exponent = ((bits >> 52) & 0x7FF) as i32;
        let raw_mantissa = bits & 0x000F_FFFF_FFFF_FFFF;
        if raw_exponent == 0 {
            (raw_mantissa, 1 - 1023 - 52)
        } else {
            (raw_mantissa | (1u64 << 52), raw_exponent - 1023 - 52)
        }
    }

    /// ✅️ The correctly-rounded (round-half-to-even) `digit_count`-digit decimal significand for
    /// `magnitude`, given the leading-digit decimal exponent `decimal_exponent` — both already
    /// established by Rust's own shortest-round-trip `{:e}` formatter, which is trusted for LENGTH
    /// and MAGNITUDE (every case in this module's own differential sweep confirms both are already
    /// correct); only the exact DIGITS at that fixed precision are recomputed here. Returns
    /// `(digits, exponent_adjust)`, where `exponent_adjust` is `1` when rounding carries all the
    /// way through (e.g. `"999"` rounds up to a truncated `"100"` with the exponent bumped), `0`
    /// otherwise.
    pub(super) fn correctly_rounded_digits(magnitude: f64, decimal_exponent: i32, digit_count: usize) -> (Digits, i32) {
        let (mantissa, binary_exponent) = decompose(magnitude);
        let scale = decimal_exponent - (digit_count as i32 - 1);
        let power_of_two = binary_exponent - scale;
        let power_of_five = -scale;

        let mut numerator = Big::from_u64(mantissa);
        if power_of_five > 0 {
            numerator.mul_pow5(power_of_five as u32);
        }
        if power_of_two > 0 {
            numerator.shl(power_of_two as u32);
        }
        let mut denominator = Big::from_u64(1);
        if power_of_five < 0 {
            denominator.mul_pow5((-power_of_five) as u32);
        }
        if power_of_two < 0 {
            denominator.shl((-power_of_two) as u32);
        }

        let (mut quotient, remainder) = numerator.div_rem(&denominator);
        let comparison = remainder.double().cmp(&denominator);
        let quotient_is_odd = quotient.0[0] & 1 == 1;
        let round_up = match comparison {
            std::cmp::Ordering::Greater => true,
            std::cmp::Ordering::Less => false,
            std::cmp::Ordering::Equal => quotient_is_odd,
        };
        if round_up {
            quotient.add_one();
        }

        let mut digit_string = quotient.to_decimal_digits();
        let mut exponent_adjust = 0;
        if digit_string.len() > digit_count {
            debug_assert_eq!(digit_string.len(), digit_count + 1, "rounding carries at most one extra digit");
            debug_assert_eq!(digit_string.last(), Some(&b'0'), "a carry past the digit budget must land on a power of ten");
            digit_string.pop();
            exponent_adjust = 1;
        }
        while digit_string.len() < digit_count {
            digit_string.prepend_zero();
        }
        (digit_string, exponent_adjust)
    }
}
//#endregion 🔖️FloatFormat

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
pub fn from_json_str<T: FromValue>(text: &str, policy: JsonMemberPolicy) -> Result<T, ValueError> {
    let value = parse(text, policy).map_err(JsonError::into_value_error)?;
    T::from_value(to_dsl_value(&value))
}

/// 🚦️ Parses and binds JSON under the caller's cumulative native ownership control.
pub fn from_json_str_controlled<T: FromValue>(text: &str, policy: JsonMemberPolicy, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<T, ValueError> {
    control.scoped_stage(|control| {
        control.begin_stage(text.len())?;
        let mut reader = ControlledReader { lexer: Lexer::new(text), policy, control };
        let value = reader.value()?.guard_decoded(); reader.whitespace()?;
        if reader.lexer.pos != text.len() { return Err(JsonError::TrailingData(reader.lexer.pos).into_value_error()); }
        reader.control.scoped_stage(|control| { control.begin_stage(0)?; T::from_value_controlled(value.get(), control) })
    })
}

/// 🛫️ Writes ordered JSON under the caller's cumulative native ownership control.
pub fn to_json_string_controlled<T: ToValue>(value: &T, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<String, ValueError> {
    control.scoped_stage(|control| {
        control.begin_stage(0)?; let value = value.to_value_controlled(control)?.guard_decoded();
        let mut measure = ControlledWriter { bytes: 0, output: None }; measure.value(value.get(), control)?;
        control.charge(measure.bytes)?; let mut output = String::new(); output.try_reserve_exact(measure.bytes).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "controlled JSON output allocation failed"))?;
        let mut writer = ControlledWriter { bytes: 0, output: Some(output) }; writer.value(value.get(), control)?;
        if writer.bytes != measure.bytes { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "controlled JSON output changed after admission")); } Ok(writer.output.unwrap())
    })
}

#[derive(Clone, Copy)]
struct JsonWriteFrame { state: u8, index: usize, position: usize }

enum JsonNativeNode<'a> { Value(&'a DslValue), Byte(u8) }

fn json_native_node<'a>(source: &'a DslValue, path: &[usize]) -> Result<JsonNativeNode<'a>, ValueError> {
    let mut value = source;
    for (depth, index) in path.iter().enumerate() {
        value = match value {
            DslValue::Array(values) => values.get(*index),
            DslValue::Object(values) => values.get(*index).map(|(_, value)| value),
            DslValue::Bytes(values) if depth + 1 == path.len() => return values.get(*index).copied().map(JsonNativeNode::Byte).ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "JSON byte path is absent")),
            _ => None,
        }.ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "JSON writer path is absent"))?;
    }
    Ok(JsonNativeNode::Value(value))
}

/// 🔎️ A canonical JSON scalar or collection borrowed directly from the retained source owner.
pub enum JsonWriteNode<'a>{Null,Bool(bool),Number(semio_framework_value::Number),String(&'a str),Array(usize),Object(usize)}

/// 🌱️ Source owners provide ordinal views without projecting or copying their payload first.
pub trait JsonWriteSource{
    fn node_at_path(&self,path:&[usize])->Result<JsonWriteNode<'_>,ValueError>;
    fn object_key_at_path(&self,path:&[usize],index:usize)->Result<&str,ValueError>;
}
impl JsonWriteSource for DslValue{
    fn node_at_path(&self,path:&[usize])->Result<JsonWriteNode<'_>,ValueError>{Ok(match json_native_node(self,path)?{
        JsonNativeNode::Byte(value)=>JsonWriteNode::Number(semio_framework_value::Number::UInt(u64::from(value))),
        JsonNativeNode::Value(value)=>match value{Self::Null=>JsonWriteNode::Null,Self::Bool(value)=>JsonWriteNode::Bool(*value),Self::Number(value)=>JsonWriteNode::Number(*value),Self::String(value)=>JsonWriteNode::String(value),Self::Array(value)=>JsonWriteNode::Array(value.len()),Self::Bytes(value)=>JsonWriteNode::Array(value.len()),Self::Object(value)=>JsonWriteNode::Object(value.len())},
    })}
    fn object_key_at_path(&self,path:&[usize],index:usize)->Result<&str,ValueError>{match json_native_node(self,path)?{JsonNativeNode::Value(Self::Object(entries))=>entries.get(index).map(|(key,_)|key.as_str()).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"JSON source key is absent")),_=>Err(ValueError::new(ValueRefusalKind::InvariantViolated,"JSON source key owner is absent"))}}
}

#[path = "🛫️encode/🫳️borrowed/🦀️.rs"]
mod borrowed_source_sink;
pub use borrowed_source_sink::write_json_source_into;

/// 🧵️ Measures and writes the same owned source through bounded canonical writer transitions.
pub struct JsonWriteCursor<S:JsonWriteSource> {
    source: Option<S>, frames: Vec<JsonWriteFrame>, path: Vec<usize>, writer: ControlledWriter, phase: u8,
}

impl<S:JsonWriteSource> JsonWriteCursor<S> {
    /// 🌱️ Takes the existing projected owner without copying or scanning its payload.
    pub fn new(value: S) -> Self { Self { source: Some(value), frames: Vec::new(), path: Vec::new(), writer: ControlledWriter { bytes: 0, output: None }, phase: 0 } }
    /// 📍️ Returns the current canonical output byte count and measure/write phase.
    pub fn progress(&self) -> (usize, bool) { (self.writer.bytes, self.phase == 2) }
    /// 📤️ Moves the original source to its next typed owner only after physical output completes.
    pub fn take_source(&mut self)->Option<S> {if self.phase==3 {self.source.take()}else{None}}
    /// ⏱️ Advances at most the supplied structural or scalar-character transitions.
    pub fn step(&mut self, maximum_units: usize, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<Option<String>, ValueError> {
        for _ in 0..maximum_units {
            control.checkpoint()?;
            if self.phase == 3 { return Ok(None); }
            if self.phase == 0 {
                self.frames = control.allocate_vec(MAX_DEPTH as usize + 1)?;
                self.path = control.allocate_vec(MAX_DEPTH as usize)?;
                self.frames.push(JsonWriteFrame { state: 0, index: 0, position: 0 });
                self.phase = 1;
            } else if self.frames.is_empty() {
                if self.phase == 1 {
                    control.charge(self.writer.bytes)?;
                    let mut output = String::new();
                    output.try_reserve_exact(self.writer.bytes).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "controlled JSON output allocation failed"))?;
                    self.writer = ControlledWriter { bytes: 0, output: Some(output) };
                    self.frames.push(JsonWriteFrame { state: 0, index: 0, position: 0 });
                    self.phase = 2;
                } else {
                    self.phase = 3;
                    return Ok(self.writer.output.take());
                }
            } else { self.advance(control)?; }
            control.step()?;
        }
        Ok(None)
    }
    fn finish_node(&mut self) { self.frames.pop(); if !self.frames.is_empty() { self.path.pop(); } }
    fn child(&mut self, index: usize) -> Result<(), ValueError> {
        if self.path.len() >= MAX_DEPTH as usize { return Err(ValueError::new(ValueRefusalKind::DepthLimit, "retained JSON writer exceeds depth limit")); }
        self.path.push(index); self.frames.push(JsonWriteFrame { state: 0, index: 0, position: 0 }); Ok(())
    }
    fn advance(&mut self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<(), ValueError> {
        let frame = *self.frames.last().expect("writer frontier is inhabited");
        let source=self.source.as_ref().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"JSON writer source is absent"))?;
        let node = source.node_at_path(&self.path)?;
        match frame.state {
            0 => {
                match node {
                    JsonWriteNode::Null => self.writer.raw("null", control)?,
                    JsonWriteNode::Bool(value) => self.writer.raw(if value { "true" } else { "false" }, control)?,
                    JsonWriteNode::Number(value) => self.writer.number(value, control)?,
                    JsonWriteNode::String(_) => { self.writer.raw("\"", control)?; self.frames.last_mut().unwrap().state = 5; return Ok(()); }
                    JsonWriteNode::Array(_) => { self.writer.raw("[", control)?; self.frames.last_mut().unwrap().state = 1; return Ok(()); }
                    JsonWriteNode::Object(_) => { self.writer.raw("{", control)?; self.frames.last_mut().unwrap().state = 2; return Ok(()); }
                }
                self.finish_node();
            }
            1 => {
                let JsonWriteNode::Array(length)=node else{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"JSON array source changed"))};
                if frame.index == length { self.writer.raw("]", control)?; self.finish_node(); }
                else { if frame.index != 0 { self.writer.raw(",", control)?; } self.frames.last_mut().unwrap().index += 1; self.child(frame.index)?; }
            }
            2 => {
                let JsonWriteNode::Object(length) = node else { return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"JSON object source changed")); };
                if frame.index == length { self.writer.raw("}", control)?; self.finish_node(); }
                else { if frame.index != 0 { self.writer.raw(",", control)?; } self.writer.raw("\"", control)?; let next = self.frames.last_mut().unwrap(); next.state = 3; next.position = 0; }
            }
            3 | 5 => {
                let text = match node { JsonWriteNode::String(text) if frame.state == 5 => text, JsonWriteNode::Object(_) if frame.state == 3 => source.object_key_at_path(&self.path,frame.index)?, _ => return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"JSON text source changed")) };
                let remaining=text.get(frame.position..).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"JSON text source position changed"))?;
                if let Some(character) = remaining.chars().next() { self.writer.character(character, control)?; self.frames.last_mut().unwrap().position += character.len_utf8(); }
                else { self.writer.raw("\"", control)?; if frame.state == 5 { self.finish_node(); } else { self.writer.raw(":", control)?; self.frames.last_mut().unwrap().state = 4; } }
            }
            4 => { let next = self.frames.last_mut().unwrap(); next.state = 2; next.index += 1; self.child(frame.index)?; }
            _ => unreachable!(),
        }
        Ok(())
    }
}

impl semio_framework_value::retirement::RetireOwned for JsonWriteFrame { fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> { semio_framework_value::retirement::leaf(self) } }
impl<S:JsonWriteSource+semio_framework_value::retirement::RetireOwned+'static> semio_framework_value::retirement::RetireOwned for JsonWriteCursor<S> { fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> { semio_framework_value::artifact_retirement_sequence!(self.source, self.frames, self.path, self.writer.output) } }

struct ControlledWriter { bytes: usize, output: Option<String> }
impl ControlledWriter {
    fn character(&mut self, character: char, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<(), ValueError> {
        match character {
            '"' => self.raw("\\\"", control), '\\' => self.raw("\\\\", control), '\u{0008}' => self.raw("\\b", control), '\u{000c}' => self.raw("\\f", control), '\n' => self.raw("\\n", control), '\r' => self.raw("\\r", control), '\t' => self.raw("\\t", control),
            character if (character as u32) < 0x20 => { use fmt::Write as _; let mut scalar = ScalarText::new(); write!(scalar, "\\u{:04x}", character as u32).map_err(|_| ValueError::new(ValueRefusalKind::InvariantViolated, "controlled JSON escape overflow"))?; self.raw(scalar.text(), control) }
            character => { let mut bytes = [0; 4]; self.raw(character.encode_utf8(&mut bytes), control) }
        }
    }
    fn raw(&mut self, text: &str, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<(), ValueError> {
        self.bytes = self.bytes.checked_add(text.len()).filter(|bytes| *bytes <= control.maximum_bytes()).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "controlled JSON output exceeds caller limit"))?;
        if self.output.as_ref().is_some_and(|output| self.bytes > output.capacity()) { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "controlled JSON output exceeds admitted allocation")); }
        if text.len() <= 65536 { if let Some(output) = self.output.as_mut() { output.push_str(text); } return Ok(()); }
        control.scoped_stage(|control| -> Result<(), ValueError> { control.begin_stage(text.len())?; let mut position = 0; while position < text.len() { let mut end = position.saturating_add(65536).min(text.len()); while !text.is_char_boundary(end) { end -= 1; } if let Some(output) = self.output.as_mut() { output.push_str(&text[position..end]); } control.advance(end - position)?; position = end; } Ok(()) })
    }
    fn string(&mut self, text: &str, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<(), ValueError> {
        self.raw("\"", control)?;
        control.scoped_stage(|control| -> Result<(), ValueError> { control.begin_stage(text.len())?; for character in text.chars() { self.character(character, control)?; control.advance(character.len_utf8())?; } Ok(()) })?;
        self.raw("\"", control)
    }
    fn number(&mut self, number: semio_framework_value::Number, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<(), ValueError> {
        use fmt::Write as _; control.checkpoint()?; let mut scalar = ScalarText::new();
        match number { semio_framework_value::Number::UInt(value) => write!(scalar, "{value}"), semio_framework_value::Number::Int(value) => write!(scalar, "{value}"), semio_framework_value::Number::Float(value) => write_float_to(value, &mut scalar) }.map_err(|_| ValueError::new(ValueRefusalKind::InvariantViolated, "controlled JSON scalar overflow"))?;
        self.raw(scalar.text(), control)?; control.checkpoint()
    }
    fn value(&mut self, value: &DslValue, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<(), ValueError> {
        control.scoped_depth(MAX_DEPTH as usize + 1, |control| { control.checkpoint()?; match value {
            DslValue::Null => self.raw("null", control), DslValue::Bool(value) => self.raw(if *value { "true" } else { "false" }, control), DslValue::Number(value) => self.number(*value, control), DslValue::String(value) => self.string(value, control),
            DslValue::Array(values) => { self.raw("[", control)?; control.scoped_stage(|control| -> Result<(), ValueError> { control.begin_stage(values.len())?; for (index, value) in values.iter().enumerate() { if index > 0 { self.raw(",", control)?; } self.value(value, control)?; control.step()?; } Ok(()) })?; self.raw("]", control) },
            DslValue::Bytes(values) => { self.raw("[", control)?; control.scoped_stage(|control| -> Result<(), ValueError> { control.begin_stage(values.len())?; for (index, value) in values.iter().enumerate() { if index > 0 { self.raw(",", control)?; } self.number(semio_framework_value::Number::UInt(u64::from(*value)), control)?; control.step()?; } Ok(()) })?; self.raw("]", control) },
            DslValue::Object(entries) => { self.raw("{", control)?; control.scoped_stage(|control| -> Result<(), ValueError> { control.begin_stage(entries.len())?; for (index, (key, value)) in entries.iter().enumerate() { if index > 0 { self.raw(",", control)?; } self.string(key, control)?; self.raw(":", control)?; self.value(value, control)?; control.step()?; } Ok(()) })?; self.raw("}", control) }
        } })
    }
}

struct ControlledReader<'text, 'control, 'progress> { lexer: Lexer<'text>, policy: JsonMemberPolicy, control: &'control mut semio_framework_value::NativeDecodeControl<'progress> }
impl ControlledReader<'_, '_, '_> {
    fn error(&self) -> ValueError { JsonError::UnexpectedByte { found: self.lexer.peek_byte().unwrap_or(0), offset: self.lexer.pos }.into_value_error() }
    fn advance(&mut self, count: usize) -> Result<(), ValueError> { self.lexer.pos += count; self.control.advance(count) }
    fn whitespace(&mut self) -> Result<(), ValueError> { while matches!(self.lexer.peek_byte(), Some(b' ' | b'\t' | b'\r' | b'\n')) { self.advance(1)?; } Ok(()) }
    fn reserve<T>(&mut self, values: &mut Vec<T>) -> Result<(), ValueError> {
        if values.len() == values.capacity() {
            let count = values.capacity().max(1).checked_mul(2).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "controlled JSON collection overflow"))?;
            let bytes = count.checked_mul(std::mem::size_of::<T>()).filter(|bytes| *bytes <= isize::MAX as usize).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "controlled JSON collection overflow"))?;
            self.control.charge(bytes)?; values.try_reserve_exact(count - values.len()).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "controlled JSON allocation failed"))?;
        } Ok(())
    }
    fn string(&mut self) -> Result<String, ValueError> {
        self.advance(1)?; let body = self.lexer.pos; let mut length = 0usize;
        loop { let start = self.lexer.pos; let character = json_character(&mut self.lexer).map_err(JsonError::into_value_error)?; self.control.advance(self.lexer.pos - start)?; let Some(character) = character else { break }; length = length.checked_add(character.len_utf8()).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "controlled JSON string overflow"))?; }
        let input = self.lexer.input; let end = self.lexer.pos; self.control.charge(length)?;
        self.control.scoped_stage(|control| {
            control.begin_stage(end - body)?; let mut output = String::new(); output.try_reserve_exact(length).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "controlled JSON text allocation failed"))?;
            let mut decoder = Lexer { input, pos: body };
            loop { let start = decoder.pos; let character = json_character(&mut decoder).map_err(JsonError::into_value_error)?; control.advance(decoder.pos - start)?; let Some(character) = character else { break }; output.push(character); }
            if output.len() != length || decoder.pos != end { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "controlled JSON string changed after admission")); } Ok(output)
        })
    }
    fn value(&mut self) -> Result<DslValue, ValueError> {
        self.control.scoped_depth(MAX_DEPTH as usize + 1, |control| { let mut reader = ControlledReader { lexer: Lexer { input: self.lexer.input, pos: self.lexer.pos }, policy: self.policy, control }; let result = reader.value_inner(); self.lexer.pos = reader.lexer.pos; result })
    }
    fn value_inner(&mut self) -> Result<DslValue, ValueError> {
        self.whitespace()?; self.control.checkpoint()?;
        match self.lexer.peek_byte() {
            Some(b'"') => self.string().map(DslValue::String),
            Some(b't' | b'f' | b'n') => { let (text, value) = match self.lexer.peek_byte().unwrap() { b't' => ("true", DslValue::Bool(true)), b'f' => ("false", DslValue::Bool(false)), _ => ("null", DslValue::Null) }; if !self.lexer.input[self.lexer.pos..].starts_with(text) { return Err(self.error()); } self.advance(text.len())?; Ok(value) }
            Some(b'-' | b'0'..=b'9') => {
                let start = self.lexer.pos; while matches!(self.lexer.peek_byte(), Some(b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9')) { self.advance(1)?; }
                let mut number = Lexer::new(&self.lexer.input[start..self.lexer.pos]); let Token::Number(value) = number.read_number().map_err(JsonError::into_value_error)? else { unreachable!() }; if number.pos != number.input.len() { return Err(JsonError::InvalidNumber(start).into_value_error()); }
                Ok(DslValue::Number(match value { Number::UInt(value) => semio_framework_value::Number::UInt(value), Number::Int(value) => semio_framework_value::Number::Int(value), Number::Float(value) => semio_framework_value::Number::Float(value) }))
            }
            Some(b'[') => {
                self.advance(1)?; self.whitespace()?; let mut values = Vec::<DslValue>::new().guard_decoded();
                if self.lexer.peek_byte() == Some(b']') { self.advance(1)?; return Ok(DslValue::Array(values.take())); }
                loop { let value = self.value()?.guard_decoded(); self.reserve(values.get_mut())?; values.get_mut().push(value.take()); self.whitespace()?; match self.lexer.peek_byte() { Some(b',') => self.advance(1)?, Some(b']') => { self.advance(1)?; return Ok(DslValue::Array(values.take())); }, _ => return Err(self.error()) } }
            }
            Some(b'{') => {
                self.advance(1)?; self.whitespace()?; let mut entries = semio_framework_value::DecodedValue::new(Vec::new(), |entries: Vec<(String, DslValue)>| { for (_, value) in entries { FromValue::retire_decoded(value); } });
                if self.lexer.peek_byte() == Some(b'}') { self.advance(1)?; return Ok(DslValue::Object(entries.take())); }
                loop {
                    self.whitespace()?; if self.lexer.peek_byte() != Some(b'"') { return Err(self.error()); } let offset = self.lexer.pos; let key = self.string()?;
                    let duplicate = self.control.scoped_stage(|control| -> Result<Option<usize>, ValueError> { control.begin_stage(entries.get().len())?; for (index, (name, _)) in entries.get().iter().enumerate() { let equal = controlled_key_equal(name, &key, control)?; control.step()?; if equal { return Ok(Some(index)); } } Ok(None) })?;
                    if duplicate.is_some() && self.policy == JsonMemberPolicy::Reject { return Err(JsonError::DuplicateMember { name: key, offset }.into_value_error()); }
                    self.whitespace()?; if self.lexer.peek_byte() != Some(b':') { return Err(self.error()); } self.advance(1)?; let value = self.value()?.guard_decoded();
                    if let Some(index) = duplicate { FromValue::retire_decoded(std::mem::replace(&mut entries.get_mut()[index].1, value.take())); } else { self.reserve(entries.get_mut())?; entries.get_mut().push((key, value.take())); }
                    self.whitespace()?; match self.lexer.peek_byte() { Some(b',') => self.advance(1)?, Some(b'}') => { self.advance(1)?; return Ok(DslValue::Object(entries.take())); }, _ => return Err(self.error()) }
                }
            }
            _ => Err(self.error())
        }
    }
}

fn controlled_key_equal(left: &str, right: &str, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<bool, ValueError> {
    if left.len() != right.len() { return Ok(false); } control.scoped_stage(|control| { control.begin_stage(left.len())?; let mut position = 0; while position < left.len() { let end = position.saturating_add(65536).min(left.len()); let equal = left.as_bytes()[position..end] == right.as_bytes()[position..end]; control.advance(end - position)?; if !equal { return Ok(false); } position = end; } Ok(true) })
}

fn json_character(lexer:&mut Lexer<'_>)->Result<Option<char>,JsonError>{borrowed_read_source::json_character(lexer.input,&mut lexer.pos)}
//#endregion 🔖️ToFromValueBridge

//#region 🔖️Macro
/// 🧩️ `serde_json::json!` replacement — an object/array literal builder over [`Value`], expanded
/// via the standard TT-muncher recursion (see `json_object_internal!`/`json_array_internal!`,
/// `#[doc(hidden)]`, exported only so this macro's own expansion can call them from any crate).
/// Object keys are string literals (`"key": value`); leaf expressions are borrowed and encoded
/// through [`ToValue`], preserving access to records and fields after constructing the JSON tree.
#[macro_export]
macro_rules! json {
    (null) => { $crate::Value::Null };
    (true) => { $crate::Value::Bool(true) };
    (false) => { $crate::Value::Bool(false) };
    ([]) => { $crate::Value::Array(::std::vec::Vec::new()) };
    ([ $($tt:tt)+ ]) => {
        $crate::Value::Array($crate::json_array_internal!(@collect [] $($tt)+))
    };
    ({}) => { $crate::Value::Object($crate::Object::new()) };
    ({ $($tt:tt)+ }) => {
        $crate::Value::Object({
            let mut __object = $crate::Object::new();
            $crate::json_object_internal!(__object $($tt)+);
            __object
        })
    };
    ($other:expr) => {{
        use $crate::ToValue as _;
        $crate::from_dsl_value(&(&$other).to_value())
    }};
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "🧪️tests/🧩️members/🦀️.rs"]
mod member_tests;
//#endregion 🔖️Tests


#[cfg(test)]
#[path = "🧪️tests/⚠️refusal/🦀️.rs"]
mod refusal_tests;
