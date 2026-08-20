//! 💡️ Bmp inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📐dimensions/`).

use crate::artifacts::bmp::BmpSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

use super::dimensions::{compute_bmp_dimensions, BmpDimensions};

//#region 🔖️Inference
/// 💡️ Everything inferable from a bmp snapshot. One field per named inference under
/// `💡️inferences/` (currently: `dimensions`, backed by the `📐dimensions/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.bmp.inference")]
pub struct BmpInference {
    #[derived]
    pub dimensions: BmpDimensions,
}

impl protocol::Inference<BmpSnapshot> for BmpInference {
    async fn infer(snapshot: &BmpSnapshot) -> Self {
        Self { dimensions: compute_bmp_dimensions(snapshot).await }
    }
}

/// 🌱 Hand-fixed to agree with `infer(&BmpSnapshot::default())` rather than a naive
/// `#[derive(Default)]` — `BmpSnapshot::default()`'s `bitsPerPixel: 24` disagrees with a
/// structurally-derived all-zero `BmpDimensions`, the same "match `infer` of the real default,
/// don't derive structurally" trick as `AddInference`'s hand-written `Default` in
/// `📡️spr/🎮️command/🦀️component.rs`.
impl Default for BmpInference {
    fn default() -> Self {
        <Self as protocol::Inference<BmpSnapshot>>::infer(&BmpSnapshot::default())
    }
}

impl protocol::InferenceSpec<BmpSnapshot> for BmpInference {
    async fn inference_schema_id() -> &'static str {
        "s.stdio.bmp.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.bmp.inference.dimensions", reads: &["width", "height", "bitsPerPixel"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here (a header-field read is already O(1)) — the default
/// `infer_cached` passthrough (`ArtifactInferrer::infer_cached`) is exact.
impl semio_framework_plugin::ArtifactInferrer for crate::artifacts::bmp::standards::v_v3::subsets::any::schema::BmpBuilder {
    type Snapshot = BmpSnapshot;
    type Inference = BmpInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.bmp.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `bmp_artifact_schema_descriptor`'s registration.
pub async fn bmp_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.bmp.inference",
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
        let snapshot = BmpSnapshot::default();
        assert_eq!(BmpInference::infer(&snapshot), BmpInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(BmpInference::infer(&BmpSnapshot::default()), BmpInference::default());
    }
}
//#endregion 🧪️Tests
