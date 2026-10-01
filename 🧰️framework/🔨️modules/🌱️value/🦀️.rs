//! 🌱️ `DslValue` — the schema-erased dynamic value both sides of a replication link speak.
//!
//! Lives beside the wire contract rather than inside the os DSL because it is what a schema-less
//! payload decodes to: the authority validates it, the optimistic replica applies it, and the
//! pathmap bodies `db` stores are trees of it. The DSL's own record/field/wire types build on it
//! and stay product-side.
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde/Display) — see R9

//#region 🗂️OrderedOwnership
#[path = "🗂️ordered/🦀️.rs"]
pub mod ordered;
#[path = "📋️list/🦀️.rs"]
pub mod list;
//#endregion 🗂️OrderedOwnership

#[path = "🧬️bytes/🦀️.rs"]
pub mod bytes;

#[path = "🧬️clone/🦀️.rs"]
pub mod bounded_clone;

//#region 🔁️Codec
#[path = "🔁️codec/🦀️.rs"]
mod codec;
pub use codec::{edit_through_value, FromValue, ToValue, ValueEdit, ValueError, ValueShape};
//#endregion 🔁️Codec

//#region 🔖️Number
/// 🔢️ A JSON-equivalent number that keeps the writer's-eye distinction a bare `f64` erases:
/// an integer literal (`UInt`/`Int`) round-trips without a decimal point, `Float` always keeps one
/// (or an exponent). Mirrors [`pack::json::Number`]'s shape exactly so the wire bridge between them
/// (`🎒️pack/🔤️json/🦀️.rs`) is a straight variant-to-variant map, never a widen-then-guess. See
/// `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/
/// 🔍️research/📓️dslvalue-integer-fidelity.md`.
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

/// 🔢️ The integer one finite JSON number reads back as — [`json_integer`]'s answer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JsonInteger {
    Unsigned(u64),
    Signed(i64),
}

/// 🔢️ The integer an `f64` reads back as from its JSON text: `JSON.stringify(7.0)` is `"7"`, so a finite, integral number
/// within the safe integer range (`Number.isSafeInteger`, |v| ≤ 2^53 − 1) is an integer — unsigned unless negative — and
/// anything else (a fraction, a non-finite value, an integral value past the safe range) is not. The ONE rule every Rust
/// boundary that carries a JSON number as `f64` reads it by (`DslValue::json_number`, the UI contract's `UiValue` serializer).
pub fn json_integer(v: f64) -> Option<JsonInteger> {
    const SAFE_INTEGER_MAX: f64 = 9_007_199_254_740_991.0;
    if !v.is_finite() || v.fract() != 0.0 || v.abs() > SAFE_INTEGER_MAX {
        return None;
    }
    Some(if v >= 0.0 { JsonInteger::Unsigned(v as u64) } else { JsonInteger::Signed(v as i64) })
}
//#endregion 🔖️Number

//#region 🔖️Value
/// 🌱️ Dynamic JSON-equivalent literal for schema-less fields (`Shape::Value`).
#[derive(Clone, Debug, PartialEq)]
pub enum DslValue {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Bytes(Vec<u8>),
    Array(Vec<DslValue>),
    Object(Vec<(String, DslValue)>),
}

/// 🪪️ Immutable framework value selected from one retained domain owner.
pub trait DslValueSource {
    fn value(&self) -> &DslValue;
}

impl DslValueSource for std::sync::Arc<DslValue> {
    fn value(&self) -> &DslValue { self.as_ref() }
}

impl DslValue {
    pub fn null() -> Self {
        Self::Null
    }

    /// 🔢️ Whole-number constructor — the fidelity-preserving choice for ids/counts/indices/ms.
    pub fn uint(v: u64) -> Self {
        Self::Number(Number::UInt(v))
    }

    /// 🔢️ Signed whole-number constructor.
    pub fn int(v: i64) -> Self {
        Self::Number(Number::Int(v))
    }

    /// 🔢️ Fractional constructor — the wire always keeps an explicit `.0` for a whole float so it
    /// never collapses onto its integer twin.
    pub fn float(v: f64) -> Self {
        Self::Number(Number::Float(v))
    }

    /// 🔢️ A JSON number carried as `f64` (the UI contract's `UiValue::Number`), read the way its JSON text reads back
    /// ([`json_integer`]): an integer stays an integer, a non-finite value is `Null` (JSON text has neither NaN nor Infinity),
    /// anything else a float — so an integer argument a renderer hands back decodes as the integer React's wire delivers.
    pub fn json_number(v: f64) -> Self {
        match json_integer(v) {
            Some(JsonInteger::Unsigned(value)) => Self::uint(value),
            Some(JsonInteger::Signed(value)) => Self::int(value),
            None if v.is_finite() => Self::float(v),
            None => Self::Null,
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// 🔎️ Widens to `f64` regardless of variant — lossy for `u64`/`i64` magnitudes beyond 2^53. Use
    /// [`DslValue::as_i64`]/[`DslValue::as_u64`] when exactness matters.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Number(n) => Some(n.as_f64()),
            _ => None,
        }
    }

