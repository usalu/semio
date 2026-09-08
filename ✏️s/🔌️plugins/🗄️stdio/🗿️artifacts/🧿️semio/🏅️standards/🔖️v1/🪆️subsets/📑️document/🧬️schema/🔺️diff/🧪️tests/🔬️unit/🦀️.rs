
use super::*;
use protocol::DiffCodec;

/// 🧪️ `DiffCodec` round-trip laws — exercises the recursive enum tree (`DocBlockDiff`'s
/// Paragraph/Table variants, incl. a nested table-cell block list), tri-states, and every
/// removed/modified/added flavor via a real `between()` result in both directions.
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    let a = snapshot_a();
    let b = snapshot_b();
    let cases = vec![SemioDocumentDiff::default(), SemioDocumentDiff::between(&a, &b), SemioDocumentDiff::between(&b, &a), SemioDocumentDiff::between(&a, &a)];
    for d in cases {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = SemioDocumentDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = SemioDocumentDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }

    let diff_ab = SemioDocumentDiff::between(&a, &b);
    let styles_diff = diff_ab.styles.as_ref().expect("styles diff present");
    assert!(!styles_diff.removed.is_empty() && !styles_diff.modified.is_empty() && !styles_diff.added.is_empty(), "styles: not every flavor exercised");
    let style_mod = styles_diff.modified.iter().find(|m| m.key == "keep").expect("keep style modified");
    assert_eq!(style_mod.diff.based_on, Some(None), "based_on tri-state Some(None) not exercised");
    let images_diff = diff_ab.images.as_ref().expect("images diff present");
    assert!(!images_diff.removed.is_empty() && !images_diff.added.is_empty());
    let blocks_diff = diff_ab.blocks.as_ref().expect("blocks diff present");
    assert!(!blocks_diff.removed.is_empty(), "blocks: removed not exercised");
    assert_eq!(blocks_diff.modified.len(), 1);
    let DocBlockDiff::Paragraph(p_diff) = &blocks_diff.modified[0].diff else { panic!("expected paragraph diff") };
    assert_eq!(p_diff.style_id, Some(Some("keep".to_string())), "style_id tri-state Some(Some(_)) not exercised");
    let runs_diff = p_diff.runs.as_ref().expect("runs diff present");
    assert!(!runs_diff.modified.is_empty() && !runs_diff.added.is_empty(), "runs: modified/added not exercised");
}
