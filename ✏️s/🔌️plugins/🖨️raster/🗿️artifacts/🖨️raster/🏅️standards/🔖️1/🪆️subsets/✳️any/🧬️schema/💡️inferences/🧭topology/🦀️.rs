//! 🧭 `topology` — one named inference: a real pre-order traversal of `RasterSnapshot.layers`'
//! structural nesting (`RasterLayerNode::Group.children: Vec<RasterLayerNode>` is a genuine tree,
//! owned by value). `topoOrder` is document order with every ancestor preceding its descendants
//! (already a valid topological order for a tree); `depth` is each layer's nesting depth from its
//! root (0 at the top level); `cycleFree` is always `true` — a Rust `Vec<Self>` embedded by value
//! cannot express a structural cycle, so this is a static invariant, not an empirical result;
//! `nodeCount` is the total flattened layer count (`Pixel`/`Group`/`Adjustment` all counted).

use crate::{RasterLayerNode, RasterSnapshot};
use std::collections::BTreeMap;

//#region 🔖️Topology
fn layer_id(layer: &RasterLayerNode) -> &str {
    match layer {
        RasterLayerNode::Pixel { id, .. } | RasterLayerNode::Group { id, .. } | RasterLayerNode::Adjustment { id, .. } => id,
    }
}

fn walk(layers: &[RasterLayerNode], level: u32, topo_order: &mut Vec<String>, depth: &mut BTreeMap<String, u32>) {
    for layer in layers {
        let id = layer_id(layer).to_string();
        topo_order.push(id.clone());
        depth.insert(id, level);
        if let RasterLayerNode::Group { children, .. } = layer {
            walk(children, level + 1, topo_order, depth);
        }
    }
}

/// 🧭️ Raster's layer-tree topology — see module doc for the structural-nesting derivation.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase")]
pub struct RasterTopology {
    pub topo_order: Vec<String>,
    pub depth: BTreeMap<String, u32>,
    pub cycle_free: bool,
    pub node_count: u32,
}

/// 🧮️ Computes [`RasterTopology`] via a pre-order walk of `layers`' `Group.children` nesting.
pub fn compute_raster_topology(snapshot: &RasterSnapshot) -> RasterTopology {
    let mut topo_order = Vec::new();
    let mut depth = BTreeMap::new();
    walk(&snapshot.layers, 0, &mut topo_order, &mut depth);
    let node_count = topo_order.len() as u32;
    RasterTopology { topo_order, depth, cycle_free: true, node_count }
}
//#endregion 🔖️Topology

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