    /// 🔎️ Exact `i64`, only when the underlying [`Number`] is representable as one.
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Self::Number(n) => n.as_i64(),
            _ => None,
        }
    }

    /// 🔎️ Exact `u64`, only when the underlying [`Number`] is representable as one.
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Self::Number(n) => n.as_u64(),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s.as_str()),
            _ => None,
        }
    }

    /// 🧬️ Borrows intrinsic octets without projecting one numeric node per byte.
    pub fn as_bytes(&self)->Option<&[u8]>{match self{Self::Bytes(bytes)=>Some(bytes),_=>None}}

    pub fn as_array(&self) -> Option<&[DslValue]> {
        match self {
            Self::Array(items) => Some(items.as_slice()),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&[(String, DslValue)]> {
        match self {
            Self::Object(entries) => Some(entries.as_slice()),
            _ => None,
        }
    }

    pub fn get(&self, key: &str) -> Option<&DslValue> {
        let Self::Object(entries) = self else {
            return None;
        };
        entries.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    pub fn object(entries: impl IntoIterator<Item = (String, DslValue)>) -> Self {
        Self::Object(entries.into_iter().collect())
    }
}

impl std::ops::Index<&str> for DslValue {
    type Output = DslValue;
    fn index(&self, key: &str) -> &Self::Output {
        static NULL: DslValue = DslValue::Null;
        self.get(key).unwrap_or(&NULL)
    }
}

impl std::ops::Index<usize> for DslValue {
    type Output = DslValue;
    fn index(&self, index: usize) -> &Self::Output {
        static NULL: DslValue = DslValue::Null;
        match self {
            DslValue::Array(items) => items.get(index).unwrap_or(&NULL),
            _ => &NULL,
        }
    }
}

impl From<&DslValue> for serde_json::Value {
    fn from(val: &DslValue) -> Self {
        match val {
            DslValue::Null => serde_json::Value::Null,
            DslValue::Bool(b) => serde_json::Value::Bool(*b),
            DslValue::Number(Number::UInt(v)) => serde_json::Value::Number((*v).into()),
            DslValue::Number(Number::Int(v)) => serde_json::Value::Number((*v).into()),
            DslValue::Number(Number::Float(v)) => serde_json::json!(*v),
            DslValue::String(s) => serde_json::Value::String(s.clone()),
            DslValue::Bytes(bytes) => serde_json::Value::Array(bytes.iter().map(|byte|serde_json::Value::Number((*byte).into())).collect()),
            DslValue::Array(arr) => serde_json::Value::Array(arr.iter().map(serde_json::Value::from).collect()),
            DslValue::Object(obj) => {
                let map = obj.iter().map(|(k, v)| (k.clone(), serde_json::Value::from(v))).collect();
                serde_json::Value::Object(map)
            }
        }
    }
}

impl From<DslValue> for serde_json::Value {
    fn from(val: DslValue) -> Self {
        serde_json::Value::from(&val)
    }
}

/// 🌉️ The reverse bridge: a plugin decoding `ArtifactEditor::command_from_action`/
/// `host_configuration_mutation`'s trait-mandated `Option<&serde_json::Value>` args into a
/// `ToValue`/`FromValue` domain type routes through here — preserves whichever of `serde_json`'s
/// own `u64`/`i64`/`f64` storage the number parsed into (same convention `pack::json`'s
/// `to_dsl_value` bridge uses for its sibling `Value` type), never widening an integer to `f64`.
impl From<&serde_json::Value> for DslValue {
    fn from(val: &serde_json::Value) -> Self {
        match val {
            serde_json::Value::Null => DslValue::Null,
            serde_json::Value::Bool(b) => DslValue::Bool(*b),
            serde_json::Value::Number(n) => {
                if let Some(v) = n.as_u64().filter(|_| n.is_u64()) {
                    DslValue::Number(Number::UInt(v))
                } else if let Some(v) = n.as_i64().filter(|_| n.is_i64()) {
                    DslValue::Number(Number::Int(v))
                } else {
                    DslValue::Number(Number::Float(n.as_f64().unwrap_or(f64::NAN)))
                }
            }
            serde_json::Value::String(s) => DslValue::String(s.clone()),
            serde_json::Value::Array(items) => DslValue::Array(items.iter().map(DslValue::from).collect()),
            serde_json::Value::Object(obj) => DslValue::object(obj.iter().map(|(k, v)| (k.clone(), DslValue::from(v)))),
        }
    }
}

impl From<serde_json::Value> for DslValue {
    fn from(val: serde_json::Value) -> Self {
        DslValue::from(&val)
    }
}

/// 🌉️ Lets a type that still derives `serde` hold a `DslValue` field — the transitional state the
/// serde-elimination sweep leaves behind (e.g. `ActionDescriptor.args: Option<DslValue>` in
/// `🖱️ui/🎯️targets/🧊️wgpu`). Delegating through the `From` conversions directly above rather than
/// hand-rolling a visitor makes the encoding identical to `serde_json::Value`'s BY CONSTRUCTION,
/// which is the property that matters: both encodings share a wire, so a `DslValue` must serialize
/// to exactly the JSON its own `to_value`/`json` path would produce. Remove once no serde-deriving
/// type holds a `DslValue`.
impl serde::Serialize for DslValue {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serde::Serialize::serialize(&serde_json::Value::from(self), serializer)
    }
}

/// 🌉️ Mirror of the `Serialize` bridge directly above — see its note.
impl<'de> serde::Deserialize<'de> for DslValue {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        serde_json::Value::deserialize(deserializer).map(DslValue::from)
    }
}

