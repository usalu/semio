//! 📚️ Source-authored JSON assets compile to owned values with independent JSON parity.
#[test]
fn compile_source_asset_without_runtime_json_decode() {
    let actual = semio_framework_value_derive::owned_json_file!("../../🧫️fixtures/📚️owned-json/🔣️.json");
    let expected: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📚️owned-json/🔣️.json")).unwrap();
    assert_eq!(serde_json::Value::from(&actual),expected);
    let array = actual.get("array").unwrap().as_array().unwrap();
    assert_eq!(array[2].as_u64(),Some(u64::MAX));
    assert_eq!(array[3].as_i64(),Some(i64::MIN));
    assert!(array[5].as_f64().unwrap().is_sign_negative());
    eprintln!("[DEBUG] compile-time authored JSON parity oracle=serde_json");
}
