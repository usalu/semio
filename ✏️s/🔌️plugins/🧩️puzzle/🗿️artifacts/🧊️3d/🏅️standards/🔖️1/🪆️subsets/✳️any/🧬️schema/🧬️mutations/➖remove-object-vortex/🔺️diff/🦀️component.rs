//! 🔺️ Sparse diff builder for `RemoveObjectVortex` — patches the owner object's `vortices` list
//! and severs any attraction referencing the removed vortex (full id `object_id:vortex_id`).
use crate::artifacts::puzzle3d::diff::{Puzzle3dAttractionsDelta, Puzzle3dDiff, Puzzle3dObjectPatch, Puzzle3dObjectPatchEntry, Puzzle3dObjectsDelta};
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::RemoveObjectVortex, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    let Some(object) = base.objects.iter().find(|entry| entry.id == payload.object_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "object-vortex", payload.object_id), vec![payload.object_id.clone()]);
    };
    if !object.vortices.iter().any(|vortex| vortex.id == payload.vortex_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Vortex \"{}\" not found on object \"{}\".", payload.vortex_id, payload.object_id), vec![payload.vortex_id.clone()]);
    }
    let mut next = object.clone();
    next.vortices.retain(|vortex| vortex.id != payload.vortex_id);
    let full_id = format!("{}:{}", payload.object_id, payload.vortex_id);
    let severed: Vec<String> = base.attractions.iter().filter(|attraction| attraction.attracting == full_id || attraction.attracted == full_id).map(|attraction| attraction.id.clone()).collect();
    protocol::MutationOutcome::new(Puzzle3dDiff {
        objects: Some(Puzzle3dObjectsDelta { patched: vec![Puzzle3dObjectPatchEntry { id: payload.object_id.clone(), patch: Puzzle3dObjectPatch { replacement: Some(next) } }], ..Default::default() }),
        attractions: if severed.is_empty() { None } else { Some(Puzzle3dAttractionsDelta { removed: severed, ..Default::default() }) },
        ..Default::default()
    })
}
//#endregion 🔖️Diff
