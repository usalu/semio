//! 🗺️ 🗺️ Trinity Jack app command — `patch-nodes`.

use crate::artifacts::jack::mutations::rename_node;
use crate::artifacts::jack::op::TrinityGraphMutation;
use crate::artifacts::jack::JackSnapshot;
use crate::editor::jack::config::JackConfigMutation;
use semio_framework_plugin::{Emit, Fault};

pub(crate) async fn patch_nodes(fixture: &JackSnapshot, node_ids: &[String], field: &str, value: &str) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    if field == "name" && !node_ids.is_empty() && !value.trim().is_empty() {
        let scene_nodes = fixture.nodes();
        let operations: Vec<TrinityGraphMutation> = node_ids.iter().filter(|id| scene_nodes.iter().any(|node| &node.id == *id)).map(|id| rename_node(id.clone(), value.trim().into())).collect();
        Ok(Emit::mutations(operations))
    } else {
        Ok(Emit::default())
    }
}