impl PartialEq<serde_json::Value> for DslValue {
    fn eq(&self, other: &serde_json::Value) -> bool {
        &serde_json::Value::from(self) == other
    }
}

impl PartialEq<DslValue> for serde_json::Value {
    fn eq(&self, other: &DslValue) -> bool {
        self == &serde_json::Value::from(other)
    }
}

//#region 🔖️Literal
/// 🧾️ Builds a [`DslValue`] from a JSON-shaped literal — the first-party replacement for
/// `serde_json::json!` + `DslValue::from` at every action-argument site. Grammar: `null`, `true`,
/// `false`, `[ … ]`, `{ key: value, … }` (a key is a string literal or any expression convertible
/// `Into<String>`, e.g. a `const`), and any other expression, converted through [`ToValue`] by method call
/// (so a borrowed `&String`/`&[T]` auto-derefs to its implementation, as `serde_json::json!` accepts it). Object
/// entries keep their written order (a `DslValue::Object` is ordered); trailing commas are
/// accepted. The equivalence law against `serde_json::json!` lives in `🧪️tests/🔬️unit/🦀️.rs`.
#[macro_export]
macro_rules! dsl_value {
    (@array [$($elements:expr,)*]) => { ::std::vec![$($elements,)*] };
    (@array [$($elements:expr),*]) => { ::std::vec![$($elements),*] };
    (@array [$($elements:expr,)*] null $($rest:tt)*) => { $crate::dsl_value!(@array [$($elements,)* $crate::dsl_value!(null)] $($rest)*) };
    (@array [$($elements:expr,)*] true $($rest:tt)*) => { $crate::dsl_value!(@array [$($elements,)* $crate::dsl_value!(true)] $($rest)*) };
    (@array [$($elements:expr,)*] false $($rest:tt)*) => { $crate::dsl_value!(@array [$($elements,)* $crate::dsl_value!(false)] $($rest)*) };
    (@array [$($elements:expr,)*] [$($array:tt)*] $($rest:tt)*) => { $crate::dsl_value!(@array [$($elements,)* $crate::dsl_value!([$($array)*])] $($rest)*) };
    (@array [$($elements:expr,)*] {$($object:tt)*} $($rest:tt)*) => { $crate::dsl_value!(@array [$($elements,)* $crate::dsl_value!({$($object)*})] $($rest)*) };
    (@array [$($elements:expr,)*] $next:expr, $($rest:tt)*) => { $crate::dsl_value!(@array [$($elements,)* $crate::dsl_value!($next),] $($rest)*) };
    (@array [$($elements:expr,)*] $last:expr) => { $crate::dsl_value!(@array [$($elements,)* $crate::dsl_value!($last)]) };
    (@array [$($elements:expr),*] , $($rest:tt)*) => { $crate::dsl_value!(@array [$($elements,)*] $($rest)*) };
    (@array [$($elements:expr),*] $unexpected:tt $($rest:tt)*) => { ::std::compile_error!("dsl_value!: unexpected token in array") };
    (@object $entries:ident () () ()) => {};
    (@object $entries:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
        $entries.push((::std::string::String::from($($key)+), $value));
        $crate::dsl_value!(@object $entries () ($($rest)*) ($($rest)*));
    };
    (@object $entries:ident [$($key:tt)+] ($value:expr) $unexpected:tt $($rest:tt)*) => { ::std::compile_error!("dsl_value!: expected `,` after an object entry") };
    (@object $entries:ident [$($key:tt)+] ($value:expr)) => { $entries.push((::std::string::String::from($($key)+), $value)); };
    (@object $entries:ident ($($key:tt)+) (: null $($rest:tt)*) $copy:tt) => { $crate::dsl_value!(@object $entries [$($key)+] ($crate::dsl_value!(null)) $($rest)*); };
    (@object $entries:ident ($($key:tt)+) (: true $($rest:tt)*) $copy:tt) => { $crate::dsl_value!(@object $entries [$($key)+] ($crate::dsl_value!(true)) $($rest)*); };
    (@object $entries:ident ($($key:tt)+) (: false $($rest:tt)*) $copy:tt) => { $crate::dsl_value!(@object $entries [$($key)+] ($crate::dsl_value!(false)) $($rest)*); };
    (@object $entries:ident ($($key:tt)+) (: [$($array:tt)*] $($rest:tt)*) $copy:tt) => { $crate::dsl_value!(@object $entries [$($key)+] ($crate::dsl_value!([$($array)*])) $($rest)*); };
    (@object $entries:ident ($($key:tt)+) (: {$($object:tt)*} $($rest:tt)*) $copy:tt) => { $crate::dsl_value!(@object $entries [$($key)+] ($crate::dsl_value!({$($object)*})) $($rest)*); };
    (@object $entries:ident ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => { $crate::dsl_value!(@object $entries [$($key)+] ($crate::dsl_value!($value)) , $($rest)*); };
    (@object $entries:ident ($($key:tt)+) (: $value:expr) $copy:tt) => { $crate::dsl_value!(@object $entries [$($key)+] ($crate::dsl_value!($value))); };
    (@object $entries:ident ($($key:tt)+) (:) $copy:tt) => { ::std::compile_error!("dsl_value!: missing value after `:`") };
    (@object $entries:ident ($($key:tt)+) () $copy:tt) => { ::std::compile_error!("dsl_value!: missing `:` after an object key") };
    (@object $entries:ident () (: $($rest:tt)*) ($colon:tt $($copy:tt)*)) => { ::std::compile_error!("dsl_value!: unexpected `:`") };
    (@object $entries:ident ($($key:tt)*) (, $($rest:tt)*) ($comma:tt $($copy:tt)*)) => { ::std::compile_error!("dsl_value!: unexpected `,`") };
    (@object $entries:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => { $crate::dsl_value!(@object $entries ($key) (: $($rest)*) (: $($rest)*)); };
    (@object $entries:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => { $crate::dsl_value!(@object $entries ($($key)* $tt) ($($rest)*) ($($rest)*)); };
    (null) => { $crate::value::DslValue::Null };
    (true) => { $crate::value::DslValue::Bool(true) };
    (false) => { $crate::value::DslValue::Bool(false) };
    ([]) => { $crate::value::DslValue::Array(::std::vec::Vec::new()) };
    ([ $($tt:tt)+ ]) => { $crate::value::DslValue::Array($crate::dsl_value!(@array [] $($tt)+)) };
    ({}) => { $crate::value::DslValue::Object(::std::vec::Vec::new()) };
    ({ $($tt:tt)+ }) => {
        $crate::value::DslValue::Object({
            let mut entries: ::std::vec::Vec<(::std::string::String, $crate::value::DslValue)> = ::std::vec::Vec::new();
            $crate::dsl_value!(@object entries () ($($tt)+) ($($tt)+));
            entries
        })
    };
    ($other:expr) => {{
        use $crate::value::ToValue as _;
        (&$other).to_value()
    }};
}
//#endregion 🔖️Literal

//#region 🔖️SerDe
/// 🔀️ Materializes a `ToValue` value into a `DslValue` tree — first-party analog of the
/// former `serde::Serialize`-bound bridge, kept as `Result` for source compatibility with every
/// existing `?`/`.map_err(...)`/`.unwrap_or(...)` call site even though `ToValue::to_value` itself
/// is infallible. See
/// `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/
/// 🔍️research/📓️dsl-value-bridge-conversion.md`.
pub fn to_dsl_value<T: ToValue + ?Sized>(value: &T) -> Result<DslValue, String> {
    Ok(value.to_value())
}

/// 🔀️ Hydrates a `FromValue` value from a `DslValue` tree — first-party analog of the
/// former `serde::de::DeserializeOwned`-bound bridge.
pub fn from_dsl_value<T: FromValue>(value: DslValue) -> Result<T, String> {
    T::from_value(value).map_err(|error| error.to_string())
}
//#endregion 🔖️SerDe

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
