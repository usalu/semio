//! 🗺️ Trinity Jack app command — `delete-selection`.

use crate::standards::v1::subsets::any::schema::mutations::TrinityGraphMutation;
use crate::JackSnapshot;
use semio_framework_plugin::{Emit, Fault, NoConfigMutation};
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::mutations::{delete_edge::DeleteEdge, delete_node::DeleteNode, SemioGraphMutation};

/// 🕹️ `selected_node_ids` comes from `interaction.selection("ast").ids` (framework-owned); the framework prunes the
/// deleted ids from the selection once the document dispatch lands. The deletion is ONE edit of the composed `content`
/// child (design §20.15). ✂️ Every severed edge is its OWN `delete-edge` row ahead of its node, so each row stays
/// point-invertible (one forward, one inverse): a bare `delete-node` over a connected node inverts to `create-node` PLUS
/// one `create-edge` per severed edge.
pub(crate) fn delete_selection(snapshot: &JackSnapshot, children: &semio_framework_plugin::app::ChildContentView, selected_node_ids: &[String]) -> Result<Emit<TrinityGraphMutation, NoConfigMutation>, Fault> {
    let content = crate::jack_content_from_children(snapshot, children)?;
    let doomed: Vec<_> = content.nodes.iter().filter(|node| selected_node_ids.contains(&node.id.value)).map(|node| node.id.clone()).collect();
    let mut leaves: Vec<SemioGraphMutation> = content.edges.iter().filter(|edge| doomed.contains(&edge.source) || doomed.contains(&edge.target)).map(|edge| SemioGraphMutation::DeleteEdge(DeleteEdge { id: edge.id.clone() })).collect();
    leaves.extend(doomed.into_iter().map(|id| SemioGraphMutation::DeleteNode(DeleteNode { id })));
    Ok(crate::jack_child_emit(snapshot, leaves))
}
