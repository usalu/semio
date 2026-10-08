//! 🔺️ Sparse diff builder for `RemoveObjectVortex` — removes one vortex from the owner object's `vortices`
//! and severs any attraction referencing the removed vortex (full id `object_id:vortex_id`).
use crate::standards::v1::subsets::any::schema::diff::{Puzzle3dAttractionsDelta, Puzzle3dDiff, Puzzle3dObjectPatch, Puzzle3dObjectsDelta, Puzzle3dVorticesDelta};
use crate::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RemoveObjectVortex, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    let Some(object) = base.objects.iter().find(|entry| entry.id == payload.object_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "object-vortex", payload.object_id), vec![payload.object_id.clone()]);
    };
    if !object.vortices.iter().any(|vortex| vortex.id == payload.vortex_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Vortex \"{}\" not found on object \"{}\".", payload.vortex_id, payload.object_id), vec![payload.vortex_id.clone()]);
    }
    let full_id = format!("{}:{}", payload.object_id, payload.vortex_id);
    let severed: Vec<String> = base.attractions.iter().filter(|attraction| attraction.attracting == full_id || attraction.attracted == full_id).map(|attraction| attraction.id.clone()).collect();
    let patch = Puzzle3dObjectPatch { vortices: Some(Puzzle3dVorticesDelta::removing(vec![payload.vortex_id.clone()])), ..Default::default() };
    protocol::MutationOutcome::new(Puzzle3dDiff {
        objects: Some(Puzzle3dObjectsDelta::patching(payload.object_id.clone(), patch)),
        attractions: (!severed.is_empty()).then(|| Puzzle3dAttractionsDelta::removing(severed)),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
