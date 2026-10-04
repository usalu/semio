//! 🩹️ Trinity Rewriting app command — `patch-nodes`.

use crate::content::{read, working_child_emit};
use crate::standards::v1::subsets::any::schema::mutations::text::RewriteRuleMutation;
use crate::RewritingSnapshot;
use semio_framework_plugin::app::ChildContentView;
use semio_framework_plugin::{Emit, Fault, FaultCode, FaultOrigin, NoConfigMutation};
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::mutations::{change_node_kind::ChangeNodeKind, change_node_label::ChangeNodeLabel, SemioGraphMutation};
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::GraphNodeId;

/// 🩹️ Sets `name` or `kind` of the named nodes of the rule's working (before) graph — or, when `node_ids` is empty, of the nodes
/// selected in the `graph` domain, which is what a rail press means — as ONE edit of the composed `workingGraph` child: one
/// `change-node-label` or `change-node-kind` leaf per node (design §20.15). Every request that cannot move the document is refused
/// by name instead of answering an empty emit (the silent empty emit read as an accepted edit that moved nothing, S15 session 11):
/// `app.command.targets-required` when neither `nodeIds` nor a selection names a node — an agent has no selection and names them —
/// `mutation.target-missing`, `app.command.invalid-args` (also for a kind the resolved manifest does not declare).
pub(crate) fn patch_nodes(state: &RewritingSnapshot, children: &ChildContentView, node_ids: &[String], selection: &[String], field: &str, value: &str) -> Result<Emit<RewriteRuleMutation, NoConfigMutation>, Fault> {
    let invalid = |detail: String| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-args"), detail);
    if !matches!(field, "name" | "kind") {
        return Err(invalid(format!("rewriting nodes have no patchable field '{field}' (only 'name' or 'kind')")));
    }
    let value = value.trim();
    if value.is_empty() {
        return Err(invalid("patchNodes needs a non-empty value".into()));
    }
    let targets = if node_ids.is_empty() { selection } else { node_ids };
    if targets.is_empty() {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.targets-required"), "patchNodes needs the nodes it patches: name them in nodeIds, or select them in the graph"));
    }
    let work = read(state, children)?;
    let missing: Vec<&str> = targets.iter().filter(|id| !work.nodes.iter().any(|node| &node.id.value == *id)).map(String::as_str).collect();
    if !missing.is_empty() {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("mutation.target-missing"), format!("the working graph has no node {}", missing.join(", "))));
    }
    if field == "kind" && crate::editor::rewriting::resolved_working_manifest(state).is_some_and(|manifest| manifest.node_kind(value).is_none()) {
        return Err(invalid(format!("the working graph's manifest declares no node kind “{value}”")));
    }
    let leaf = |id: &String| match field {
        "name" => SemioGraphMutation::ChangeNodeLabel(ChangeNodeLabel { id: GraphNodeId::new(id.clone()), new_label: value.into() }),
        _ => SemioGraphMutation::ChangeNodeKind(ChangeNodeKind { id: GraphNodeId::new(id.clone()), new_kind: value.into() }),
    };
    Ok(working_child_emit(state, &targets.iter().map(leaf).collect::<Vec<_>>()))
}
