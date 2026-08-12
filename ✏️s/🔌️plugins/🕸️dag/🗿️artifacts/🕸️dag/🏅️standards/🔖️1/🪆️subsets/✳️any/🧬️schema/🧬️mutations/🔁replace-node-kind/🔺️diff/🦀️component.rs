//! 🔺️ Sparse diff builder for `ReplaceNodeKind`.
use crate::artifacts::dag::diff::{DagDiff, DagNodePatchEntry, DagNodesDelta};
use crate::artifacts::dag::{DagNodePatch, DagSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReplaceNodeKind, _base: &DagSnapshot) -> DagDiff {
    let patch = DagNodePatch { kind: Some(payload.new_kind.clone()), ..Default::default() };
    DagDiff { nodes: Some(DagNodesDelta { patched: vec![DagNodePatchEntry { id: payload.id.clone(), patch }], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
