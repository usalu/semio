//! 📜️ 📜️ Trinity Rewrite app command — `patch-nodes`.

use crate::artifacts::jack::{Graph, JackSnapshot};
use crate::artifacts::rewrite::rewrite_snapshot_mutations;
use crate::artifacts::rewrite::op::RewriteRuleMutation;
use crate::artifacts::rewrite::RewriteSnapshot;
use crate::editor::rewrite::config::RewriteConfigMutation;
use semio_framework_plugin::{Emit, Fault};

fn patch_fixture_nodes(fixture_json: &str, node_ids: &[String], field: &str, value: &str) -> Option<String> {
    let fixture = JackSnapshot::from_json(fixture_json).ok()?;
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
    let fixture = JackSnapshot::with_content(fixture.schema.clone(), fixture.name.clone(), fixture.manifest_id.clone(), fixture.manifest.clone(), fixture.camera.clone(), nodes, fixture.edges(), fixture.root_node_id.clone());
    Graph::from_fixture(fixture).ok()?.fixture_json().ok()
}

pub(crate) fn patch_nodes(state: &RewriteSnapshot, node_ids: &[String], field: &str, value: &str) -> Result<Emit<RewriteRuleMutation, RewriteConfigMutation>, Fault> {
    let trimmed = value.trim();
    if node_ids.is_empty() || field.is_empty() || trimmed.is_empty() {
        return Ok(Emit::default());
    }
    match patch_fixture_nodes(&state.before_fixture_json, node_ids, field, trimmed) {
        Some(patched) => {
            let mut next = state.clone();
            next.before_fixture_json = patched;
            Ok(Emit::mutations(rewrite_snapshot_mutations(state, &next)))
        }
        None => Ok(Emit::default()),
    }
}
