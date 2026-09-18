//! 🗺️ 🗺️ Trinity Jack app command — `delete-selection`.

use crate::standards::v1::subsets::any::schema::mutations::{delete_edge, delete_node};
use crate::standards::v1::subsets::any::schema::mutations::text::TrinityGraphMutation;
use crate::JackSnapshot;
use semio_framework_plugin::{Emit, NoConfigMutation};

/// 🕹️ `selected_node_ids` now comes from `interaction.selection("ast").ids` (framework-owned) — no
/// explicit selection-clearing mutation is emitted anymore: the framework re-validates/prunes the
/// "ast" domain's selection against the fresh `interaction_topology` right after this document
/// dispatch lands, dropping the just-deleted ids on its own.
/// ✂️ Every severed edge is its OWN `delete-edge` row ahead of its node, so each published row stays
/// point-invertible (one forward, one inverse): a bare `delete-node` over a connected node inverts to
/// `create-node` PLUS one `create-edge` per severed edge, which overruns the one-item fold contract the
/// retained document lane admits (`batched item candidate failed its exact fixed fold contract`). The
/// `delete-node` diff still captures the cascade for any other caller.
pub(crate) fn delete_selection(snapshot: &JackSnapshot, selected_node_ids: &[String]) -> Emit<TrinityGraphMutation, NoConfigMutation> {
    let scene = crate::jack_working_scene(snapshot);
    let doomed: Vec<&String> = selected_node_ids.iter().filter(|id| scene.nodes.iter().any(|node| &node.id == *id)).collect();
    let mut mutations: Vec<TrinityGraphMutation> = Vec::new();
    for edge in &scene.edges {
        let touches = |endpoint: &str| crate::port_node_id(endpoint).is_some_and(|node| doomed.iter().any(|id| id.as_str() == node));
        if touches(&edge.source) || touches(&edge.target) {
            mutations.push(delete_edge(edge.id.clone()));
        }
    }
    mutations.extend(doomed.into_iter().map(|id| delete_node(id.clone())));
    if mutations.is_empty() {
        Emit::default()
    } else {
        Emit { artifact_mutations: mutations, ..Default::default() }
    }
}
