//! 🧬️ Open GLTF mutation descriptor contract and root assembly.

use crate::artifacts::gltf::schema::mutations::bind_node_child::DESCRIPTOR as BIND_NODE_CHILD_DESCRIPTOR;
use crate::artifacts::gltf::schema::mutations::bind_scene_root_node::DESCRIPTOR as BIND_SCENE_ROOT_NODE_DESCRIPTOR;
use crate::artifacts::gltf::schema::mutations::change_material_alpha_mode::DESCRIPTOR as CHANGE_MATERIAL_ALPHA_MODE_DESCRIPTOR;
use crate::artifacts::gltf::schema::mutations::change_material_double_sided::DESCRIPTOR as CHANGE_MATERIAL_DOUBLE_SIDED_DESCRIPTOR;
use crate::artifacts::gltf::schema::mutations::create_scene::DESCRIPTOR as CREATE_SCENE_DESCRIPTOR;
use crate::artifacts::gltf::schema::mutations::unbind_node_child::DESCRIPTOR as UNBIND_NODE_CHILD_DESCRIPTOR;
use crate::artifacts::gltf::schema::mutations::unbind_scene_root_node::DESCRIPTOR as UNBIND_SCENE_ROOT_NODE_DESCRIPTOR;
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
// 🧭️ All seven entries below are already mounted as production modules in `📦️glue.rs` and already
// carry a complete `DESCRIPTOR` const (`plan`/`plan_inverse`/`apply_diff`/`apply_inverse`) and a
// passing fixture case under `mod fixture_tests` above -- the four bind/unbind-node/scene-root
// entries were simply never listed here. Wired for ticket 26/08/23/END-TO-END-TESTING-REFACTOR so
// the mutation-dispatch registry (`🔨️modules/🧭️mutation-dispatch`) and this ticket's oracle can
// exercise all seven; the other 113 leaves on disk are real but not yet mounted in `📦️glue.rs` at
// all, which is a `📦️glue.rs`-owned wiring step out of this ticket's scope.
pub const GLTF_MUTATION_LEAF_DESCRIPTORS: &[GltfMutationLeafDescriptor] = &[
    CHANGE_MATERIAL_ALPHA_MODE_DESCRIPTOR,
    CHANGE_MATERIAL_DOUBLE_SIDED_DESCRIPTOR,
    CREATE_SCENE_DESCRIPTOR,
    BIND_NODE_CHILD_DESCRIPTOR,
    UNBIND_NODE_CHILD_DESCRIPTOR,
    BIND_SCENE_ROOT_NODE_DESCRIPTOR,
    UNBIND_SCENE_ROOT_NODE_DESCRIPTOR,
];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn gltf_mutation_leaf_descriptors() -> &'static [GltfMutationLeafDescriptor] {
    GLTF_MUTATION_LEAF_DESCRIPTORS
}

/// 🏷️ Kebab-case spelling of every currently-registered `GLTF_MUTATION_LEAF_DESCRIPTORS` command id
/// -- the vocabulary the `gltf-2-0-any` mutation catalog
/// (`../../../../🧪️oracle/🔣️component.json`) declares and ticket
/// 26/08/23/END-TO-END-TESTING-REFACTOR's `mutate-gltf-2-0` case measures itself against.
/// `kinds_match_registered_descriptors` below is what keeps this list honest against the assembly,
/// since the framework never parses Rust.
pub const KINDS: &[&str] = &["bind-node-child", "bind-scene-root-node", "change-material-alpha-mode", "change-material-double-sided", "create-scene", "unbind-node-child", "unbind-scene-root-node"];
//#endregion 🔖️Assembly

//#region 🧪️KindsCoverageLaw
#[cfg(test)]
mod kinds_tests {
    use super::*;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn kind_of(command_id: &str) -> &str {
        command_id.strip_prefix("s.stdio.gltf.mutation.").and_then(|rest| rest.strip_suffix(".v1")).unwrap_or(command_id)
    }

    #[test]
    fn kinds_match_registered_descriptors() {
        let mut registered: Vec<&str> = GLTF_MUTATION_LEAF_DESCRIPTORS.iter().map(|descriptor| kind_of(descriptor.command_id)).collect();
        registered.sort_unstable();
        let mut declared: Vec<&str> = KINDS.to_vec();
        declared.sort_unstable();
        assert_eq!(registered, declared, "KINDS must name exactly the descriptors GLTF_MUTATION_LEAF_DESCRIPTORS registers");
        assert_eq!(KINDS.len(), 7, "gltf-2-0-any declares 7 registered descriptor kinds");
    }
}
//#endregion 🧪️KindsCoverageLaw

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
