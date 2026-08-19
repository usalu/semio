//! 🧭 `topology` — one named inference: a real pre-order traversal of `RasterSnapshot.layers`'
//! structural nesting (`RasterLayerNode::Group.children: Vec<RasterLayerNode>` is a genuine tree,
//! owned by value). `topoOrder` is document order with every ancestor preceding its descendants
//! (already a valid topological order for a tree); `depth` is each layer's nesting depth from its
//! root (0 at the top level); `cycleFree` is always `true` — a Rust `Vec<Self>` embedded by value
//! cannot express a structural cycle, so this is a static invariant, not an empirical result;
//! `nodeCount` is the total flattened layer count (`Pixel`/`Group`/`Adjustment` all counted).

use crate::artifacts::raster::{RasterLayerNode, RasterSnapshot};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Topology
async fn layer_id(layer: &RasterLayerNode) -> &str {
    match layer {
        RasterLayerNode::Pixel { id, .. } | RasterLayerNode::Group { id, .. } | RasterLayerNode::Adjustment { id, .. } => id,
    }
}

async fn walk(layers: &[RasterLayerNode], level: u32, topo_order: &mut Vec<String>, depth: &mut BTreeMap<String, u32>) {
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RasterTopology {
    pub topo_order: Vec<String>,
    pub depth: BTreeMap<String, u32>,
    pub cycle_free: bool,
    pub node_count: u32,
}

/// 🧮️ Computes [`RasterTopology`] via a pre-order walk of `layers`' `Group.children` nesting.
pub async fn compute_raster_topology(snapshot: &RasterSnapshot) -> RasterTopology {
    let mut topo_order = Vec::new();
    let mut depth = BTreeMap::new();
    walk(&snapshot.layers, 0, &mut topo_order, &mut depth);
    let node_count = topo_order.len() as u32;
    RasterTopology { topo_order, depth, cycle_free: true, node_count }
}
//#endregion 🔖️Topology

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::raster::{RasterTransform, RASTER_DOCUMENT_SCHEMA};

    async fn pixel_layer(id: &str) -> RasterLayerNode {
        RasterLayerNode::Pixel { id: id.into(), name: id.into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, width: None, height: None, image_key: None }
    }

    async fn group_layer(id: &str, children: Vec<RasterLayerNode>) -> RasterLayerNode {
        RasterLayerNode::Group { id: id.into(), name: id.into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, children }
    }

    async fn snapshot(layers: Vec<RasterLayerNode>) -> RasterSnapshot {
        RasterSnapshot { schema: RASTER_DOCUMENT_SCHEMA.into(), id: "test".into(), title: None, layers, assets: Default::default() }
    }

    #[semio_framework_async_macros::async_test]
    async fn flat_layers_are_all_at_depth_zero() {
        let topology = compute_raster_topology(&snapshot(vec![pixel_layer("a"), pixel_layer("b")]));
        assert_eq!(topology.topo_order, vec!["a".to_string(), "b".to_string()]);
        assert_eq!(topology.depth.get("a"), Some(&0));
        assert_eq!(topology.depth.get("b"), Some(&0));
        assert_eq!(topology.node_count, 2);
        assert!(topology.cycle_free);
    }

    #[semio_framework_async_macros::async_test]
    async fn nested_group_children_get_incrementing_depth() {
        let topology = compute_raster_topology(&snapshot(vec![group_layer("g1", vec![pixel_layer("child"), group_layer("g2", vec![pixel_layer("grandchild")])])]));
        assert_eq!(topology.topo_order, vec!["g1".to_string(), "child".to_string(), "g2".to_string(), "grandchild".to_string()]);
        assert_eq!(topology.depth.get("g1"), Some(&0));
        assert_eq!(topology.depth.get("child"), Some(&1));
        assert_eq!(topology.depth.get("g2"), Some(&1));
        assert_eq!(topology.depth.get("grandchild"), Some(&2));
        assert_eq!(topology.node_count, 4);
        assert!(topology.cycle_free);
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_layers_produce_an_empty_topology() {
        let topology = compute_raster_topology(&snapshot(Vec::new()));
        assert!(topology.topo_order.is_empty());
        assert_eq!(topology.node_count, 0);
        assert!(topology.cycle_free);
    }
}
//#endregion 🧪️Tests
