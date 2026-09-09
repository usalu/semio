//! 📦️ Boxed field ownership preserves the neutral DSL value and operation wire shape.

use super::*;

#[derive(Clone, Debug, PartialEq, DslRecord, serde::Serialize, serde::Deserialize)]
struct BoxedFieldRecord {
    label: String,
    count: u32,
}

#[derive(Clone, Debug, PartialEq, DslOps)]
enum BoxedFieldOperation {
    #[dsl(key = "replace")]
    Replace {
        #[dsl(block)]
        config: Box<BoxedFieldRecord>,
    },
}

#[derive(Clone, Debug, PartialEq, DslOps)]
enum InlineFieldOperation {
    #[dsl(key = "replace")]
    Replace {
        #[dsl(block)]
        config: BoxedFieldRecord,
    },
}

fn boxed_field_vectors() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/📦️boxed-fields/🔣️.json")).expect("neutral boxed field vectors")
}

#[test]
fn boxed_dsl_fields_match_neutral_values_and_serde() {
    let vectors = boxed_field_vectors();
    let reference: Box<String> = serde_json::from_value(vectors["text"].clone()).expect("serde boxed string");
    let value = FieldValue::Text(reference.as_ref().clone());
    let restored = <Box<String> as DslField>::from_value(&value).expect("boxed text field");
    assert_eq!(restored, reference);
    assert_eq!(DslField::to_value(&restored), value);
    assert!(matches!(<Box<String> as DslField>::shape(), Shape::Text));
    assert_eq!(<Box<String> as DslField>::from_value(&FieldValue::UInt(7)).unwrap_err(), <String as DslField>::from_value(&FieldValue::UInt(7)).unwrap_err());
    let reference: Box<BoxedFieldRecord> = serde_json::from_value(vectors["record"].clone()).expect("serde boxed record");
    let value = DslField::to_value(reference.as_ref());
    let restored = <Box<BoxedFieldRecord> as DslField>::from_value(&value).expect("boxed record field");
    assert_eq!(DslField::to_value(&restored), value);
    assert_eq!(serde_json::to_value(&restored).expect("serde restored record"), vectors["record"]);
    assert_eq!(restored, reference);
    println!("[DEBUG] Boxed scalar and record values match the neutral serde oracle and retain inner errors");
}

#[test]
fn boxed_dsl_operation_matches_unboxed_text_and_binary() {
    let vectors = boxed_field_vectors();
    let config: BoxedFieldRecord = serde_json::from_value(vectors["record"].clone()).expect("serde neutral record");
    let inline = InlineFieldOperation::Replace { config: config.clone() };
    let boxed = BoxedFieldOperation::Replace { config: Box::new(config) };
    let inline_bytes = variants_binary::encode_op(&inline).expect("inline binary");
    let boxed_bytes = variants_binary::encode_op(&boxed).expect("boxed binary");
    assert_eq!(boxed_bytes, inline_bytes);
    assert_eq!(variants_binary::decode_op::<BoxedFieldOperation>(&inline_bytes).expect("boxed binary round trip"), boxed);
    assert_eq!(variants_binary::decode_op::<InlineFieldOperation>(&boxed_bytes).expect("inline binary round trip"), inline);
    let (inline_key, inline_record) = inline.to_named_record();
    let (boxed_key, boxed_record) = boxed.to_named_record();
    assert_eq!(boxed_key, inline_key);
    assert_eq!(boxed_record, inline_record);
    let inline_spec = InlineFieldOperation::variants()[0].1();
    let boxed_spec = BoxedFieldOperation::variants()[0].1();
    let inline_text = print(&inline_record, &inline_spec, JoinMode::Inline);
    let boxed_text = print(&boxed_record, &boxed_spec, JoinMode::Inline);
    assert_eq!(boxed_text, inline_text);
    let parsed = parse(&boxed_text, &boxed_spec, &ParseOptions { limits: Limits::default(), mode: SourceMode::Inline }).expect("boxed text parse");
    assert_eq!(BoxedFieldOperation::from_named_record(&boxed_key, &parsed).expect("boxed text round trip"), boxed);
    let BoxedFieldOperation::Replace { config } = boxed;
    assert_eq!(serde_json::to_value(config).expect("serde operation payload"), vectors["record"]);
    println!("[DEBUG] Boxed and inline operations share text and binary bytes with the neutral record oracle");
}
