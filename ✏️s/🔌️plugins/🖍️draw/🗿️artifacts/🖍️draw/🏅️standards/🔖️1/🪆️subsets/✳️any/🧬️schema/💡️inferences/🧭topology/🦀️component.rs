//! 🧭 `topology` — one named inference: a real pre-order traversal of `DrawSnapshot.layers`'
//! structural nesting (`DrawLayerNode::Group.children: Vec<DrawLayerNode>` is a genuine tree, owned
//! by value — not an id-reference graph like `sequence`'s step DAG). `topoOrder` is document order
//! with every ancestor preceding its descendants (already a valid topological order for a tree);
//! `depth` is each layer's nesting depth from its root (0 at the top level); `cycleFree` is always
//! `true` — a Rust `Vec<Self>` embedded by value cannot express a structural cycle, unlike an
//! id-reference graph, so this is a static invariant, not an empirical result; `nodeCount` is the
//! total flattened layer count. `DrawLayerNode::Boolean.children: Vec<String>` are id REFERENCES
//! (like an asset key), not structural nesting, so they are honestly excluded from this topology —
//! conflating the two would let a dangling/self reference fabricate a fake cycle in what is
//! otherwise a real tree invariant.

use crate::artifacts::draw::{DrawLayerNode, DrawSnapshot};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Topology
fn layer_id(layer: &DrawLayerNode) -> &str {
    match layer {
        DrawLayerNode::Shape(body) => &body.base.id,
        DrawLayerNode::Path(body) => &body.base.id,
        DrawLayerNode::Text(body) => &body.base.id,
        DrawLayerNode::Image(body) => &body.base.id,
        DrawLayerNode::Group(body) => &body.base.id,
        DrawLayerNode::Boolean(body) => &body.base.id,
        DrawLayerNode::Trace(body) => &body.base.id,
    }
}

fn walk(layers: &[DrawLayerNode], level: u32, topo_order: &mut Vec<String>, depth: &mut BTreeMap<String, u32>) {
    for layer in layers {
        let id = layer_id(layer).to_string();
        topo_order.push(id.clone());
        depth.insert(id, level);
        if let DrawLayerNode::Group(group) = layer {
            walk(&group.children, level + 1, topo_order, depth);
        }
    }
}

/// 🧭️ Draw's layer-tree topology — see module doc for the structural-nesting derivation.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawTopology {
    pub topo_order: Vec<String>,
    pub depth: BTreeMap<String, u32>,
    pub cycle_free: bool,
    pub node_count: u32,
}

/// 🧮️ Computes [`DrawTopology`] via a pre-order walk of `layers`' `Group.children` nesting.
pub fn compute_draw_topology(snapshot: &DrawSnapshot) -> DrawTopology {
    let mut topo_order = Vec::new();
    let mut depth = BTreeMap::new();
    walk(&snapshot.layers, 0, &mut topo_order, &mut depth);
    let node_count = topo_order.len() as u32;
    DrawTopology { topo_order, depth, cycle_free: true, node_count }
}
//#endregion 🔖️Topology

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::draw::{DrawAttributes, DrawGroupBody, DrawLayerBase, DrawPathBody};

    fn base(id: &str) -> DrawLayerBase {
        DrawLayerBase { id: id.into(), name: id.into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: crate::artifacts::draw::default_draw_transform(), attributes: DrawAttributes::default() }
    }

    fn path_layer(id: &str) -> DrawLayerNode {
        DrawLayerNode::Path(DrawPathBody { base: base(id), segments: Vec::new() })
    }

    fn group_layer(id: &str, children: Vec<DrawLayerNode>) -> DrawLayerNode {
        DrawLayerNode::Group(DrawGroupBody { base: base(id), children })
    }

    #[semio_framework_async_macros::async_test]
    async fn flat_layers_are_all_at_depth_zero() {
        let snapshot = DrawSnapshot { layers: vec![path_layer("a"), path_layer("b")], ..DrawSnapshot::default() };
        let topology = compute_draw_topology(&snapshot);
        assert_eq!(topology.topo_order, vec!["a".to_string(), "b".to_string()]);
        assert_eq!(topology.depth.get("a"), Some(&0));
        assert_eq!(topology.depth.get("b"), Some(&0));
        assert_eq!(topology.node_count, 2);
        assert!(topology.cycle_free);
    }

    #[semio_framework_async_macros::async_test]
    async fn nested_group_children_get_incrementing_depth_and_precede_nothing_before_their_parent() {
        let snapshot = DrawSnapshot { layers: vec![group_layer("g1", vec![path_layer("child"), group_layer("g2", vec![path_layer("grandchild")])])], ..DrawSnapshot::default() };
        let topology = compute_draw_topology(&snapshot);
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
        let topology = compute_draw_topology(&DrawSnapshot { layers: Vec::new(), ..DrawSnapshot::default() });
        assert!(topology.topo_order.is_empty());
        assert_eq!(topology.node_count, 0);
        assert!(topology.cycle_free);
    }
}
//#endregion 🧪️Tests
