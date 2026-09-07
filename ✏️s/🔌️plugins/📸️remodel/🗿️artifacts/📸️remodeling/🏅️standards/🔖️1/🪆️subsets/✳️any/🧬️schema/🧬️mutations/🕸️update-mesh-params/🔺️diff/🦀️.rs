//! 🔺️ Sparse diff builder for `UpdateMeshParams` — always present, no target-missing check possible
//! (a struct field, not an id-keyed collection). The invariant is checked BEFORE the
//! identical-resubmission warning, the one guard order the whole vocabulary follows — non-finite voxel
//! sizes ⇒ Fatal.
use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::UpdateMeshParams, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if !payload.params.tsdf_voxel_size_mm.is_finite() || !payload.params.tsdf_truncation_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Mesh params have a non-finite TSDF voxel size or truncation.".to_string(), [base.id.clone()]);
    }
    if payload.params == base.params.mesh {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Mesh params are already up to date.".to_string());
    }
    let mut params = base.params.clone();
    params.mesh = payload.params.clone();
    protocol::MutationOutcome::new(RemodelingDiff { params: Some(params), ..Default::default() })
}
//#endregion 🔖️Diff
