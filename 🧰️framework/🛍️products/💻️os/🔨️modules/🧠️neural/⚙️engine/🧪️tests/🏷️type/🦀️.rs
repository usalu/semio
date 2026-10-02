//! 🔮️ Independent test-only Serde reference for the neutral type wire.

use super::ValueType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind", content = "of")]
enum ReferenceType {
    Boolean,
    Integer,
    Decimal,
    Text,
    List(Box<ReferenceType>),
    Schema(String),
    Any,
}

impl From<&ValueType> for ReferenceType {
    fn from(value: &ValueType) -> Self {
        match value {
            ValueType::Boolean => Self::Boolean,
            ValueType::Integer => Self::Integer,
            ValueType::Decimal => Self::Decimal,
            ValueType::Text => Self::Text,
            ValueType::List(inner) => Self::List(Box::new(Self::from(inner.as_ref()))),
            ValueType::Schema(id) => Self::Schema(id.clone()),
            ValueType::Any => Self::Any,
        }
    }
}

impl From<ReferenceType> for ValueType {
    fn from(value: ReferenceType) -> Self {
        match value {
            ReferenceType::Boolean => Self::Boolean,
            ReferenceType::Integer => Self::Integer,
            ReferenceType::Decimal => Self::Decimal,
            ReferenceType::Text => Self::Text,
            ReferenceType::List(inner) => Self::List(Box::new(Self::from(*inner))),
            ReferenceType::Schema(id) => Self::Schema(id),
            ReferenceType::Any => Self::Any,
        }
    }
}

pub fn serialize<S: serde::Serializer>(value: &ValueType, serializer: S) -> Result<S::Ok, S::Error> {
    ReferenceType::from(value).serialize(serializer)
}

pub fn deserialize<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<ValueType, D::Error> {
    ReferenceType::deserialize(deserializer).map(ValueType::from)
}

#[test]
fn neutral_value_type_wire_matches_the_independent_serde_reference() {
    use semio_framework_value::{DslValue, FromValue, ToValue};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🌱️value/🏷️type/🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["wire"].as_array().unwrap() {
        let subject = ValueType::from_value(DslValue::from(row.clone())).unwrap();
        let reference: ReferenceType = serde_json::from_value(row.clone()).unwrap();
        assert_eq!(serde_json::to_value(&reference).unwrap(), serde_json::Value::from(subject.to_value()));
        assert_eq!(ValueType::from(reference), subject);
    }
}
