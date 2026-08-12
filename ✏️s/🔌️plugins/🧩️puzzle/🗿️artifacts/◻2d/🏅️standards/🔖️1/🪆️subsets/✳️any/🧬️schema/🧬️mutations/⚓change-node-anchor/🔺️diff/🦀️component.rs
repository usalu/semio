//! 🔺️ Sparse diff builder for `ChangeNodeAnchor` — patches the one addressed node in place.
use crate::artifacts::puzzle2d::diff::{Puzzle2dDiff, Puzzle2dNodePatch, Puzzle2dNodePatchEntry, Puzzle2dNodesDelta};
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeNodeAnchor, base: &Puzzle2dSnapshot) -> Puzzle2dDiff {
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.id) else {
        return Puzzle2dDiff::default();
    };
    let mut next = node.clone();
    next.anchor = payload.new_anchor;
    Puzzle2dDiff {
        nodes: Some(Puzzle2dNodesDelta { patched: vec![Puzzle2dNodePatchEntry { id: payload.id.clone(), patch: Puzzle2dNodePatch { replacement: Some(next) } }], ..Default::default() }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
