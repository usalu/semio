use super::*;

/// 🧪️ F6: `diff_codec_text_binary_roundtrip_law` — exercises every scalar field plus all
/// three sections (`removed`/`modified`/`added`) of the `palette` collection triple, via a
/// real `between()` result (`f6-recon-report.md` §9 STEP-3's mandated shape).
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    use protocol::DiffCodec;

    for d in demo_diff_cases() {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = BmpDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch for {d:?} (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff({d:?}) failed: {e}"));
        let decoded = BmpDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch for {d:?}");
    }

    // 🔍 Sanity: `a->b` must actually populate both `modified` and `added` (the collection
    // triple's own coverage, not just the codec round-trip).
    let a = demo_snap_a();
    let b = demo_snap_b();
    let ab = BmpDiff::between(&a, &b);
    let pd = ab.palette.as_ref().expect("palette diff must be populated a->b");
    assert!(pd.removed.is_empty(), "a->b must not need a removal (palette grows)");
    assert!(!pd.modified.is_empty(), "a->b must show the modified entry");
    assert!(!pd.added.is_empty(), "a->b must show the added entry");

    let ba = BmpDiff::between(&b, &a);
    let pd_ba = ba.palette.as_ref().expect("palette diff must be populated b->a");
    assert!(!pd_ba.removed.is_empty(), "b->a must show the removed entry");
}
