//! 📝️ Native text ownership for dependency hashes and inference cache entries.

use semio_framework_value::{DslValue,FromValue,ToValue};
use std::collections::BTreeMap;

pub fn encode<T: ToValue>(value: &T) -> Vec<u8> {
    semio_framework_pack_json::to_json_string(value).into_bytes()
}

pub fn decode<T: FromValue>(bytes: &[u8]) -> T {
    let text = std::str::from_utf8(bytes).expect("inference cache bytes are always UTF-8 JSON text produced by `encode`");
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("cached inference bytes must decode as the field's own Value type")
}

/// 🗺️ `BTreeMap<K, V>` has no generic [`ToValue`]/[`FromValue`] impl (the codec only covers
/// `BTreeMap<String, V>` — a JSON object needs string keys, but `F::Key` here is any
/// `Ord`-implementing type, e.g. `WeightSum`'s own `String` or a real caller's compound key), so
/// the session's whole-result gate cache (below) hand-rolls the wire shape as a `[[key, value],
/// …]` pair array instead — the same shape `serde_json` would give a `Vec<(K, V)>`.
pub fn encode_map<K: ToValue, V: ToValue>(map: &BTreeMap<K, V>) -> Vec<u8> {
    let pairs = semio_framework_value::DslValue::Array(map.iter().map(|(key, value)| semio_framework_value::DslValue::Array(vec![key.to_value(), value.to_value()])).collect());
    semio_framework_pack_json::to_json_string(&pairs).into_bytes()
}

pub fn decode_map<K: Ord + FromValue, V: FromValue>(bytes: &[u8]) -> BTreeMap<K, V> {
    let text = std::str::from_utf8(bytes).expect("inference cache bytes are always UTF-8 JSON text produced by `encode_map`");
    let parsed: DslValue = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("cached inference session bytes must decode as a key/value pair array");
    let semio_framework_value::DslValue::Array(items) = parsed else {
        panic!("cached inference session bytes must decode as a key/value pair array");
    };
    items
        .into_iter()
        .map(|item| {
            let semio_framework_value::DslValue::Array(pair) = item else {
                panic!("cached inference session entry must be a 2-element [key, value] pair");
            };
            let mut iter = pair.into_iter();
            let key_value = iter.next().expect("session entry pair has exactly 2 elements");
            let value_value = iter.next().expect("session entry pair has exactly 2 elements");
            let key = K::from_value(key_value).expect("cached inference session key must decode as the field's own Key type");
            let value = V::from_value(value_value).expect("cached inference session value must decode as the field's own Value type");
            (key, value)
        })
        .collect()
}

