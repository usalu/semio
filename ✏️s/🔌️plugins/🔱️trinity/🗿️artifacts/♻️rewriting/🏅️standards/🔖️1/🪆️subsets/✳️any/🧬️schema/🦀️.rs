//! 🧬️ Rewriting artifact schema — every field of the artifact with its state class.

use ::semio_framework_schema::ArtifactSchema;
pub use semio_s_artifact_trinity_rewriting_layout::standards::v1::subsets::any::schema::layout;
use layout::RuleLayout;

//#region 🔖️Artifact
/// 🧬️ Rewriting document state owned by the artifact.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.trinity.rewriting")]
pub struct RewritingArtifact {
    #[state(artifact)]
    #[child(kind="s.stdio.semio")]
    pub working_graph: semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::snapshot::JackSnapshot,
    #[state(artifact)]
    pub lhs: Lhs,
    #[state(artifact)]
    pub rhs: Rhs,
    #[state(artifact)]
    pub parameter_bindings: semio_framework_graph_manifest::PropertyBag,
    #[state(artifact)]
    pub rule_layout: RuleLayout,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for RewritingArtifact {
    fn default() -> Self {
        Self { working_graph: Default::default(), lhs: Default::default(), rhs: Default::default(), parameter_bindings: Default::default(), rule_layout: Default::default() }
    }
}

impl RewritingArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot {
        crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot { working_graph: self.working_graph.clone(), lhs: self.lhs.clone(), rhs: self.rhs.clone(), parameter_bindings: self.parameter_bindings.clone(), rule_layout: self.rule_layout.clone() }
    }

    /// 🧬️ Builds the artifact from its document snapshot.
    pub fn from_snapshot(snapshot: crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot) -> Self {
        Self { working_graph: snapshot.working_graph, lhs: snapshot.lhs, rhs: snapshot.rhs, parameter_bindings: snapshot.parameter_bindings, rule_layout: snapshot.rule_layout }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot) {
        self.working_graph = snapshot.working_graph;
        self.lhs = snapshot.lhs;
        self.rhs = snapshot.rhs;
        self.parameter_bindings = snapshot.parameter_bindings;
        self.rule_layout = snapshot.rule_layout;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.trinity.rewriting` — twenty handcrafted schema leaves.
pub fn rewriting_artifact_schema_descriptor() -> ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
    ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.trinity.rewriting",
        artifact: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto")
        },
        snapshot: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

//#region 🔖️RuleApplication
pub use semio_s_artifact_trinity_rewriting_model::standards::v1::subsets::any::schema::rule;
use rule::{Lhs,Rhs};

//#endregion 🔖️RuleApplication

//#region 📐️RuleGraphSlots
/// 📐️ The LHS window's semantic nodes with their default positions: the match, then the WHERE clause when the rule has one.
pub fn lhs_graph_slots(lhs: &Lhs) -> Vec<(String, LayoutPoint)> {
    let mut slots = vec![("lhs-match".to_string(), LayoutPoint { x: 0.0, y: 0.0 })];
    if lhs.where_clause.as_deref().is_some_and(|clause| !clause.trim().is_empty()) {
        slots.push(("lhs-where".to_string(), LayoutPoint { x: 220.0, y: 80.0 }));
    }
    slots
}

/// 📐️ The RHS window's semantic nodes with their default positions: one row per clause list (create, merge, set, delete,
/// parameter), 220 units between the clauses of a row and 80 between rows; `rhs-empty` stands for a side without clauses.
pub fn rhs_graph_slots(rhs: &Rhs) -> Vec<(String, LayoutPoint)> {
    let rows = [("rhs-create", rhs.create.len()), ("rhs-merge", rhs.merge.len()), ("rhs-set", rhs.set.len()), ("rhs-delete", rhs.delete.len()), ("rhs-parameter", rhs.parameters.len())];
    let slots: Vec<(String, LayoutPoint)> =
        rows.into_iter().enumerate().flat_map(|(row, (prefix, count))| (0..count).map(move |index| (format!("{prefix}-{index}"), LayoutPoint { x: index as f64 * 220.0, y: row as f64 * 80.0 }))).collect();
    match slots.is_empty() {
        true => vec![("rhs-empty".to_string(), LayoutPoint { x: 0.0, y: 0.0 })],
        false => slots,
    }
}

/// 📍️ Where the semantic rule-graph node `id` sits on `snapshot`: its `rule_layout` point, else its default slot; `None` for
/// an id the rule draws no node for, or a side that does not decode.
pub fn rule_graph_position(snapshot: &crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot, id: &str) -> Option<LayoutPoint> {
    let lhs = lhs_graph_slots(&snapshot.lhs);
    let rhs = rhs_graph_slots(&snapshot.rhs);
    let default = lhs.into_iter().chain(rhs).find(|(slot, _)| slot == id).map(|(_, point)| point)?;
    Some(snapshot.rule_layout.get(id).copied().unwrap_or(default))
}
//#endregion 📐️RuleGraphSlots

//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🔁️Re-exports
/// 🔁️ The model record the rule graph slots are made of, imported from its owner.
use crate::standards::v1::subsets::any::schema::model::LayoutPoint;
//#endregion 🔁️Re-exports
