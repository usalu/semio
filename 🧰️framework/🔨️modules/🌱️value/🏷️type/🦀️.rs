//! 🏷️ Canonical typed value schema and borrowed classification contract.

use crate::{DslValue, FromValue, ToValue, ValueError};
use crate::retirement::RetireOwned;

/// 📐️ One domain-neutral type declaration.
#[derive(Clone, Debug, Default, PartialEq, crate::RetainedClone)]
pub enum ValueType {
    Boolean,
    Integer,
    Decimal,
    Text,
    List(Box<ValueType>),
    Schema(String),
    #[default]
    Any,
}

/// 🔎️ Classification borrowed from a concrete value owner.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueKind<'a> {
    Null,
    Boolean,
    Integer,
    Decimal,
    Text,
    Dictionary(Option<&'a str>),
}

impl ValueType {
    pub fn id(&self) -> String {
        match self {
            ValueType::Boolean => "boolean".into(),
            ValueType::Integer => "integer".into(),
            ValueType::Decimal => "number".into(),
            ValueType::Text => "text".into(),
            ValueType::List(_) => "list".into(),
            ValueType::Schema(id) => id.clone(),
            ValueType::Any => "value".into(),
        }
    }

    /// 🎯️ Matches an owner-supplied classification without constructing a value carrier.
    pub fn matches(&self, value: ValueKind<'_>) -> bool {
        if value == ValueKind::Null {
            return false;
        }
        match self {
            ValueType::Any => true,
            ValueType::Boolean => value == ValueKind::Boolean,
            ValueType::Integer => value == ValueKind::Integer,
            ValueType::Decimal => matches!(value, ValueKind::Boolean | ValueKind::Integer | ValueKind::Decimal),
            ValueType::Text => value == ValueKind::Text,
            ValueType::List(_) => value == ValueKind::Dictionary(Some("list")),
            ValueType::Schema(schema) => value == ValueKind::Dictionary(Some(schema)),
        }
    }
}

impl ToValue for ValueType {
    fn to_value_controlled(&self, control: &mut crate::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
        controlled::encode(self, control)
    }
    fn to_value(&self) -> DslValue {
        match self {
            ValueType::Boolean => DslValue::object([("kind".to_string(), "boolean".to_value())]),
            ValueType::Integer => DslValue::object([("kind".to_string(), "integer".to_value())]),
            ValueType::Decimal => DslValue::object([("kind".to_string(), "decimal".to_value())]),
            ValueType::Text => DslValue::object([("kind".to_string(), "text".to_value())]),
            ValueType::List(inner) => DslValue::object([("kind".to_string(), "list".to_value()), ("of".to_string(), inner.to_value())]),
            ValueType::Schema(id) => DslValue::object([("kind".to_string(), "schema".to_value()), ("of".to_string(), id.to_value())]),
            ValueType::Any => DslValue::object([("kind".to_string(), "any".to_value())]),
        }
    }
}

impl FromValue for ValueType {
    fn from_value_controlled(value: &DslValue, control: &mut crate::NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        controlled::decode(value, control)
    }
    fn retire_decoded(self) {
        controlled::retire(self)
    }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(fields) = &value else {
            return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "expected a type object"));
        };
        let kind = value.get("kind").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, "kind"))?;
        let expected = if matches!(kind.as_str(), "list" | "schema") { 2 } else { 1 };
        if fields.len() != expected || fields.iter().any(|(key, _)| key != "kind" && key != "of") {
            return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "invalid type fields"));
        }
        match kind.as_str() {
            "boolean" => Ok(ValueType::Boolean),
            "integer" => Ok(ValueType::Integer),
            "decimal" => Ok(ValueType::Decimal),
            "text" => Ok(ValueType::Text),
            "list" => Ok(ValueType::List(Box::new(value.get("of").cloned().map(ValueType::from_value).transpose()?.ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, "of"))?))),
            "schema" => Ok(ValueType::Schema(value.get("of").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, "of"))?)),
            "any" => Ok(ValueType::Any),
            other => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("unknown ValueType kind '{other}'"))),
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

#[path = "🛬️controlled/🦀️.rs"]
mod controlled;

impl RetireOwned for ValueType {
    fn retirement(self)->Box<dyn crate::retirement::RetirementCursor> {match self {Self::List(value)=>value.retirement(),Self::Schema(value)=>value.retirement(),_=>crate::retirement::leaf(())}}
    fn retirement_birth_bytes(&self)->Option<usize> {match self {Self::List(value)=>value.retirement_birth_bytes(),Self::Schema(value)=>value.retirement_birth_bytes(),_=>Some(crate::retirement::leaf_birth_bytes::<()>())}}
    fn controlled_retirement_supported()->bool {true}
}
