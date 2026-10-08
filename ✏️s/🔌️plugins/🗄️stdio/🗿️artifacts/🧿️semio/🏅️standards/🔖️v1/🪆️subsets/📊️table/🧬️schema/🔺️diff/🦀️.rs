//! 🔺️ SemioTableDiff — sparse keyed-row diff over `SemioTableSnapshot`. `table` has TWO mutable fields (`columns`, `rows` —
//! both intrinsically ordered collections), so the diff carries two index-keyed triples: removed base indices, modified rows
//! (a sparse [`SemioTableColumnDiff`] / [`SemioTableRowDiff`] per row) and added rows with their final position. A column change
//! that must reach every row (create, delete, reorder) names exactly one cell edit per row — never a rebuilt list. No
//! `snapshot: Option<SemioTableSnapshot>` full-replace slot anywhere — whole-document replace is `ArtifactStore::reset`,
//! outside history.

use crate::standards::v1::subsets::base::schema::triples::{absorb_indexed_slot, apply_indexed_rows, between_indexed_rows, inverse_indexed_rows, validate_indexed_triple, IndexedRow, IndexedTripleDiff, Replace};
use crate::standards::v1::subsets::table::schema::snapshot::{SemioTableCellKind, SemioTableColumn, SemioTableRow, SemioTableSnapshot};
use crate::standards::v1::subsets::value::schema::snapshot::SemioValue;
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;

//#region 🔖️ColumnDiff
/// 🏛️ Sparse diff of one column: each present field is the new value.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioTableColumnDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<SemioTableCellKind>,
}

impl IndexedRow<SemioTableColumn> for SemioTableColumnDiff {
    fn apply_row(&self, base: &SemioTableColumn) -> SemioTableColumn {
        SemioTableColumn { name: self.name.clone().unwrap_or_else(|| base.name.clone()), kind: self.kind.unwrap_or(base.kind) }
    }
    fn inverse_row(&self, base: &SemioTableColumn) -> Self {
        Self { name: self.name.as_ref().map(|_| base.name.clone()), kind: self.kind.map(|_| base.kind) }
    }
    fn absorb_row(&mut self, other: Self) {
        if other.name.is_some() {
            self.name = other.name;
        }
        if other.kind.is_some() {
            self.kind = other.kind;
        }
    }
    fn row_is_empty(&self) -> bool {
        self.name.is_none() && self.kind.is_none()
    }
    fn between_row(base: &SemioTableColumn, other: &SemioTableColumn) -> Self {
        Self { name: (base.name != other.name).then(|| other.name.clone()), kind: (base.kind != other.kind).then_some(other.kind) }
    }
}
//#endregion 🔖️ColumnDiff

//#region 🔖️RowDiff
/// 🧱 Sparse diff of one row: `cells` nests an index-keyed triple whose modified rows replace one cell value.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioTableRowDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub cells: Option<IndexedTripleDiff<Replace<SemioValue>, SemioValue>>,
}

impl IndexedRow<SemioTableRow> for SemioTableRowDiff {
    fn apply_row(&self, base: &SemioTableRow) -> SemioTableRow {
        SemioTableRow { cells: self.cells.as_ref().map_or_else(|| base.cells.clone(), |cells| apply_indexed_rows(cells, &base.cells)) }
    }
    fn inverse_row(&self, base: &SemioTableRow) -> Self {
        Self { cells: self.cells.as_ref().map(|cells| inverse_indexed_rows(cells, &base.cells)) }
    }
    fn absorb_row(&mut self, other: Self) {
        absorb_indexed_slot(&mut self.cells, other.cells);
    }
    fn row_is_empty(&self) -> bool {
        self.cells.as_ref().is_none_or(IndexedTripleDiff::is_unchanged)
    }
    fn between_row(base: &SemioTableRow, other: &SemioTableRow) -> Self {
        let cells = between_indexed_rows(&base.cells, &other.cells);
        Self { cells: (!cells.is_unchanged()).then_some(cells) }
    }
}
//#endregion 🔖️RowDiff

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.table.diff")]
pub struct SemioTableDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub columns: Option<IndexedTripleDiff<SemioTableColumnDiff, SemioTableColumn>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rows: Option<IndexedTripleDiff<SemioTableRowDiff, SemioTableRow>>,
}

