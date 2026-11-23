//! 🩹️ Trinity Jack app command — `patch-nodes`.

use crate::standards::v1::subsets::any::schema::snapshot::JackSnapshot;
use semio_plugin::{Fault, FaultCode, FaultOrigin};
use semio_s_artifact_trinity_jack_graph_query::JackGraphQuery;

/// 🩹️ Renames the named nodes — or, when `node_ids` is empty, the nodes selected in the `ast` domain, which is what a
/// rail press means — as the effects of ONE edit of the composed `content` child: one `change-node-label` leaf per node (design
/// §20.15). Every request that cannot move the document is refused by name instead of answering an empty emit:
/// `app.command.targets-required` when neither `nodeIds` nor a selection names a node, `mutation.target-missing`,
/// `app.command.invalid-args`.
pub fn patch_nodes_effects<C:crate::standards::v1::subsets::any::schema::composition::JackComposition>(snapshot: &JackSnapshot, children: &semio_plugin::app::ChildContentView, node_ids: &[String], selection: &[String], field: &str, value: &str) -> Result<Vec<crate::standards::v1::subsets::any::schema::graph::GraphEffect>, Fault> {
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
    let content = C::bind(snapshot, children)?;
    let missing: Vec<&str> = targets.iter().filter(|id| !(0..content.node_count()).filter_map(|index|content.node_id(index)).any(|node|node==id.as_str())).map(String::as_str).collect();
    if !missing.is_empty() {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("mutation.target-missing"), format!("the jack graph has no node {}", missing.join(", "))));
    }
    Ok(targets.iter().map(|id|crate::standards::v1::subsets::any::schema::graph::GraphEffect::RenameNode{id:id.clone(),name:value.into()}).collect())
}
