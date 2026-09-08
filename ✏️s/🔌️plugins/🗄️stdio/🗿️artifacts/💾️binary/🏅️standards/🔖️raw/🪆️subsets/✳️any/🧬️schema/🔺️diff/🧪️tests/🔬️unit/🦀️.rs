
use super::*;

#[semio_framework_async_macros::async_test]
async fn insert_then_remove_before_matches_canonical_shape() {
    // Insert(0xAA) at offset 2, then Remove 1 byte at offset 0 -- byte-level analog of the
    // line-diff canonical case: {removed:[0], added:[(1,0xAA)]}.
    let d1 = vec![ByteSplice { offset: 2, remove_len: 0, insert: vec![0xAA] }];
    let d2 = vec![ByteSplice { offset: 0, remove_len: 1, insert: vec![] }];
    let merged = absorb_splices(&d1, &d2);

    let base = BinarySnapshot { bytes: vec![1, 2, 3, 4], ..Default::default() };
    let mid = BinaryDiff { splices: d1.clone() }.apply(&base).unwrap();
    let after = BinaryDiff { splices: d2.clone() }.apply(&mid).unwrap();
    assert_eq!(BinaryDiff { splices: merged }.apply(&base).unwrap(), after);
}

#[semio_framework_async_macros::async_test]
async fn insert_insert_same_offset_both_survive() {
    let d1 = vec![ByteSplice { offset: 2, remove_len: 0, insert: vec![0xAA] }];
    let d2 = vec![ByteSplice { offset: 2, remove_len: 0, insert: vec![0xBB] }];
    let merged = absorb_splices(&d1, &d2);

    let base = BinarySnapshot { bytes: vec![1, 2, 3, 4], ..Default::default() };
    let mid = BinaryDiff { splices: d1.clone() }.apply(&base).unwrap();
    let after = BinaryDiff { splices: d2.clone() }.apply(&mid).unwrap();
    assert_eq!(BinaryDiff { splices: merged }.apply(&base).unwrap(), after);
    assert!(after.bytes.windows(2).any(|w| w == [0xBB, 0xAA]) || after.bytes.contains(&0xAA) && after.bytes.contains(&0xBB));
}

#[semio_framework_async_macros::async_test]
async fn modify_then_remove_drops_the_modify() {
    let d1 = vec![ByteSplice { offset: 0, remove_len: 1, insert: vec![0xFF] }];
    let d2 = vec![ByteSplice { offset: 0, remove_len: 1, insert: vec![] }];
    let merged = absorb_splices(&d1, &d2);

    let base = BinarySnapshot { bytes: vec![1, 2, 3], ..Default::default() };
    let mid = BinaryDiff { splices: d1.clone() }.apply(&base).unwrap();
    let after = BinaryDiff { splices: d2.clone() }.apply(&mid).unwrap();
    assert_eq!(BinaryDiff { splices: merged }.apply(&base).unwrap(), after);
}

#[semio_framework_async_macros::async_test]
async fn absorb_associative_over_a_triple() {
    let base = BinarySnapshot { bytes: vec![10, 20, 30, 40, 50], ..Default::default() };
    let d1 = BinaryDiff { splices: vec![ByteSplice { offset: 1, remove_len: 1, insert: vec![] }] };
    let d2 = BinaryDiff { splices: vec![ByteSplice { offset: 0, remove_len: 0, insert: vec![99] }] };
    let d3 = BinaryDiff { splices: vec![ByteSplice { offset: 2, remove_len: 1, insert: vec![7, 8] }] };

    let mut left = d1.clone();
    left.absorb(d2.clone());
    left.absorb(d3.clone());

    let mut mid = d2.clone();
    mid.absorb(d3.clone());
    let mut right = d1.clone();
    right.absorb(mid);

    assert_eq!(left.apply(&base).unwrap(), right.apply(&base).unwrap());
    let sequential = {
        let s1 = d1.apply(&base).unwrap();
        let s2 = d2.apply(&s1).unwrap();
        d3.apply(&s2).unwrap()
    };
    assert_eq!(left.apply(&base).unwrap(), sequential);
}

#[semio_framework_async_macros::async_test]
async fn between_roundtrip_synthetic() {
    let a = BinarySnapshot { bytes: vec![1, 2, 3, 4, 5], ..Default::default() };
    let b = BinarySnapshot { bytes: vec![1, 9, 9, 4, 5, 6], ..Default::default() };
    assert_eq!(BinaryDiff::between(&a, &b).apply(&a).unwrap(), b);
    assert_eq!(BinaryDiff::between(&b, &a).apply(&b).unwrap(), a);
    assert!(BinaryDiff::between(&a, &a).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diff_level_roundtrip() {
    let base = BinarySnapshot { bytes: vec![1, 2, 3, 4], ..Default::default() };
    let d = BinaryDiff { splices: vec![ByteSplice { offset: 1, remove_len: 2, insert: vec![9, 9, 9] }] };
    let next = d.apply(&base).unwrap();
    let inv = d.inverse(&base);
    assert_eq!(inv.apply(&next).unwrap(), base);
}

#[semio_framework_async_macros::async_test]
async fn apply_rejects_invalid_splice_without_mutating_base() {
    let base = BinarySnapshot { bytes: vec![1, 2, 3], ..Default::default() };
    let diff = BinaryDiff { splices: vec![ByteSplice { offset: 2, remove_len: 2, insert: vec![9] }] };
    assert!(diff.apply(&base).is_err());
    assert_eq!(base.bytes, vec![1, 2, 3]);
}

/// 🧪️ F6-PILOT: `DiffCodec` round-trip laws (derived via `dsl::DslDiff`).
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    use protocol::DiffCodec;
    for d in demo_diff_cases() {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = BinaryDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch for {d:?} (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff({d:?}) failed: {e}"));
        let decoded = BinaryDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch for {d:?}");
    }
}
