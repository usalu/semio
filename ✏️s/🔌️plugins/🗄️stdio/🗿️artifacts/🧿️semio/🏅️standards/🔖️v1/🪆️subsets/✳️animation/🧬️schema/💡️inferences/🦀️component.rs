//! 💡️ SemioAnimationInference — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `⏱duration/`, honestly derivable
//! from `timelines` alone — its own nested `channels`/`keyframes`).

use crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::duration::{compute_semio_animation_duration, SemioAnimationDuration};

//#region 🔖️Inference
/// 💡️ Everything inferable from a semio animation snapshot. One field per named inference under
/// `💡️inferences/` (currently: `duration`, backed by the `⏱duration/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.animation.inference")]
pub struct SemioAnimationInference {
    #[derived]
    pub duration: SemioAnimationDuration,
}

impl protocol::Inference<SemioAnimationSnapshot> for SemioAnimationInference {
    async fn infer(snapshot: &SemioAnimationSnapshot) -> Self {
        Self { duration: compute_semio_animation_duration(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `SemioAnimationSnapshot::default()`'s `timelines` ever stops being empty.
impl Default for SemioAnimationInference {
    fn default() -> Self {
        <Self as protocol::Inference<SemioAnimationSnapshot>>::infer(&SemioAnimationSnapshot::default())
    }
}

impl protocol::InferenceSpec<SemioAnimationSnapshot> for SemioAnimationInference {
    async fn inference_schema_id() -> &'static str {
        "s.stdio.semio.animation.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.semio.animation.inference.duration", reads: &["timelines"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here — `duration` is a single max-`t` fold over every keyframe of every
/// channel of every timeline, already O(n) in total keyframe count with no honest per-entity
/// incremental decomposition — the default `infer_cached` passthrough is exact.
impl ArtifactInferrer for crate::artifacts::semio::standards::v1::subsets::animation::schema::SemioAnimationBuilder {
    type Snapshot = SemioAnimationSnapshot;
    type Inference = SemioAnimationInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.semio.animation.inference`'s facet leaves into the OS-wide inference
/// catalog — call once at plugin init, alongside `semio_animation_artifact_schema_descriptor`'s
/// registration.
pub async fn semio_animation_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.semio.animation.inference",
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
        let snapshot = SemioAnimationSnapshot::default();
        assert_eq!(SemioAnimationInference::infer(&snapshot), SemioAnimationInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(SemioAnimationInference::infer(&SemioAnimationSnapshot::default()), SemioAnimationInference::default());
    }
}
//#endregion 🧪️Tests
