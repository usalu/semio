//! 🧬️ Rewriting snapshot schema — artifact-lane fields only.

use crate::LayoutPoint;
use ::semio_framework_schema::ArtifactSchema;
use std::collections::BTreeMap;

//#region 🔖️Snapshot
/// 📸️ Persisted rewrite-rule document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[dsl(extension = "rewriting", layout = "lines")]
#[artifact_schema(id = "s.trinity.rewriting")]
pub struct RewritingSnapshot {
    #[state(artifact)]
    #[child(kind="s.stdio.semio")]
    pub working_graph: semio_s_artifact_trinity_jack::JackSnapshot,
    #[state(artifact)]
    pub lhs: crate::standards::v1::subsets::any::schema::Lhs,
    #[state(artifact)]
    pub rhs: crate::standards::v1::subsets::any::schema::Rhs,
    #[state(artifact)]
    #[value(default)]
    pub parameter_bindings: semio_framework_graph::manifest::PropertyBag,
    #[state(artifact)]
    #[value(default)]
    pub rule_layout: crate::standards::v1::subsets::any::schema::RuleLayout,
}
//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs
//#endregion 🔖️Snapshot

//#region 🔣️DeclaredJsonCodec








/// 🔎️ Lists typed rule cardinalities, retained child identity and literal sorted map keys.
pub fn rewrite_rule_summary(snapshot: &RewritingSnapshot) -> String {
    format!("bindings[{}] layout[{}] graph={} lhs={}:{} rhs={}/{}/{}/{}/{}",snapshot.parameter_bindings.keys().cloned().collect::<Vec<_>>().join(" "),snapshot.rule_layout.keys().cloned().collect::<Vec<_>>().join(" "),snapshot.working_graph.content.child_id,snapshot.lhs.pattern.left_var,snapshot.lhs.pattern.left_kind,snapshot.rhs.create.len(),snapshot.rhs.delete.len(),snapshot.rhs.set.len(),snapshot.rhs.merge.len(),snapshot.rhs.parameters.len())
}
//#endregion 🌉️ExternalCodecBridge

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use semio_framework_graph::manifest::PropertyValue;
//#endregion 🔁️Re-exports



#[path="🔣️json/🦀️.rs"]
pub mod json;

