//! 🔺️ Diff for `RenameColumn`.

use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff, Replace};
use crate::standards::v1::subsets::table::schema::diff::{SemioTableColumnDiff, SemioTableDiff, SemioTableRowDiff};
use crate::standards::v1::subsets::table::schema::snapshot::{SemioTableColumn, SemioTableSnapshot};
use crate::standards::v1::subsets::value::schema::snapshot::SemioValue;

//#region 🔖️Diff
/// 🧮️ Renames one column: a sparse `modified` row naming only the new name.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RenameColumn, base: &SemioTableSnapshot) -> protocol::MutationOutcome<SemioTableDiff> {
    let Some(at) = base.columns.iter().position(|c| c.name == payload.name) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Column \"{}\" does not exist.", payload.name), [payload.name.clone()]);
    };
    if payload.name == payload.new_name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Column \"{}\" is already named \"{}\".", payload.name, payload.new_name));
    }
    if base.columns.iter().any(|c| c.name == payload.new_name) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A column named \"{}\" already exists.", payload.new_name), [payload.new_name.clone()]);
    }
    protocol::MutationOutcome::new(SemioTableDiff { columns: Some(IndexedTripleDiff { modified: vec![IndexModified { index: at, diff: SemioTableColumnDiff { name: Some(payload.new_name.clone()), kind: None } }], ..Default::default() }), rows: None })
}
//#endregion 🔖️Diff
