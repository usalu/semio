//! 💡️ SemioAudioInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `⏱️duration/`, honestly derivable
//! from `sampleRate`/`channels` alone).

use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::SemioAudioSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

use super::duration::{compute_semio_audio_duration, SemioAudioDuration};

//#region 🔖️Inference
/// 💡️ Everything inferable from a semio audio snapshot. One field per named inference under
/// `💡️inferences/` (currently: `duration`, backed by the `⏱️duration/` slug dir).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.audio.inference")]
pub struct SemioAudioInference {
    #[derived]
    pub duration: SemioAudioDuration,
}

impl protocol::Inference<SemioAudioSnapshot> for SemioAudioInference {
    fn infer(snapshot: &SemioAudioSnapshot) -> Self {
        Self { duration: compute_semio_audio_duration(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `SemioAudioSnapshot::default()`'s `channels`/`sampleRate` ever stop being empty/zero.
impl Default for SemioAudioInference {
    fn default() -> Self {
        <Self as protocol::Inference<SemioAudioSnapshot>>::infer(&SemioAudioSnapshot::default())
    }
}

impl protocol::InferenceSpec<SemioAudioSnapshot> for SemioAudioInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.semio.audio.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.semio.audio.inference.duration", reads: &["sampleRate", "channels"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here — `duration` is a single max-length fold over `channels`, already
/// O(n) in total sample count with no honest per-entity incremental decomposition (a merkle
/// dep-chain over one flat `Vec<SemioAudioChannel>` costs more than the fold it would cache) — the
/// default `infer_cached` passthrough is exact.
impl ArtifactInferrer for crate::artifacts::semio::standards::v1::subsets::audio::schema::SemioAudioBuilder {
    type Snapshot = SemioAudioSnapshot;
    type Inference = SemioAudioInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.semio.audio.inference`'s facet leaves into the OS-wide inference catalog
/// — call once at plugin init, alongside `semio_audio_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_audio_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.semio.audio.inference",
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
        let snapshot = SemioAudioSnapshot::default();
        assert_eq!(SemioAudioInference::infer(&snapshot), SemioAudioInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(SemioAudioInference::infer(&SemioAudioSnapshot::default()), SemioAudioInference::default());
    }
}
//#endregion 🧪️Tests
