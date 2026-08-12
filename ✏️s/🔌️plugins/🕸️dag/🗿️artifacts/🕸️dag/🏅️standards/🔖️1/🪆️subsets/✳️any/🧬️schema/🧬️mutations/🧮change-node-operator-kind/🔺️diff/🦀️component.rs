//! 🔺️ Sparse diff builder for `ChangeNodeOperatorKind` — via `DagNodeExtraPatch` (see
//! `🖼️change-node-icon/🔺️diff` for why). `operator_kind` is double-`Option`ed since the field
//! itself is `Option<String>` — `Some(new_operator_kind)` always sets it here.
use crate::artifacts::dag::diff::{DagDiff, DagNodeExtraPatch, DagNodeExtraPatchEntry, DagNodesDelta};
use crate::artifacts::dag::DagSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeNodeOperatorKind, _base: &DagSnapshot) -> DagDiff {
    let patch = DagNodeExtraPatch { operator_kind: Some(payload.new_operator_kind.clone()), ..Default::default() };
    DagDiff { nodes: Some(DagNodesDelta { extra_patched: vec![DagNodeExtraPatchEntry { id: payload.id.clone(), patch }], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
