//! 💡️ Mp4Inference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `⏱️duration/`, derived from every
//! track's real ISO-BMFF `stts`-flattened per-sample `duration`/`timescale` pair).

use crate::standards::isobmff::subsets::any::schema::snapshot::Mp4Snapshot;
use framework_schema::ArtifactSchema;

use super::duration::compute_mp4_duration;
//#region 🔖️Inference
/// 💡️ Everything inferable from an mp4 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `duration`, backed by the `⏱️duration/` slug dir).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.mp4.inference")]
pub struct Mp4Inference {
    #[derived]
    pub duration: Mp4Duration,
}

impl protocol::Inference<Mp4Snapshot> for Mp4Inference {
    fn infer(snapshot: &Mp4Snapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { duration: compute_mp4_duration(snapshot) }
    
        })
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `Mp4Snapshot::default()`'s `tracks` ever stop being empty (its own hand-rolled `Default`
/// already picks a real minimal `ftyp`, not a zeroed struct).
impl Default for Mp4Inference {
    fn default() -> Self {
        let snapshot = &Mp4Snapshot::default();

        Self { duration: compute_mp4_duration(snapshot) }
    }
}

impl protocol::InferenceSpec<Mp4Snapshot> for Mp4Inference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.mp4.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.mp4.inference.duration", reads: &["tracks"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.mp4.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `mp4_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mp4_artifact_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.stdio.mp4.inference",
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
pub use super::duration::Mp4Duration;
//#endregion 🔁️Re-exports
