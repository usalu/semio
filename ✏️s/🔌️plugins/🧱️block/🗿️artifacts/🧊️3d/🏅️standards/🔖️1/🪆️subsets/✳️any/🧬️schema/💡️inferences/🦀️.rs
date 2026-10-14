//! 💡️ Block3d inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`).
//!
//! Unlike puzzle3d, block3d has no parent/child object graph — it is a single kind DEFINITION (one
//! `ObjectKind` plus a catalog of rim `Block3dVortexTemplate`s), so the honest whole-snapshot
//! inference here is a geometric bounding box + vertex count over the vortex templates' rim
//! positions, expressed as a plain `Inference` impl (no per-entity `InferredField` caching needed —
//! there is nothing to invalidate incrementally over a flat template list).

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_block_3d_model::Block3dSnapshot;
use ::semio_framework_schema::ArtifactSchema;

use crate::bounds::compute_block3d_bounds;
//#region 🔖️Inference
/// 💡️ Everything inferable from a block3d snapshot. One field per named inference under
/// `💡️inferences/` (currently: `bounds`, backed by the `📦bounds/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[artifact_schema(id = "s.block.block3d.inference")]
pub struct Block3dInference {
    #[derived]
    pub bounds: Block3dBounds,
}

impl protocol::Inference<Block3dSnapshot> for Block3dInference {
    fn infer(snapshot: &Block3dSnapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { bounds: compute_block3d_bounds(snapshot) }
    
        })
    }
}

impl protocol::InferenceSpec<Block3dSnapshot> for Block3dInference {
    fn inference_schema_id() -> &'static str {
        "s.block.block3d.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.block.block3d.inference.bounds", reads: &["vortices"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️PuzzleCatalogFragment
/// 🌐️ Resolves the active representation's mesh url — the first representation whose `tags` all
/// appear in `wanted_tags`, or the first representation overall, or `None` for an empty catalog.
pub fn resolve_active_mesh_url<'a>(definition: &'a Block3dSnapshot, wanted_tags: &[&str]) -> Option<&'a str> {
    definition
        .representations
        .iter()
        .find(|representation| wanted_tags.iter().all(|tag| representation.tags.iter().any(|other| other == tag)))
        .or_else(|| definition.representations.first())
        .and_then(|representation| representation.mesh_url.as_deref())
}

//#endregion 🔖️PuzzleCatalogFragment

//#region 🔖️Descriptor
/// 💡️ Registers `s.block.block3d.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `block3d_artifact_schema_descriptor`'s registration.
pub fn block3d_artifact_inference_descriptor() -> ::semio_framework_schema_registry::ArtifactInferenceDescriptor {
    ::semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.block.block3d.inference",
        inference: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::bounds::Block3dBounds;
//#endregion 🔁️Re-exports

//#region 🪢️Mounts
#[path = "📦bounds/🦀️.rs"]
pub mod bounds;
//#endregion 🪢️Mounts
