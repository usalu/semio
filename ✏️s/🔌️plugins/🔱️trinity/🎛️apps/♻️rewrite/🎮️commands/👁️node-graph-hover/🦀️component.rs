//! 👁️ 👁️ Trinity Rewrite app command — `node-graph-hover`.

use crate::apps::rewrite::config::RewriteConfigMutation;
use crate::artifacts::jack::Camera;
use crate::artifacts::rewrite::op::RewriteRuleMutation;
use crate::artifacts::rewrite::RewriteSnapshot;
use semio_framework_plugin::{Emit, Fault};

pub(crate) fn node_graph_hover(state: &RewriteSnapshot, surface_id: &Option<String>, node_id: &Option<String>, hover_epoch: u64) -> Result<Emit<RewriteRuleMutation, RewriteConfigMutation>, Fault> {
    match node_id {
        Some(node_id) => {
            let fixture_json = crate::apps::rewrite::fixture_json_for_surface(surface_id.as_deref().unwrap_or(""), state);
            let mut config_mutations = vec![RewriteConfigMutation::SetHoverEpoch { value: hover_epoch + 1 }];
            if let Some(var) = crate::apps::rewrite::sync_select_var_from_node(&fixture_json, node_id) {
                config_mutations.push(RewriteConfigMutation::SetActiveHoverVar { value: var });
            }
            Ok(Emit::config(config_mutations))
        }
        None => Ok(Emit::default()),
    }
}
