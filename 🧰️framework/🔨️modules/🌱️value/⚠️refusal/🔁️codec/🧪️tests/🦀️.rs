use crate::{DslValue, NativeDecodeControl, NativeEncodeControl, ValueError, ValueRefusalKind};

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap() }
fn intrinsic(value: &serde_json::Value) -> DslValue {
    match value { serde_json::Value::Null => DslValue::Null, serde_json::Value::Bool(value) => DslValue::Bool(*value), serde_json::Value::String(value) => DslValue::String(value.clone()), _ => panic!("closed fixture leaf") }
}
fn wire(row: &serde_json::Value) -> DslValue {
    DslValue::Object(vec![("kind".into(), intrinsic(&row["kind"])), ("message".into(), intrinsic(&row["message"]))])
}
#[test]
fn controlled_value_refusal_codec_closed_wire_matches_original_kind_message_and_serde() {
    let mut progress = |_| true;
    for row in fixture()["valid"].as_array().unwrap() {
        let source = wire(&row["wire"]);
        let mut decode = NativeDecodeControl::new(1_000_000, &mut progress);
        let error = ValueError::from_value_controlled(&source, &mut decode).unwrap();
        assert_eq!(error.kind.as_str(), row["wire"]["kind"].as_str().unwrap());
        assert_eq!(error.message, row["wire"]["message"].as_str().unwrap());
        let mut encoding = |_| true;
        let output = error.to_value_controlled(&mut NativeEncodeControl::new(1_000_000, &mut encoding)).unwrap();
        assert_eq!(output, source);
        assert_eq!(serde_json::to_value(&output).unwrap(), row["wire"]);
        let mut kind_encoding = |_| true;
        let encoded_kind = error.kind.to_value_controlled(&mut NativeEncodeControl::new(1_000_000, &mut kind_encoding)).unwrap();
        let mut kind_decoding = |_| true;
        assert_eq!(ValueRefusalKind::from_value_controlled(&encoded_kind, &mut NativeDecodeControl::new(1_000_000, &mut kind_decoding)).unwrap(), error.kind);
    }
    println!("[DEBUG] controlled refusal codec preserves every original kind, Unicode message and authored path against serde wire output");
}
#[test]
fn controlled_value_refusal_codec_rejects_every_non_closed_record_without_mutating_input() {
    for row in fixture()["invalid"].as_array().unwrap() {
        let source = DslValue::Object(row["members"].as_array().unwrap().iter().map(|member| (member["key"].as_str().unwrap().to_owned(), intrinsic(&member["value"]))).collect());
        let original = source.clone();
        let mut progress = |_| true;
        let error = ValueError::from_value_controlled(&source, &mut NativeDecodeControl::new(1_000_000, &mut progress)).unwrap_err();
        assert_eq!(error.kind.as_str(), row["expectedKind"].as_str().unwrap(), "{}", row["id"]);
        assert_eq!(source, original);
    }
    for source in [DslValue::Null, DslValue::Bool(true), DslValue::Array(vec![])] {
        let mut progress = |_| true;
        assert_eq!(ValueError::from_value_controlled(&source, &mut NativeDecodeControl::new(1_000_000, &mut progress)).unwrap_err().kind, ValueRefusalKind::InvalidValue);
    }
}
#[test]
fn controlled_value_refusal_codec_keeps_cancellation_and_cumulative_ownership_causes() {
    let corpus = fixture();
    let original = ValueError::new(ValueRefusalKind::InvariantViolated, corpus["longMessage"]["unit"].as_str().unwrap().repeat(corpus["longMessage"]["repeat"].as_u64().unwrap() as usize));
    let mut progress = |_| true;
    let source = original.to_value_controlled(&mut NativeEncodeControl::new(1_000_000, &mut progress)).unwrap();
    for row in corpus["controls"].as_array().unwrap() {
        let ceiling = row["maximumBytes"].as_u64().unwrap() as usize;
        let cancel_at = row["cancelAt"].as_u64().unwrap();
        let mut callbacks = 0;
        let error = if row["operation"] == "encode" {
            let mut progress = |_| { callbacks += 1; cancel_at == 0 || callbacks < cancel_at };
            original.to_value_controlled(&mut NativeEncodeControl::new(ceiling, &mut progress)).unwrap_err()
        } else {
            let mut progress = |_| { callbacks += 1; cancel_at == 0 || callbacks < cancel_at };
            ValueError::from_value_controlled(&source, &mut NativeDecodeControl::new(ceiling, &mut progress)).unwrap_err()
        };
        assert_eq!(error.kind.as_str(), row["expectedKind"].as_str().unwrap(), "{}", row["id"]);
        if cancel_at != 0 { assert_eq!(callbacks, cancel_at); }
    }
    assert_eq!(original.kind, ValueRefusalKind::InvariantViolated);
    assert_eq!(source.get("message").unwrap().as_str().unwrap(), original.message);
    println!("[DEBUG] refusal transport preserves source ownership and distinct cancellation/quota causes through long-message construction");
}
