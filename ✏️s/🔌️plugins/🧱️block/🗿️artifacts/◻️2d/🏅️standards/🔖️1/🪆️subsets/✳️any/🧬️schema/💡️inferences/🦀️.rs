//! 💡️ Block2d inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`).
//!
//! Like block3d, block2d has no parent/child object graph — it is a single kind DEFINITION (one
//! `NodeKind` plus a catalog of rim `Block2dHandleTemplate`s placed by polar `angle`/`radius`), so
//! the honest whole-snapshot inference here is a geometric bounding box + vertex count over the
//! handle templates' rim positions (converted from polar to cartesian), expressed as a plain
//! `Inference` impl (no per-entity `InferredField` caching needed).

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_block_2d_model::Block2dSnapshot;
use ::semio_framework_schema::ArtifactSchema;

use crate::bounds::compute_block2d_bounds;
//#region 🔖️Inference
/// 💡️ Everything inferable from a block2d snapshot. One field per named inference under
/// `💡️inferences/` (currently: `bounds`, backed by the `📦bounds/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[artifact_schema(id = "s.block.block2d.inference")]
pub struct Block2dInference {
    #[derived]
    pub bounds: Block2dBounds,
}

impl protocol::Inference<Block2dSnapshot> for Block2dInference {
    fn infer(snapshot: &Block2dSnapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { bounds: compute_block2d_bounds(snapshot) }
    
        })
    }
}

impl protocol::InferenceSpec<Block2dSnapshot> for Block2dInference {
    fn inference_schema_id() -> &'static str {
        "s.block.block2d.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.block.block2d.inference.bounds", reads: &["handles"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️Descriptor
/// 💡️ Registers `s.block.block2d.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `block2d_artifact_schema_descriptor`'s registration.
pub fn block2d_artifact_inference_descriptor() -> ::semio_framework_schema_registry::ArtifactInferenceDescriptor {
    ::semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.block.block2d.inference",
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

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::bounds::Block2dBounds;
//#endregion 🔁️Re-exports

//#region 🪢️Mounts
#[path = "📦bounds/🦀️.rs"]
pub mod bounds;
//#endregion 🪢️Mounts
