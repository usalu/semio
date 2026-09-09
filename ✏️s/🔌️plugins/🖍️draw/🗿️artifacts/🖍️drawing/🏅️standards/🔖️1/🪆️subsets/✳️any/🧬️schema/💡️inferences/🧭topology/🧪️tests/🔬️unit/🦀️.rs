use super::*;
use crate::{DrawingAttributes, DrawingGroupBody, DrawingLayerBase, DrawingPathBody};

fn base(id: &str) -> DrawingLayerBase {
    DrawingLayerBase { id: id.into(), name: id.into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: crate::default_drawing_transform(), attributes: DrawingAttributes::default() }
}

fn path_layer(id: &str) -> DrawingLayerNode {
    DrawingLayerNode::Path(DrawingPathBody { base: base(id), segments: Vec::new() })
}

fn group_layer(id: &str, children: Vec<DrawingLayerNode>) -> DrawingLayerNode {
    DrawingLayerNode::Group(DrawingGroupBody { base: base(id), children })
}

#[semio_framework_async_macros::async_test]
async fn flat_layers_are_all_at_depth_zero() {
    let snapshot = DrawingSnapshot { layers: vec![path_layer("a"), path_layer("b")], ..DrawingSnapshot::default() };
    let topology = compute_drawing_topology(&snapshot);
    assert_eq!(topology.topo_order, vec!["a".to_string(), "b".to_string()]);
    assert_eq!(topology.depth.get("a"), Some(&0));
    assert_eq!(topology.depth.get("b"), Some(&0));
    assert_eq!(topology.node_count, 2);
    assert!(topology.cycle_free);
}

#[semio_framework_async_macros::async_test]
async fn nested_group_children_get_incrementing_depth_and_precede_nothing_before_their_parent() {
    let snapshot = DrawingSnapshot { layers: vec![group_layer("g1", vec![path_layer("child"), group_layer("g2", vec![path_layer("grandchild")])])], ..DrawingSnapshot::default() };
    let topology = compute_drawing_topology(&snapshot);
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
    let topology = compute_drawing_topology(&DrawingSnapshot { layers: Vec::new(), ..DrawingSnapshot::default() });
    assert!(topology.topo_order.is_empty());
    assert_eq!(topology.node_count, 0);
    assert!(topology.cycle_free);
}
