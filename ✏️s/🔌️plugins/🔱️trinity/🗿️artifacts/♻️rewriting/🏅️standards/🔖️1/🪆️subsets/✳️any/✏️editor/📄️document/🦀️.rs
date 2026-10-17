//! 📄️ Rewriting editor document helpers: the default and reset document, the compiled rule query and the semantic rule graphs.

use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::content::JackWorkingScene;
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::standards::v1::subsets::any::schema::{self};
use crate::standards::v1::subsets::any::schema::rule::{ParameterKind, Rhs};
use crate::standards::v1::subsets::any::schema::model::{LayoutPoint, REWRITE_RULE_SCHEMA, TRINITY_REWRITING_DIALECT};
use crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot;
use semio_framework_graph_manifest::{PropertyBag};
use semio_framework_plugin::TopologyNode;
use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::graph::{Camera, Node};
use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::snapshot::JackSnapshot;
use crate::editor::rewriting::ids::*;
use store::pack::snapshot::ArtifactPack;
use semio_s_artifact_trinity_jack_semio_content::SemioJackContent;

//#region 🔖️DocumentHelpers
/// 🔧️ The parameter-binding leaves that carry `before` to `after`: one `change-parameter-binding` per key whose value differs or
/// is new, one `remove-parameter-binding` per key `after` drops, in key order.
pub fn parameter_binding_mutations(before: &PropertyBag, after: &PropertyBag) -> Vec<RewriteRuleMutation> {
    use crate::standards::v1::subsets::any::schema::mutations::change_parameter_binding::change_parameter_binding;
use crate::standards::v1::subsets::any::schema::mutations::remove_parameter_binding::remove_parameter_binding;

    let changed = after.iter().filter(|(key, value)| before.get(*key) != Some(*value)).map(|(key, value)| change_parameter_binding(key.clone(), value.clone()));
    let removed = before.keys().filter(|key| !after.contains_key(*key)).map(|key| remove_parameter_binding(key.clone()));
    changed.chain(removed).collect()
}

/// 🆕️ A new editor begins with the canonical empty rule and graph.
pub fn default_rule_state() -> RewritingSnapshot {
    RewritingSnapshot::default()
}
/// 🧬️ Whole-document replace is banned from the `Mutation` enum outright (`SetState` — see
/// `📓️taxonomy.md`'s forbidden vocabulary), so `resetRule` builds a `Effect::LoadDocument`
/// (outside undo history) instead of an `artifact_mutations` entry.
pub fn reset_document_effect(state: &RewritingSnapshot) -> semio_framework_plugin::Effect {
    let pack = <RewritingSnapshot as ArtifactPack>::encode_pack(state);
    let spr = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::empty_document_spr("rewriting", REWRITE_RULE_SCHEMA));
    semio_framework_plugin::Effect::LoadDocument { pack, spr }
}



pub fn build_rule_from_state(state: &RewritingSnapshot) -> schema::rule::Rule {
    schema::rule::Rule { name: TRINITY_REWRITING_PLAY_RULE_NAME.into(), lhs: state.lhs.clone(), rhs: state.rhs.clone() }
}
pub fn compiled_jack_query(state: &RewritingSnapshot) -> String {
    crate::standards::v1::subsets::any::schema::application::build_rule_query(&build_rule_from_state(state), &state.parameter_bindings)
}
/// ♻️ The checked query projection produces a new typed graph without changing the retained source child.
pub fn rewritten_graph(state: &RewritingSnapshot) -> Result<JackSnapshot, String> {
    let mut graph = semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::memory::Graph::from_snapshot(state.working_graph.clone()).map_err(|error|error.to_string())?;
    schema::application::apply_rule(&mut graph, &build_rule_from_state(state), &state.parameter_bindings).map_err(|error|error.to_string())?;
    Ok(graph.to_snapshot::<SemioJackContent>())
}
/// 🧾️ The working graph's resolved manifest (embedded, else named by `manifestId`); `None` when it resolves to none.
pub fn resolved_working_manifest(state: &RewritingSnapshot) -> Option<semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::manifest::Manifest> {
    let mut graph = state.working_graph.clone();
    graph.resolve_manifest().ok().map(|()| graph.manifest.clone())
}
/// 🧩️ One semantic rule-graph node at its `rule_layout` point, else at its default slot ([`schema::lhs_graph_slots`],
/// [`schema::rhs_graph_slots`] — the same positions a node drag moves from).
pub fn semantic_rule_node(id: &str, kind: &str, name: &str, slots: &[(String, LayoutPoint)], rule_layout: &schema::layout::RuleLayout) -> Node {
    let default = slots.iter().find(|(slot, _)| slot == id).map_or(LayoutPoint { x: 0.0, y: 0.0 }, |(_, point)| *point);
    let point = rule_layout.get(id).copied().unwrap_or(default);
    Node { id: id.into(), name: name.into(), kind: kind.into(), x: point.x, y: point.y, width: 160.0, height: 56.0, ports: vec![], properties: Default::default() }
}

