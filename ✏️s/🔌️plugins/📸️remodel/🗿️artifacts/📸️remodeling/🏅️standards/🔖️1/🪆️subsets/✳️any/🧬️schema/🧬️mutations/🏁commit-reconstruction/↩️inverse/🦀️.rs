//! ↩️ Inverse for the atomic reconstruction result — one `commit-reconstruction` carrying the BASE result
//! lanes and the BASE binding of every asset id the payload bound. It never touches durable content:
//! the `append-content` steps of the same edit own and undo the leaves.
use crate::mutations::{commit_reconstruction, CommitReconstruction, ReconstructionAssetCommit, RemodelingMutation};
use crate::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CommitReconstruction, base: &RemodelingSnapshot) -> Result<Vec<RemodelingMutation>, semio_framework_value::ValueError> {
    let mut assets: Vec<ReconstructionAssetCommit> = Vec::new();
    for binding in &payload.assets {
        let restored = ReconstructionAssetCommit { id: binding.id.clone(), content_id: base.assets.get(&binding.id).map(|handle| handle.child_id.clone()) };
        match assets.iter_mut().find(|held| held.id == restored.id) {
            Some(held) => *held = restored,
            None => assets.push(restored),
        }
    }
    Ok(vec![commit_reconstruction(CommitReconstruction {
        sparse: base.results.sparse.clone(),
        trajectory: base.results.trajectory.clone(),
        mesh: payload.mesh.as_ref().map(|_| Box::new(base.results.mesh.clone())),
        geo: base.results.geo.clone(),
        qc: base.results.qc.clone(),
        assets,
    })])
}
//#endregion 🔖️Inverse
