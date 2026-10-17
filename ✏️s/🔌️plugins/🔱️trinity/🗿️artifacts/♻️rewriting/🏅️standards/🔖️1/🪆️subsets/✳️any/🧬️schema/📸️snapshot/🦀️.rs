//! 🧬️ Rewriting snapshot schema — artifact-lane fields only.

use ::semio_framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted rewrite-rule document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetainedClone, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[dsl(extension = "rewriting", layout = "lines")]
#[artifact_schema(id = "s.trinity.rewriting")]
pub struct RewritingSnapshot {
    #[state(artifact)]
    #[child(kind="s.stdio.semio")]
    pub working_graph: semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::snapshot::JackSnapshot,
    #[state(artifact)]
    pub lhs: crate::standards::v1::subsets::any::schema::rule::Lhs,
    #[state(artifact)]
    pub rhs: crate::standards::v1::subsets::any::schema::rule::Rhs,
    #[state(artifact)]
    #[value(default)]
    pub parameter_bindings: semio_framework_graph_manifest::PropertyBag,
    #[state(artifact)]
    #[value(default)]
    pub rule_layout: crate::standards::v1::subsets::any::schema::layout::RuleLayout,
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
pub use semio_framework_graph_manifest::PropertyValue;
//#endregion 🔁️Re-exports



pub use semio_s_artifact_trinity_rewriting_model::standards::v1::subsets::any::schema::snapshot::json;

//#region 🧬️ChildRestoreProjection
/// 🧬️ The loaded-parent child projection, read off the snapshot's own derived composition fields — the ONE
/// definition the editor and the viewer both declare. The `ArtifactEditor`/`ArtifactViewer` trait default
/// refuses it (`… did not declare a loaded-parent child projection`), and since PX1 the live envelope load
/// asks for it before the decoded document may replace the store, so an undeclared app fails every live load.
pub fn rewriting_child_restore_projection(snapshot: &RewritingSnapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, semio_framework_plugin::Fault> {
    semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("trinity.rewriting.child-projection"), error.to_string()))
}
//#endregion 🧬️ChildRestoreProjection

semio_framework_value::artifact_retire_struct!(RewritingSnapshot { working_graph, lhs, rhs, parameter_bindings, rule_layout });


