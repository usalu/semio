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

//#region 🧪️FixtureTests
// 🧪️ Handcrafted mutation fixtures (contract D1, ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION): one
// committed case per leaf under `<leaf>/🧪️tests/<case>/`, each with its own `⬅️before`/`➡️after`
// snapshots, `🦠️mutation` payload, `🔺️diff` (or `🚫️component.absent` when the case is a refusal),
// `🎯️outcome` and a `🦀️component.rs` whose seven assertions are worded for that one mutation.
// Wired HERE and not in `📦️glue.rs`: that file is shared with the agents migrating the other stdio
// artifacts, so its production mounts stay untouched while this artifact owns its own test mounts.
// `#[path = "."]` re-bases the children on this file's own directory, which is what makes each
// leaf-relative path below resolve.
//
// ⚠️ Only the SEVEN leaves that `📦️glue.rs` currently mounts as production modules are listed below.
// All 120 leaves carry a committed fixture case on disk, and every one of the 120 test files is
// written against that leaf's own `mutation`/`diff`/`inverse` entry points — but the other 113 leaf
// modules are not in the crate's module tree yet, so mounting their tests here would not compile.
// Add the matching `mod tests_…;` line the moment a leaf is wired into `📦️glue.rs`. The full
// inventory, and the reachability findings behind this split, are in
// `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/COMPOSE-TO-PUZZLE5D-MIGRATION/📓️census/📓️fixtures-stdio-gltf.md`.
#[cfg(test)]
#[path = "."]
mod fixture_tests {
    #[path = "bind-node-child/🧪️tests/adopts-the-child-node-under-the-parent-node/🦀️component.rs"]
    mod tests_bind_node_child_adopts_the_child_node_under_the_parent_node;
    #[path = "bind-scene-root-node/🧪️tests/promotes-the-root-node-into-the-main-scene/🦀️component.rs"]
    mod tests_bind_scene_root_node_promotes_the_root_node_into_the_main_scene;
    #[path = "change-material-alpha-mode/🧪️tests/switches-the-default-material-from-opaque-to-mask/🦀️component.rs"]
    mod tests_change_material_alpha_mode_switches_the_default_material_from_opaque_to_mask;
    #[path = "change-material-double-sided/🧪️tests/makes-the-default-material-double-sided/🦀️component.rs"]
    mod tests_change_material_double_sided_makes_the_default_material_double_sided;
    #[path = "create-scene/🧪️tests/inserts-an-empty-scene-ahead-of-the-default-scene/🦀️component.rs"]
    mod tests_create_scene_inserts_an_empty_scene_ahead_of_the_default_scene;
    #[path = "unbind-node-child/🧪️tests/releases-the-child-node-from-the-parent-node/🦀️component.rs"]
    mod tests_unbind_node_child_releases_the_child_node_from_the_parent_node;
    #[path = "unbind-scene-root-node/🧪️tests/demotes-the-root-node-out-of-the-main-scene/🦀️component.rs"]
    mod tests_unbind_scene_root_node_demotes_the_root_node_out_of_the_main_scene;
}
//#endregion 🧪️FixtureTests
