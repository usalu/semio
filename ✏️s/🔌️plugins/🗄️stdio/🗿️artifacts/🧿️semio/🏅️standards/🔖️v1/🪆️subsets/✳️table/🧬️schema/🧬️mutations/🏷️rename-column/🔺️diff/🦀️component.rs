//! 🔺️ `rename-column` — sparse diff construction; Error `mutation.target-missing` when `name`
//! is absent from `base`, Warning `mutation.no-op` when `new_name` equals the current name, Fatal
//! `mutation.duplicate-id` when `new_name` collides with another existing column (name is the
//! native key, see `📸️snapshot/🦀️component.rs`). Rows are untouched — a rename is a pure
//! identity-field change.

use super::mutation::RenameColumn;
use crate::artifacts::semio::standards::v1::subsets::table::schema::diff::{SemioTableColumnList, SemioTableDiff};
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &RenameColumn, base: &SemioTableSnapshot) -> protocol::MutationOutcome<SemioTableDiff> {
    if !base.columns.iter().any(|c| c.name == payload.name) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Column \"{}\" does not exist.", payload.name), [payload.name.clone()]);
    }
    if payload.name == payload.new_name {
        return protocol::MutationOutcome::empty().await.warn("mutation.no-op", format!("Column \"{}\" is already named \"{}\".", payload.name, payload.new_name));
    }
    if base.columns.iter().any(|c| c.name == payload.new_name) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A column named \"{}\" already exists.", payload.new_name), [payload.new_name.clone()]);
    }
    let mut columns = base.columns.clone();
    if let Some(col) = columns.iter_mut().find(|c| c.name == payload.name) {
        col.name = payload.new_name.clone();
    }
    protocol::MutationOutcome::new(SemioTableDiff { columns: Some(SemioTableColumnList { values: columns }), rows: None })
}
//#endregion 🔖️Diff
