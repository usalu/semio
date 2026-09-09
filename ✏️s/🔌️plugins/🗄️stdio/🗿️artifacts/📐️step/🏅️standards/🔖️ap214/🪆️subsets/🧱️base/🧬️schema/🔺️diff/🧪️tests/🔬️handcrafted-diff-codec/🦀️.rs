use super::*;
use crate::schema::snapshot::{StepFileDescription, StepFileName, StepFileSchema, StepHeader};
use crate::STDIO_STEP_DOCUMENT_SCHEMA;
use protocol::DiffCodec;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn entity(id: u64, name: &str, args: Vec<StepValue>) -> StepEntity {
    StepEntity { id, name: name.into(), args, complex: Vec::new() }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn snapshot() -> StepSnapshot {
    StepSnapshot {
        schema: STDIO_STEP_DOCUMENT_SCHEMA.into(),
        header: StepHeader {
            file_description: StepFileDescription { description: vec!["a".into()], implementation_level: "2;1".into() },
            file_name: StepFileName { name: "a.step".into(), timestamp: "t".into(), author: vec!["Ueli".into()], organization: vec!["semio".into()], preprocessor_version: "pv".into(), originating_system: "sys".into(), authorization: "auth".into() },
            file_schema: StepFileSchema { schemas: vec!["AUTOMOTIVE_DESIGN".into()] },
        },
        entities: vec![
            entity(1, "CARTESIAN_POINT", vec![StepValue::String("".into()), StepValue::Aggregate(vec![StepValue::Real(0.0), StepValue::Real(1.5), StepValue::Integer(-3)])]),
            entity(2, "COMPLEX", vec![StepValue::Unset, StepValue::Derived, StepValue::Reference(7), StepValue::Enum("T".into()), StepValue::TypedValue { type_name: "LENGTH_MEASURE".into(), value: Box::new(StepValue::Real(3000.0)) }]),
        ],
    }
}

/// 🧪️ `diff_codec_text_binary_roundtrip_law`: exercises `StepValue`'s every variant (incl. the
/// recursive `Aggregate`/`TypedValue` cases), `StepComplexType`, and all three `entities`
/// collection-triple flavors (removed/modified/added) at once via a real `between()` result.
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    let a = snapshot();
    let mut b = a.clone();
    b.header.file_schema.schemas.push("CONFIG_CONTROL_DESIGN".into());
    b.header.file_name.originating_system = "changed".into();
    b.entities[0].name = "RENAMED_POINT".into();
    b.entities[0].args.push(StepValue::Aggregate(vec![StepValue::Integer(1), StepValue::Integer(2)]));
    // Exercises `StepEntityDiff.complex: Option<Vec<StepComplexType>>` directly on a MODIFIED
    // entity (not just a freshly-added one) so `enc_entity_diff`'s `complex` field is `Some`.
    b.entities[0].complex.push(StepComplexType { name: "EXTRA_TYPE".into(), args: vec![StepValue::Real(1.5), StepValue::String("hi".into())] });
    b.entities.remove(1);
    b.entities.push(entity(3, "ADDED_WITH_COMPLEX", vec![StepValue::Unset]));
    b.entities[1].complex.push(StepComplexType { name: "ANOTHER_TYPE".into(), args: vec![StepValue::Reference(42)] });

    let cases = vec![StepDiff::default(), StepDiff::between(&a, &b), StepDiff::between(&b, &a), StepDiff::between(&a, &a)];
    for d in cases {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = StepDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = StepDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}
