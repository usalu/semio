//! ↩️ Inverse for the atomic reconstruction result — one `commit-reconstruction` carrying the BASE result
//! lanes and the BASE binding of every asset id the payload bound. It never touches durable content:
//! the `append-content` steps of the same edit own and undo the leaves.
use crate::mutations::{commit_reconstruction, CommitReconstruction, ReconstructionAssetCommit, RemodelingMutation};
use crate::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::CommitReconstruction, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    vec![commit_reconstruction(CommitReconstruction {
        sparse: base.results.sparse.clone(),
        trajectory: base.results.trajectory.clone(),
        mesh: Some(Box::new(base.results.mesh.clone())),
        geo: base.results.geo.clone(),
        qc: base.results.qc.clone(),
        assets: payload.assets.iter().map(|binding| ReconstructionAssetCommit { id: binding.id.clone(), content_id: base.assets.get(&binding.id).map(|handle| handle.child_id.clone()) }).collect(),
    })]
}
//#endregion 🔖️Inverse
