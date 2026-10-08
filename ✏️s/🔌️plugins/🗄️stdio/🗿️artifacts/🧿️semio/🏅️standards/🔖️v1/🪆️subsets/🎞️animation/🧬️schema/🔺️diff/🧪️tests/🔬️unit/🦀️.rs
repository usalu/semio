use super::*;

//#region AbsorbCanonical
/// 🧪️ Canonical absorb case 1 (at the innermost `keyframes` level): `Insert(2,f)` then
/// `Remove(0)` -> `{removed:[0], added:[(1,f)]}`.
#[semio_framework_async_macros::async_test]
async fn absorb_insert_then_remove_before_shifts_index() {
    let f = kf(9.0, AnimValue::Scalar { value: 9.0 });
    let mut d1: IndexedTripleDiff<AnimKeyframeDiff, AnimKeyframe> = IndexedTripleDiff { added: vec![IndexAdded { index: 2, item: f.clone() }], ..Default::default() };
    let d2: IndexedTripleDiff<AnimKeyframeDiff, AnimKeyframe> = IndexedTripleDiff { removed: vec![0], ..Default::default() };
    absorb_indexed(&mut d1, d2, |d, o| d.absorb(o), |d, item| d.apply_row(item));
    assert_eq!(d1.removed, vec![0]);
    assert_eq!(d1.added, vec![IndexAdded { index: 1, item: f }]);
    assert!(d1.modified.is_empty());
}

/// 🧪️ Canonical absorb case 2: `Insert(2,f)` then `Insert(2,g)` -> BOTH survive.
#[semio_framework_async_macros::async_test]
async fn absorb_insert_insert_same_index_both_survive() {
    let f = kf(1.0, AnimValue::Scalar { value: 1.0 });
    let g = kf(2.0, AnimValue::Scalar { value: 2.0 });
    let mut d1: IndexedTripleDiff<AnimKeyframeDiff, AnimKeyframe> = IndexedTripleDiff { added: vec![IndexAdded { index: 2, item: f.clone() }], ..Default::default() };
    let d2: IndexedTripleDiff<AnimKeyframeDiff, AnimKeyframe> = IndexedTripleDiff { added: vec![IndexAdded { index: 2, item: g.clone() }], ..Default::default() };
    absorb_indexed(&mut d1, d2, |d, o| d.absorb(o), |d, item| d.apply_row(item));
    assert_eq!(d1.added, vec![IndexAdded { index: 2, item: g }, IndexAdded { index: 3, item: f }]);
}

/// 🧪️ Canonical absorb case 3: `Insert(1,f)` then `SetField(1,v)` patches INTO the added
/// payload — no separate `modified` entry survives.
#[semio_framework_async_macros::async_test]
async fn absorb_insert_then_set_field_patches_into_added() {
    let f = kf(1.0, AnimValue::Scalar { value: 1.0 });
    let mut d1: IndexedTripleDiff<AnimKeyframeDiff, AnimKeyframe> = IndexedTripleDiff { added: vec![IndexAdded { index: 1, item: f.clone() }], ..Default::default() };
    let d2: IndexedTripleDiff<AnimKeyframeDiff, AnimKeyframe> = IndexedTripleDiff { modified: vec![IndexModified { index: 1, diff: AnimKeyframeDiff { t: Some(42.0), value: None } }], ..Default::default() };
    absorb_indexed(&mut d1, d2, |d, o| d.absorb(o), |d, item| d.apply_row(item));
    assert!(d1.modified.is_empty());
    assert_eq!(d1.added.len(), 1);
    assert_eq!(d1.added[0].item.t, 42.0);
    assert_eq!(d1.added[0].index, 1);
}

/// 🧪️ diff_codec_text_binary_roundtrip_law: hand-rolled `DiffCodec` text/binary grammar —
/// exercises the empty diff, the tri-state `name`, an `AnimValue` variant change, and all
/// three collection triples (removed/modified/added) at every nesting depth.
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    for d in demo_diff_cases() {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = SemioAnimationDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = SemioAnimationDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}
