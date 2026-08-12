//! 🔺️ Sparse diff builder for `DeleteEdge` — a real removal.
use crate::artifacts::jack::diff::{diff_edges_removed, JackDiff};
use crate::artifacts::jack::JackSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteEdge, _base: &JackSnapshot) -> JackDiff {
    diff_edges_removed(vec![payload.id.clone()])
}
//#endregion 🔖️Diff
