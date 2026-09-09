use super::*;
use protocol::DiffCodec;

/// 🧪️ F6: `DiffCodec` round-trip laws over the hand-rolled `DocxDiff` grammar — exercises the
/// recursive enum tree (`DocxBlockDiff`'s `Paragraph`/`Table` variants, incl. a nested
/// table-cell block list), both `style`/`based_on` tri-states, the OPC layer's content-types/
/// parts/relationships-by-owner triples, and every removed/modified/added flavor via a real
/// `between()` result in both directions.
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    let a = snapshot_a();
    let b = snapshot_b();
    let cases = vec![DocxDiff::default(), DocxDiff::between(&a, &b), DocxDiff::between(&b, &a), DocxDiff::between(&a, &a)];
    for d in cases {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = DocxDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = DocxDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }

    // Field sweep: confirm every collection flavor and both tri-states actually got exercised
    // above, not just "it round-trips" (an all-`None`/empty diff would round-trip trivially).
    let diff_ab = DocxDiff::between(&a, &b);
    let opc_diff = diff_ab.opc.as_ref().expect("opc diff present");
    assert!(opc_diff.content_types.as_ref().expect("content_types diff present").defaults.as_ref().expect("defaults diff present").added.len() > 0);
    let parts = opc_diff.parts.as_ref().expect("parts diff present");
    assert!(!parts.removed.is_empty() && !parts.modified.is_empty() && !parts.added.is_empty(), "opc.parts: not every flavor exercised");
    let rels = opc_diff.relationships.as_ref().expect("relationships diff present");
    assert!(!rels.removed.is_empty() && !rels.added.is_empty(), "opc.relationships: owner removed/added not exercised");
    let doc_diff = diff_ab.document.as_ref().expect("document diff present");
    let body_diff = doc_diff.body.as_ref().expect("body diff present");
    assert!(!body_diff.removed.is_empty(), "body: removed not exercised");
    assert_eq!(body_diff.modified.len(), 1);
    let DocxBlockDiff::Paragraph(p_diff) = &body_diff.modified[0].diff else { panic!("expected paragraph diff") };
    assert_eq!(p_diff.style, Some(Some("keep".to_string())), "style tri-state Some(Some(_)) not exercised");
    let runs_diff = p_diff.runs.as_ref().expect("runs diff present");
    assert!(!runs_diff.modified.is_empty() && !runs_diff.added.is_empty(), "runs: modified/added not exercised");
    let styles_diff = doc_diff.styles.as_ref().expect("styles diff present");
    assert!(!styles_diff.removed.is_empty() && !styles_diff.added.is_empty(), "styles: removed/added not exercised");
    let style_mod = styles_diff.modified.iter().find(|m| m.key == "keep").expect("keep style modified");
    assert_eq!(style_mod.diff.based_on, Some(None), "based_on tri-state Some(None) not exercised");
}
