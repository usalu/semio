//! 💡️ GltfInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`, honestly derivable from
//! every `document.meshes[].primitives[]`'s own `POSITION` accessor alone).

use crate::artifacts::gltf::schema::snapshot::GltfSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::bounds::{compute_gltf_bounds, GltfBounds};

//#region 🔖️Inference
/// 💡️ Everything inferable from a gltf snapshot. One field per named inference under
/// `💡️inferences/` (currently: `bounds`, backed by the `📦bounds/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.gltf.inference")]
pub struct GltfInference {
    #[derived]
    pub bounds: GltfBounds,
}

impl protocol::Inference<GltfSnapshot> for GltfInference {
    fn infer(snapshot: &GltfSnapshot) -> Self {
        Self { bounds: compute_gltf_bounds(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `GltfSnapshot::default()`'s `document` ever stops being empty.
impl Default for GltfInference {
    fn default() -> Self {
        <Self as protocol::Inference<GltfSnapshot>>::infer(&GltfSnapshot::default())
    }
}

impl protocol::InferenceSpec<GltfSnapshot> for GltfInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.gltf.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.bounds", reads: &["document"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here — `bounds` is a single min/max fold over every mesh primitive's own
/// `POSITION` accessor `min`/`max` (plus a `count`/mesh/primitive tally), already O(n) in total
/// mesh-primitive count with no honest per-entity incremental decomposition (a merkle dep-chain
/// over this flat mesh/primitive/accessor walk costs more than the fold it would cache) — the
/// default `infer_cached` passthrough is exact.
impl ArtifactInferrer for crate::artifacts::gltf::standards::v2_0::subsets::any::schema::GltfBuilder {
    type Snapshot = GltfSnapshot;
    type Inference = GltfInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.gltf.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `gltf_artifact_schema_descriptor`'s registration.
pub fn gltf_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.gltf.inference",
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
        let snapshot = GltfSnapshot::default();
        assert_eq!(GltfInference::infer(&snapshot), GltfInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(GltfInference::infer(&GltfSnapshot::default()), GltfInference::default());
    }
}
//#endregion 🧪️Tests
