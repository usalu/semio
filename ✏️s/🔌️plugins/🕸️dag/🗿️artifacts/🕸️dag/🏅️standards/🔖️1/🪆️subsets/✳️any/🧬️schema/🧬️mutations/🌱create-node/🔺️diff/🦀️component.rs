//! 🔺️ Sparse diff builder for `CreateNode` — a real append-only insert (never a whole-snapshot
//! capture).
use crate::artifacts::dag::diff::{DagDiff, DagNodesDelta};
use crate::artifacts::dag::DagSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CreateNode, _base: &DagSnapshot) -> DagDiff {
    DagDiff { nodes: Some(DagNodesDelta { added: vec![payload.node.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
