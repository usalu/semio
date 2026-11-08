//! 📐️ Grammar, bridge, property, member, refusal, read-policy, receipt, receiving and owned-byte laws of pack-json.
include!("../🧰️support/🦀️.rs");

mod tests {
    use super::*;
    include!("../../🔬️unit/📐️grammar-laws/🦀️.rs");
    include!("../../🔬️unit/🌉️value-bridge-and-macro-laws/🦀️.rs");
    include!("../../🔬️unit/🎲️property-and-differential-laws/🦀️.rs");
}

#[path = "../🔢️quoted-unsigned/🦀️.rs"]
mod quoted_unsigned_tests;

#[path = "../../🧩️members/🦀️.rs"]
mod member_tests;

#[path = "../../⚠️refusal/🦀️.rs"]
mod refusal_tests;

#[path = "../../../📥️decode/🛂️policy/🧪️tests/🦀️.rs"]
mod read_policy_tests;

#[path = "../../../📥️decode/🧾️receipt/🧪️tests/🦀️.rs"]
mod normal_refusal_receipt_tests;

#[path = "../../../📥️decode/🔣️string/📦️bytes/🧪️tests/🦀️.rs"]
mod original_string_byte_tests;

#[path = "../📥️receiving/🦀️.rs"]
mod original_receiving_tests;

#[path = "../🧮️owned-bytes/🦀️.rs"]
mod owned_bytes_laws;

#[path = "../🧵️operation/🦀️.rs"]
pub mod native_operation;

#[test]
fn owned_json_member_transfer_keeps_independent_key_value_and_backing_custody() {
    crate::warm();
    let mut key = String::with_capacity(128);
    key.push_str("second");
    let key_pointer = key.as_ptr();
    let mut object = Object::new();
    object.insert("first", Value::Bool(true));
    object.insert(key, Value::String("payload".into()));
    let independent: serde_json::Value = serde_json::from_str(r#"{"first":true,"second":"payload"}"#).expect("independent object");
    assert_eq!(object.last_member().expect("member").0.capacity(), 128);
    let backing = object.member_storage_bytes();
    let (key, value) = object.pop_member().expect("owned member");
    assert_eq!(key.as_ptr(), key_pointer);
    assert_eq!(value.as_str(), independent[&key].as_str());
    assert_eq!(object.member_storage_bytes(), backing);
    assert_eq!(object.pop_member().expect("first member").1.as_bool(), independent["first"].as_bool());
    assert!(object.last_member().is_none());
    assert_eq!(object.member_storage_bytes(), backing);
}
