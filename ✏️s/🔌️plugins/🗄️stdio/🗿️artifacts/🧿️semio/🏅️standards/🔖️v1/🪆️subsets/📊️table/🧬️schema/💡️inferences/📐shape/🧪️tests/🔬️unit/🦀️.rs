
use super::*;
use crate::standards::v1::subsets::table::schema::snapshot::{STDIO_SEMIOTABLE_DOCUMENT_SCHEMA, SemioTableColumn, SemioTableRow};
use crate::standards::v1::subsets::value::schema::snapshot::SemioValue;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn populated() -> SemioTableSnapshot {
    SemioTableSnapshot {
        schema: STDIO_SEMIOTABLE_DOCUMENT_SCHEMA.into(),
        columns: vec![SemioTableColumn { name: "label".into(), kind: SemioTableCellKind::Str }, SemioTableColumn { name: "score".into(), kind: SemioTableCellKind::Float }, SemioTableColumn { name: "active".into(), kind: SemioTableCellKind::Bool }],
        rows: vec![
            SemioTableRow { cells: vec![SemioValue::Str { value: "widget".into() }, SemioValue::Float { lexeme: "3.5".into() }, SemioValue::Bool { value: true }] },
            SemioTableRow { cells: vec![SemioValue::Null, SemioValue::Int { lexeme: "42".into() }, SemioValue::Bytes { value: vec![0, 1] }] },
        ],
    }
}

#[semio_framework_async_macros::async_test]
async fn censuses_declared_column_kinds_and_dimensions() {
    let shape = compute_semio_table_shape(&populated());
    assert_eq!(shape.column_count, 3);
    assert_eq!(shape.row_count, 2);
    assert_eq!(shape.str_column_count, 1);
    assert_eq!(shape.float_column_count, 1);
    assert_eq!(shape.bool_column_count, 1);
    assert_eq!(shape.null_column_count, 0);
    assert_eq!(shape.int_column_count, 0);
    assert_eq!(shape.bytes_column_count, 0);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = populated();
    assert_eq!(compute_semio_table_shape(&snapshot), compute_semio_table_shape(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(compute_semio_table_shape(&SemioTableSnapshot::default()), SemioTableShape::default());
}
