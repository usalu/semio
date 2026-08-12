//! 🔺️ Sparse diff builder for `CreateNode` — a real append-only insert (never a whole-snapshot
//! capture). No-op when the id already exists in `base`.
use crate::artifacts::puzzle2d::diff::{Puzzle2dDiff, Puzzle2dNodesDelta};
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CreateNode, base: &Puzzle2dSnapshot) -> Puzzle2dDiff {
    if base.nodes.iter().any(|entry| entry.id == payload.node.id) {
        return Puzzle2dDiff::default();
    }
    let mut delta = Puzzle2dNodesDelta { added: vec![payload.node.clone()], ..Default::default() };
    if let Some(index) = payload.index {
        let mut order: Vec<String> = base.nodes.iter().map(|entry| entry.id.clone()).collect();
        let at = index.min(order.len());
        order.insert(at, payload.node.id.clone());
        delta.reordered = Some(order);
    }
    Puzzle2dDiff { nodes: Some(delta), ..Default::default() }
}
//#endregion 🔖️Diff
