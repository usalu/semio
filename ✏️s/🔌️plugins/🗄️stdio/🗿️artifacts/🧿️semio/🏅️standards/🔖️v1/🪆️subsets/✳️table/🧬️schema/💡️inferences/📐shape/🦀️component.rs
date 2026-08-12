//! 📐 `shape` — one named inference: the table's own dimensions plus a census of its declared
//! column kinds (`SemioTableCellKind` — mirrors `SemioValue`'s scalar variant names, per this
//! subset's own module doc comment). Cell VALUES are read independently of a column's declared
//! `kind` (no runtime enforcement — this subset's own module doc comment), so this facet reports
//! only the DECLARED shape, never a re-derived cell-level census (that would silently paper over
//! the lenient real-world tabular format this subset honestly models).

use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::{SemioTableCellKind, SemioTableSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Shape
/// 📐️ Semio table dimensions + declared column-kind census.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioTableShape {
    pub column_count: u32,
    pub row_count: u32,
    pub null_column_count: u32,
    pub bool_column_count: u32,
    pub int_column_count: u32,
    pub float_column_count: u32,
    pub str_column_count: u32,
    pub bytes_column_count: u32,
}

/// 📐️ Computes [`SemioTableShape`] — pure, total, O(columns).
pub fn compute_semio_table_shape(snapshot: &SemioTableSnapshot) -> SemioTableShape {
    let mut shape = SemioTableShape { column_count: snapshot.columns.len() as u32, row_count: snapshot.rows.len() as u32, ..Default::default() };
    for column in &snapshot.columns {
        match column.kind {
            SemioTableCellKind::Null => shape.null_column_count += 1,
            SemioTableCellKind::Bool => shape.bool_column_count += 1,
            SemioTableCellKind::Int => shape.int_column_count += 1,
            SemioTableCellKind::Float => shape.float_column_count += 1,
            SemioTableCellKind::Str => shape.str_column_count += 1,
            SemioTableCellKind::Bytes => shape.bytes_column_count += 1,
        }
    }
    shape
}
//#endregion 🔖️Shape

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::{SemioTableColumn, SemioTableRow, STDIO_SEMIOTABLE_DOCUMENT_SCHEMA};
    use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValue;

    fn populated() -> SemioTableSnapshot {
        SemioTableSnapshot {
            schema: STDIO_SEMIOTABLE_DOCUMENT_SCHEMA.into(),
            columns: vec![
                SemioTableColumn { name: "label".into(), kind: SemioTableCellKind::Str },
                SemioTableColumn { name: "score".into(), kind: SemioTableCellKind::Float },
                SemioTableColumn { name: "active".into(), kind: SemioTableCellKind::Bool },
            ],
            rows: vec![
                SemioTableRow { cells: vec![SemioValue::Str { value: "widget".into() }, SemioValue::Float { lexeme: "3.5".into() }, SemioValue::Bool { value: true }] },
                SemioTableRow { cells: vec![SemioValue::Null, SemioValue::Int { lexeme: "42".into() }, SemioValue::Bytes { value: vec![0, 1] }] },
            ],
        }
    }

    #[test]
    fn censuses_declared_column_kinds_and_dimensions() {
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

    #[test]
    fn inference_determinism_law() {
        let snapshot = populated();
        assert_eq!(compute_semio_table_shape(&snapshot), compute_semio_table_shape(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(compute_semio_table_shape(&SemioTableSnapshot::default()), SemioTableShape::default());
    }
}
//#endregion 🧪️Tests
