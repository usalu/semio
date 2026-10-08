use super::*;
use crate::standards::v1::subsets::table::schema::snapshot::{SemioTableCellKind, STDIO_SEMIOTABLE_DOCUMENT_SCHEMA};
use crate::standards::v1::subsets::value::schema::snapshot::SemioValue;
use protocol::{DiffBinary,DiffCodec,DiffText};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn one_col_row(name: &str, kind: SemioTableCellKind, value: SemioValue) -> SemioTableSnapshot {
    SemioTableSnapshot { schema: STDIO_SEMIOTABLE_DOCUMENT_SCHEMA.into(), columns: vec![SemioTableColumn { name: name.into(), kind }], rows: vec![SemioTableRow { cells: vec![value] }] }
}

fn modify_cell(row: usize, cell: usize, value: SemioValue) -> SemioTableDiff {
    use crate::standards::v1::subsets::base::schema::triples::IndexModified;
    let cells = IndexedTripleDiff { modified: vec![IndexModified { index: cell, diff: Replace { value } }], ..Default::default() };
    SemioTableDiff { columns: None, rows: Some(IndexedTripleDiff { modified: vec![IndexModified { index: row, diff: SemioTableRowDiff { cells: Some(cells) } }], ..Default::default() }) }
}

#[semio_framework_async_macros::async_test]
async fn apply_touches_only_named_cells() {
    let base = one_col_row("a", SemioTableCellKind::Str, SemioValue::Str { value: "x".into() });
    let diff = modify_cell(0, 0, SemioValue::Str { value: "y".into() });
    let next = protocol::apply_diff(&diff, &base).expect("apply must succeed for a well-formed fixture");
    assert_eq!(next.rows[0].cells[0], SemioValue::Str { value: "y".into() });
    assert_eq!(next.columns, base.columns, "untouched columns must be preserved");
}

#[semio_framework_async_macros::async_test]
async fn absorb_equals_sequential_apply() {
    let base = one_col_row("a", SemioTableCellKind::Str, SemioValue::Str { value: "x".into() });
    let first = modify_cell(0, 0, SemioValue::Str { value: "y".into() });
    let second = modify_cell(0, 0, SemioValue::Str { value: "z".into() });
    let mut absorbed = first.clone();
    absorbed.absorb(second.clone());
    let sequential = protocol::apply_diff(&second, &protocol::apply_diff(&first, &base).unwrap()).unwrap();
    assert_eq!(protocol::apply_diff(&absorbed, &base).unwrap(), sequential);
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_base() {
    let base = one_col_row("a", SemioTableCellKind::Str, SemioValue::Str { value: "x".into() });
    let diff = modify_cell(0, 0, SemioValue::Str { value: "y".into() });
    let next = protocol::apply_diff(&diff, &base).unwrap();
    let inverse = protocol::command::DiffAlgebra::inverse(&diff, &base);
    assert_eq!(protocol::apply_diff(&inverse, &next).unwrap(), base);
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
