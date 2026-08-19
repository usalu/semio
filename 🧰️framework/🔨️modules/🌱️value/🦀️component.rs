//! 🌱️ `DslValue` — the schema-erased dynamic value both sides of a replication link speak.
//!
//! Lives beside the wire contract rather than inside the os DSL because it is what a schema-less
//! payload decodes to: the authority validates it, the optimistic replica applies it, and the
//! pathmap bodies `db` stores are trees of it. The DSL's own record/field/wire types build on it
//! and stay product-side.
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde/Display) — see R9

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
#[path = "🔀️serde/🦀️component.rs"]
mod dsl_value_serde;

/// @emoji 🔀️ Materializes a `serde::Serialize` value into a `DslValue` tree (no JSON text).
pub fn to_dsl_value<T: serde::Serialize>(value: &T) -> Result<DslValue, String> {
    value.serialize(dsl_value_serde::ValueSerializer).map_err(|error| error.to_string())
}

/// @emoji 🔀️ Hydrates a `serde::DeserializeOwned` value from a `DslValue` tree (no JSON text).
pub fn from_dsl_value<T: serde::de::DeserializeOwned>(value: DslValue) -> Result<T, String> {
    T::deserialize(&mut dsl_value_serde::ValueDeserializer::new(value)).map_err(|error| error.to_string())
}
//#endregion 🔖️SerDe
