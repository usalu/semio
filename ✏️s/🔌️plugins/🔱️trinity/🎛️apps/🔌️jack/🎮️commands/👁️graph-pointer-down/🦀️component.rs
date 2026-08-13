//! 👁️ 👁️ Trinity Jack app command — `graph-pointer-down`.

use crate::apps::jack::config::{JackConfigMutation, JackEditorSelection};
use crate::artifacts::jack::op::TrinityGraphMutation;
use crate::artifacts::jack::Camera;
use semio_framework_plugin::{Emit, Fault};

pub(crate) fn graph_pointer_down(node_id: &Option<String>) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    Ok(Emit::config(vec![JackConfigMutation::SetSelection { node_ids: node_id.clone().map(|id| vec![id]).unwrap_or_default() }]))
}
