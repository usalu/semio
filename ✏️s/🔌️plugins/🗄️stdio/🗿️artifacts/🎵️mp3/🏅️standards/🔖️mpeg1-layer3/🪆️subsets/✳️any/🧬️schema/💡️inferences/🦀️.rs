//! 💡️ Mp3Inference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `⏱️duration/`, derived from the real
//! MPEG-1/2/2.5 Layer III frame header fields — bitrate/sample-rate table lookups, not a guess).

use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::snapshot::Mp3Snapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

use super::duration::{compute_mp3_duration, Mp3Duration};

//#region 🔖️Inference
/// 💡️ Everything inferable from an mp3 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `duration`, backed by the `⏱️duration/` slug dir).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.mp3.inference")]
pub struct Mp3Inference {
    #[derived]
    pub duration: Mp3Duration,
}

impl protocol::Inference<Mp3Snapshot> for Mp3Inference {
    fn infer(snapshot: &Mp3Snapshot) -> Self {
        Self { duration: compute_mp3_duration(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `Mp3Snapshot::default()`'s `frames` ever stop being empty.
impl Default for Mp3Inference {
    fn default() -> Self {
        <Self as protocol::Inference<Mp3Snapshot>>::infer(&Mp3Snapshot::default())
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

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here — `duration` is a single per-frame fold over `frames`, already
/// O(n) in frame count with no honest per-entity incremental decomposition (a merkle dep-chain
/// over one flat `Vec<Mp3Frame>` costs more than the fold it would cache) — the default
/// `infer_cached` passthrough is exact.
impl ArtifactInferrer for crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::Mp3Builder {
    type Snapshot = Mp3Snapshot;
    type Inference = Mp3Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.mp3.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `mp3_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mp3_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.mp3.inference",
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
        let snapshot = Mp3Snapshot::default();
        assert_eq!(Mp3Inference::infer(&snapshot), Mp3Inference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(Mp3Inference::infer(&Mp3Snapshot::default()), Mp3Inference::default());
    }
}
//#endregion 🧪️Tests
