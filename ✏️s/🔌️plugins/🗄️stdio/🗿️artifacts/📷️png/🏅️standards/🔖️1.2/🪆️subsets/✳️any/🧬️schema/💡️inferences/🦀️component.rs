//! 💡️ Png inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📐dimensions/`).

use crate::artifacts::png::PngSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

use super::dimensions::{compute_png_dimensions, PngDimensions};

//#region 🔖️Inference
/// 💡️ Everything inferable from a png snapshot. One field per named inference under
/// `💡️inferences/` (currently: `dimensions`, backed by the `📐dimensions/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.png.inference")]
pub struct PngInference {
    #[state(inferred)]
    pub dimensions: PngDimensions,
}

impl protocol::Inference<PngSnapshot> for PngInference {
    fn infer(snapshot: &PngSnapshot) -> Self {
        Self { dimensions: compute_png_dimensions(snapshot) }
    }
}

/// 🌱 Hand-fixed to agree with `infer(&PngSnapshot::default())` rather than a naive
/// `#[derive(Default)]` — `PngSnapshot::default()`'s `bitDepth: 8`/`colorType: Rgba` disagree with
/// a structurally-derived all-zero `PngDimensions`, the same "match `infer` of the real default,
/// don't derive structurally" trick as `AddInference`'s hand-written `Default` in
/// `📡️spr/🎮️command/🦀️component.rs`.
impl Default for PngInference {
    fn default() -> Self {
        <Self as protocol::Inference<PngSnapshot>>::infer(&PngSnapshot::default())
    }
}

impl protocol::InferenceSpec<PngSnapshot> for PngInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.png.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.png.inference.dimensions", reads: &["width", "height", "bitDepth", "colorType"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here (an IHDR-field read is already O(1)) — the default `infer_cached`
/// passthrough (`ArtifactInferrer::infer_cached`) is exact.
impl semio_framework_plugin::ArtifactInferrer for crate::artifacts::png::standards::v1_2::subsets::any::schema::PngBuilder {
    type Snapshot = PngSnapshot;
    type Inference = PngInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.png.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `png_artifact_schema_descriptor`'s registration.
pub fn png_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.png.inference",
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
        let snapshot = PngSnapshot::default();
        assert_eq!(PngInference::infer(&snapshot), PngInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(PngInference::infer(&PngSnapshot::default()), PngInference::default());
    }
}
//#endregion 🧪️Tests
