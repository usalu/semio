#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_base::schema::triples::absorb_indexed_slot;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_base::schema::triples::apply_indexed_rows;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_base::schema::triples::inverse_indexed_rows;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_base::schema::triples::validate_indexed_triple;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_base::schema::triples::IndexedRow;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_base::schema::triples::Replace;
#[allow(unused_imports)]
use crate::schema::snapshot::{SemioKitConnection, SemioKitDesign, SemioKitPiece, SemioKitSnapshot, SemioKitType};
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_model::schema::snapshot::SemioModelSnapshot;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_object::schema::snapshot::SemioObjectSnapshot;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot;
#[allow(unused_imports)]
use framework_schema::ArtifactSchema;
#[allow(unused_imports)]
use protocol::MutationDiff;
use crate::schema::diff::*;
use semio_s_artifact_stdio_semio_base::schema::triples::IndexedTripleDiff;
use crate::schema::snapshot::demo_kit_snapshot;
use protocol::{DiffBinary,DiffText};

fn rename_first_type(name: &str) -> SemioKitDiff {
    use semio_s_artifact_stdio_semio_base::schema::triples::IndexModified;
    SemioKitDiff { types: Some(IndexedTripleDiff { modified: vec![IndexModified { index: 0, diff: SemioKitTypeDiff { name: Some(name.into()), ..Default::default() } }], ..Default::default() }), ..Default::default() }
}

#[semio_framework_async_macros::async_test]
async fn apply_touches_only_named_fields() {
    let base = demo_kit_snapshot();
    let next = protocol::apply_diff_cold(&rename_first_type("renamed"), &base).expect("apply must succeed for a well-formed fixture");
    assert_eq!(next.types[0].name, "renamed");
    assert_eq!(next.types[0].id, base.types[0].id, "untouched row fields must be preserved");
    assert_eq!(next.designs, base.designs, "untouched collections must be preserved");
}

#[semio_framework_async_macros::async_test]
async fn absorb_equals_sequential_apply() {
    let base = demo_kit_snapshot();
    let (first, second) = (rename_first_type("one"), rename_first_type("two"));
    let mut absorbed = first.clone();
    absorbed.absorb(second.clone());
    let sequential = protocol::apply_diff_cold(&second, &protocol::apply_diff_cold(&first, &base).unwrap()).unwrap();
    assert_eq!(protocol::apply_diff_cold(&absorbed, &base).unwrap(), sequential);
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_base() {
    let base = demo_kit_snapshot();
    let diff = rename_first_type("renamed");
    let next = protocol::apply_diff_cold(&diff, &base).unwrap();
    let inverse = protocol::command::DiffAlgebra::inverse(&diff, &base);
    assert_eq!(protocol::apply_diff_cold(&inverse, &next).unwrap(), base);
}

#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    for d in demo_diff_cases() {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = SemioKitDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = SemioKitDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}
