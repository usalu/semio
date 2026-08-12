//! 🔺️ Sparse diff builder for `ChangeReferenceHidden` — patches the one addressed reference in place.
use crate::artifacts::puzzle3d::diff::{Puzzle3dDiff, Puzzle3dReferencePatch, Puzzle3dReferencePatchEntry, Puzzle3dReferencesDelta};
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeReferenceHidden, base: &Puzzle3dSnapshot) -> Puzzle3dDiff {
    let Some(item) = base.references.iter().find(|entry| entry.id == payload.id) else {
        return Puzzle3dDiff::default();
    };
    let mut next = item.clone();
    next.hidden = payload.new_hidden;
    Puzzle3dDiff {
        references: Some(Puzzle3dReferencesDelta { patched: vec![Puzzle3dReferencePatchEntry { id: payload.id.clone(), patch: Puzzle3dReferencePatch { replacement: Some(next) } }], ..Default::default() }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
