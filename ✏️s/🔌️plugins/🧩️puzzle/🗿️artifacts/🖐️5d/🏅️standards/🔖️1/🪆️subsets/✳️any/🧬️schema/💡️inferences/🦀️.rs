//! 💡️ Puzzle5d inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors puzzle3d's own `💡️inferences/` (this artifact's exemplar): this file is the
//! family-root assembly (never mod's/includes the slug dirs directly — `🦀️.rs` is the sole
//! mounting mechanism, same as mutations); each named inference gets its own `<emoji><slug>/` child
//! (currently: `🎛️flat-position/`).
//!
//! 🎛️ Puzzle5d's flatten math lives at `💡️inferences/🎛️flat-position` (`flatten_snapshot`, which maps
//! parts/grips/fasteners onto the 3d object/vortex/attraction graph and runs puzzle3d's own solver) —
//! unlike puzzle3d's own inference, there is no separate low-level per-edge decomposition exposed
//! here to drive an incremental `InferredField` chain, so this inference is a plain whole-snapshot
//! `Inference` impl (per the family root's own "simple whole-snapshot scalars" guidance) that calls
//! that sibling slug's function directly; `ArtifactInferrer::infer_cached`'s default passthrough
//! (just calls `infer`) is used as-is, uncached.


use crate::standards::v1::subsets::any::schema::inferences::flat_position::flatten_snapshot;
use crate::Puzzle5dSnapshot;
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use std::collections::BTreeMap;

//#region 🔖️Inference
/// 💡️ Everything inferable from a puzzle5d snapshot. One field per named inference under
/// `💡️inferences/` (currently: `flatPositions`, backed by the `🎛️flat-position/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.puzzle.puzzle5d.inference")]
pub struct Puzzle5dInference {
    #[derived]
    pub flat_positions: BTreeMap<String, FlattenPose>,
}

impl protocol::Inference<Puzzle5dSnapshot> for Puzzle5dInference {
    fn infer(snapshot: &Puzzle5dSnapshot) -> Self {
        Self { flat_positions: flatten_snapshot(snapshot).into_iter().collect() }
    }
}

impl protocol::InferenceSpec<Puzzle5dSnapshot> for Puzzle5dInference {
    fn inference_schema_id() -> &'static str {
        "s.puzzle.puzzle5d.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.puzzle.puzzle5d.inference.flatPosition", reads: &["parts", "fasteners"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 🧠️ Uncached: `flatten_snapshot` recomputes the whole graph in one pass and puzzle5d's engine
/// exposes no per-edge decomposition to key an `InferredField` chain off of (see the module doc) —
/// the default `infer_cached` passthrough (just calls `infer`) is exactly right here.
impl ArtifactInferrer for crate::standards::v1::subsets::any::schema::Puzzle5dBuilder {
    type Snapshot = Puzzle5dSnapshot;
    type Inference = Puzzle5dInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.puzzle.puzzle5d.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `puzzle5d_artifact_schema_descriptor`'s registration.
pub fn puzzle5d_artifact_inference_descriptor() -> ::semio_framework_schema::ArtifactInferenceDescriptor {
    ::semio_framework_schema::ArtifactInferenceDescriptor {
        id: "s.puzzle.puzzle5d.inference",
        inference: ::semio_framework_schema::FacetLeaves {
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
pub use semio_s_artifact_puzzle_3d::FlattenPose;
//#endregion 🔁️Re-exports
