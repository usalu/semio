//! 💡️ SemioAnimationInference — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `⏱️duration/`, honestly derivable
//! from `timelines` alone — its own nested `channels`/`keyframes`).

use crate::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot;
use framework_schema::ArtifactSchema;

use super::duration::compute_semio_animation_duration;
//#region 🔖️Inference
/// 💡️ Everything inferable from a semio animation snapshot. One field per named inference under
/// `💡️inferences/` (currently: `duration`, backed by the `⏱️duration/` slug dir).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.animation.inference")]
pub struct SemioAnimationInference {
    #[derived]
    pub duration: SemioAnimationDuration,
}

impl protocol::Inference<SemioAnimationSnapshot> for SemioAnimationInference {
    fn infer(snapshot: &SemioAnimationSnapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { duration: compute_semio_animation_duration(snapshot) }
    
        })
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `SemioAnimationSnapshot::default()`'s `timelines` ever stops being empty.
impl Default for SemioAnimationInference {
    fn default() -> Self {
        let snapshot = &SemioAnimationSnapshot::default();

        Self { duration: compute_semio_animation_duration(snapshot) }
    }
}

impl protocol::InferenceSpec<SemioAnimationSnapshot> for SemioAnimationInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.semio.animation.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.semio.animation.inference.duration", reads: &["timelines"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.semio.animation.inference`'s facet leaves into the OS-wide inference
/// catalog — call once at plugin init, alongside `semio_animation_artifact_schema_descriptor`'s
/// registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_animation_artifact_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.stdio.semio.animation.inference",
        inference: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
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
pub use super::duration::SemioAnimationDuration;
//#endregion 🔁️Re-exports
