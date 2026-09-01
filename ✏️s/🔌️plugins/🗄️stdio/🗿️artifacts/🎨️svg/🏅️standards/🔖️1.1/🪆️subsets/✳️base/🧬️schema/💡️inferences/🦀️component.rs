//! 💡️ Svg inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📐dimensions/`).

use crate::artifacts::svg::SvgSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

use super::dimensions::{compute_svg_dimensions, SvgDimensions};

//#region 🔖️Inference
/// 💡️ Everything inferable from an svg snapshot. One field per named inference under
/// `💡️inferences/` (currently: `dimensions`, backed by the `📐dimensions/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.svg.inference")]
pub struct SvgInference {
    #[derived]
    pub dimensions: SvgDimensions,
}

impl protocol::Inference<SvgSnapshot> for SvgInference {
    fn infer(snapshot: &SvgSnapshot) -> Self {
        Self { dimensions: compute_svg_dimensions(snapshot) }
    }
}

/// 🌱 Hand-fixed to agree with `infer(&SvgSnapshot::default())` rather than a naive
/// `#[derive(Default)]` — same "match `infer` of the real default, don't derive structurally"
/// trick as `AddInference`'s hand-written `Default` in `📡️spr/🎮️command/🦀️component.rs`.
impl Default for SvgInference {
    fn default() -> Self {
        <Self as protocol::Inference<SvgSnapshot>>::infer(&SvgSnapshot::default())
    }
}

impl protocol::InferenceSpec<SvgSnapshot> for SvgInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.svg.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.svg.inference.dimensions", reads: &["doc"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here (a root-element read is already O(1), nothing to incrementally
/// cache) — the default `infer_cached` passthrough (`ArtifactInferrer::infer_cached`) is exact.
impl semio_framework_plugin::ArtifactInferrer for crate::artifacts::svg::standards::v1_1::subsets::base::schema::SvgBuilder {
    type Snapshot = SvgSnapshot;
    type Inference = SvgInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.svg.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `svg_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn svg_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.svg.inference",
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
        let snapshot = SvgSnapshot::default();
        assert_eq!(SvgInference::infer(&snapshot), SvgInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(SvgInference::infer(&SvgSnapshot::default()), SvgInference::default());
    }
}
//#endregion 🧪️Tests
