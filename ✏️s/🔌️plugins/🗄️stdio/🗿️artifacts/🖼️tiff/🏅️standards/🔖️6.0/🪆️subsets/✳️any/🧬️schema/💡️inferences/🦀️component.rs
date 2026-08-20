//! 💡️ Tiff inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📐dimensions/`).

use crate::artifacts::tiff::TiffSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

use super::dimensions::{compute_tiff_dimensions, TiffDimensions};

//#region 🔖️Inference
/// 💡️ Everything inferable from a tiff snapshot. One field per named inference under
/// `💡️inferences/` (currently: `dimensions`, backed by the `📐dimensions/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.tiff.inference")]
pub struct TiffInference {
    #[derived]
    pub dimensions: TiffDimensions,
}

impl protocol::Inference<TiffSnapshot> for TiffInference {
    async fn infer(snapshot: &TiffSnapshot) -> Self {
        Self { dimensions: compute_tiff_dimensions(snapshot) }
    }
}

/// 🌱 Hand-fixed to agree with `infer(&TiffSnapshot::default())` rather than a naive
/// `#[derive(Default)]` — the missing-`BitsPerSample` fallback (`bitDepth: 1`, TIFF6 §8's own
/// documented default) disagrees with a structurally-derived all-zero `TiffDimensions`, the same
/// "match `infer` of the real default, don't derive structurally" trick as `AddInference`'s
/// hand-written `Default` in `📡️spr/🎮️command/🦀️component.rs`.
impl Default for TiffInference {
    fn default() -> Self {
        <Self as protocol::Inference<TiffSnapshot>>::infer(&TiffSnapshot::default())
    }
}

impl protocol::InferenceSpec<TiffSnapshot> for TiffInference {
    async fn inference_schema_id() -> &'static str {
        "s.stdio.tiff.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.tiff.inference.dimensions", reads: &["ifds"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here (a tag lookup is already O(1)) — the default `infer_cached`
/// passthrough (`ArtifactInferrer::infer_cached`) is exact.
impl semio_framework_plugin::ArtifactInferrer for crate::artifacts::tiff::standards::v6_0::subsets::any::schema::TiffBuilder {
    type Snapshot = TiffSnapshot;
    type Inference = TiffInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.tiff.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `tiff_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn tiff_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.tiff.inference",
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
        let snapshot = TiffSnapshot::default();
        assert_eq!(TiffInference::infer(&snapshot), TiffInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(TiffInference::infer(&TiffSnapshot::default()), TiffInference::default());
    }
}
//#endregion 🧪️Tests
