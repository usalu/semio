
use super::*;
use crate::{RASTER_DOCUMENT_SCHEMA, RasterTransform};

fn pixel_layer(id: &str) -> RasterLayerNode {
    RasterLayerNode::Pixel { id: id.into(), name: id.into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, width: None, height: None, image_key: None }
}

fn group_layer(id: &str, children: Vec<RasterLayerNode>) -> RasterLayerNode {
    RasterLayerNode::Group { id: id.into(), name: id.into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, children }
}

fn snapshot(layers: Vec<RasterLayerNode>) -> RasterSnapshot {
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
