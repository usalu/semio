//! 🔺️ Sparse diff builder for `CreateEdge` — a real append-only insert.
use crate::artifacts::jack::diff::{diff_edges_added, JackDiff};
use crate::artifacts::jack::JackSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CreateEdge, _base: &JackSnapshot) -> JackDiff {
    diff_edges_added(vec![payload.edge.clone()])
}
//#endregion 🔖️Diff
