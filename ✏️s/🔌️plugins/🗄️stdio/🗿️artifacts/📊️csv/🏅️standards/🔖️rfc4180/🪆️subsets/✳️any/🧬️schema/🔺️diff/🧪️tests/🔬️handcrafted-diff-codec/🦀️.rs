use super::*;
use crate::schema::snapshot::CsvField;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn field(value: &str, quoted: bool) -> CsvField {
    CsvField { value: value.into(), quoted }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn record(fields: &[(&str, bool)]) -> CsvRecord {
    CsvRecord { fields: fields.iter().map(|(v, q)| field(v, *q)).collect() }
}

#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    let a = CsvSnapshot { schema: "stdio.csv".into(), has_header: true, records: vec![record(&[("name", false), ("note, with comma", true)]), record(&[("a", false), ("b", false)]), record(&[("x", false), ("y", false)])] };
    let b = CsvSnapshot { schema: "stdio.csv".into(), has_header: false, records: vec![record(&[("new-a", true), ("new-b", false)]), record(&[("x", false), ("y", false)]), record(&[("brand [new]", true)])] };
    let cases = vec![CsvDiff::default(), CsvDiff::between(&a, &b), CsvDiff::between(&b, &a)];
    for d in cases {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = CsvDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = CsvDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}

//#region 🔖️DiffGrammarConformanceLaw
/// 🧪️ P2-P1 item 6: `dsl::parse_grammar` + `dsl::Recognizer` recognize REAL `print_diff`
/// output for several real diffs, including the `records` COLLECTION-TRIPLE production
/// (removed/modified/added) — the first real collection-triple grammar in this program.
#[semio_framework_async_macros::async_test]
async fn diff_grammar_conformance_law() {
    let grammar_text = crate::schema::diff::text::COMPONENT_GRAMMAR_SEMIO;
    let grammar = dsl::parse_grammar(grammar_text).expect("parse diff grammar");
    let recognizer = dsl::Recognizer::compile(&grammar);

    let a = CsvSnapshot { schema: "stdio.csv".into(), has_header: true, records: vec![record(&[("name", false), ("note, with comma", true)]), record(&[("a", false), ("b", false)]), record(&[("x", false), ("y", false)])] };
    let b = CsvSnapshot { schema: "stdio.csv".into(), has_header: false, records: vec![record(&[("new-a", true), ("new-b", false)]), record(&[("x", false), ("y", false)]), record(&[("brand [new]", true)])] };
    let diffs = vec![CsvDiff::default(), CsvDiff::between(&a, &b), CsvDiff::between(&b, &a)];
    for d in diffs {
        let printed = d.print_diff();
        let ok = recognizer.recognize(&printed).unwrap_or_else(|e| panic!("recognize({printed:?}) errored: {e:?}"));
        assert!(ok, "diff grammar must recognize real print_diff output {printed:?}");
    }
}
//#endregion 🔖️DiffGrammarConformanceLaw
