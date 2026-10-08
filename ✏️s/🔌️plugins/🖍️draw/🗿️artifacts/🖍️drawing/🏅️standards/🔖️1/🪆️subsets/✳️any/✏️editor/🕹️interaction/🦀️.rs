//! 🎯️ Document-derived layer membership for selection and stale-target pruning.
#[path="🎯️points/🦀️.rs"]
pub(crate) mod points;
use crate::{DrawingLayerNode, DrawingSnapshot};
use semio_framework_plugin::{DomainTopology, TopologyNode};
use super::DRAWING_INTERACTION_GRANULARITY;

pub(super) fn drawing_interaction_topology(snapshot: &DrawingSnapshot) -> DomainTopology {
    let mut ordered = Vec::new();
    let mut stack = vec![(snapshot.layers.iter(), None::<&semio_framework_value::paged::PagedUtf8<{usize::MAX}>>)];
    while let Some((layers, parent)) = stack.last_mut() {
        let Some(layer) = layers.next() else { stack.pop(); continue; };
        let id = crate::schema::layer_id(layer);
        ordered.push(TopologyNode { id: id.to_string_owner(), granularity: DRAWING_INTERACTION_GRANULARITY.into(), parent: parent.map(semio_framework_value::paged::PagedUtf8::to_string_owner) });
        if let DrawingLayerNode::Group(group) = layer {
            stack.push((group.children.iter(), Some(id)));
        }
    }
    DomainTopology { ordered }
}

/// 📍️ Only visible, unlocked path coordinates belong to the node-selection domain.
pub(super) fn drawing_point_topology(snapshot:&DrawingSnapshot)->DomainTopology {
    let mut ordered=Vec::new();
    let mut stack=vec![snapshot.layers.iter()];
    while let Some(layers)=stack.last_mut() {
        let Some(layer)=layers.next() else {stack.pop();continue;};
        let base=crate::schema::layer_base(layer);
        if !base.visible || base.locked {continue;}
        match layer {
            DrawingLayerNode::Group(group)=>stack.push(group.children.iter()),
            DrawingLayerNode::Path(path)=>{
                let Some(geometry)=points::geometry_id(&path.segments) else {continue;};
                for (index,segment) in path.segments.iter().enumerate() {
                    for point in points::point_slots(segment) {
                        if let Some(id)=points::point_id(&path.base.id,&geometry,index,*point) {ordered.push(TopologyNode {id,granularity:super::DRAWING_POINT_GRANULARITY.into(),parent:None});}
                    }
                }
            },
            _=>{},
        }
    }
    DomainTopology {ordered}
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
