//! 🔺️ `delete-column` — sparse diff construction; a column name absent from `base` is
//! `mutation.target-missing`. Removing the column also removes its aligned cell from every row —
//! a real cascade, reported via `mutation.cascade` whenever at least one row was actually touched.

use super::mutation::DeleteColumn;
use crate::artifacts::semio::standards::v1::subsets::table::schema::diff::{SemioTableColumnList, SemioTableDiff, SemioTableRowList};
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &DeleteColumn, base: &SemioTableSnapshot) -> protocol::MutationOutcome<SemioTableDiff> {
    let Some(at) = base.columns.iter().position(|c| c.name == payload.name) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Column \"{}\" does not exist.", payload.name), [payload.name.clone()]);
    };
    let mut columns = base.columns.clone();
    columns.remove(at);
    let mut rows = base.rows.clone();
    let mut cascaded_rows = 0usize;
    for row in &mut rows {
        if at < row.cells.len() {
            row.cells.remove(at);
            cascaded_rows += 1;
        }
    }
    let outcome = protocol::MutationOutcome::new(SemioTableDiff { columns: Some(SemioTableColumnList { values: columns }), rows: Some(SemioTableRowList { values: rows }) });
    if cascaded_rows == 0 {
        outcome
    } else {
        outcome.info("mutation.cascade", format!("Deleting column \"{}\" also removed its cell from {} row(s).", payload.name, cascaded_rows))
    }
}
//#endregion 🔖️Diff
