//! 🔺️ Sparse diff builder for `AddObjectVortex` — patches the owner object's `vortices` list.
//! No-op when the vortex id already exists on that object.
use crate::artifacts::puzzle3d::diff::{Puzzle3dDiff, Puzzle3dObjectPatch, Puzzle3dObjectPatchEntry, Puzzle3dObjectsDelta};
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::AddObjectVortex, base: &Puzzle3dSnapshot) -> Puzzle3dDiff {
    let Some(object) = base.objects.iter().find(|entry| entry.id == payload.object_id) else {
        return Puzzle3dDiff::default();
    };
    if object.vortices.iter().any(|vortex| vortex.id == payload.vortex.id) {
        return Puzzle3dDiff::default();
    }
    let mut next = object.clone();
    let at = payload.index.unwrap_or(next.vortices.len()).min(next.vortices.len());
    next.vortices.insert(at, payload.vortex.clone());
    Puzzle3dDiff {
        objects: Some(Puzzle3dObjectsDelta { patched: vec![Puzzle3dObjectPatchEntry { id: payload.object_id.clone(), patch: Puzzle3dObjectPatch { replacement: Some(next) } }], ..Default::default() }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
