//! 🌉️ DSL value bridge: the conversions between JSON values and domain-neutral DSL values. See [`super`].

use super::*;

//#region 🔖️DslValueBridge
/// 🌱️ Structural conversion from `semio_framework_value::DslValue` (the in-memory tree
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

/// ⏪️ The reverse of [`from_dsl_value`] — maps numbers variant-for-variant, so an integer stays an
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
