//! 🧰️ The fixture test kit of the schema twin: the shared vectors under `🧫️fixtures`, typed decoding, JSON projection and the
//! tolerant comparison every quiz test suite reads them with. Compiled for the schema's own tests and under the `testkit` feature.
//!
//! @see ../🔬️unit/🦀️.rs — the schema's own tests
//! @see ../../../🧫️fixtures — the shared vectors

use super::*;
use semio_framework_test_fixtures::fixture_path;
use serde::de::DeserializeOwned;
use serde_json::Value;

const TOLERANCE: f64 = 1e-12;

pub fn fixture(case: &str) -> Value {
    let root = fixture_path!("../../../🧫️fixtures");
    let directory = std::fs::read_dir(&root).into_iter().flatten().flatten().map(|entry| entry.path()).find(|path| path.file_name().and_then(|name| name.to_str()).is_some_and(|name| name.ends_with(case)));
    let path = directory.unwrap_or_else(|| panic!("no fixture case {case} under {}", root.display())).join("🔣️.json");
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

pub fn assert_close(context: &str, produced: &Value, expected: &Value) {
    match (produced, expected) {
        (Value::Number(left), Value::Number(right)) => {
            let (left, right) = (left.as_f64().unwrap_or(f64::NAN), right.as_f64().unwrap_or(f64::NAN));
            assert!((left - right).abs() <= TOLERANCE, "{context}: {left} ≠ {right}");
        }
        (Value::Object(left), Value::Object(right)) => {
            assert_eq!(left.keys().collect::<Vec<_>>(), right.keys().collect::<Vec<_>>(), "{context}: keys");
            for (key, value) in left {
                assert_close(&format!("{context}/{key}"), value, &right[key]);
            }
        }
        (Value::Array(left), Value::Array(right)) => {
            assert_eq!(left.len(), right.len(), "{context}: length");
            for (index, (left, right)) in left.iter().zip(right).enumerate() {
                assert_close(&format!("{context}/{index}"), left, right);
            }
        }
        _ => assert_eq!(produced, expected, "{context}"),
    }
}
