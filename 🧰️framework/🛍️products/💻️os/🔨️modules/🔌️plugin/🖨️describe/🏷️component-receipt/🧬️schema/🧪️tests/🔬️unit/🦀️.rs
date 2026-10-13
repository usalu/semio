use super::{ComponentProfileV1, ComponentReceiptV1, ITERATE_COMPONENT_MAX_BYTES, SHIPPED_COMPONENT_MAX_BYTES};

/// 🧫️ The language-agnostic receipt law the TypeScript projection and the Ajv oracle run as well.
const FIXTURE: &str = include_str!("../../../🧫️fixtures/🔣️.json");

fn rows() -> Vec<(String, bool, serde_json::Value)> {
    let fixture: serde_json::Value = serde_json::from_str(FIXTURE).expect("receipt fixture json");
    fixture["receipts"].as_array().expect("receipt rows").iter().map(|row| (row["id"].as_str().expect("row id").to_owned(), row["valid"].as_bool().expect("row validity"), row["value"].clone())).collect()
}

/// 🗂️ The schema's member order (`ComponentReceiptV1.required`), the order the canonical encoding writes.
fn schema_order() -> Vec<String> {
    let schema: serde_json::Value = serde_json::from_str(include_str!("../../🔣️.json")).expect("receipt schema");
    schema["$defs"]["ComponentReceiptV1"]["required"].as_array().expect("required members").iter().map(|key| key.as_str().expect("member").to_owned()).collect()
}

/// 🖋️ The two-space, one-newline text of `value`, members in `order` and any foreign member after them.
fn text_in(value: &serde_json::Value, order: &[String]) -> Vec<u8> {
    let object = value.as_object().expect("receipt row object");
    let keys = order.iter().filter(|key| object.contains_key(*key)).cloned().chain(object.keys().filter(|key| !order.contains(key)).cloned());
    let members: Vec<String> = keys.map(|key| format!("  {}: {}", serde_json::to_string(&key).expect("key"), serde_json::to_string(&object[&key]).expect("member"))).collect();
    format!("{{\n{}\n}}\n", members.join(",\n")).into_bytes()
}

fn canonical(value: &serde_json::Value) -> Vec<u8> {
    text_in(value, &schema_order())
}

#[test]
fn admits_every_fixture_row_exactly_as_the_schema_does() {
    for (id, valid, value) in rows() {
        let decoded = ComponentReceiptV1::decode_canonical(&canonical(&value));
        assert_eq!(decoded.is_ok(), valid, "{id}: {decoded:?}");
        if let Ok(receipt) = decoded {
            assert_eq!(receipt.encode_canonical(), canonical(&value), "{id} re-encodes to its own canonical bytes");
        }
    }
}

#[test]
fn refuses_every_non_canonical_encoding_of_a_valid_receipt() {
    let (_, _, value) = rows().into_iter().find(|(id, _, _)| id == "shipped-release").expect("shipped-release row");
    assert!(ComponentReceiptV1::decode_canonical(&serde_json::to_vec(&value).expect("compact")).is_err());
    let text = String::from_utf8(canonical(&value)).expect("utf-8");
    assert!(ComponentReceiptV1::decode_canonical(text.replacen("\"pluginId\": \"note\"", "\"pluginId\": \"note\", \"pluginId\": \"note\"", 1).as_bytes()).is_err());
    let reversed: Vec<String> = schema_order().into_iter().rev().collect();
    assert!(ComponentReceiptV1::decode_canonical(&text_in(&value, &reversed)).is_err());
}

#[test]
fn binds_only_shipped_bytes_of_the_named_package_and_descriptor() {
    let (_, _, shipped) = rows().into_iter().find(|(id, _, _)| id == "shipped-release").expect("shipped-release row");
    let (_, _, iterate) = rows().into_iter().find(|(id, _, _)| id == "local-iterate").expect("local-iterate row");
    let shipped: ComponentReceiptV1 = serde_json::from_value(shipped).expect("shipped receipt");
    let iterate: ComponentReceiptV1 = serde_json::from_value(iterate).expect("iterate receipt");
    let (wasm, descriptor) = ("1".repeat(64), "2".repeat(64));
    assert!(shipped.binds("note", "semio:note", &wasm, &descriptor).is_ok());
    assert!(shipped.binds("draw", "semio:note", &wasm, &descriptor).is_err());
    assert!(shipped.binds("note", "semio:draw", &wasm, &descriptor).is_err());
    assert!(shipped.binds("note", "semio:note", &"3".repeat(64), &descriptor).is_err());
    assert!(shipped.binds("note", "semio:note", &wasm, &"3".repeat(64)).is_err());
    assert!(iterate.binds("note", "semio:note", &wasm, &descriptor).is_err());
}

#[test]
fn mirrors_the_schema_profile_policy() {
    let schema: serde_json::Value = serde_json::from_str(include_str!("../../🔣️.json")).expect("receipt schema");
    let policies = &schema["$defs"]["ComponentProfilePolicies"]["const"];
    assert_eq!(policies["wasm-release"]["maximumBytes"].as_u64(), Some(SHIPPED_COMPONENT_MAX_BYTES));
    assert_eq!(policies["wasm-prototype"]["maximumBytes"].as_u64(), Some(ITERATE_COMPONENT_MAX_BYTES));
    assert_eq!(policies["wasm-release"]["shipped"].as_bool(), Some(ComponentProfileV1::WasmRelease.shipped()));
    assert_eq!(policies["wasm-prototype"]["shipped"].as_bool(), Some(ComponentProfileV1::WasmIterate.shipped()));
    assert_eq!(schema["$defs"]["ComponentReceiptV1"]["properties"]["schema"]["const"].as_str(), Some(super::COMPONENT_RECEIPT_SCHEMA));
    assert_eq!(schema["$defs"]["ComponentReceiptV1"]["properties"]["target"]["const"].as_str(), Some(super::COMPONENT_TARGET));
}
