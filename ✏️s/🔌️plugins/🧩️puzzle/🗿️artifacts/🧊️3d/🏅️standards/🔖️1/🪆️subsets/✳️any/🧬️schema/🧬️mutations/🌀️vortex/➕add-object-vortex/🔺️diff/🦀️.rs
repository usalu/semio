//! No-op when the vortex id already exists on that object.
//! 🔺️ Sparse diff builder for `AddObjectVortex` — adds one vortex to the owner object's `vortices`.
use semio_s_artifact_puzzle_3d_diff::Puzzle3dDiff;
use semio_s_artifact_puzzle_3d_diff::Puzzle3dObjectPatch;
use semio_s_artifact_puzzle_3d_diff::Puzzle3dObjectsDelta;
use semio_s_artifact_puzzle_3d_diff::Puzzle3dVorticesDelta;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::AddObjectVortex, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    let Some(object) = base.objects.iter().find(|entry| entry.id == payload.object_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "object-vortex", payload.object_id), vec![payload.object_id.clone()]);
    };
    if object.vortices.iter().any(|vortex| vortex.id == payload.vortex.id) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Vortex \"{}\" already exists on object \"{}\".", payload.vortex.id, payload.object_id));
    }
    let index = payload.index.map_or(object.vortices.len(), |index| index.min(object.vortices.len()));
    let patch = Puzzle3dObjectPatch { vortices: Some(Puzzle3dVorticesDelta::insertion(index, payload.vortex.clone())), ..Default::default() };
    protocol::MutationOutcome::new(Puzzle3dDiff { objects: Some(Puzzle3dObjectsDelta::modification(payload.object_id.clone(), patch)), ..Default::default() })
}
//#endregion 🔖️Diff
