//! 🔺️ Sparse diff builder for `ScaleObject` — patches the one addressed object in place.
use crate::artifacts::puzzle3d::diff::{Puzzle3dDiff, Puzzle3dObjectPatch, Puzzle3dObjectPatchEntry, Puzzle3dObjectsDelta};
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ScaleObject, base: &Puzzle3dSnapshot) -> Puzzle3dDiff {
    let Some(item) = base.objects.iter().find(|entry| entry.id == payload.id) else {
        return Puzzle3dDiff::default();
    };
    let mut next = item.clone();
    next.scale = payload.new_scale;
    Puzzle3dDiff {
        objects: Some(Puzzle3dObjectsDelta { patched: vec![Puzzle3dObjectPatchEntry { id: payload.id.clone(), patch: Puzzle3dObjectPatch { replacement: Some(next) } }], ..Default::default() }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
