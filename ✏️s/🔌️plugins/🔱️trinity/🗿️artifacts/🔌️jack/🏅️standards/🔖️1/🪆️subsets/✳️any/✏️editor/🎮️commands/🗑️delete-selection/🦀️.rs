//! 🗺️ Trinity Jack app command — `delete-selection`.

use crate::standards::v1::subsets::any::schema::mutations::{TrinityGraphMutation, clear_deleted_root};
use crate::standards::v1::subsets::any::schema::snapshot::JackSnapshot;
use semio_plugin::Fault;
use semio_s_artifact_trinity_jack_graph_query::JackGraphQuery;

/// 🕹️ `selected_node_ids` comes from `interaction.selection("ast").ids` (framework-owned); the framework prunes the
/// deleted ids from the selection once the document dispatch lands. The deletion is ONE edit of the composed `content`
/// child (design §20.15). ✂️ Every severed edge is its OWN `delete-edge` row ahead of its node, so each row stays
/// point-invertible (one forward, one inverse): a bare `delete-node` over a connected node inverts to `create-node` PLUS
/// one `create-edge` per severed edge.
pub fn delete_selection_effects<C:crate::standards::v1::subsets::any::schema::composition::JackComposition>(snapshot: &JackSnapshot, children: &semio_plugin::app::ChildContentView, selected_node_ids: &[String]) -> Result<(Vec<crate::standards::v1::subsets::any::schema::graph::GraphEffect>, Option<TrinityGraphMutation>), Fault> {
    let content=C::bind(snapshot,children)?;
    let doomed:Vec<_>=(0..content.node_count()).filter_map(|index|content.node_id(index)).filter(|id|selected_node_ids.iter().any(|selected|selected==id)).map(str::to_string).collect();
    let mut effects:Vec<_>=(0..content.edge_count()).filter_map(|index|content.edge(index)).filter(|edge|doomed.iter().any(|id|id==edge.source||id==edge.target)).map(|edge|crate::standards::v1::subsets::any::schema::graph::GraphEffect::DeleteEdge(edge.id.to_string())).collect();
    effects.extend(doomed.into_iter().map(crate::standards::v1::subsets::any::schema::graph::GraphEffect::DeleteNode));
    let root=clear_deleted_root(snapshot, &effects);
    Ok((effects, root))
}
