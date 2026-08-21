//! ⚙️ ⚙️ Remodel play app commands command — `set-mesh-params`.

use crate::artifacts::remodel::mutations::update_mesh_params;
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::{MeshParams, RemodelSnapshot};
use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "mesh-params")]
pub struct SetMeshParams {
    pub tsdf_voxel_size_mm: f32,
    pub tsdf_truncation_mm: f32,
    pub decimate_target_triangles: u32,
    pub smoothing_iterations: u32,
    pub texture_enabled: bool,
    pub texture_size: u32,
    pub guarantee_watertight: bool,
    pub hole_fill_max_boundary_verts: u32,
    pub self_intersection_check: bool,
}

pub async fn handle(payload: &SetMeshParams, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![update_mesh_params(MeshParams {
        tsdf_voxel_size_mm: payload.tsdf_voxel_size_mm,
        tsdf_truncation_mm: payload.tsdf_truncation_mm,
        decimate_target_triangles: payload.decimate_target_triangles,
        smoothing_iterations: payload.smoothing_iterations,
        texture_enabled: payload.texture_enabled,
        texture_size: payload.texture_size,
        guarantee_watertight: payload.guarantee_watertight,
        hole_fill_max_boundary_verts: payload.hole_fill_max_boundary_verts,
        self_intersection_check: payload.self_intersection_check,
    })]))
}