impl SemioTableDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty_diff(&self) -> bool {
        self.columns.as_ref().is_none_or(IndexedTripleDiff::is_unchanged) && self.rows.as_ref().is_none_or(IndexedTripleDiff::is_unchanged)
    }
}

impl MutationDiff<SemioTableSnapshot> for SemioTableDiff {
    fn apply(&self, base: &SemioTableSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<SemioTableSnapshot> {
        let mut next = base.clone();
        if let Some(columns) = &self.columns {
            validate_indexed_triple(columns, base.columns.len(), ["columns"])?;
            next.columns = apply_indexed_rows(columns, &base.columns);
        }
        if let Some(rows) = &self.rows {
            validate_indexed_triple(rows, base.rows.len(), ["rows"])?;
            for modified in &rows.modified {
                if let Some(cells) = &modified.diff.cells {
                    validate_indexed_triple(cells, base.rows[modified.index].cells.len(), ["rows".to_string(), modified.index.to_string(), "cells".to_string()])?;
                }
            }
            next.rows = apply_indexed_rows(rows, &base.rows);
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        absorb_indexed_slot(&mut self.columns, other.columns);
        absorb_indexed_slot(&mut self.rows, other.rows);
    }
}

/// 🧮️ `table`'s own `DiffAlgebra` — required by the `✉️base` envelope's own dispatch. `inverse` is the concrete negative diff of
/// the keyed rows; `between` is the positional sync/import delta, never used by mutation leaves.
impl protocol::command::DiffAlgebra<SemioTableSnapshot> for SemioTableDiff {
    fn between(base: &SemioTableSnapshot, other: &SemioTableSnapshot) -> Self {
        let columns = between_indexed_rows(&base.columns, &other.columns);
        let rows = between_indexed_rows(&base.rows, &other.rows);
        SemioTableDiff { columns: (!columns.is_unchanged()).then_some(columns), rows: (!rows.is_unchanged()).then_some(rows) }
    }
    fn inverse(&self, base: &SemioTableSnapshot) -> Self {
        SemioTableDiff { columns: self.columns.as_ref().map(|columns| inverse_indexed_rows(columns, &base.columns)), rows: self.rows.as_ref().map(|rows| inverse_indexed_rows(rows, &base.rows)) }
    }
    fn is_empty(&self) -> bool {
        self.is_empty_diff()
    }
}
//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec














//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️Demo
/// 🌱 Representative `SemioTableDiff` cases — single source of truth for `diff_grammar_conformance_
/// law`/`protocol_walk_law` in `🚪️io/🦀️.rs`. Covers: empty, columns-only, rows-only, and both-present.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<SemioTableDiff> {
    use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified};
    use crate::standards::v1::subsets::table::schema::snapshot::demo_table_snapshot;
    let demo = demo_table_snapshot();
    vec![
        SemioTableDiff::default(),
        SemioTableDiff { columns: Some(IndexedTripleDiff { added: demo.columns.iter().cloned().enumerate().map(|(index, item)| IndexAdded { index, item }).collect(), ..Default::default() }), rows: None },
        SemioTableDiff { columns: None, rows: Some(IndexedTripleDiff { added: demo.rows.iter().cloned().enumerate().map(|(index, item)| IndexAdded { index, item }).collect(), ..Default::default() }) },
        SemioTableDiff {
            columns: Some(IndexedTripleDiff { removed: vec![1], modified: vec![IndexModified { index: 0, diff: SemioTableColumnDiff { name: Some("renamed".into()), kind: Some(SemioTableCellKind::Int) } }], ..Default::default() }),
            rows: Some(IndexedTripleDiff {
                removed: vec![0],
                modified: vec![IndexModified { index: 1, diff: SemioTableRowDiff { cells: Some(IndexedTripleDiff { modified: vec![IndexModified { index: 0, diff: Replace { value: SemioValue::Int { lexeme: "7".into() } } }], ..Default::default() }) } }],
                ..Default::default()
            }),
        },
    ]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
