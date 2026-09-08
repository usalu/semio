//! 💡️ AviInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `⏱️duration/`, derived from the real
//! `avih` MainAVIHeader's `dwTotalFrames`/`dwMicroSecPerFrame` fields).

use crate::standards::v1_0::subsets::any::schema::snapshot::AviSnapshot;
use framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

use super::duration::{compute_avi_duration, AviDuration};

//#region 🔖️Inference
/// 💡️ Everything inferable from an avi snapshot. One field per named inference under
/// `💡️inferences/` (currently: `duration`, backed by the `⏱️duration/` slug dir).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.avi.inference")]
pub struct AviInference {
    #[derived]
    pub duration: AviDuration,
}

impl protocol::Inference<AviSnapshot> for AviInference {
    fn infer(snapshot: &AviSnapshot) -> Self {
        Self { duration: compute_avi_duration(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — `AviSnapshot` itself derives `Default` (all
/// header fields zero, `streams` empty), so this is belt-and-suspenders rather than a strict
/// necessity here, but keeps the same hand-rolled convention every sibling family uses (safer
/// against a future `AviMainHeader` default that stops being all-zero).
impl Default for AviInference {
    fn default() -> Self {
        <Self as protocol::Inference<AviSnapshot>>::infer(&AviSnapshot::default())
    }
}

impl protocol::InferenceSpec<AviSnapshot> for AviInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.avi.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.avi.inference.duration", reads: &["mainHeader", "streams"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here — `duration` is a fixed-field read off `main_header` plus a single
/// `streams.len()` count, already O(1)/O(streams) with no honest per-entity incremental
/// decomposition worth a merkle dep-chain — the default `infer_cached` passthrough is exact.
impl ArtifactInferrer for crate::standards::v1_0::subsets::any::schema::AviBuilder {
    type Snapshot = AviSnapshot;
    type Inference = AviInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.avi.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `avi_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn avi_artifact_inference_descriptor() -> framework_schema::ArtifactInferenceDescriptor {
    framework_schema::ArtifactInferenceDescriptor {
        id: "s.stdio.avi.inference",
        inference: framework_schema::FacetLeaves {
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
