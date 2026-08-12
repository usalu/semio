//! 🔺️ Sparse diff builder for `ReplaceNodeProperties` — via `DagNodeExtraPatch` (see
//! `🖼️change-node-icon/🔺️diff` for why: `properties` isn't a `DagNodePatch` field either).
use crate::artifacts::dag::diff::{DagDiff, DagNodeExtraPatch, DagNodeExtraPatchEntry, DagNodesDelta};
use crate::artifacts::dag::DagSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReplaceNodeProperties, _base: &DagSnapshot) -> DagDiff {
    let patch = DagNodeExtraPatch { properties: Some(payload.new_properties.clone()), ..Default::default() };
    DagDiff { nodes: Some(DagNodesDelta { extra_patched: vec![DagNodeExtraPatchEntry { id: payload.id.clone(), patch }], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
