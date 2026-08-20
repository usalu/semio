//! 🧬️ Open GLTF mutation descriptor contract and root assembly.

use crate::artifacts::gltf::schema::mutations::change_material_alpha_mode::DESCRIPTOR as CHANGE_MATERIAL_ALPHA_MODE_DESCRIPTOR;
use crate::artifacts::gltf::schema::mutations::change_material_double_sided::DESCRIPTOR as CHANGE_MATERIAL_DOUBLE_SIDED_DESCRIPTOR;
use crate::artifacts::gltf::schema::mutations::create_scene::DESCRIPTOR as CREATE_SCENE_DESCRIPTOR;
use crate::artifacts::gltf::schema::snapshot::GltfSnapshot;

//#region 🔖️DescriptorContract
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GltfMutationLeafError {
    pub code: String,
    pub path: String,
    pub detail: String,
}

impl std::fmt::Display for GltfMutationLeafError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{} at {}: {}", self.code, self.path, self.detail)
    }
}

impl std::error::Error for GltfMutationLeafError {}

pub struct GltfMutationLeafPlan {
    pub diff_payload: Vec<u8>,
    pub inverse_payload: Vec<u8>,
    pub touched_paths: Vec<String>,
}

pub struct GltfMutationLeafApplication {
    pub snapshot: GltfSnapshot,
    pub touched_paths: Vec<String>,
}

#[derive(Clone, Copy)]
pub struct GltfMutationLeafDescriptor {
    pub command_id: &'static str,
    pub version: u32,
    pub plan: fn(&[u8], &GltfSnapshot) -> Result<GltfMutationLeafPlan, GltfMutationLeafError>,
    pub plan_inverse: fn(&[u8], &GltfSnapshot) -> Result<GltfMutationLeafPlan, GltfMutationLeafError>,
    pub apply_diff: fn(&[u8], &GltfSnapshot) -> Result<GltfMutationLeafApplication, GltfMutationLeafError>,
    pub apply_inverse: fn(&[u8], &GltfSnapshot) -> Result<GltfMutationLeafApplication, GltfMutationLeafError>,
}
//#endregion 🔖️DescriptorContract

//#region 🔖️Assembly
pub const GLTF_MUTATION_LEAF_DESCRIPTORS: &[GltfMutationLeafDescriptor] = &[CHANGE_MATERIAL_ALPHA_MODE_DESCRIPTOR, CHANGE_MATERIAL_DOUBLE_SIDED_DESCRIPTOR, CREATE_SCENE_DESCRIPTOR];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn gltf_mutation_leaf_descriptors() -> &'static [GltfMutationLeafDescriptor] {
    GLTF_MUTATION_LEAF_DESCRIPTORS
}
//#endregion 🔖️Assembly
