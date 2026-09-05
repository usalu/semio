//! 💡️ SemioVideoInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `⏱️duration/` — same shape
//! `animation`'s/`audio`'s own duration facets establish: the container's real elapsed time,
//! derived from each stream's own `pts`/`rate`, never the opaque sample payload this subset's own
//! module doc comment names as W3/W4's job, not this snapshot's).

use crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::SemioVideoSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

use super::duration::{compute_semio_video_duration, SemioVideoDuration};

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
    fn infer(snapshot: &SemioVideoSnapshot) -> Self {
        Self { duration: compute_semio_video_duration(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — `SemioVideoSnapshot::default()` happens to be
/// all-empty today (no streams), so a naive derive would happen to agree, but tying `Default` to
/// `infer` keeps the law correct even if that default ever stops being all-empty (the same
/// defensive pattern raster's `RasterInference` documents).
impl Default for SemioVideoInference {
    fn default() -> Self {
        <Self as protocol::Inference<SemioVideoSnapshot>>::infer(&SemioVideoSnapshot::default())
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

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here (a per-stream max-pts fold across already-flat `streams` is a
/// single whole-snapshot pass) — the default `infer_cached` passthrough
/// (`ArtifactInferrer::infer_cached`) is exact.
impl ArtifactInferrer for crate::artifacts::semio::standards::v1::subsets::video::schema::SemioVideoBuilder {
    type Snapshot = SemioVideoSnapshot;
    type Inference = SemioVideoInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.semio.video.inference`'s facet leaves into the OS-wide inference catalog
/// — call once at plugin init, alongside `semio_video_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_video_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.semio.video.inference",
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
        let snapshot = SemioVideoSnapshot::default();
        assert_eq!(SemioVideoInference::infer(&snapshot), SemioVideoInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(SemioVideoInference::infer(&SemioVideoSnapshot::default()), SemioVideoInference::default());
    }
}
//#endregion 🧪️Tests
