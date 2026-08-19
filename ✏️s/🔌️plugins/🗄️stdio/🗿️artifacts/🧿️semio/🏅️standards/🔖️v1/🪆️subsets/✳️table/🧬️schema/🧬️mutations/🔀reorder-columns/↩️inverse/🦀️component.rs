//! ↩️ `reorder-columns` — undo moves the column back to its ORIGINAL BASE index
//! (`📓️taxonomy.md`'s "`reorder` back"). Column absent from `base` ⇒ `Vec::new()`.

use super::mutation::ReorderColumns;
use crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::SemioTableMutation;
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &ReorderColumns, base: &SemioTableSnapshot) -> Vec<SemioTableMutation> {
    let Some(from) = base.columns.iter().position(|c| c.name == payload.name) else {
        return Vec::new();
    };
    vec![SemioTableMutation::ReorderColumns(ReorderColumns { name: payload.name.clone(), to_index: from })]
}
//#endregion 🔖️Inverse
