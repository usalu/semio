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
//#endregion 🗂️OrderedOwnership

//#region 🔁️Codec
#[path = "🔁️codec/🦀️.rs"]
mod codec;
pub use codec::{FromValue, ToValue, ValueError};
//#endregion 🔁️Codec

//#region 🔖️Number
/// @emoji 🔢️ A JSON-equivalent number that keeps the writer's-eye distinction a bare `f64` erases:
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
//#endregion 🔖️Number

//#region 🔖️Value
/// @emoji 🌱️ Dynamic JSON-equivalent literal for schema-less fields (`Shape::Value`).
#[derive(Clone, Debug, PartialEq)]
pub enum DslValue {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<DslValue>),
    Object(Vec<(String, DslValue)>),
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

//#region 🔖️SerDe
/// @emoji 🔀️ Materializes a `ToValue` value into a `DslValue` tree — first-party analog of the
/// former `serde::Serialize`-bound bridge, kept as `Result` for source compatibility with every
/// existing `?`/`.map_err(...)`/`.unwrap_or(...)` call site even though `ToValue::to_value` itself
/// is infallible. See
/// `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/
/// 🔍️research/📓️dsl-value-bridge-conversion.md`.
pub fn to_dsl_value<T: ToValue>(value: &T) -> Result<DslValue, String> {
    Ok(value.to_value())
}

/// @emoji 🔀️ Hydrates a `FromValue` value from a `DslValue` tree — first-party analog of the
/// former `serde::de::DeserializeOwned`-bound bridge.
pub fn from_dsl_value<T: FromValue>(value: DslValue) -> Result<T, String> {
    T::from_value(value).map_err(|error| error.to_string())
}
//#endregion 🔖️SerDe

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_json_value_round_trips_through_dsl_value() {
        let original = serde_json::json!({"name": "saw", "count": 3, "tags": ["a", "b"], "active": true, "note": null});
        let dsl: DslValue = DslValue::from(&original);
        let back = serde_json::Value::from(&dsl);
        assert_eq!(back, original);
    }

    #[test]
    fn serde_json_integer_stays_exact_not_widened_to_f64() {
        let dsl: DslValue = DslValue::from(&serde_json::json!(7));
        assert_eq!(dsl.as_u64(), Some(7));
        assert_eq!(dsl.as_f64(), Some(7.0));
        assert!(matches!(dsl, DslValue::Number(Number::UInt(7))));
    }

    #[test]
    fn serde_json_negative_integer_becomes_int_variant() {
        let dsl: DslValue = DslValue::from(&serde_json::json!(-7));
        assert_eq!(dsl.as_i64(), Some(-7));
        assert!(matches!(dsl, DslValue::Number(Number::Int(-7))));
    }

    #[test]
    fn serde_json_float_stays_float_variant() {
        let dsl: DslValue = DslValue::from(&serde_json::json!(7.5));
        assert!(matches!(dsl, DslValue::Number(Number::Float(v)) if v == 7.5));
    }

    #[test]
    fn uint_round_trips_as_bare_integer_text_through_serde_json_value() {
        let dsl = DslValue::uint(3600);
        let json = serde_json::Value::from(&dsl);
        assert_eq!(json.to_string(), "3600");
        assert_eq!(DslValue::from(&json).as_u64(), Some(3600));
    }

    #[test]
    fn whole_float_keeps_its_decimal_point_through_serde_json_value() {
        let dsl = DslValue::float(3600.0);
        let json = serde_json::Value::from(&dsl);
        assert_eq!(json.to_string(), "3600.0");
    }

    /// 🪆️ Object-key-order-insensitive equality — a JSON object's key order carries no semantics,
    /// and `DslValue::Object`'s `Vec`-backed derived `PartialEq` is positional, so a value that
    /// round-tripped through `serde_json`'s (key-sorting) `Map` legitimately comes back with a
    /// different entry order than it started with. Recurses into `Array`/`Object` children.
    fn dsl_value_eq_ignoring_object_order(a: &DslValue, b: &DslValue) -> bool {
        match (a, b) {
            (DslValue::Array(x), DslValue::Array(y)) => x.len() == y.len() && x.iter().zip(y).all(|(x, y)| dsl_value_eq_ignoring_object_order(x, y)),
            (DslValue::Object(x), DslValue::Object(y)) => {
                x.len() == y.len() && x.iter().all(|(k, v)| y.iter().find(|(ok, _)| ok == k).is_some_and(|(_, ov)| dsl_value_eq_ignoring_object_order(v, ov)))
            }
            _ => a == b,
        }
    }

    #[test]
    fn serde_json_uses_the_same_json_shape_as_the_dsl_value_bridge() {
        let original = DslValue::Object(vec![
            ("name".into(), DslValue::String("saw".into())),
            ("count".into(), DslValue::uint(3)),
            ("tags".into(), DslValue::Array(vec![DslValue::String("a".into()), DslValue::String("b".into())])),
            ("active".into(), DslValue::Bool(true)),
            ("note".into(), DslValue::Null),
        ]);
        let expected = serde_json::Value::from(&original);
        let actual = serde_json::to_value(&original).unwrap();
        assert_eq!(actual, expected);
        let round_tripped = serde_json::from_value::<DslValue>(actual).unwrap();
        assert!(dsl_value_eq_ignoring_object_order(&round_tripped, &original), "{round_tripped:?} != {original:?}");
    }
}
