//! ♻️ `trinity.rewrite.rule` semantic mutation aggregate.
//!
//! Every variant wraps the payload owned by its direct `<mutation>/🦀️.rs` leaf.

use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::RewritingSnapshot;

pub use super::change_parameter_binding::{change_parameter_binding, ChangeParameterBinding};
pub use super::change_rule_layout_point::{change_rule_layout_point, ChangeRuleLayoutPoint};
pub use super::edit_before_fixture::{edit_before_fixture, EditBeforeFixture};
pub use super::edit_lhs::{edit_lhs, EditLhs};
pub use super::edit_rhs::{edit_rhs, EditRhs};
pub use super::remove_parameter_binding::{remove_parameter_binding, RemoveParameterBinding};
pub use super::remove_rule_layout_point::{remove_rule_layout_point, RemoveRuleLayoutPoint};
pub use super::drag_working_nodes::{drag_working_nodes, DragWorkingNodes};
pub use super::patch_working_nodes::{patch_working_nodes, PatchWorkingNodes};
pub use super::drag_rule_nodes::{drag_rule_nodes, DragRuleNodes};
pub use super::set_rule_layout_points::{set_rule_layout_points, RuleLayoutPlacement, SetRuleLayoutPoints};
pub use super::delete_working_nodes::{delete_working_nodes, DeleteWorkingNodes};
pub use super::connect_working_ports::{connect_working_ports, ConnectWorkingPorts};
pub use super::disconnect_working_edges::{disconnect_working_edges, DisconnectWorkingEdges};

//#region 🔖️Aggregate
/// 🧮️ Semantic rewrite-rule mutation vocabulary.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslEnum, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = RewritingSnapshot, diff = RewritingDiff, schema = "s.trinity.rewriting")]
pub enum RewriteRuleMutation {
    EditBeforeFixture(EditBeforeFixture),
    EditLhs(EditLhs),
    EditRhs(EditRhs),
    ChangeParameterBinding(ChangeParameterBinding),
    RemoveParameterBinding(RemoveParameterBinding),
    ChangeRuleLayoutPoint(ChangeRuleLayoutPoint),
    RemoveRuleLayoutPoint(RemoveRuleLayoutPoint),
    DragWorkingNodes(DragWorkingNodes),
    PatchWorkingNodes(PatchWorkingNodes),
    DragRuleNodes(DragRuleNodes),
    SetRuleLayoutPoints(SetRuleLayoutPoints),
    DeleteWorkingNodes(DeleteWorkingNodes),
    ConnectWorkingPorts(ConnectWorkingPorts),
    DisconnectWorkingEdges(DisconnectWorkingEdges),
}
//#endregion 🔖️Aggregate

//#region 🕸️WorkingGraph
/// 🕸️ The working graph JSON `json` with every node `targets` names edited in place by `edit` (which answers whether it changed
/// the node), re-serialized as compact JSON with sorted keys — the form every implementation of these leaves writes — together
/// with the targets the graph lacks and whether any node changed. `None` when `json` is no object with a `nodes` array.
pub(crate) fn edit_working_graph_nodes(json: &str, targets: &[String], mut edit: impl FnMut(&mut pack::JsonObject) -> bool) -> Option<(String, Vec<String>, bool)> {
    let mut graph = pack::parse_json(json).ok()?;
    let mut found: Vec<String> = Vec::new();
    let mut changed = false;
    for node in graph.get_mut("nodes")?.as_array_mut()?.iter_mut().filter_map(pack::JsonValue::as_object_mut) {
        let Some(id) = node.get("id").and_then(pack::JsonValue::as_str).map(str::to_string) else { continue };
        if targets.contains(&id) {
            changed |= edit(node);
            found.push(id);
        }
    }
    let missing = targets.iter().filter(|target| !found.contains(target)).cloned().collect();
    Some((pack::json_to_string(&sorted_keys(graph)), missing, changed))
}

/// 🔤️ `value` with every object's keys in ascending order, recursively — the canonical key order these leaves write.
fn sorted_keys(value: pack::JsonValue) -> pack::JsonValue {
    match value {
        pack::JsonValue::Array(items) => pack::JsonValue::Array(items.into_iter().map(sorted_keys).collect()),
        pack::JsonValue::Object(object) => {
            let mut entries: Vec<(String, pack::JsonValue)> = object.iter().map(|(key, value)| (key.to_string(), sorted_keys(value.clone()))).collect();
            entries.sort_by(|left, right| left.0.cmp(&right.0));
            pack::JsonValue::Object(entries.into_iter().collect())
        }
        scalar => scalar,
    }
}

