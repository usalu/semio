//! 🔺️ Sparse diff builder for `CreateNode` — a real append-only insert (never a whole-snapshot
//! capture).
use crate::artifacts::jack::diff::{diff_nodes_added, JackDiff};
use crate::artifacts::jack::JackSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CreateNode, _base: &JackSnapshot) -> JackDiff {
    diff_nodes_added(vec![payload.node.clone()])
}
//#endregion 🔖️Diff
