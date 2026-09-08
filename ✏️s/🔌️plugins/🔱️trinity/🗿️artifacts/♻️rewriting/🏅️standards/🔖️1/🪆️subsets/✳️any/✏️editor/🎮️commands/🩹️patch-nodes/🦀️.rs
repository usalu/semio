//! 📜️ 📜️ Trinity Rewriting app command — `patch-nodes`.

use semio_s_artifact_trinity_jack::{Graph, JackSnapshot};
use crate::rewriting_snapshot_mutations;
use crate::standards::v1::subsets::any::schema::mutations::text::RewriteRuleMutation;
use crate::RewritingSnapshot;
use crate::editor::rewriting::config::RewritingConfigMutation;
use semio_framework_plugin::Emit;

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
    let fixture = JackSnapshot::with_content(fixture.schema.clone(), fixture.name.clone(), fixture.manifest_id.clone(), fixture.manifest.clone(), fixture.camera.clone(), nodes, fixture.edges(), fixture.root_node_id);
    Graph::from_fixture(fixture).ok()?.fixture_json().ok()
}

pub(crate) fn patch_nodes(state: &RewritingSnapshot, node_ids: &[String], field: &str, value: &str) -> Emit<RewriteRuleMutation, RewritingConfigMutation> {
    let trimmed = value.trim();
    if node_ids.is_empty() || field.is_empty() || trimmed.is_empty() {
        return Emit::default();
    }
    match patch_fixture_nodes(&state.before_fixture_json, node_ids, field, trimmed) {
        Some(patched) => {
            let mut next = state.clone();
            next.before_fixture_json = patched;
            Emit::mutations(rewriting_snapshot_mutations(state, &next))
        }
        None => Emit::default(),
    }
}
