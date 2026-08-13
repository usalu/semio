//! 👁️ 👁️ Trinity Rewrite app command — `graph-pointer-down`.

use crate::apps::rewrite::config::RewriteConfigMutation;
use crate::artifacts::jack::Camera;
use crate::artifacts::rewrite::op::RewriteRuleMutation;
use crate::artifacts::rewrite::RewriteSnapshot;
use semio_framework_plugin::{Emit, Fault};

pub(crate) fn graph_pointer_down(node_id: &Option<String>) -> Result<Emit<RewriteRuleMutation, RewriteConfigMutation>, Fault> {
    Ok(Emit::config(vec![RewriteConfigMutation::SetSelection { node_ids: node_id.clone().map(|id| vec![id]).unwrap_or_default() }]))
}
