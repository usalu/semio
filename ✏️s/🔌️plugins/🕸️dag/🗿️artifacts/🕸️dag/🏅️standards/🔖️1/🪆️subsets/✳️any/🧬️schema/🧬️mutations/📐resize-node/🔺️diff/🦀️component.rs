//! 🔺️ Sparse diff builder for `ResizeNode`.
use crate::artifacts::dag::diff::{DagDiff, DagNodePatchEntry, DagNodesDelta};
use crate::artifacts::dag::{DagNodePatch, DagSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ResizeNode, _base: &DagSnapshot) -> DagDiff {
    let patch = DagNodePatch { width: Some(payload.width), height: Some(payload.height), ..Default::default() };
    DagDiff { nodes: Some(DagNodesDelta { patched: vec![DagNodePatchEntry { id: payload.id.clone(), patch }], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
