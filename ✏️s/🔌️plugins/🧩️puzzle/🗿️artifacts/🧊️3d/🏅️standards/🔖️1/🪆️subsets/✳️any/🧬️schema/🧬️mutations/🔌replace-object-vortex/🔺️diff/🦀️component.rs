//! 🔺️ Sparse diff builder for `ReplaceObjectVortex` — patches one vortex inside the owner object.
use crate::artifacts::puzzle3d::diff::{Puzzle3dDiff, Puzzle3dObjectPatch, Puzzle3dObjectPatchEntry, Puzzle3dObjectsDelta};
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReplaceObjectVortex, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    let Some(object) = base.objects.iter().find(|entry| entry.id == payload.object_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "object-vortex", payload.object_id), vec![payload.object_id.clone()]);
    };
    if !object.vortices.iter().any(|vortex| vortex.id == payload.vortex_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Vortex \"{}\" not found on object \"{}\".", payload.vortex_id, payload.object_id), vec![payload.vortex_id.clone()]);
    }
    let mut next = object.clone();
    if next == *object {
        return protocol::MutationOutcome::new(Puzzle3dDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "no changes to apply").at(vec![payload.object_id.clone()])]);
    }
    for vortex in next.vortices.iter_mut() {
        if vortex.id == payload.vortex_id {
            *vortex = payload.new_vortex.clone();
        }
    }
    protocol::MutationOutcome::new(Puzzle3dDiff {
        objects: Some(Puzzle3dObjectsDelta { patched: vec![Puzzle3dObjectPatchEntry { id: payload.object_id.clone(), patch: Puzzle3dObjectPatch { replacement: Some(next) } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
