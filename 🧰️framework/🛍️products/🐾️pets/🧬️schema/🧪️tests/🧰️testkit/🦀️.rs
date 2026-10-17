//! 🧰️ The test kit the unit suites of every pets crate share: the shared fixture documents read from `🧪️tests/🧫️fixtures`, the normative schema document, typed decoding and encoding through serde, the number and bit helpers, and the exact structural comparison. Compiled for the unit tests of the contract crate and for the `testkit` feature that the unit tests of its dependants switch on.
//!
//! @see ../../🦀️.rs — the schema twin the kit decodes with
//! @see ../../../🧪️tests/🧫️fixtures — the committed vectors

use semio_framework_test_fixtures::fixture_path;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

pub fn fixture(case: &str) -> Value {
    let root = fixture_path!("../../../../🧪️tests/🧫️fixtures");
    let directory = std::fs::read_dir(&root).into_iter().flatten().flatten().map(|entry| entry.path()).find(|path| path.file_name().and_then(|name| name.to_str()).is_some_and(|name| name.ends_with(case)));
    let path = directory.unwrap_or_else(|| panic!("no fixture case {case} under {}", root.display())).join("🔣️.json");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    serde_json::from_str(&text).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

pub fn normative() -> Value {
    let path = fixture_path!("../../../../🧬️schema/🔣️.json");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    serde_json::from_str(&text).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

pub fn typed<T: DeserializeOwned>(value: &Value) -> T {
    serde_json::from_value(value.clone()).unwrap_or_else(|error| panic!("{error}: {value}"))
}

pub fn json<T: Serialize>(value: &T) -> Value {
    serde_json::to_value(value).unwrap_or_else(|error| panic!("{error}"))
}

pub fn entries(value: &Value) -> &[Value] {
    value.as_array().map_or(&[], Vec::as_slice)
}

pub fn number(value: &Value) -> f64 {
    value.as_f64().unwrap_or_else(|| panic!("not a number: {value}"))
}

pub fn bits(value: f64) -> String {
    format!("{:016x}", value.to_bits())
}

pub fn assert_same(context: &str, produced: &Value, expected: &Value) {
    match (produced, expected) {
        (Value::Number(left), Value::Number(right)) => {
            let (left, right) = (left.as_f64().unwrap_or(f64::NAN), right.as_f64().unwrap_or(f64::NAN));
            assert!(left == right, "{context}: {left} ≠ {right}");
        }
        (Value::Object(left), Value::Object(right)) => {
            assert_eq!(left.keys().collect::<Vec<_>>(), right.keys().collect::<Vec<_>>(), "{context}: keys");
            for (key, value) in left {
                assert_same(&format!("{context}/{key}"), value, &right[key]);
            }
        }
        (Value::Array(left), Value::Array(right)) => {
            assert_eq!(left.len(), right.len(), "{context}: length");
            for (index, (left, right)) in left.iter().zip(right).enumerate() {
                assert_same(&format!("{context}/{index}"), left, right);
            }
        }
        _ => assert_eq!(produced, expected, "{context}"),
    }
}
