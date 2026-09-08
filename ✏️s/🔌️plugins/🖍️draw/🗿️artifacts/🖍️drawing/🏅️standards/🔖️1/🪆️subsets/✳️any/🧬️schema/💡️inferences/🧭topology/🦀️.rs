//! 🧭 `topology` — one named inference: a real pre-order traversal of `DrawingSnapshot.layers`'
//! structural nesting (`DrawingLayerNode::Group.children: Vec<DrawingLayerNode>` is a genuine tree, owned
//! by value — not an id-reference graph like `sequence`'s step DAG). `topoOrder` is document order
//! with every ancestor preceding its descendants (already a valid topological order for a tree);
//! `depth` is each layer's nesting depth from its root (0 at the top level); `cycleFree` is always
//! `true` — a Rust `Vec<Self>` embedded by value cannot express a structural cycle, unlike an
//! id-reference graph, so this is a static invariant, not an empirical result; `nodeCount` is the
//! total flattened layer count. `DrawingLayerNode::Boolean.children: Vec<String>` are id REFERENCES
//! (like an asset key), not structural nesting, so they are honestly excluded from this topology —
//! conflating the two would let a dangling/self reference fabricate a fake cycle in what is
//! otherwise a real tree invariant.

use crate::{DrawingLayerNode, DrawingSnapshot};
use std::collections::BTreeMap;

//#region 🔖️Topology
fn layer_id(layer: &DrawingLayerNode) -> &str {
    match layer {
        DrawingLayerNode::Shape(body) => &body.base.id,
        DrawingLayerNode::Path(body) => &body.base.id,
        DrawingLayerNode::Text(body) => &body.base.id,
        DrawingLayerNode::Image(body) => &body.base.id,
        DrawingLayerNode::Group(body) => &body.base.id,
        DrawingLayerNode::Boolean(body) => &body.base.id,
        DrawingLayerNode::Trace(body) => &body.base.id,
    }
}

fn walk(layers: &[DrawingLayerNode], level: u32, topo_order: &mut Vec<String>, depth: &mut BTreeMap<String, u32>) {
    for layer in layers {
        let id = layer_id(layer).to_string();
        topo_order.push(id.clone());
        depth.insert(id, level);
        if let DrawingLayerNode::Group(group) = layer {
            walk(&group.children, level + 1, topo_order, depth);
        }
    }
}

/// 🧭️ Drawing's layer-tree topology — see module doc for the structural-nesting derivation.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DrawingTopology {
    pub topo_order: Vec<String>,
    pub depth: BTreeMap<String, u32>,
    pub cycle_free: bool,
    pub node_count: u32,
}

/// 🧮️ Computes [`DrawingTopology`] via a pre-order walk of `layers`' `Group.children` nesting.
pub fn compute_drawing_topology(snapshot: &DrawingSnapshot) -> DrawingTopology {
    let mut topo_order = Vec::new();
    let mut depth = BTreeMap::new();
    walk(&snapshot.layers, 0, &mut topo_order, &mut depth);
    let node_count = topo_order.len() as u32;
    DrawingTopology { topo_order, depth, cycle_free: true, node_count }
}
//#endregion 🔖️Topology

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
