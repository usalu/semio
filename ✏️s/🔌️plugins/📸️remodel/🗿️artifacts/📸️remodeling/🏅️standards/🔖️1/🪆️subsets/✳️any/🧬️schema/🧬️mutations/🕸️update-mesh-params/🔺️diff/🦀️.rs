//! 🔺️ Sparse diff builder for `UpdateMeshParams` — always present, no target-missing check possible
//! (a struct field, not an id-keyed collection). The invariant is checked BEFORE the
//! identical-resubmission warning, the one guard order the whole vocabulary follows — non-finite voxel
//! sizes ⇒ Fatal.
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingParamsDiff;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::UpdateMeshParams, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if !payload.params.tsdf_voxel_size_mm.is_finite() || !payload.params.tsdf_truncation_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Mesh params have a non-finite TSDF voxel size or truncation.".to_string(), [base.id.clone()]);
    }
    if payload.params == base.params.mesh {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Mesh params are already up to date.".to_string());
    }
    protocol::MutationOutcome::new(RemodelingDiff { params: Some(RemodelingParamsDiff { mesh: Some(payload.params.clone()), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
