//! 💡️ Block5d inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`).
//!
//! Like block2d/block3d, block5d has no parent/child object graph — it is a single kind DEFINITION
//! (one `PartKind` plus a catalog of rim `Block5dGripTemplate`s, each carrying both a 2d polar
//! placement and a 3d cartesian placement — see `Block5dGripTemplate`'s doc). The honest
//! whole-snapshot inference here mirrors block3d: a 3d bounding box + vertex count over the grip
//! templates' `position`/`radius3d` fields (the part's 3d-projection rim geometry), expressed as a
//! plain `Inference` impl (no per-entity `InferredField` caching needed).

use crate::Block5dSnapshot;
use ::semio_framework_schema::ArtifactSchema;


use super::bounds::{compute_block5d_bounds};
//#region 🔖️Inference
/// 💡️ Everything inferable from a block5d snapshot. One field per named inference under
/// `💡️inferences/` (currently: `bounds`, backed by the `📦bounds/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[artifact_schema(id = "s.block.block5d.inference")]
pub struct Block5dInference {
    #[derived]
    pub bounds: Block5dBounds,
}

impl protocol::Inference<Block5dSnapshot> for Block5dInference {
    fn infer(snapshot: &Block5dSnapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { bounds: compute_block5d_bounds(snapshot) }
    
        })
    }
}

impl protocol::InferenceSpec<Block5dSnapshot> for Block5dInference {
    fn inference_schema_id() -> &'static str {
        "s.block.block5d.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.block.block5d.inference.bounds", reads: &["grips"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️PuzzleCatalogFragment
//#endregion 🔖️PuzzleCatalogFragment

//#region 🔖️Descriptor
/// 💡️ Registers `s.block.block5d.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `block5d_artifact_schema_descriptor`'s registration.
pub fn block5d_artifact_inference_descriptor() -> ::semio_framework_schema_registry::ArtifactInferenceDescriptor {
    ::semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.block.block5d.inference",
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
pub use super::bounds::Block5dBounds;
//#endregion 🔁️Re-exports
