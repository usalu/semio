//! 🔺️ `rename-column` — sparse diff construction; a column name absent from `base` is a no-op
//! clone. Rows are untouched — a rename is a pure identity-field change.

use super::mutation::RenameColumn;
use crate::artifacts::semio::standards::v1::subsets::table::schema::diff::{SemioTableColumnList, SemioTableDiff};
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &RenameColumn, base: &SemioTableSnapshot) -> SemioTableDiff {
    let mut columns = base.columns.clone();
    if let Some(col) = columns.iter_mut().find(|c| c.name == payload.name) {
        col.name = payload.new_name.clone();
    }
    SemioTableDiff { columns: Some(SemioTableColumnList { values: columns }), rows: None }
}
//#endregion 🔖️Diff
