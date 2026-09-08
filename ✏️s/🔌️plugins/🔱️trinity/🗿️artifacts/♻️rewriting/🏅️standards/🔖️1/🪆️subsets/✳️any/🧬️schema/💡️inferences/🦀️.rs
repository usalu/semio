//! 💡️ Rewriting inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors puzzle3d's own `💡️inferences/` (the pattern's exemplar): this file is the
//! family-root assembly (never mod's/includes the slug dirs directly — `🦀️.rs` is the sole
//! mounting mechanism, same as mutations); each named inference gets its own `<emoji><slug>/` child
//! (currently: `📦bounds/`, the only positioned data this rule-editing artifact's snapshot carries
//! a typed shape for — `before_fixture_json`/`lhs_json`/`rhs_json` are opaque JSON blobs, not
//! structured graph data this artifact's own snapshot exposes).

use crate::RewritingSnapshot;
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

use super::bounds::{compute_bounds};
//#region 🔖️Inference
/// 💡️ Everything inferable from a rewrite-rule snapshot. One field per named inference under
/// `💡️inferences/` (currently: `bounds`, backed by the `📦bounds/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.trinity.rewriting.inference")]
pub struct RewritingInference {
    #[derived]
    pub bounds: RewritingBounds,
}

impl protocol::Inference<RewritingSnapshot> for RewritingInference {
    fn infer(snapshot: &RewritingSnapshot) -> Self {
        Self { bounds: compute_bounds(snapshot) }
    }
}

impl protocol::InferenceSpec<RewritingSnapshot> for RewritingInference {
    fn inference_schema_id() -> &'static str {
        "s.trinity.rewriting.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.trinity.rewriting.inference.bounds", reads: &["rule_layout"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 🧠️ Uncached: a handful of `{x, y}` points recomputed in one pass — the default `infer_cached`
/// passthrough (just calls `infer`) is exactly right here, no `InferredField` chain needed.
impl ArtifactInferrer for crate::standards::v1::subsets::any::schema::RewritingBuilder {
    type Snapshot = RewritingSnapshot;
    type Inference = RewritingInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.trinity.rewriting.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `rewriting_artifact_schema_descriptor`'s registration.
pub fn rewriting_artifact_inference_descriptor() -> ::semio_framework_schema::ArtifactInferenceDescriptor {
    ::semio_framework_schema::ArtifactInferenceDescriptor {
        id: "s.trinity.rewriting.inference",
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
pub use super::bounds::RewritingBounds;
//#endregion 🔁️Re-exports
