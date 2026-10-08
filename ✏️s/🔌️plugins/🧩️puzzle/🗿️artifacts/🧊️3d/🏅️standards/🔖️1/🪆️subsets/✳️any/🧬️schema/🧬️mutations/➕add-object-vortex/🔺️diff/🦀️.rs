//! 🔺️ Sparse diff builder for `AddObjectVortex` — adds one vortex to the owner object's `vortices`.
//! No-op when the vortex id already exists on that object.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle3dDiff, Puzzle3dObjectPatch, Puzzle3dObjectsDelta, Puzzle3dVorticesDelta};
use crate::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::AddObjectVortex, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    let Some(object) = base.objects.iter().find(|entry| entry.id == payload.object_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "object-vortex", payload.object_id), vec![payload.object_id.clone()]);
    };
    if object.vortices.iter().any(|vortex| vortex.id == payload.vortex.id) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Vortex \"{}\" already exists on object \"{}\".", payload.vortex.id, payload.object_id));
    }
    let reordered = payload.index.filter(|index| *index < object.vortices.len()).map(|index| {
        let mut order: Vec<String> = object.vortices.iter().map(|vortex| vortex.id.clone()).collect();
        order.insert(index, payload.vortex.id.clone());
        order
    });
    let patch = Puzzle3dObjectPatch { vortices: Some(Puzzle3dVorticesDelta::adding(payload.vortex.clone(), reordered)), ..Default::default() };
    protocol::MutationOutcome::new(Puzzle3dDiff { objects: Some(Puzzle3dObjectsDelta::patching(payload.object_id.clone(), patch)), ..Default::default() })
}
//#endregion 🔖️Diff
