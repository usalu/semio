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
}
//#endregion 🔖️Aggregate

//#region 🕸️WorkingGraph
/// 🕸️ The working graph JSON `json` with every node `targets` names edited in place by `edit` (which answers whether it changed
/// the node), re-serialized as compact JSON with sorted keys — the form every implementation of these leaves writes — together
/// with the targets the graph lacks and whether any node changed. `None` when `json` is no object with a `nodes` array.
pub(crate) fn edit_working_graph_nodes(json: &str, targets: &[String], mut edit: impl FnMut(&mut serde_json::Map<String, serde_json::Value>) -> bool) -> Option<(String, Vec<String>, bool)> {
    let mut graph: serde_json::Value = serde_json::from_str(json).ok()?;
    let mut found: Vec<String> = Vec::new();
    let mut changed = false;
    for node in graph.get_mut("nodes")?.as_array_mut()?.iter_mut().filter_map(serde_json::Value::as_object_mut) {
        let Some(id) = node.get("id").and_then(serde_json::Value::as_str).map(str::to_string) else { continue };
        if targets.contains(&id) {
            changed |= edit(node);
            found.push(id);
        }
    }
    let missing = targets.iter().filter(|target| !found.contains(target)).cloned().collect();
    Some((serde_json::to_string(&graph).ok()?, missing, changed))
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
