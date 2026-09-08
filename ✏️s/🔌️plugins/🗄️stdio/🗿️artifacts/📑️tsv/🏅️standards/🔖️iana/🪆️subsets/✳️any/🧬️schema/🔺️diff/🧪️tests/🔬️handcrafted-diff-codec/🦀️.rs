
use super::*;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn row(fields: &[&str]) -> Vec<String> {
    fields.iter().map(|s| s.to_string()).collect()
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn snapshot(records: Vec<Vec<String>>, trailing_newline: bool) -> TsvSnapshot {
    TsvSnapshot { records, trailing_newline, ..TsvSnapshot::default() }
}

#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    let a = snapshot(vec![row(&["id", "name"]), row(&["1", "Oak"]), row(&["2", "Steel"])], true);
    let mut b = snapshot(vec![row(&["id", "name"]), row(&["1", "Oak, tricky [value]"]), row(&["2", "Steel"])], false);
    b.records.push(row(&["3", "new"]));
    let cases = vec![TsvDiff::default(), TsvDiff::between(&a, &b), TsvDiff::between(&b, &a)];
    for d in cases {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = TsvDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = TsvDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}
