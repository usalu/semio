//! 🗺️ 🗺️ Trinity Jack app command — `delete-selection`.

use crate::editor::jack::config::JackConfigMutation;
use crate::standards::v1::subsets::any::schema::mutations::delete_node;
use crate::standards::v1::subsets::any::schema::mutations::text::TrinityGraphMutation;
use crate::JackSnapshot;
use semio_framework_plugin::Emit;

/// 🕹️ `selected_node_ids` now comes from `interaction.selection("ast").ids` (framework-owned) — no
/// explicit selection-clearing mutation is emitted anymore: the framework re-validates/prunes the
/// "ast" domain's selection against the fresh `interaction_topology` right after this document
/// dispatch lands, dropping the just-deleted ids on its own.
pub(crate) fn delete_selection(fixture: &JackSnapshot, selected_node_ids: &[String]) -> Emit<TrinityGraphMutation, JackConfigMutation> {
    let scene_nodes = fixture.nodes();
    let deletes: Vec<TrinityGraphMutation> = selected_node_ids.iter().filter(|id| scene_nodes.iter().any(|node| &node.id == *id)).map(|id| delete_node(id.clone())).collect();
    if deletes.is_empty() {
        Emit::default()
    } else {
        Emit { artifact_mutations: deletes, ..Default::default() }
    }
}
