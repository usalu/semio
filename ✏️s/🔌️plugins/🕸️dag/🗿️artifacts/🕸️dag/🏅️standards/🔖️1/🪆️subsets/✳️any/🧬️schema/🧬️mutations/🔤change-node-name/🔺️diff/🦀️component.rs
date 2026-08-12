//! 🔺️ Sparse diff builder for `ChangeNodeName`.
use crate::artifacts::dag::diff::{DagDiff, DagNodePatchEntry, DagNodesDelta};
use crate::artifacts::dag::{DagNodePatch, DagSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeNodeName, _base: &DagSnapshot) -> DagDiff {
    let patch = DagNodePatch { name: Some(payload.new_name.clone()), ..Default::default() };
    DagDiff { nodes: Some(DagNodesDelta { patched: vec![DagNodePatchEntry { id: payload.id.clone(), patch }], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
