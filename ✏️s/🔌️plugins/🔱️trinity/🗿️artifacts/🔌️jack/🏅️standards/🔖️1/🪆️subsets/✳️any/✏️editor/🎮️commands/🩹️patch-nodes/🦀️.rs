//! 🩹️ Trinity Jack app command — `patch-nodes`.

use crate::standards::v1::subsets::any::schema::mutations::TrinityGraphMutation;
use crate::JackSnapshot;
use semio_framework_plugin::{Emit, Fault, FaultCode, FaultOrigin, NoConfigMutation};
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::mutations::{change_node_label::ChangeNodeLabel, SemioGraphMutation};
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::GraphNodeId;

/// 🩹️ Renames the named nodes — or, when `node_ids` is empty, the nodes selected in the `ast` domain, which is what a
/// rail press means — as ONE edit of the composed `content` child: one `change-node-label` leaf per node (design
/// §20.15). Every request that cannot move the document is refused by name instead of answering an empty emit:
/// `app.command.targets-required` when neither `nodeIds` nor a selection names a node, `mutation.target-missing`,
/// `app.command.invalid-args`.
pub(crate) fn patch_nodes(snapshot: &JackSnapshot, children: &semio_framework_plugin::app::ChildContentView, node_ids: &[String], selection: &[String], field: &str, value: &str) -> Result<Emit<TrinityGraphMutation, NoConfigMutation>, Fault> {
    let invalid = |detail: String| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-args"), detail);
    if field != "name" {
        return Err(invalid(format!("jack nodes have no patchable field '{field}' (only 'name')")));
    }
    let value = value.trim();
    if value.is_empty() {
        return Err(invalid("patchNodes needs a non-empty value".into()));
    }
    let targets = if node_ids.is_empty() { selection } else { node_ids };
    if targets.is_empty() {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.targets-required"), "patchNodes needs the nodes it renames: name them in nodeIds, or select them in the graph"));
    }
    let content = crate::jack_content_from_children(snapshot, children)?;
    let missing: Vec<&str> = targets.iter().filter(|id| !content.nodes.iter().any(|node| &node.id.value == *id)).map(String::as_str).collect();
    if !missing.is_empty() {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("mutation.target-missing"), format!("the jack graph has no node {}", missing.join(", "))));
    }
    let leaves: Vec<SemioGraphMutation> = targets.iter().map(|id| SemioGraphMutation::ChangeNodeLabel(ChangeNodeLabel { id: GraphNodeId::new(id.clone()), new_label: value.into() })).collect();
    Ok(crate::jack_child_emit(snapshot, leaves))
}
