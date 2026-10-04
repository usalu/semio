//! 🔢️ Native numeric refusal authority separates caller work bounds from malformed values.
use semio_framework_dsl_record::FieldSpec;
use semio_framework_dsl_record::ParseOptions;
use semio_framework_dsl_record::RecordLayout;
use semio_framework_dsl_record::RecordSpec;
use semio_framework_dsl_record::Shape;
use semio_framework_dsl_record::parse_exact_controlled;
use semio_framework_value::NativeDecodeControl;

#[test]
fn os_controlled_numeric_token_bounds_preserve_distinct_work_and_value_refusals() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🔨️modules/🗣️dsl/🧬️schema/🧫️fixtures/🔢️number-refusal/🔣️.json")).unwrap();
    let spec = semio_framework_dsl_record::RecordSpec::new(Some("measure"), semio_framework_dsl_record::RecordLayout::Inline, vec![FieldSpec::new(1, "value", Shape::Float)]);
    for row in corpus["cases"].as_array().unwrap() {
        let fragment = row["fragment"].as_str().unwrap().repeat(row["repeat"].as_u64().unwrap() as usize);
        let text = format!("measure value={fragment}");
        let mut callback = |_| true;
        let mut control = NativeDecodeControl::new(1_048_576, &mut callback);
        let error = parse_exact_controlled(&text, &spec, &semio_framework_dsl_record::ParseOptions::default(), &mut control).unwrap_err();
        assert_eq!(serde_json::json!({"kind": error.kind.as_str(), "message": error.message}), row["expected"], "{}", row["id"]);
    }
}
