//! 💡️ LasInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`, honestly derivable from
//! `header` alone — LAS's own spec puts the authoritative bounding box and point count directly
//! in the public header block, not derived from `points`).

use crate::artifacts::las::LasSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::bounds::{compute_las_bounds, LasBounds};

//#region 🔖️Inference
/// 💡️ Everything inferable from a las snapshot. One field per named inference under
/// `💡️inferences/` (currently: `bounds`, backed by the `📦bounds/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.las.inference")]
pub struct LasInference {
    #[derived]
    pub bounds: LasBounds,
}

impl protocol::Inference<LasSnapshot> for LasInference {
    async fn infer(snapshot: &LasSnapshot) -> Self {
        Self { bounds: compute_las_bounds(snapshot).await }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `LasSnapshot::default()`'s `header` ever stops being all-zero.
impl Default for LasInference {
    fn default() -> Self {
        <Self as protocol::Inference<LasSnapshot>>::infer(&LasSnapshot::default())
    }
}

impl protocol::InferenceSpec<LasSnapshot> for LasInference {
    async fn inference_schema_id() -> &'static str {
        "s.stdio.las.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.las.inference.bounds", reads: &["header"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here — `bounds` is a direct honest read of the header's own
/// spec-mandated declared bounds/count fields, already O(1) with no per-entity decomposition (a
/// merkle dep-chain over a flat header-field read costs more than the read it would cache) — the
/// default `infer_cached` passthrough is exact.
impl ArtifactInferrer for crate::artifacts::las::standards::v1_0::subsets::any::schema::LasBuilder {
    type Snapshot = LasSnapshot;
    type Inference = LasInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.las.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `las_artifact_schema_descriptor`'s registration.
pub async fn las_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.las.inference",
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

    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = LasSnapshot::default();
        assert_eq!(LasInference::infer(&snapshot), LasInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(LasInference::infer(&LasSnapshot::default()), LasInference::default());
    }
}
//#endregion 🧪️Tests
