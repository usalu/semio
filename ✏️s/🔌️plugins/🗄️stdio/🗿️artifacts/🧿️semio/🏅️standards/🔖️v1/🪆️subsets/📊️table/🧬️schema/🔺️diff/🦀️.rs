//! 🔺️ SemioTableDiff — sparse per-field diff over `SemioTableSnapshot`. `table` has TWO mutable
//! fields (`columns`, `rows` — both intrinsically ordered, anonymous collections per
//! `📓️taxonomy.md`'s addressing rule #3 for `rows`, and rule #2 name-keyed for `columns`), so the
//! diff carries two `Option<…>` slots: whole-list wrappers rebuilt POSITIONALLY from `base` by
//! each mutation triad's own `🔺️diff` leaf (never a generic `between()` re-derivation) — same
//! shape `🔤️text`'s own `SemioTextDiff` uses for its single `runs` field. No
//! `snapshot: Option<SemioTableSnapshot>` full-replace slot anywhere — whole-document replace is
//! `ArtifactStore::reset`, outside history.


use crate::standards::v1::subsets::table::schema::snapshot::{SemioTableColumn, SemioTableRow, SemioTableSnapshot};
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;

//#region 🔖️ColumnList
/// 📋 Whole-list wrapper for the `columns` field diff — every mutation triad rebuilds the full
/// ordered `values` vec from `base` and wraps it here (`SemioTextRunList`'s own shape).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SemioTableColumnList {
    pub values: Vec<SemioTableColumn>,
}
//#endregion 🔖️ColumnList

//#region 🔖️RowList
/// 📋 Whole-list wrapper for the `rows` field diff — same shape as [`SemioTableColumnList`].
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SemioTableRowList {
    pub values: Vec<SemioTableRow>,
}
//#endregion 🔖️RowList

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.table.diff")]
pub struct SemioTableDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub columns: Option<SemioTableColumnList>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rows: Option<SemioTableRowList>,
}

impl SemioTableDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty_diff(&self) -> bool {
        self.columns.is_none() && self.rows.is_none()
    }
}

impl MutationDiff<SemioTableSnapshot> for SemioTableDiff {
    fn apply(&self, base: &SemioTableSnapshot) -> protocol::MutationApplyResult<SemioTableSnapshot> {
        let mut next = base.clone();
        if let Some(list) = &self.columns {
            next.columns = list.values.clone();
        }
        if let Some(list) = &self.rows {
            next.rows = list.values.clone();
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.columns.is_some() {
            self.columns = other.columns;
        }
        if other.rows.is_some() {
            self.rows = other.rows;
        }
    }
}

/// 🧮️ `table`'s own `DiffAlgebra` — required by the `✉️base` envelope's own dispatch (`SemioDiff`
/// delegates `between`/`inverse`/`is_empty` straight through to every wrapped subset's own impl).
/// Whole-list `between`/`inverse` are honest here (not apply-then-capture): a change is fully
/// described by "the new/old `columns`/`rows` value", same shape every mutation triad's own
/// `🔺️diff` leaf already produces.
impl protocol::command::DiffAlgebra<SemioTableSnapshot> for SemioTableDiff {
    fn between(base: &SemioTableSnapshot, other: &SemioTableSnapshot) -> Self {
        SemioTableDiff { columns: (base.columns != other.columns).then(|| SemioTableColumnList { values: other.columns.clone() }), rows: (base.rows != other.rows).then(|| SemioTableRowList { values: other.rows.clone() }) }
    }
    fn inverse(&self, base: &SemioTableSnapshot) -> Self {
        SemioTableDiff { columns: self.columns.as_ref().map(|_| SemioTableColumnList { values: base.columns.clone() }), rows: self.rows.as_ref().map(|_| SemioTableRowList { values: base.rows.clone() }) }
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
/// law`/`protocol_walk_law` in `🚪️io/🦀️.rs`. Covers: empty, columns-only, rows-only, and
/// both-present (the two-optional-field `;`-joined `print_diff` shape).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<SemioTableDiff> {
    use crate::standards::v1::subsets::table::schema::snapshot::{demo_table_snapshot, SemioTableCellKind, SemioTableColumn, SemioTableRow};
    use crate::standards::v1::subsets::value::schema::snapshot::SemioValue;
    vec![
        SemioTableDiff::default(),
        SemioTableDiff { columns: Some(SemioTableColumnList { values: demo_table_snapshot().columns }), rows: None },
        SemioTableDiff { columns: None, rows: Some(SemioTableRowList { values: demo_table_snapshot().rows }) },
        SemioTableDiff {
            columns: Some(SemioTableColumnList { values: vec![SemioTableColumn { name: "extra".into(), kind: SemioTableCellKind::Int }] }),
            rows: Some(SemioTableRowList { values: vec![SemioTableRow { cells: vec![SemioValue::Int { lexeme: "7".into() }] }] }),
        },
    ]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
