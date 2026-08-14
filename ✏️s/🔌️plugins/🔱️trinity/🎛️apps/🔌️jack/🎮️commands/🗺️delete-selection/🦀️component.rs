//! 🗺️ 🗺️ Trinity Jack app command — `delete-selection`.

use crate::apps::jack::config::JackConfigMutation;
use crate::artifacts::jack::mutations::{delete_node, move_node, rename_node};
use crate::artifacts::jack::op::TrinityGraphMutation;
use crate::artifacts::jack::{JackSnapshot, Node};
use semio_framework_plugin::{Emit, Fault};

/// 🕹️ `selected_node_ids` now comes from `interaction.selection("ast").ids` (framework-owned) — no
/// explicit selection-clearing mutation is emitted anymore: the framework re-validates/prunes the
/// "ast" domain's selection against the fresh `interaction_topology` right after this document
/// dispatch lands, dropping the just-deleted ids on its own.
pub(crate) fn delete_selection(fixture: &JackSnapshot, selected_node_ids: &[String]) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    let scene_nodes = fixture.nodes();
    let deletes: Vec<TrinityGraphMutation> = selected_node_ids.iter().filter(|id| scene_nodes.iter().any(|node| &node.id == *id)).map(|id| delete_node(id.clone())).collect();
    if deletes.is_empty() {
        Ok(Emit::default())
    } else {
        Ok(Emit { artifact_mutations: deletes, ..Default::default() })
    }
}
