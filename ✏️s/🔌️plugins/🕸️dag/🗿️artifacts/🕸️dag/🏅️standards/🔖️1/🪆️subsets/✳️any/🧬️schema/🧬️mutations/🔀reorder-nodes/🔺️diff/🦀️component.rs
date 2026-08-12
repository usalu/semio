//! 🔺️ Sparse diff builder for `ReorderNodes`.
use crate::artifacts::dag::diff::{DagDiff, DagNodesDelta};
use crate::artifacts::dag::DagSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReorderNodes, _base: &DagSnapshot) -> DagDiff {
    DagDiff { nodes: Some(DagNodesDelta { reordered: Some(payload.order.clone()), ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
