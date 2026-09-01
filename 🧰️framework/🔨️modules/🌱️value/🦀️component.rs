//! 🌱️ `DslValue` — the schema-erased dynamic value both sides of a replication link speak.
//!
//! Lives beside the wire contract rather than inside the os DSL because it is what a schema-less
//! payload decodes to: the authority validates it, the optimistic replica applies it, and the
//! pathmap bodies `db` stores are trees of it. The DSL's own record/field/wire types build on it
//! and stay product-side.
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde/Display) — see R9

//#region 🗂️OrderedOwnership
#[path = "🗂️ordered/🦀️component.rs"]
pub mod ordered;
//#endregion 🗂️OrderedOwnership

//#region 🔁️Codec
#[path = "🔁️codec/🦀️component.rs"]
mod codec;
pub use codec::{FromValue, ToValue, ValueError};
//#endregion 🔁️Codec

//#region 🔖️Value
/// @emoji 🌱️ Dynamic JSON-equivalent literal for schema-less fields (`Shape::Value`).
#[derive(Clone, Debug, PartialEq)]
pub enum DslValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<DslValue>),
    Object(Vec<(String, DslValue)>),
}

impl DslValue {
    pub fn null() -> Self {
        Self::Null
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

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Number(n) => Some(*n),
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
            DslValue::Number(n) => serde_json::json!(*n),
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
/// `ToValue`/`FromValue` domain type routes through here — widens every JSON number to `f64` on
/// the way in, matching `DslValue::Number`'s own single-`f64` contract (same convention
/// `pack::json`'s `to_dsl_value` bridge uses for its sibling `Value` type).
impl From<&serde_json::Value> for DslValue {
    fn from(val: &serde_json::Value) -> Self {
        match val {
            serde_json::Value::Null => DslValue::Null,
            serde_json::Value::Bool(b) => DslValue::Bool(*b),
            serde_json::Value::Number(n) => DslValue::Number(n.as_f64().unwrap_or(f64::NAN)),
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
    fn serde_json_number_widens_to_f64_like_pack_json() {
        let dsl: DslValue = DslValue::from(&serde_json::json!(7));
        assert_eq!(dsl.as_f64(), Some(7.0));
    }

    #[test]
    fn serde_json_uses_the_same_json_shape_as_the_dsl_value_bridge() {
        let original = DslValue::Object(vec![
            ("name".into(), DslValue::String("saw".into())),
            ("count".into(), DslValue::Number(3.0)),
            ("tags".into(), DslValue::Array(vec![DslValue::String("a".into()), DslValue::String("b".into())])),
            ("active".into(), DslValue::Bool(true)),
            ("note".into(), DslValue::Null),
        ]);
        let expected = serde_json::Value::from(&original);
        let actual = serde_json::to_value(&original).unwrap();
        assert_eq!(actual, expected);
        assert_eq!(serde_json::from_value::<DslValue>(actual).unwrap(), original);
    }
}
