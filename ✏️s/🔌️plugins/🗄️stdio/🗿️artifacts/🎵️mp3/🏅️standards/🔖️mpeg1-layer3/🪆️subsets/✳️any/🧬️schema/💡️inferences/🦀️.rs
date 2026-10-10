//! 💡️ Mp3Inference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `⏱️duration/`, derived from the real
//! MPEG-1/2/2.5 Layer III frame header fields — bitrate/sample-rate table lookups, not a guess).

use crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::Mp3Snapshot;
use framework_schema::ArtifactSchema;

use super::duration::compute_mp3_duration;
//#region 🔖️Inference
/// 💡️ Everything inferable from an mp3 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `duration`, backed by the `⏱️duration/` slug dir).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.mp3.inference")]
pub struct Mp3Inference {
    #[derived]
    pub duration: Mp3Duration,
}

impl protocol::Inference<Mp3Snapshot> for Mp3Inference {
    fn infer(snapshot: &Mp3Snapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { duration: compute_mp3_duration(snapshot) }
    
        })
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `Mp3Snapshot::default()`'s `frames` ever stop being empty.
impl Default for Mp3Inference {
    fn default() -> Self {
        let snapshot = &Mp3Snapshot::default();

        Self { duration: compute_mp3_duration(snapshot) }
    }
}

impl protocol::InferenceSpec<Mp3Snapshot> for Mp3Inference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.mp3.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.mp3.inference.duration", reads: &["frames"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.mp3.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `mp3_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mp3_artifact_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.stdio.mp3.inference",
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
pub use super::duration::Mp3Duration;
//#endregion 🔁️Re-exports
