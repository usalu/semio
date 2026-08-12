//! 🔺️ Sparse diff builder for `ChangeNodeAbbreviation` — via `DagNodeExtraPatch` (see
//! `🖼️change-node-icon/🔺️diff` for why).
use crate::artifacts::dag::diff::{DagDiff, DagNodeExtraPatch, DagNodeExtraPatchEntry, DagNodesDelta};
use crate::artifacts::dag::DagSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeNodeAbbreviation, _base: &DagSnapshot) -> DagDiff {
    let patch = DagNodeExtraPatch { abbreviation: Some(payload.new_abbreviation.clone()), ..Default::default() };
    DagDiff { nodes: Some(DagNodesDelta { extra_patched: vec![DagNodeExtraPatchEntry { id: payload.id.clone(), patch }], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
