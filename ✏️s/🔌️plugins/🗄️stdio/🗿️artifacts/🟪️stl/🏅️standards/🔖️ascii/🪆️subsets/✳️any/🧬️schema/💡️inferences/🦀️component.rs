//! 💡️ StlInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`, honestly derivable from
//! `triangles` alone — real STL has no shared vertex index space, so this is a direct fold over
//! every triangle's own 3 vertices).

use crate::artifacts::stl::StlSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::bounds::{compute_stl_bounds, StlBounds};

//#region 🔖️Inference
/// 💡️ Everything inferable from a stl snapshot. One field per named inference under
/// `💡️inferences/` (currently: `bounds`, backed by the `📦bounds/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.stl.inference")]
pub struct StlInference {
    #[state(inferred)]
    pub bounds: StlBounds,
}

impl protocol::Inference<StlSnapshot> for StlInference {
    fn infer(snapshot: &StlSnapshot) -> Self {
        Self { bounds: compute_stl_bounds(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `StlSnapshot::default()`'s `triangles` ever stops being empty.
impl Default for StlInference {
    fn default() -> Self {
        <Self as protocol::Inference<StlSnapshot>>::infer(&StlSnapshot::default())
    }
}

impl protocol::InferenceSpec<StlSnapshot> for StlInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.stl.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.stl.inference.bounds", reads: &["triangles"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here — `bounds` is a single min/max fold over every triangle's own 3
/// vertices, already O(n) in total triangle count with no honest per-triangle incremental
/// decomposition (a merkle dep-chain over this flat triangle list costs more than the fold it
/// would cache) — the default `infer_cached` passthrough is exact.
impl ArtifactInferrer for crate::artifacts::stl::standards::v_ascii::subsets::any::schema::StlBuilder {
    type Snapshot = StlSnapshot;
    type Inference = StlInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.stl.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `stl_artifact_schema_descriptor`'s registration.
pub fn stl_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.stl.inference",
        inference: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use protocol::Inference;

    #[test]
    fn inference_determinism_law() {
        let snapshot = StlSnapshot::default();
        assert_eq!(StlInference::infer(&snapshot), StlInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(StlInference::infer(&StlSnapshot::default()), StlInference::default());
    }
}
//#endregion 🧪️Tests
