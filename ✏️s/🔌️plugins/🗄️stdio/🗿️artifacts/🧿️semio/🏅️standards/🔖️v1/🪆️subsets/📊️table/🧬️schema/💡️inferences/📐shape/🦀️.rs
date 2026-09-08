//! 📐 `shape` — one named inference: the table's own dimensions plus a census of its declared
//! column kinds (`SemioTableCellKind` — mirrors `SemioValue`'s scalar variant names, per this
//! subset's own module doc comment). Cell VALUES are read independently of a column's declared
//! `kind` (no runtime enforcement — this subset's own module doc comment), so this facet reports
//! only the DECLARED shape, never a re-derived cell-level census (that would silently paper over
//! the lenient real-world tabular format this subset honestly models).

use crate::standards::v1::subsets::table::schema::snapshot::{SemioTableCellKind, SemioTableSnapshot};

//#region 🔖️Shape
/// 📐️ Semio table dimensions + declared column-kind census.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
