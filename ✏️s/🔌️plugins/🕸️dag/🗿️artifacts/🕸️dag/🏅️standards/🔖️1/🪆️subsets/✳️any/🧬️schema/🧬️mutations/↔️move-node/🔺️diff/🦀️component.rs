//! 🔺️ Sparse diff builder for `MoveNode`.
use crate::artifacts::dag::diff::{DagDiff, DagNodePatchEntry, DagNodesDelta};
use crate::artifacts::dag::{DagNodePatch, DagSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::MoveNode, _base: &DagSnapshot) -> DagDiff {
    let patch = DagNodePatch { x: Some(payload.x), y: Some(payload.y), ..Default::default() };
    DagDiff { nodes: Some(DagNodesDelta { patched: vec![DagNodePatchEntry { id: payload.id.clone(), patch }], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
