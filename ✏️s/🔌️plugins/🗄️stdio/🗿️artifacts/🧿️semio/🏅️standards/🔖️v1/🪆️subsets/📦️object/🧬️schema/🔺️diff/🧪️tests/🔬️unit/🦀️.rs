
use super::*;
use crate::standards::v1::subsets::object::schema::snapshot::demo_object_snapshot;
use protocol::DiffCodec;

#[semio_framework_async_macros::async_test]
async fn apply_replaces_touched_fields_only() {
    let base = demo_object_snapshot();
    let diff = SemioObjectDiff { brep: Some(None), ..Default::default() };
    let next = diff.apply(&base).expect("apply must succeed for a well-formed fixture");
    assert!(next.brep.is_none());
    assert_eq!(next.mesh, base.mesh, "untouched fields must be preserved");
}

#[semio_framework_async_macros::async_test]
async fn absorb_last_write_wins_per_field() {
    let mut d1 = SemioObjectDiff { brep: Some(None), ..Default::default() };
    let d2 = SemioObjectDiff { mesh: Some(None), ..Default::default() };
    d1.absorb(d2.clone());
    assert_eq!(d1.brep, Some(None));
    assert_eq!(d1.mesh, Some(None));
}

#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    for d in demo_diff_cases() {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = SemioObjectDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = SemioObjectDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}
