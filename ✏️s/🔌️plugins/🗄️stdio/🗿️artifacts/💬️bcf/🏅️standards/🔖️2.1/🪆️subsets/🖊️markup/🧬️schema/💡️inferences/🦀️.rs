//! 💡️ BcfInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🗒️topicstats/`, honestly derivable
//! from `topics` alone — BCF is an issue-tracking format, not geometry, so the closest honest
//! derived statistic is a count/fold over topics/comments/viewpoints/authors, not a bounding box).

use crate::standards::v2_1::subsets::any::schema::snapshot::BcfSnapshot;
use framework_schema::ArtifactSchema;

use super::topicstats::compute_bcf_topic_stats;
//#region 🔖️Inference
/// 💡️ Everything inferable from a bcf snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topicStats`, backed by the `🗒️topicstats/` slug dir).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.bcf.inference")]
pub struct BcfInference {
    #[derived]
    pub topic_stats: BcfTopicStats,
}

impl protocol::Inference<BcfSnapshot> for BcfInference {
    fn infer(snapshot: &BcfSnapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { topic_stats: compute_bcf_topic_stats(snapshot) }
    
        })
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `BcfSnapshot::default()`'s `topics` ever stops being empty.
impl Default for BcfInference {
    fn default() -> Self {
        let snapshot = &BcfSnapshot::default();

        Self { topic_stats: compute_bcf_topic_stats(snapshot) }
    }
}

impl protocol::InferenceSpec<BcfSnapshot> for BcfInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.bcf.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.bcf.inference.topicStats", reads: &["topics"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.bcf.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `bcf_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn bcf_artifact_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.stdio.bcf.inference",
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
pub use super::topicstats::BcfTopicStats;
//#endregion 🔁️Re-exports
