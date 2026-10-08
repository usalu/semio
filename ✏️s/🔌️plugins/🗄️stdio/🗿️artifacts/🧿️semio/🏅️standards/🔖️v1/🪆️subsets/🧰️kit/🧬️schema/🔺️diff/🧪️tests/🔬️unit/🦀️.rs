use super::*;
use crate::standards::v1::subsets::base::schema::triples::IndexedTripleDiff;
use crate::standards::v1::subsets::kit::schema::snapshot::demo_kit_snapshot;
use protocol::{DiffBinary,DiffCodec,DiffText};

fn rename_first_type(name: &str) -> SemioKitDiff {
    use crate::standards::v1::subsets::base::schema::triples::IndexModified;
    SemioKitDiff { types: Some(IndexedTripleDiff { modified: vec![IndexModified { index: 0, diff: SemioKitTypeDiff { name: Some(name.into()), ..Default::default() } }], ..Default::default() }), ..Default::default() }
}

#[semio_framework_async_macros::async_test]
async fn apply_touches_only_named_fields() {
    let base = demo_kit_snapshot();
    let next = protocol::apply_diff(&rename_first_type("renamed"), &base).expect("apply must succeed for a well-formed fixture");
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
    let sequential = protocol::apply_diff(&second, &protocol::apply_diff(&first, &base).unwrap()).unwrap();
    assert_eq!(protocol::apply_diff(&absorbed, &base).unwrap(), sequential);
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_base() {
    let base = demo_kit_snapshot();
    let diff = rename_first_type("renamed");
    let next = protocol::apply_diff(&diff, &base).unwrap();
    let inverse = protocol::command::DiffAlgebra::inverse(&diff, &base);
    assert_eq!(protocol::apply_diff(&inverse, &next).unwrap(), base);
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
