//! 🧪️ The language-neutral intrinsic corpus preserves values, framing and caller authority.
use crate::intrinsic::{decode, IntrinsicFormat};
use semio_framework_dsl_record::{FieldSpec, FieldValue, RecordFields, RecordLayout, RecordSpec, RecordValue, Shape};
use semio_framework_value::{DslValue, NativeDecodeControl};
const FIELD_ID: u16 = 1;
use serde_json::Value;

fn fixture() -> Value { serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap() }
fn value(row: &Value) -> DslValue {
    if row["kind"] == "bytes" { DslValue::Bytes(row["expected"].as_array().unwrap().iter().map(|byte| byte.as_u64().unwrap() as u8).collect()) } else { DslValue::from(&row["expected"]) }
}
fn encoded(value: DslValue, format: IntrinsicFormat) -> Vec<u8> {
    let spec = RecordSpec::new(None, RecordLayout::Lines, vec![FieldSpec::new(FIELD_ID, "value", Shape::Value)]);
    let record = RecordValue { fields: [(FIELD_ID, FieldValue::Value(value))].into_iter().collect() };
    match format { IntrinsicFormat::Body => crate::record::encode_record_body(&spec, &record, &Default::default()).unwrap(), IntrinsicFormat::Document => crate::record::encode_document(&spec, &record, &Default::default()).unwrap() }
}

#[test]
fn intrinsic_decoding_preserves_the_language_neutral_corpus_and_serde_oracle() {
    let rows = fixture();
    for row in rows["cases"].as_array().unwrap() {
        for format in [IntrinsicFormat::Body, IntrinsicFormat::Document] {
            let bytes = encoded(value(row), format);
            let mut steps = 0;
            let mut callback = |_| { steps += 1; true };
            let mut control = NativeDecodeControl::new(16_777_216, &mut callback);
            let actual = decode(&bytes, format, &Default::default(), &mut control).unwrap();
            assert_eq!(serde_json::Value::from(actual), row["expected"], "{} {format:?}", row["id"]);
            drop(control);
            assert!(steps > 0);
        }
    }
    eprintln!("[DEBUG] intrinsic decoding eleven corpus values match serde_json across both physical formats");
}

#[test]
fn intrinsic_decoding_refuses_an_absent_or_foreign_document_field() {
    for fields in [RecordFields::default(), [(2, FieldValue::Value(DslValue::Null))].into_iter().collect()] {
        let id = if fields.get(&2).is_some() { 2 } else { FIELD_ID };
        let spec = RecordSpec::new(None, RecordLayout::Lines, vec![FieldSpec::new(id, "value", Shape::Value)]);
        let bytes = crate::record::encode_document(&spec, &RecordValue { fields }, &Default::default()).unwrap();
        let mut callback = |_| true;
        let mut control = NativeDecodeControl::new(16_777_216, &mut callback);
        assert!(decode(&bytes, IntrinsicFormat::Document, &Default::default(), &mut control).is_err());
    }
}

#[test]
fn intrinsic_decoding_refuses_a_trailing_or_truncated_body() {
    let bytes = encoded(DslValue::String("terminal".into()), IntrinsicFormat::Body);
    let mut trailing = bytes.clone(); trailing.push(0);
    for malformed in [&trailing[..], &bytes[..bytes.len() - 1]] {
        let mut callback = |_| true;
        let mut control = NativeDecodeControl::new(16_777_216, &mut callback);
        assert!(decode(malformed, IntrinsicFormat::Body, &Default::default(), &mut control).is_err());
    }
}

#[test]
fn intrinsic_decoding_obeys_the_original_caller_cancellation_and_allocation_ceiling() {
    for format in [IntrinsicFormat::Body, IntrinsicFormat::Document] {
        let bytes = encoded(DslValue::String("bounded".repeat(1024)), format);
        let mut cancel = |_| false;
        let mut canceled = NativeDecodeControl::new(16_777_216, &mut cancel);
        assert!(decode(&bytes, format, &Default::default(), &mut canceled).is_err());
        let mut admit = |_| true;
        let mut bounded = NativeDecodeControl::new(1, &mut admit);
        assert!(decode(&bytes, format, &Default::default(), &mut bounded).is_err());
        assert!(bounded.owned_bytes() <= bounded.maximum_bytes());
    }
}
