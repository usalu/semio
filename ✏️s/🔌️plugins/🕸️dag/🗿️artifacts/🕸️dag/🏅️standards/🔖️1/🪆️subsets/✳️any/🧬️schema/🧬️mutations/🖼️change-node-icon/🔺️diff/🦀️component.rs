//! 🔺️ Sparse diff builder for `ChangeNodeIcon` — `icon` isn't one of
//! `infinite_board_port_directed_dag::DagNodePatch`'s fields, so it goes through this facet's own
//! `DagNodeExtraPatch` extension instead.
use crate::artifacts::dag::diff::{DagDiff, DagNodeExtraPatch, DagNodeExtraPatchEntry, DagNodesDelta};
use crate::artifacts::dag::DagSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeNodeIcon, _base: &DagSnapshot) -> DagDiff {
    let patch = DagNodeExtraPatch { icon: Some(payload.new_icon.clone()), ..Default::default() };
    DagDiff { nodes: Some(DagNodesDelta { extra_patched: vec![DagNodeExtraPatchEntry { id: payload.id.clone(), patch }], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
