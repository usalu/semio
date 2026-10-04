use super::*;
use std::collections::HashMap;

use semio_framework_dsl_record::FieldSpec;
use semio_framework_dsl_record::FieldValue;
use semio_framework_dsl_record::RecordLayout;
use semio_framework_dsl_record::RecordSpec;
use semio_framework_dsl_record::RecordValue;
use semio_framework_dsl_record::Shape;

//#region 🔖️Fixtures
/// 🧬️ A small 3-field record spec exercising a few different `Shape` variants
/// (`Text`, `UInt`, `Bool`) — enough to prove the facade's wiring end to end without
/// duplicating `pack_value`'s own exhaustive wire-tag coverage.
fn sample_spec() -> RecordSpec {
    semio_framework_dsl_record::RecordSpec::new(Some("sample"), semio_framework_dsl_record::RecordLayout::Lines, vec![FieldSpec::new(1, "name", Shape::Text), FieldSpec::new(2, "age", Shape::UInt), FieldSpec::new(3, "active", Shape::Bool)])
}

fn sample_record() -> RecordValue {
    let mut fields = semio_framework_dsl_record::RecordValue::default().fields;
    fields.insert(1, semio_framework_dsl_record::FieldValue::Text("Ada Lovelace".to_string()));
    fields.insert(2, semio_framework_dsl_record::FieldValue::UInt(42));
    fields.insert(3, semio_framework_dsl_record::FieldValue::Bool(true));
    RecordValue { fields }
}
//#endregion 🔖️Fixtures

//#region 🔖️Document
#[test]
fn facade_encode_document_decode_document_round_trip() {
    let spec = sample_spec();
    let record = sample_record();

    let bytes = encode_document(&spec, &record, &EncodeOptions::default()).unwrap();
    let (decoded, report) = decode_document(&bytes, &spec, &DecodeOptions::default()).unwrap();

    assert_eq!(decoded.get(1), Some(&FieldValue::Text("Ada Lovelace".to_string())));
    assert_eq!(decoded.get(2), Some(&FieldValue::UInt(42)));
    assert_eq!(decoded.get(3), Some(&FieldValue::Bool(true)));
    assert!(report.unknown_field_ids.is_empty());
    assert!(!report.schema_drift);
}

#[test]
fn facade_content_hash_is_stable_across_two_encodes() {
    let spec = sample_spec();
    let record = sample_record();
    let options = EncodeOptions::default();

    let bytes_a = encode_document(&spec, &record, &options).unwrap();
    let bytes_b = encode_document(&spec, &record, &options).unwrap();

    // 🔒️ `encode_document` is a pure function of `(spec, record)` — byte-identical output.
    assert_eq!(bytes_a, bytes_b);

    let hash_a = content_hash(&bytes_a).unwrap();
    let hash_b = content_hash(&bytes_b).unwrap();
    assert_eq!(hash_a, hash_b);
}
//#endregion 🔖️Document
