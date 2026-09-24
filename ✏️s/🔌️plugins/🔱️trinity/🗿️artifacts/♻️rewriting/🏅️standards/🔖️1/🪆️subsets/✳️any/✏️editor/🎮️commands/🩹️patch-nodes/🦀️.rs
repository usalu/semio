//! 🩹️ Trinity Rewriting app command — `patch-nodes`.

use semio_s_artifact_trinity_jack::JackWorkingScene;
use crate::rewriting_snapshot_mutations;
use crate::standards::v1::subsets::any::schema::mutations::text::RewriteRuleMutation;
use crate::RewritingSnapshot;
use semio_framework_plugin::{Emit, Fault, FaultCode, FaultOrigin, NoConfigMutation};
use semio_s_artifact_trinity_jack::{Graph, JackSnapshot};

fn patch_fixture_nodes(fixture_json: &str, node_ids: &[String], field: &str, value: &str) -> Result<String, String> {
    let fixture = JackSnapshot::from_json(fixture_json).map_err(|error| error.to_string())?;
    let mut nodes = fixture.nodes();
    for node in nodes.iter_mut() {
        if !node_ids.iter().any(|id| id == &node.id) {
            continue;
        }
        match field {
            "name" => node.name = value.into(),
            "kind" => node.kind = value.into(),
            _ => {}
        }
    }
    let fixture = JackSnapshot::with_content(fixture.schema.clone(), fixture.name.clone(), fixture.manifest_id.clone(), fixture.manifest.clone(), fixture.camera.clone(), JackWorkingScene { nodes: nodes, edges: fixture.edges() }, fixture.root_node_id);
    Graph::from_snapshot(fixture).and_then(|graph| graph.host_snapshot_json()).map_err(|error| error.to_string())
}

/// 🩹️ Patches `name` or `kind` of the named nodes of the rule's working (before) graph — or, when
/// `node_ids` is empty, of the nodes selected in the `graph` domain, which is what a rail press means.
/// Every request that cannot move the document is refused by name (`mutation.target-missing`,
/// `app.command.invalid-args`) instead of answering an empty emit: the silent empty emit read as an
/// accepted edit that moved nothing (S15, session 11).
pub(crate) fn patch_nodes(state: &RewritingSnapshot, node_ids: &[String], selection: &[String], field: &str, value: &str) -> Result<Emit<RewriteRuleMutation, NoConfigMutation>, Fault> {
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
        return Err(invalid("patchNodes needs node ids or a node selection in the graph".into()));
    }
    let fixture = JackSnapshot::from_json(&state.before_fixture_json).map_err(|error| invalid(format!("the working graph does not decode: {error}")))?;
    let nodes = fixture.nodes();
    let missing: Vec<&str> = targets.iter().filter(|id| !nodes.iter().any(|node| &node.id == *id)).map(String::as_str).collect();
    if !missing.is_empty() {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("mutation.target-missing"), format!("the working graph has no node {}", missing.join(", "))));
    }
    let patched = patch_fixture_nodes(&state.before_fixture_json, targets, field, value).map_err(|error| invalid(format!("the patched working graph is not a valid graph: {error}")))?;
    let mut next = state.clone();
    next.before_fixture_json = patched;
    Ok(Emit::mutations(rewriting_snapshot_mutations(state, &next)))
}
