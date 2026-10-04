use super::super::{ValueError, ValueRefusalKind};

#[test]
fn standard_utf8_refusals_preserve_owned_kind_display_and_decoder_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let rows = fixture["cases"].as_array().unwrap();
    for row in rows {
        let bytes: Vec<u8> = row["bytes"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u8).collect();
        let borrowed = match std::str::from_utf8(&bytes) {
            Ok(text) => serde_json::json!({"kind":null,"text":text}),
            Err(error) => {
                assert_eq!(error.to_string(), row["expected"]["display"].as_str().unwrap());
                let projected = ValueError::from(error);
                assert_eq!(projected.kind, ValueRefusalKind::InvalidValue);
                assert_eq!(projected.message, row["expected"]["display"].as_str().unwrap());
                serde_json::json!({"kind":projected.kind.as_str(),"text":null})
            }
        };
        let owned = match String::from_utf8(bytes.clone()) {
            Ok(text) => serde_json::json!({"kind":null,"text":text}),
            Err(error) => {
                assert_eq!(error.as_bytes(), bytes.as_slice());
                assert_eq!(error.to_string(), row["expected"]["display"].as_str().unwrap());
                let projected = ValueError::from(error);
                assert_eq!(projected.kind, ValueRefusalKind::InvalidValue);
                assert_eq!(projected.message, row["expected"]["display"].as_str().unwrap());
                serde_json::json!({"kind":projected.kind.as_str(),"text":null})
            }
        };
        let reference = match encoding_rs::UTF_8.decode_without_bom_handling_and_without_replacement(&bytes) {
            Some(text) => serde_json::json!({"kind":null,"text":text.as_ref()}),
            None => serde_json::json!({"kind":"invalidValue","text":null})
        };
        let expected = serde_json::json!({"kind":row["expected"]["kind"],"text":row["expected"]["text"]});
        assert_eq!(borrowed, expected, "{}", row["id"]);
        assert_eq!(owned, expected, "{}", row["id"]);
        assert_eq!(reference, expected, "{}", row["id"]);
    }
    println!("[DEBUG] standard UTF8 projections:8 corpus vectors,borrowed/owned displays preserved,encoding_rs validity/text/kind oracle agrees");
}
