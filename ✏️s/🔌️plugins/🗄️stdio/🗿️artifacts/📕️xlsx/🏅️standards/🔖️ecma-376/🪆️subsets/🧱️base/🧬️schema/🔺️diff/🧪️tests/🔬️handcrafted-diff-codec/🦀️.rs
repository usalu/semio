use super::*;
use protocol::DiffCodec;

/// 🧪️ `DiffCodec` round-trip laws over the hand-rolled `XlsxDiff` grammar — exercises every
/// `XlsxCellValue` variant (incl. `Formula.cached` and a value containing raw `,`/`:`/`[`/`]`
/// bytes-through-hex), the OPC content-types/parts/relationships triples (incl.
/// `OpcTargetMode::External`), and both `opc`/`workbook` top-level tokens together and alone.
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    let a = snapshot_a();
    let b = snapshot_b();
    let empty = XlsxSnapshot::default();

    let cases = vec![
        XlsxDiff::default(),
        <XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(&a, &b),
        <XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(&b, &a),
        <XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(&a, &empty),
        <XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(&empty, &a),
    ];
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
fn archive_comment_only_diff_and_snapshot_replay_preserve_exact_text() {
    use crate::schema::mutations::{set_snapshot, XlsxMutation};
    use protocol::{MutationDiff, OpBinary, OpText, ToValue};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🎒️zip/📦️opc/🧫️fixtures/💬️archive-comment/🔣️.json")).unwrap();
    let mut before = XlsxSnapshot::default();
    before.opc.comment = fixture["before"].as_str().unwrap().into();
    let mut after = before.clone();
    after.opc.comment = fixture["after"].as_str().unwrap().into();
    let diff = XlsxDiff::between(&before, &after);
    assert!(!diff.is_empty(), "a comment-only edit is a persisted change");
    for replay in [XlsxDiff::parse_diff(&diff.print_diff()).unwrap(), XlsxDiff::decode_diff(&diff.encode_diff().unwrap()).unwrap()] {
        assert_eq!(replay.apply(&before).unwrap(), after);
        assert_eq!(replay.inverse(&before).apply(&after).unwrap(), before);
    }
    let mut cleared = after.clone();
    cleared.opc.comment = fixture["cleared"].as_str().unwrap().into();
    let mut combined = diff;
    combined.absorb(XlsxDiff::between(&after, &cleared));
    assert_eq!(combined.apply(&before).unwrap(), cleared, "an empty comment remains an explicit edit");
    let mutation = XlsxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: after.clone() });
    for replay in [XlsxMutation::parse_op(&mutation.print_op()).unwrap(), XlsxMutation::decode_op(&mutation.encode_op().unwrap()).unwrap()] {
        assert_eq!(replay, mutation, "complete snapshot replay preserves the archive comment");
    }
    let oracle: serde_json::Value = serde_json::from_str(&protocol::os_pack::json::to_json_string(&after.to_value())).unwrap();
    assert_eq!(oracle["opc"]["comment"], fixture["after"]);
}
