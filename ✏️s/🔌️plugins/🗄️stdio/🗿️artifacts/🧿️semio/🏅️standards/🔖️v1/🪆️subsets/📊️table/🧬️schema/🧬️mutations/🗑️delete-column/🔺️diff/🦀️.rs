//! 🔺️ Diff for `DeleteColumn`.

use semio_s_artifact_stdio_semio_base::schema::triples::IndexModified;
use semio_s_artifact_stdio_semio_base::schema::triples::IndexedTripleDiff;
use semio_s_artifact_stdio_semio_table_model::schema::diff::SemioTableDiff;
use semio_s_artifact_stdio_semio_table_model::schema::diff::SemioTableRowDiff;
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Diff
/// 🧮️ Removes one column and its cell from every row that has one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::DeleteColumn, base: &SemioTableSnapshot) -> protocol::MutationOutcome<SemioTableDiff> {
    let Some(at) = base.columns.iter().position(|c| c.name == payload.name) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Column \"{}\" does not exist.", payload.name), [payload.name.clone()]);
    };
    let rows = IndexedTripleDiff {
        modified: base
            .rows
            .iter()
            .enumerate()
            .filter(|(_, row)| at < row.cells.len())
            .map(|(index, _)| IndexModified { index, diff: SemioTableRowDiff { cells: Some(IndexedTripleDiff { removed: vec![at], ..Default::default() }) } })
            .collect(),
        ..Default::default()
    };
    let cascaded_rows = rows.modified.len();
    let outcome = protocol::MutationOutcome::new(SemioTableDiff { columns: Some(IndexedTripleDiff { removed: vec![at], ..Default::default() }), rows: (cascaded_rows > 0).then_some(rows) });
    if cascaded_rows == 0 {
        outcome
    } else {
        outcome.info("mutation.cascade", format!("Deleting column \"{}\" also removed its cell from {} row(s).", payload.name, cascaded_rows))
    }
}
//#endregion 🔖️Diff
