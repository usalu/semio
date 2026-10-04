//! 💡️ SemioVideoInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `⏱️duration/` — same shape
//! `animation`'s/`audio`'s own duration facets establish: the container's real elapsed time,
//! derived from each stream's own `pts`/`rate`, never the opaque sample payload this subset's own
//! module doc comment names as W3/W4's job, not this snapshot's).

use crate::standards::v1::subsets::video::schema::snapshot::SemioVideoSnapshot;
use framework_schema::ArtifactSchema;

use super::duration::compute_semio_video_duration;
//#region 🔖️Inference
/// 💡️ Everything inferable from a semio video snapshot. One field per named inference under
/// `💡️inferences/` (currently: `duration`, backed by the `⏱️duration/` slug dir).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.video.inference")]
pub struct SemioVideoInference {
    #[derived]
    pub duration: SemioVideoDuration,
}

impl protocol::Inference<SemioVideoSnapshot> for SemioVideoInference {
    fn infer(snapshot: &SemioVideoSnapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { duration: compute_semio_video_duration(snapshot) }
    
        })
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — `SemioVideoSnapshot::default()` happens to be
/// all-empty today (no streams), so a naive derive would happen to agree, but tying `Default` to
/// `infer` keeps the law correct even if that default ever stops being all-empty (the same
/// defensive pattern raster's `RasterInference` documents).
impl Default for SemioVideoInference {
    fn default() -> Self {
        let snapshot = &SemioVideoSnapshot::default();

        Self { duration: compute_semio_video_duration(snapshot) }
    }
}

impl protocol::InferenceSpec<SemioVideoSnapshot> for SemioVideoInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.semio.video.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.semio.video.inference.duration", reads: &["streams"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.semio.video.inference`'s facet leaves into the OS-wide inference catalog
/// — call once at plugin init, alongside `semio_video_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_video_artifact_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.stdio.semio.video.inference",
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
pub use super::duration::SemioVideoDuration;
//#endregion 🔁️Re-exports
