//! 🔺️ Sparse diff builder for `UpdateMeshParams` — always present, no target-missing check possible
//! (a struct field, not an id-keyed collection). Identical resubmission ⇒ Warning; non-finite voxel
//! sizes ⇒ Fatal.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::UpdateMeshParams, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
    if payload.params == base.params.mesh {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Mesh params are already up to date.".to_string());
    }
    if !payload.params.tsdf_voxel_size_mm.is_finite() || !payload.params.tsdf_truncation_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Mesh params have a non-finite TSDF voxel size or truncation.".to_string(), [base.id.clone()]);
    }
    let mut params = base.params.clone();
    params.mesh = payload.params.clone();
    protocol::MutationOutcome::new(RemodelDiff { params: Some(params), ..Default::default() })
}
//#endregion 🔖️Diff
