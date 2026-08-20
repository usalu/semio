//! 💡️ BcfInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🗒topicstats/`, honestly derivable
//! from `topics` alone — BCF is an issue-tracking format, not geometry, so the closest honest
//! derived statistic is a count/fold over topics/comments/viewpoints/authors, not a bounding box).

use crate::artifacts::bcf::standards::v2_1::subsets::any::schema::snapshot::BcfSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

use super::topicstats::{compute_bcf_topic_stats, BcfTopicStats};

//#region 🔖️Inference
/// 💡️ Everything inferable from a bcf snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topicStats`, backed by the `🗒topicstats/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.bcf.inference")]
pub struct BcfInference {
    #[derived]
    pub topic_stats: BcfTopicStats,
}

impl protocol::Inference<BcfSnapshot> for BcfInference {
    async fn infer(snapshot: &BcfSnapshot) -> Self {
        Self { topic_stats: compute_bcf_topic_stats(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `BcfSnapshot::default()`'s `topics` ever stops being empty.
impl Default for BcfInference {
    fn default() -> Self {
        <Self as protocol::Inference<BcfSnapshot>>::infer(&BcfSnapshot::default())
    }
}

impl protocol::InferenceSpec<BcfSnapshot> for BcfInference {
    async fn inference_schema_id() -> &'static str {
        "s.stdio.bcf.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.bcf.inference.topicStats", reads: &["topics"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here — `topicStats` is a single fold over `topics` (already O(n) in
/// total topic/comment/viewpoint count), with no honest per-entity incremental decomposition (a
/// merkle dep-chain over this flat whole-snapshot count/fold costs more than the fold it would
/// cache) — the default `infer_cached` passthrough is exact.
impl semio_framework_plugin::ArtifactInferrer for crate::artifacts::bcf::standards::v2_1::subsets::any::schema::BcfBuilder {
    type Snapshot = BcfSnapshot;
    type Inference = BcfInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.bcf.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `bcf_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn bcf_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.bcf.inference",
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
        let snapshot = BcfSnapshot::default();
        assert_eq!(BcfInference::infer(&snapshot), BcfInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(BcfInference::infer(&BcfSnapshot::default()), BcfInference::default());
    }
}
//#endregion 🧪️Tests
