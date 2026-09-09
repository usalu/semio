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
