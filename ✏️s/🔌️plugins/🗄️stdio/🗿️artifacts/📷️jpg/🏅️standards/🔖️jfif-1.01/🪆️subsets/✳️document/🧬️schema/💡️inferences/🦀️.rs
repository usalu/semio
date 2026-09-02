//! 💡️ Jpg inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📐dimensions/`).

use crate::artifacts::jpg::JpgSnapshot;
use schema::ArtifactSchema;

use super::dimensions::{compute_jpg_dimensions, JpgDimensions};

//#region 🔖️Inference
/// 💡️ Everything inferable from a jpg snapshot. One field per named inference under
/// `💡️inferences/` (currently: `dimensions`, backed by the `📐dimensions/` slug dir).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.jpg.inference")]
pub struct JpgInference {
    #[derived]
    pub dimensions: JpgDimensions,
}

impl protocol::Inference<JpgSnapshot> for JpgInference {
    fn infer(snapshot: &JpgSnapshot) -> Self {
        Self { dimensions: compute_jpg_dimensions(snapshot) }
    }
}

/// 🌱 Hand-fixed to agree with `infer(&JpgSnapshot::default())` rather than a naive
/// `#[derive(Default)]` — the canonical `bitDepth: 8` fallback disagrees with a
/// structurally-derived all-zero `JpgDimensions`, the same "match `infer` of the real default,
/// don't derive structurally" trick as `AddInference`'s hand-written `Default` in
/// `📡️spr/🎮️command/🦀️.rs`.
impl Default for JpgInference {
    fn default() -> Self {
        <Self as protocol::Inference<JpgSnapshot>>::infer(&JpgSnapshot::default())
    }
}

impl protocol::InferenceSpec<JpgSnapshot> for JpgInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.jpg.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.jpg.inference.dimensions", reads: &["width", "height", "frame"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here (a canonical-field read is already O(1)) — the default
/// `infer_cached` passthrough (`ArtifactInferrer::infer_cached`) is exact.
impl semio_framework_plugin::ArtifactInferrer for crate::artifacts::jpg::standards::v_jfif_1_01::subsets::document::schema::JpgBuilder {
    type Snapshot = JpgSnapshot;
    type Inference = JpgInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.jpg.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `jpg_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn jpg_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.jpg.inference",
        inference: schema::FacetLeaves {
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
mod tests {
    use super::*;
    use protocol::Inference;

    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = JpgSnapshot::default();
        assert_eq!(JpgInference::infer(&snapshot), JpgInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(JpgInference::infer(&JpgSnapshot::default()), JpgInference::default());
    }
}
//#endregion 🧪️Tests
