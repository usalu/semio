//! 🏷️ Owned value codecs for the declaration crate’s icon vocabulary.

use super::IconName;

impl semio_framework_value::ToValue for IconName {
    fn to_value(&self) -> semio_framework_value::DslValue {
        let json = serde_json::to_value(self).expect("IconName's generated Serialize impl never fails");
        semio_framework_value::DslValue::from(&json)
    }
}

impl semio_framework_value::FromValue for IconName {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let json = serde_json::Value::from(value);
        serde_json::from_value(json).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))
    }
}

#[cfg(test)]
#[path = "../../../../🖱️ui/🧪️tests/🔬️targets-wgpu-icon-name-value-icon-name-value-round-trip/🦀️.rs"]
mod icon_name_value_round_trip_tests;
