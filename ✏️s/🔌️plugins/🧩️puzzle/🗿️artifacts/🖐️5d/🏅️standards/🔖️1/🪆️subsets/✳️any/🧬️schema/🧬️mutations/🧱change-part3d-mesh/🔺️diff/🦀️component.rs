//! 🔺️ Sparse diff builder for `ChangePart3dMesh` — patches the one addressed part in place.
use crate::artifacts::puzzle5d::diff::{Puzzle5dDiff, Puzzle5dPartPatch, Puzzle5dPartPatchEntry, Puzzle5dPartsDelta};
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangePart3dMesh, base: &Puzzle5dSnapshot) -> Puzzle5dDiff {
    let Some(item) = base.parts.iter().find(|entry| entry.id == payload.id) else {
        return Puzzle5dDiff::default();
    };
    let mut next = item.clone();
    next.part_3d.mesh_url = payload.new_mesh_url.clone();
    Puzzle5dDiff {
        parts: Some(Puzzle5dPartsDelta { patched: vec![Puzzle5dPartPatchEntry { id: payload.id.clone(), patch: Puzzle5dPartPatch { replacement: Some(next) } }], ..Default::default() }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
