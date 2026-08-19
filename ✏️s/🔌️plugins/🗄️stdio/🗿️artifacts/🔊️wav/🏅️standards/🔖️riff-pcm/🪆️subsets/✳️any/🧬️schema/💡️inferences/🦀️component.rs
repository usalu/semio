//! 💡️ WavInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `⏱duration/`, derived from the real
//! `fmt ` chunk's `sampleRate`/`channels` plus the real decoded `data` sample count).

use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::snapshot::WavSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::duration::{compute_wav_duration, WavDuration};

//#region 🔖️Inference
/// 💡️ Everything inferable from a wav snapshot. One field per named inference under
/// `💡️inferences/` (currently: `duration`, backed by the `⏱duration/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.wav.inference")]
pub struct WavInference {
    #[derived]
    pub duration: WavDuration,
}

impl protocol::Inference<WavSnapshot> for WavInference {
    async fn infer(snapshot: &WavSnapshot) -> Self {
        Self { duration: compute_wav_duration(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `WavSnapshot::default()`'s `fmt`/`data` ever stop matching this shape (its own hand-rolled
/// `Default` already picks a real 44.1kHz mono PCM16 form, not a zeroed struct).
impl Default for WavInference {
    fn default() -> Self {
        <Self as protocol::Inference<WavSnapshot>>::infer(&WavSnapshot::default())
    }
}

impl protocol::InferenceSpec<WavSnapshot> for WavInference {
    async fn inference_schema_id() -> &'static str {
        "s.stdio.wav.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.wav.inference.duration", reads: &["fmt", "data"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here — `duration` is a single sample-count fold over `data`, already
/// O(n) in sample count with no honest per-entity incremental decomposition (a merkle dep-chain
/// over one flat sample buffer costs more than the fold it would cache) — the default
/// `infer_cached` passthrough is exact.
impl ArtifactInferrer for crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::WavBuilder {
    type Snapshot = WavSnapshot;
    type Inference = WavInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.wav.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `wav_artifact_schema_descriptor`'s registration.
pub async fn wav_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.wav.inference",
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
        let snapshot = WavSnapshot::default();
        assert_eq!(WavInference::infer(&snapshot), WavInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(WavInference::infer(&WavSnapshot::default()), WavInference::default());
    }
}
//#endregion 🧪️Tests
