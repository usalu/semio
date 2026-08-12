//! 🔺️ Sparse diff builder for `ReplaceNodeGeometry` — patches the one addressed node's shape/extent.
use crate::artifacts::puzzle2d::diff::{Puzzle2dDiff, Puzzle2dNodePatch, Puzzle2dNodePatchEntry, Puzzle2dNodesDelta};
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReplaceNodeGeometry, base: &Puzzle2dSnapshot) -> Puzzle2dDiff {
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.id) else {
        return Puzzle2dDiff::default();
    };
    let mut next = node.clone();
    next.shape = payload.new_shape.clone();
    next.radius = payload.new_radius;
    next.width = payload.new_width;
    next.height = payload.new_height;
    Puzzle2dDiff {
        nodes: Some(Puzzle2dNodesDelta { patched: vec![Puzzle2dNodePatchEntry { id: payload.id.clone(), patch: Puzzle2dNodePatch { replacement: Some(next) } }], ..Default::default() }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
