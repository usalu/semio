//! 🧳️ Mesh-parameter changed-value projection and inverse, after its retained decision.
use super::UpdateMeshParams;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::MeshParams;
crate::retained_remodeling_parameter_field!(UpdateMeshParams, MeshParams, mesh, 5);
impl crate::RetainedParameterScalar for UpdateMeshParams {
    const WORK_DEMAND: u64 = 11;
    const NOOP_MESSAGE: &'static str = "Mesh params are already up to date.";
    const REFUSAL_MESSAGE: Option<&'static str> = Some("Mesh params have a non-finite TSDF voxel size or truncation.");
    fn retained_parameter_disposition(&self, original: &semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot) -> (crate::RetainedParameterDisposition, u64) {
        use crate::RetainedParameterDisposition::*;
        let p = &self.params; let old = &original.params.mesh;
        if !p.tsdf_voxel_size_mm.is_finite() | !p.tsdf_truncation_mm.is_finite() { return (TargetedRefusal, 2); }
        let same = (p.tsdf_voxel_size_mm == old.tsdf_voxel_size_mm) & (p.tsdf_truncation_mm == old.tsdf_truncation_mm) & (p.decimate_target_triangles == old.decimate_target_triangles) & (p.smoothing_iterations == old.smoothing_iterations) & (p.texture_enabled == old.texture_enabled) & (p.texture_size == old.texture_size) & (p.guarantee_watertight == old.guarantee_watertight) & (p.hole_fill_max_boundary_verts == old.hole_fill_max_boundary_verts) & (p.self_intersection_check == old.self_intersection_check);
        (if same { Unchanged } else { Changed }, 11)
    }
}
crate::retained_parameter_scalar_vectors!(UpdateMeshParams, "mesh", include_str!("../../🧪️tests/🧫️retained-params/🧪️.json"), |original: &semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot, change: &serde_json::Map<String, serde_json::Value>| {
    let mut params = original.params.mesh.clone();
    if let Some(value) = change.get("texture_size") { params.texture_size = value.as_u64().unwrap() as u32; }
    if change.contains_key("tsdf_voxel_size_mm") { params.tsdf_voxel_size_mm = f32::NAN; }
    UpdateMeshParams { params }
});
