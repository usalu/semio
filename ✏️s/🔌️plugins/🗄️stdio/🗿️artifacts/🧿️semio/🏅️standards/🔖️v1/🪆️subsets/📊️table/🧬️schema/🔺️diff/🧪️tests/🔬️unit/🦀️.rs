
use super::*;
use crate::standards::v1::subsets::table::schema::snapshot::{STDIO_SEMIOTABLE_DOCUMENT_SCHEMA, SemioTableCellKind};
use crate::standards::v1::subsets::value::schema::snapshot::SemioValue;
use protocol::DiffCodec;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn one_col_row(name: &str, kind: SemioTableCellKind, value: SemioValue) -> SemioTableSnapshot {
    SemioTableSnapshot { schema: STDIO_SEMIOTABLE_DOCUMENT_SCHEMA.into(), columns: vec![SemioTableColumn { name: name.into(), kind }], rows: vec![SemioTableRow { cells: vec![value] }] }
}

#[semio_framework_async_macros::async_test]
async fn apply_replaces_columns_and_rows_wholesale() {
    let base = one_col_row("a", SemioTableCellKind::Str, SemioValue::Str { value: "x".into() });
    let diff = SemioTableDiff {
        columns: Some(SemioTableColumnList { values: vec![SemioTableColumn { name: "b".into(), kind: SemioTableCellKind::Int }] }),
        rows: Some(SemioTableRowList { values: vec![SemioTableRow { cells: vec![SemioValue::Int { lexeme: "1".into() }] }] }),
    };
    let next = diff.apply(&base).expect("apply must succeed for a well-formed fixture");
    assert_eq!(next.columns[0].name, "b");
    assert_eq!(next.rows[0].cells[0], SemioValue::Int { lexeme: "1".into() });
}

#[semio_framework_async_macros::async_test]
async fn absorb_last_write_wins() {
    let mut d1 = SemioTableDiff { columns: None, rows: Some(SemioTableRowList { values: vec![SemioTableRow { cells: vec![SemioValue::Str { value: "a".into() }] }] }) };
    let d2 = SemioTableDiff { columns: None, rows: Some(SemioTableRowList { values: vec![SemioTableRow { cells: vec![SemioValue::Str { value: "b".into() }] }] }) };
    d1.absorb(d2.clone());
    assert_eq!(d1, d2);
}

#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    for d in demo_diff_cases() {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = SemioTableDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = SemioTableDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}

#[semio_framework_async_macros::async_test]
async fn print_diff_joins_both_fields_with_semicolon_on_one_line() {
    let d = &demo_diff_cases()[3];
    let printed = d.print_diff();
    assert!(printed.contains(';'), "expected both-present diff to join with ';', got {printed:?}");
    assert_eq!(printed.matches('\n').count(), 0);
}
