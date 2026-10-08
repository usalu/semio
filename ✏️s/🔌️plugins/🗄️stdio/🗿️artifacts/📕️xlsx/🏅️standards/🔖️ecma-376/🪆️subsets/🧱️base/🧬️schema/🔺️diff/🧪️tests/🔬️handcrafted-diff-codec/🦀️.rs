use super::*;
use protocol::{DiffBinary,DiffCodec,DiffText};

/// 🧪️ `DiffCodec` round-trip laws over the hand-rolled `XlsxDiff` grammar — exercises every
/// `XlsxCellValue` variant (incl. `Formula.cached` and a value containing raw `,`/`:`/`[`/`]`
/// bytes-through-hex), the OPC content-types/parts/relationships triples (incl.
/// `OpcTargetMode::External`), and both `opc`/`xmlParts` top-level tokens together and alone.
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    let cases = demo_diff_cases();
    for d in cases {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = XlsxDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = XlsxDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}

#[test]
fn archive_comment_only_diff_preserves_exact_text() {
    use protocol::{MutationDiff, OpBinary, OpText};
use semio_framework_value::ToValue;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🎒️zip/📦️opc/🧫️fixtures/💬️archive-comment/🔣️.json")).unwrap();
    let mut before = XlsxSnapshot::default();
    before.opc.comment = fixture["before"].as_str().unwrap().into();
    let mut after = before.clone();
    after.opc.comment = fixture["after"].as_str().unwrap().into();
    let comment = |text: &serde_json::Value| XlsxDiff { opc: Some(OpcDiff { comment: Some(text.as_str().unwrap().into()), ..Default::default() }), xml_parts: None };
    let diff = comment(&fixture["after"]);
    assert!(!diff.is_empty(), "a comment-only edit is a persisted change");
    for replay in [XlsxDiff::parse_diff(&diff.print_diff()).unwrap(), XlsxDiff::decode_diff(&diff.encode_diff().unwrap()).unwrap()] {
        assert_eq!(protocol::apply_diff(&replay, &before).unwrap(), after);
        assert_eq!(protocol::apply_diff(&replay.inverse(&before), &after).unwrap(), before);
    }
    let mut cleared = after.clone();
    cleared.opc.comment = fixture["cleared"].as_str().unwrap().into();
    let mut combined = diff;
    combined.absorb(comment(&fixture["cleared"]));
    assert_eq!(protocol::apply_diff(&combined, &before).unwrap(), cleared, "an empty comment remains an explicit edit");
    let oracle: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&after.to_value())).unwrap();
    assert_eq!(oracle["opc"]["comment"], fixture["after"]);
}
