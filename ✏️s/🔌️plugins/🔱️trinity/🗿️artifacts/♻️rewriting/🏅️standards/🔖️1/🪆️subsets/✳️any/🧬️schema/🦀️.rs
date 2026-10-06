//! 🧬️ Rewriting artifact schema — every field of the artifact with its state class.

use crate::TrinityRewritingError;
use ::semio_framework_schema::ArtifactSchema;
use semio_s_artifact_trinity_jack::ast::{Pattern as JackPattern, PatternEdge, PatternNode, QueryResult};
use semio_s_artifact_trinity_jack::executor::execute;
use semio_s_artifact_trinity_jack::language_service::parse;
use semio_s_artifact_trinity_jack::Graph;
use std::collections::BTreeMap;
#[path="🌳️typed/🗂️layout/🦀️.rs"]
pub mod layout;
pub use layout::RuleLayout;

#[path = "♻️retirement/🦀️.rs"]
pub mod retirement;

//#region 🔖️Artifact
/// 🧬️ Rewriting document state owned by the artifact.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.trinity.rewriting")]
pub struct RewritingArtifact {
    #[state(artifact)]
    #[child(kind="s.stdio.semio")]
    pub working_graph: semio_s_artifact_trinity_jack::JackSnapshot,
    #[state(artifact)]
    pub lhs: crate::standards::v1::subsets::any::schema::Lhs,
    #[state(artifact)]
    pub rhs: crate::standards::v1::subsets::any::schema::Rhs,
    #[state(artifact)]
    pub parameter_bindings: semio_framework_graph::manifest::PropertyBag,
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
    pub fn to_snapshot(&self) -> crate::RewritingSnapshot {
        crate::RewritingSnapshot { working_graph: self.working_graph.clone(), lhs: self.lhs.clone(), rhs: self.rhs.clone(), parameter_bindings: self.parameter_bindings.clone(), rule_layout: self.rule_layout.clone() }
    }

    /// 🧬️ Builds the artifact from its document snapshot.
    pub fn from_snapshot(snapshot: crate::RewritingSnapshot) -> Self {
        Self { working_graph: snapshot.working_graph, lhs: snapshot.lhs, rhs: snapshot.rhs, parameter_bindings: snapshot.parameter_bindings, rule_layout: snapshot.rule_layout }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::RewritingSnapshot) {
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
#[path="🌳️typed/📜️rule/🦀️.rs"]
pub mod rule;
pub use rule::{Pattern,Lhs,Rhs,Assignment,ParameterKind,ParameterSpec,Rule};

impl Pattern {
    fn to_jack_pattern(&self) -> JackPattern {
        let left = PatternNode { var: self.left_var.clone(), kind: self.left_kind.clone() };
        if let (Some(right_var), Some(right_kind)) = (&self.right_var, &self.right_kind) {
            JackPattern { nodes: vec![left], edge: Some(PatternEdge { var: self.edge_var.clone(), kind: self.edge_kind.clone(), directed: true, right: PatternNode { var: right_var.clone(), kind: right_kind.clone() } }) }
        } else {
            JackPattern { nodes: vec![left], edge: None }
        }
    }
}















/// ♻️ Apply a rewrite rule to a graph.
pub fn apply_rule(graph: &mut Graph, rule: &Rule, bindings: &semio_framework_graph::manifest::PropertyBag) -> Result<QueryResult, TrinityRewritingError> {
    let query = build_rule_query(rule, bindings);
    let parsed = parse(&query).map_err(TrinityRewritingError::Jack)?;
    let (result, effects) = execute(graph, &parsed).map_err(TrinityRewritingError::Jack)?;
    semio_s_artifact_trinity_jack::apply_graph_effects(graph, &effects)?;
    Ok(result)
}





#[derive(value_derive::ToValue)]
#[value(rename_all = "camelCase")]
pub struct ApplyRuleResult {
    pub snapshot_json: String,
    pub query: QueryResult,
}

#[derive(value_derive::ToValue)]
#[value(rename_all = "camelCase")]
pub struct RuleQueryResult {
    pub query: String,
}
//#endregion 🔖️RuleApplication

//#region 📐️RuleGraphSlots
/// 📐️ The LHS window's semantic nodes with their default positions: the match, then the WHERE clause when the rule has one.
pub fn lhs_graph_slots(lhs: &Lhs) -> Vec<(String, crate::LayoutPoint)> {
    let mut slots = vec![("lhs-match".to_string(), crate::LayoutPoint { x: 0.0, y: 0.0 })];
    if lhs.where_clause.as_deref().is_some_and(|clause| !clause.trim().is_empty()) {
        slots.push(("lhs-where".to_string(), crate::LayoutPoint { x: 220.0, y: 80.0 }));
    }
    slots
}

/// 📐️ The RHS window's semantic nodes with their default positions: one row per clause list (create, merge, set, delete,
/// parameter), 220 units between the clauses of a row and 80 between rows; `rhs-empty` stands for a side without clauses.
pub fn rhs_graph_slots(rhs: &Rhs) -> Vec<(String, crate::LayoutPoint)> {
    let rows = [("rhs-create", rhs.create.len()), ("rhs-merge", rhs.merge.len()), ("rhs-set", rhs.set.len()), ("rhs-delete", rhs.delete.len()), ("rhs-parameter", rhs.parameters.len())];
    let slots: Vec<(String, crate::LayoutPoint)> =
        rows.into_iter().enumerate().flat_map(|(row, (prefix, count))| (0..count).map(move |index| (format!("{prefix}-{index}"), crate::LayoutPoint { x: index as f64 * 220.0, y: row as f64 * 80.0 }))).collect();
    match slots.is_empty() {
        true => vec![("rhs-empty".to_string(), crate::LayoutPoint { x: 0.0, y: 0.0 })],
        false => slots,
    }
}

/// 📍️ Where the semantic rule-graph node `id` sits on `snapshot`: its `rule_layout` point, else its default slot; `None` for
/// an id the rule draws no node for, or a side that does not decode.
pub fn rule_graph_position(snapshot: &crate::RewritingSnapshot, id: &str) -> Option<crate::LayoutPoint> {
    let lhs = lhs_graph_slots(&snapshot.lhs);
    let rhs = rhs_graph_slots(&snapshot.rhs);
    let default = lhs.into_iter().chain(rhs).find(|(slot, _)| slot == id).map(|(_, point)| point)?;
    Some(snapshot.rule_layout.get(id).copied().unwrap_or(default))
}
//#endregion 📐️RuleGraphSlots

//#region 🧪️RuleApplicationTests
#[cfg(test)]
#[path = "🧪️tests/🔬️rule-application/🦀️.rs"]
mod rule_application_tests;
//#endregion 🧪️RuleApplicationTests

//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🔁️Re-exports
pub use crate::LayoutPoint;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use semio_framework_graph::manifest::PropertyValue;
pub use semio_s_artifact_trinity_jack::Camera;
//#endregion 🔁️Re-exports
