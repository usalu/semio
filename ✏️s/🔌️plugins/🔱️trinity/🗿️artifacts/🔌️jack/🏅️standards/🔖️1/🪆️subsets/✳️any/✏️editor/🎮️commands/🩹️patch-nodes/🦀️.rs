//! 🩹️ Trinity Jack app command — `patch-nodes`.

use crate::standards::v1::subsets::any::schema::mutations::rename_node;
use crate::standards::v1::subsets::any::schema::mutations::text::TrinityGraphMutation;
use crate::JackSnapshot;
use semio_framework_plugin::{Emit, Fault, FaultCode, FaultOrigin, NoConfigMutation};

/// 🩹️ Renames the named nodes — or, when `node_ids` is empty, the nodes selected in the `ast`
/// domain, which is what a rail press means. Every request that cannot move the document is refused
/// by name (`mutation.target-missing`, `app.command.invalid-args`) instead of answering an empty emit:
/// the silent empty emit read as an accepted edit that moved nothing (S15, session 11).
pub(crate) fn patch_nodes(snapshot: &JackSnapshot, node_ids: &[String], selection: &[String], field: &str, value: &str) -> Result<Emit<TrinityGraphMutation, NoConfigMutation>, Fault> {
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
        return Err(invalid("patchNodes needs node ids or a node selection in the graph".into()));
    }
    let scene_nodes = snapshot.nodes();
    let missing: Vec<&str> = targets.iter().filter(|id| !scene_nodes.iter().any(|node| &node.id == *id)).map(String::as_str).collect();
    if !missing.is_empty() {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("mutation.target-missing"), format!("the jack graph has no node {}", missing.join(", "))));
    }
    Ok(Emit::mutations(targets.iter().map(|id| rename_node(id.clone(), value.into())).collect()))
}