pub fn lhs_semantic_graph_snapshot(lhs: &schema::rule::Lhs, rule_layout: &schema::layout::RuleLayout) -> JackSnapshot {
    let slots = schema::lhs_graph_slots(lhs);
    let mut nodes = vec![semantic_rule_node("lhs-match", "rewriting.match", &format!("{}:{}", lhs.pattern.left_var, lhs.pattern.left_kind), &slots, rule_layout)];
    let mut edges = Vec::new();
    if let Some(where_clause) = lhs.where_clause.as_deref().filter(|value| !value.trim().is_empty()) {
        nodes.push(semantic_rule_node("lhs-where", "rewriting.where", where_clause, &slots, rule_layout));
        edges.push(semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::graph::Edge { id: "lhs-match-where".into(), kind: "rewriting.flow".into(), source: "lhs-match@out".into(), target: "lhs-where@in".into(), properties: Default::default() });
    }
    JackSnapshot::with_content::<SemioJackContent>(JackSnapshot::SCHEMA.into(), "lhs".into(), Some("nakagin".into()), semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::manifest::Manifest::nakagin_default(), Camera { x: 0.0, y: 0.0, zoom: 1.0 }, JackWorkingScene { nodes: nodes, edges: edges }, None)
}

pub fn rhs_semantic_graph_snapshot(rhs: &Rhs, rule_layout: &schema::layout::RuleLayout) -> JackSnapshot {
    let slots = schema::rhs_graph_slots(rhs);
    let node = |id: String, kind: &str, name: String| semantic_rule_node(&id, kind, &name, &slots, rule_layout);
    let mut nodes = Vec::new();
    nodes.extend(rhs.create.iter().enumerate().map(|(index, pattern)| node(format!("rhs-create-{index}"), "rewriting.create", format!("{}:{}", pattern.left_var, pattern.left_kind))));
    nodes.extend(rhs.merge.iter().enumerate().map(|(index, pattern)| node(format!("rhs-merge-{index}"), "rewriting.merge", format!("{}:{}", pattern.left_var, pattern.left_kind))));
    nodes.extend(rhs.set.iter().enumerate().map(|(index, assignment)| node(format!("rhs-set-{index}"), "rewriting.set", format!("{}.{} = {:?}", assignment.var, assignment.prop, assignment.value))));
    nodes.extend(rhs.delete.iter().enumerate().map(|(index, name)| node(format!("rhs-delete-{index}"), "rewriting.delete", name.clone())));
    nodes.extend(rhs.parameters.iter().enumerate().map(|(index, parameter)| {
        let kind = match parameter.kind {
            ParameterKind::String => "string",
            ParameterKind::Number => "number",
            ParameterKind::Boolean => "boolean",
        };
        node(format!("rhs-parameter-{index}"), "rewriting.parameter", format!("{}:{kind}", parameter.name))
    }));
    if nodes.is_empty() {
        nodes.push(node("rhs-empty".into(), "rewriting.create", "result:Piece".into()));
    }
    JackSnapshot::with_content::<SemioJackContent>(JackSnapshot::SCHEMA.into(), "rhs".into(), Some("nakagin".into()), semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::manifest::Manifest::nakagin_default(), Camera { x: 0.0, y: 0.0, zoom: 1.0 }, JackWorkingScene { nodes: nodes, edges: Vec::new() }, None)
}

pub fn lhs_graph_snapshot(lhs: &schema::rule::Lhs, rule_layout: &schema::layout::RuleLayout) -> JackSnapshot {
    lhs_semantic_graph_snapshot(lhs, rule_layout)
}
pub fn rhs_graph_snapshot(rhs: &Rhs, rule_layout: &schema::layout::RuleLayout) -> JackSnapshot {
    rhs_semantic_graph_snapshot(rhs, rule_layout)
}
/// 🕹️ Used by `interaction_topology` to hang a var-reference `TopologyNode` off its graph node
/// (domain "graph" — "AST parents + variable references").
pub fn var_from_node_name(name: &str) -> Option<String> {
    let trimmed = name.trim();
    if let Some((var, _)) = trimmed.split_once(':') {
        return Some(var.trim().into());
    }
    if let Some((var, _)) = trimmed.split_once(" : ") {
        return Some(var.trim().into());
    }
    None
}
//#endregion 🔖️DocumentHelpers
