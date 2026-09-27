//! 🎯️ Document-derived layer membership for selection and stale-target pruning.
use crate::{DrawingLayerNode, DrawingSnapshot};
use semio_framework_plugin::{DomainTopology, TopologyNode};
use super::DRAWING_INTERACTION_GRANULARITY;

pub(super) fn drawing_interaction_topology(snapshot: &DrawingSnapshot) -> DomainTopology {
    let mut ordered = Vec::new();
    let mut stack = vec![(snapshot.layers.iter(), None::<&str>)];
    while let Some((layers, parent)) = stack.last_mut() {
        let Some(layer) = layers.next() else { stack.pop(); continue; };
        let id = crate::schema::layer_id(layer);
        ordered.push(TopologyNode { id: id.to_owned(), granularity: DRAWING_INTERACTION_GRANULARITY.into(), parent: parent.map(str::to_owned) });
        if let DrawingLayerNode::Group(group) = layer {
            stack.push((group.children.iter(), Some(id)));
        }
    }
    DomainTopology { ordered }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
