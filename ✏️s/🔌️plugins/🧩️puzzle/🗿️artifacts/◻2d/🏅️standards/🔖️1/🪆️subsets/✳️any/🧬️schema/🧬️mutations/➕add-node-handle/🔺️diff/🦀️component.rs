//! 🔺️ Sparse diff builder for `AddNodeHandle` — patches the owner node's `handles` list. No-op
//! when the handle id already exists on that node.
use crate::artifacts::puzzle2d::diff::{Puzzle2dDiff, Puzzle2dNodePatch, Puzzle2dNodePatchEntry, Puzzle2dNodesDelta};
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::AddNodeHandle, base: &Puzzle2dSnapshot) -> Puzzle2dDiff {
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.node_id) else {
        return Puzzle2dDiff::default();
    };
    if node.handles.iter().any(|handle| handle.id == payload.handle.id) {
        return Puzzle2dDiff::default();
    }
    let mut next = node.clone();
    let at = payload.index.unwrap_or(next.handles.len()).min(next.handles.len());
    next.handles.insert(at, payload.handle.clone());
    Puzzle2dDiff {
        nodes: Some(Puzzle2dNodesDelta { patched: vec![Puzzle2dNodePatchEntry { id: payload.node_id.clone(), patch: Puzzle2dNodePatch { replacement: Some(next) } }], ..Default::default() }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
