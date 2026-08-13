//! 👁️ 👁️ Trinity Rewrite app command — `set-selection`.

use crate::apps::rewrite::config::RewriteConfigMutation;
use crate::artifacts::jack::Camera;
use crate::artifacts::rewrite::op::RewriteRuleMutation;
use crate::artifacts::rewrite::RewriteSnapshot;
use semio_framework_plugin::{Emit, Fault};

pub(crate) fn set_selection(state: &RewriteSnapshot, ids: &[String], surface_id: &Option<String>, select_epoch: u64) -> Result<Emit<RewriteRuleMutation, RewriteConfigMutation>, Fault> {
    let mut config_mutations = vec![RewriteConfigMutation::SetSelection { node_ids: ids.to_vec() }];
    if let Some(node_id) = ids.first() {
        let fixture_json = crate::apps::rewrite::fixture_json_for_surface(surface_id.as_deref().unwrap_or(""), state);
        if let Some(var) = crate::apps::rewrite::sync_select_var_from_node(&fixture_json, node_id) {
            config_mutations.push(RewriteConfigMutation::SetActiveSelectVar { value: var });
        }
        config_mutations.push(RewriteConfigMutation::SetSelectEpoch { value: select_epoch + 1 });
    }
    Ok(Emit::config(config_mutations))
}