/// ✂️ The working graph JSON `json` without the nodes `targets` names and without every edge with an endpoint on one of them (a
/// `node@port` endpoint names its node before the `@`), its `rootNodeId` cleared to `null` when it named a removed node, re-serialized as
/// compact JSON with sorted keys, together with the targets the graph lacks. `None` when `json` is no object with a `nodes` array.
pub(crate) fn remove_working_graph_nodes(json: &str, targets: &[String]) -> Option<(String, Vec<String>)> {
    let mut graph = pack::parse_json(json).ok()?;
    let nodes = graph.get_mut("nodes")?.as_array_mut()?;
    let found: Vec<String> = nodes.iter().filter_map(|node| node.get("id").and_then(pack::JsonValue::as_str)).filter(|id| targets.iter().any(|target| target == id)).map(str::to_string).collect();
    nodes.retain(|node| node.get("id").and_then(pack::JsonValue::as_str).is_none_or(|id| !found.iter().any(|gone| gone == id)));
    let removed = |edge: &pack::JsonValue| ["source", "target"].iter().any(|side| edge.get(side).and_then(pack::JsonValue::as_str).is_some_and(|key| found.iter().any(|gone| gone == semio_s_artifact_trinity_jack::port_node_id(key).unwrap_or(key))));
    if let Some(edges) = graph.get_mut("edges").and_then(pack::JsonValue::as_array_mut) {
        edges.retain(|edge| !removed(edge));
    }
    if graph.get("rootNodeId").and_then(pack::JsonValue::as_str).is_some_and(|root| found.iter().any(|gone| gone == root)) {
        graph.as_object_mut()?.insert("rootNodeId", pack::JsonValue::Null);
    }
    let missing = targets.iter().filter(|target| !found.contains(target)).cloned().collect();
    Some((pack::json_to_string(&sorted_keys(graph)), missing))
}

/// 🔌️ The working graph JSON `json` with ONE edge `id` of `kind` from the `source` to the `target` endpoint appended (an `edges`
/// array is created when the graph has none), re-serialized as compact JSON with sorted keys, together with the endpoint nodes the
/// graph lacks (a `node@port` endpoint names its node before the `@`) and whether the wire is new; a graph that lacks an endpoint
/// or already holds the wire is answered unchanged. `None` when `json` is no object with a `nodes` array.
pub(crate) fn connect_working_graph_ports(json: &str, id: &str, source: &str, target: &str, kind: &str) -> Option<(String, Vec<String>, bool)> {
    let mut graph = pack::parse_json(json).ok()?;
    let nodes = graph.get("nodes")?.as_array()?;
    let missing: Vec<String> = [source, target].iter().map(|key| semio_s_artifact_trinity_jack::port_node_id(key).unwrap_or(*key)).filter(|node| !nodes.iter().any(|held| held.get("id").and_then(pack::JsonValue::as_str) == Some(*node))).map(str::to_string).collect();
    let held = |edge: &pack::JsonValue| edge.get("source").and_then(pack::JsonValue::as_str) == Some(source) && edge.get("target").and_then(pack::JsonValue::as_str) == Some(target);
    if !missing.is_empty() || graph.get("edges").and_then(pack::JsonValue::as_array).is_some_and(|edges| edges.iter().any(held)) {
        return Some((json.to_string(), missing, false));
    }
    let edge = pack::json_object([("id".to_string(), pack::JsonValue::String(id.into())), ("kind".to_string(), pack::JsonValue::String(kind.into())), ("source".to_string(), pack::JsonValue::String(source.into())), ("target".to_string(), pack::JsonValue::String(target.into()))]);
    match graph.get_mut("edges").and_then(pack::JsonValue::as_array_mut) {
        Some(edges) => edges.push(edge),
        None => {
            graph.as_object_mut()?.insert("edges", pack::json_array([edge]));
        }
    }
    Some((pack::json_to_string(&sorted_keys(graph)), Vec::new(), true))
}

/// 🪚️ The working graph JSON `json` without the edges `targets` names, re-serialized as compact JSON with sorted keys, together
/// with the targets the graph lacks. `None` when `json` is no object with a `nodes` array.
pub(crate) fn remove_working_graph_edges(json: &str, targets: &[String]) -> Option<(String, Vec<String>)> {
    let mut graph = pack::parse_json(json).ok()?;
    graph.get("nodes")?.as_array()?;
    let mut found: Vec<String> = Vec::new();
    if let Some(edges) = graph.get_mut("edges").and_then(pack::JsonValue::as_array_mut) {
        found = edges.iter().filter_map(|edge| edge.get("id").and_then(pack::JsonValue::as_str)).filter(|id| targets.iter().any(|target| target == id)).map(str::to_string).collect();
        edges.retain(|edge| edge.get("id").and_then(pack::JsonValue::as_str).is_none_or(|id| !found.iter().any(|gone| gone == id)));
    }
    let missing = targets.iter().filter(|target| !found.contains(target)).cloned().collect();
    Some((pack::json_to_string(&sorted_keys(graph)), missing))
}

/// 🕸️ Whether `json` is a working graph whose every node and edge kind its own manifest declares.
pub(crate) fn working_graph_is_valid(json: &str) -> bool {
    semio_s_artifact_trinity_jack::JackSnapshot::from_json(json).ok().is_some_and(|graph| semio_s_artifact_trinity_jack::Graph::from_snapshot(graph).is_ok())
}

/// 🔢️ A canvas offset as a label shows it: two decimals at most, trailing zeros dropped, `(en, de)`.
pub(crate) fn offset_text(value: f64) -> (String, String) {
    let rounded = (value * 100.0).round() / 100.0;
    let text = format!("{:.2}", if rounded == 0.0 { 0.0 } else { rounded });
    let en = text.trim_end_matches('0').trim_end_matches('.').to_string();
    let de = en.replace('.', ",");
    (en, de)
}
//#endregion 🕸️WorkingGraph

//#region 🧪️StructuralCorrespondence
#[cfg(test)]
#[path = "🧪️tests/🔬️structural-correspondence/🦀️.rs"]
mod structural_correspondence_tests;
//#endregion 🧪️StructuralCorrespondence
