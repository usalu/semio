
use super::*;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioTransform};
use crate::standards::v1::subsets::drawing::schema::diff::NodePath;
use crate::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer, DrawNode, DrawStyle, PathSegment, STDIO_SEMIODRAWING_DOCUMENT_SCHEMA};
use protocol::{Mutation, MutationDiff, SemanticMutation};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn fixture() -> SemioDrawingSnapshot {
    SemioDrawingSnapshot {
        schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
        canvas: DrawCanvas { width: 10.0, height: 10.0, background: None },
        styles: vec![DrawStyle { name: "s1".into(), fill: Some(crate::standards::v1::subsets::base::schema::geometry::SemioRgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }), stroke: None, stroke_width: Some(1.0), opacity: None }],
        layers: vec![DrawLayer {
            id: "l0".into(),
            name: "base".into(),
            visible: true,
            root: DrawNode::Group {
                transform: SemioTransform::identity(),
                children: vec![
                    DrawNode::Text { value: "hi".into(), at: SemioPoint2 { x: 0.0, y: 0.0 }, style: None },
                    DrawNode::Path { segments: vec![PathSegment::MoveTo { to: SemioPoint2 { x: 0.0, y: 0.0 } }, PathSegment::LineTo { to: SemioPoint2 { x: 1.0, y: 1.0 } }], style: Some("s1".into()) },
                    DrawNode::Group { transform: SemioTransform::identity(), children: vec![DrawNode::Text { value: "nested".into(), at: SemioPoint2 { x: 2.0, y: 2.0 }, style: None }] },
                ],
            },
        }],
    }
}

/// 🔧️ Diffs/inverses each step against the CURRENT (evolving) state, never the stale
/// pre-operation `base` — the din4108 harness's own documented bug, deliberately NOT copied
/// here (this ticket's binding instruction).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn round_trip(base: &SemioDrawingSnapshot, operation: &SemioDrawingMutation) -> SemioDrawingSnapshot {
    let forward = operation.diff(base).diff().apply(base).expect("apply must succeed for a well-formed fixture");
    let backwards = operation.inverse(base);
    let mut restored = forward.clone();
    for back in &backwards {
        restored = back.diff(&restored).diff().apply(&restored).expect("apply must succeed for a well-formed fixture");
    }
    assert_eq!(&restored, base, "inverse must exactly restore the pre-operation fixture for {operation:?}");
    forward
}

#[semio_framework_async_macros::async_test]
async fn every_demo_variant_round_trips() {
    let base = fixture();
    for m in demo_mutation_cases() {
        let _ = round_trip(&base, &m);
    }
}

#[semio_framework_async_macros::async_test]
async fn create_delete_layer_round_trip() {
    let base = fixture();
    let new_layer = DrawLayer { id: "l1".into(), name: "new".into(), visible: true, root: DrawNode::default() };
    let create = SemioDrawingMutation::CreateLayer(create_layer::CreateLayer { index: 1, layer: new_layer.clone() });
    let after_create = round_trip(&base, &create);
    assert_eq!(after_create.layers.len(), base.layers.len() + 1);
    assert_eq!(after_create.layers[1], new_layer);

    let delete = SemioDrawingMutation::DeleteLayer(delete_layer::DeleteLayer { id: "l0".into() });
    let after_delete = round_trip(&base, &delete);
    assert!(after_delete.layers.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn delete_layer_of_an_absent_id_has_an_empty_inverse() {
    let base = fixture();
    let delete = SemioDrawingMutation::DeleteLayer(delete_layer::DeleteLayer { id: "missing".into() });
    assert!(delete.inverse(&base).is_empty());
    assert_eq!(delete.diff(&base).diff().apply(&base).expect("apply must succeed for a well-formed fixture"), base);
}

#[semio_framework_async_macros::async_test]
async fn create_delete_node_round_trip() {
    let base = fixture();
    let root_path = NodePath { layer: 0, path: vec![] };
    let new_node = DrawNode::Text { value: "created".into(), at: SemioPoint2 { x: 9.0, y: 9.0 }, style: None };
    let create = SemioDrawingMutation::CreateNode(create_node::CreateNode { parent: root_path.clone(), index: 0, node: new_node.clone() });
    let after_create = round_trip(&base, &create);
    let DrawNode::Group { children, .. } = &after_create.layers[0].root else { panic!("expected group root") };
    assert_eq!(children.len(), 4);
    assert_eq!(children[0], new_node);

    let delete = SemioDrawingMutation::DeleteNode(delete_node::DeleteNode { at: NodePath { layer: 0, path: vec![0] } });
    let _ = round_trip(&base, &delete);
}

#[semio_framework_async_macros::async_test]
async fn move_and_drag_nodes_round_trip() {
    let base = fixture();
    let text_path = NodePath { layer: 0, path: vec![0] };
    let m = SemioDrawingMutation::MoveNode(move_node::MoveNode { at: text_path.clone(), new_origin: SemioPoint2 { x: 7.0, y: 7.0 } });
    let after = round_trip(&base, &m);
    let DrawNode::Group { children, .. } = &after.layers[0].root else { panic!() };
    let DrawNode::Text { at, .. } = &children[0] else { panic!() };
    assert_eq!(*at, SemioPoint2 { x: 7.0, y: 7.0 });

    let path_path = NodePath { layer: 0, path: vec![1] };
    let no_op_move = SemioDrawingMutation::MoveNode(move_node::MoveNode { at: path_path, new_origin: SemioPoint2 { x: 1.0, y: 1.0 } });
    assert!(no_op_move.inverse(&base).is_empty(), "Path has no origin field; move must be a no-op");
    assert_eq!(no_op_move.diff(&base).diff().apply(&base).expect("apply must succeed for a well-formed fixture"), base);

    let drag = SemioDrawingMutation::DragNodes(drag_nodes::DragNodes { ats: vec![text_path], offset: SemioPoint2 { x: 3.0, y: -2.0 } });
    let _ = round_trip(&base, &drag);
}

#[semio_framework_async_macros::async_test]
async fn rotate_and_scale_round_trip() {
    let base = fixture();
    let root_path = NodePath { layer: 0, path: vec![] };
    let rotate_m = SemioDrawingMutation::RotateNode(rotate_node::RotateNode { at: root_path.clone(), new_rotation: crate::standards::v1::subsets::base::schema::geometry::SemioQuaternion { x: 0.0, y: 0.0, z: 1.0, w: 0.0 } });
    let _ = round_trip(&base, &rotate_m);
    let scale_m = SemioDrawingMutation::ScaleNode(scale_node::ScaleNode { at: root_path, new_scale: crate::standards::v1::subsets::base::schema::geometry::SemioPoint3 { x: 2.0, y: 2.0, z: 1.0 } });
    let _ = round_trip(&base, &scale_m);
}

#[semio_framework_async_macros::async_test]
async fn reorder_nodes_round_trips() {
    let base = fixture();
    let root_path = NodePath { layer: 0, path: vec![] };
    let m = SemioDrawingMutation::ReorderNodes(reorder_nodes::ReorderNodes { parent: root_path, from: 0, to: 2 });
    let after = round_trip(&base, &m);
    let DrawNode::Group { children: base_children, .. } = &base.layers[0].root else { panic!() };
    let DrawNode::Group { children: after_children, .. } = &after.layers[0].root else { panic!() };
    assert_eq!(after_children[2], base_children[0]);
}

#[semio_framework_async_macros::async_test]
async fn group_and_ungroup_round_trip() {
    let base = fixture();
    let root_path = NodePath { layer: 0, path: vec![] };
    let group_m = SemioDrawingMutation::GroupNodes(group_nodes::GroupNodes { parent: root_path.clone(), indices: vec![0, 1], transform: SemioTransform::identity() });
    let after_group = round_trip(&base, &group_m);
    let DrawNode::Group { children, .. } = &after_group.layers[0].root else { panic!() };
    assert_eq!(children.len(), 2, "two nodes wrapped into one new group, one nested group left alone");
    let DrawNode::Group { children: inner, .. } = &children[0] else { panic!("expected the new wrapper group at #0") };
    assert_eq!(inner.len(), 2);

    let ungroup_m = SemioDrawingMutation::UngroupNode(ungroup_node::UngroupNode { at: NodePath { layer: 0, path: vec![2] } });
    let _ = round_trip(&base, &ungroup_m);
}

#[semio_framework_async_macros::async_test]
async fn group_refuses_non_contiguous_indices() {
    let base = fixture();
    let root_path = NodePath { layer: 0, path: vec![] };
    let m = SemioDrawingMutation::GroupNodes(group_nodes::GroupNodes { parent: root_path, indices: vec![0, 2], transform: SemioTransform::identity() });
    assert!(m.inverse(&base).is_empty());
    assert_eq!(m.diff(&base).diff().apply(&base).expect("apply must succeed for a well-formed fixture"), base);
}

#[semio_framework_async_macros::async_test]
async fn flatten_and_unflatten_round_trip() {
    let base = fixture();
    let root_path = NodePath { layer: 0, path: vec![] };
    let flatten_m = SemioDrawingMutation::FlattenNode(flatten_node::FlattenNode { at: root_path.clone() });
    let after = round_trip(&base, &flatten_m);
    let DrawNode::Group { children, .. } = &after.layers[0].root else { panic!() };
    assert_eq!(children.len(), 3, "the nested identity group's one child is promoted, no group wrapper left");
    assert!(children.iter().all(|c| !matches!(c, DrawNode::Group { .. })), "no descendant groups remain");
}

#[semio_framework_async_macros::async_test]
async fn flatten_refuses_a_non_identity_descendant_group() {
    let mut base = fixture();
    let DrawNode::Group { children, .. } = &mut base.layers[0].root else { panic!() };
    let DrawNode::Group { transform, .. } = &mut children[2] else { panic!() };
    transform.translation.x = 5.0;
    let root_path = NodePath { layer: 0, path: vec![] };
    let m = SemioDrawingMutation::FlattenNode(flatten_node::FlattenNode { at: root_path });
    assert!(m.inverse(&base).is_empty());
    assert_eq!(m.diff(&base).diff().apply(&base).expect("apply must succeed for a well-formed fixture"), base);
}

#[semio_framework_async_macros::async_test]
async fn replace_path_round_trips() {
    let base = fixture();
    let m = SemioDrawingMutation::ReplacePath(replace_path::ReplacePath { at: NodePath { layer: 0, path: vec![1] }, new_segments: vec![PathSegment::Close] });
    let after = round_trip(&base, &m);
    let DrawNode::Group { children, .. } = &after.layers[0].root else { panic!() };
    let DrawNode::Path { segments, .. } = &children[1] else { panic!() };
    assert_eq!(segments, &vec![PathSegment::Close]);
}

#[semio_framework_async_macros::async_test]
async fn replace_fill_and_change_stroke_round_trip() {
    let base = fixture();
    let fill_m = SemioDrawingMutation::ReplaceFill(replace_fill::ReplaceFill { style_name: "s1".into(), new_fill: None });
    let after_fill = round_trip(&base, &fill_m);
    assert_eq!(after_fill.styles[0].fill, None);

    let color_m = SemioDrawingMutation::ChangeStrokeColor(change_stroke_color::ChangeStrokeColor { style_name: "s1".into(), new_color: Some(crate::standards::v1::subsets::base::schema::geometry::SemioRgba { r: 0.0, g: 1.0, b: 0.0, a: 1.0 }) });
    let after_color = round_trip(&base, &color_m);
    assert_eq!(after_color.styles[0].stroke, Some(crate::standards::v1::subsets::base::schema::geometry::SemioRgba { r: 0.0, g: 1.0, b: 0.0, a: 1.0 }));

    let width_m = SemioDrawingMutation::ChangeStrokeWidth(change_stroke_width::ChangeStrokeWidth { style_name: "s1".into(), new_width: Some(5.0) });
    let after_width = round_trip(&base, &width_m);
    assert_eq!(after_width.styles[0].stroke_width, Some(5.0));

    let absent_m = SemioDrawingMutation::ChangeStrokeWidth(change_stroke_width::ChangeStrokeWidth { style_name: "missing".into(), new_width: Some(1.0) });
    assert!(absent_m.inverse(&base).is_empty());
    assert_eq!(absent_m.diff(&base).diff().apply(&base).expect("apply must succeed for a well-formed fixture"), base);
}

#[semio_framework_async_macros::async_test]
async fn semantic_kinds_cover_every_declared_variant() {
    assert_eq!(SemioDrawingMutation::kinds().len(), 17);
    let mutation = SemioDrawingMutation::DeleteLayer(delete_layer::DeleteLayer { id: "l0".into() });
    assert_eq!(mutation.semantics().kind, "delete-layer");
    assert_eq!(mutation.semantics().record, "DeletedLayer");
    assert_eq!(mutation.target(), vec!["l0".to_string()]);
}

/// 🏷️ `KINDS` (this facet's own const, consumed by `mutate-semio-drawing`'s adapter) must name
/// every declared variant, in the exact order and spelling `#[derive(dsl::Mutations)]` assigns,
/// and every one of them must appear in the committed oracle manifest's catalog — the framework
/// never parses Rust, so this is what keeps the declaration honest in both directions.
#[test]
fn kinds_match_the_enum_and_the_catalog() {
    let descriptors = SemioDrawingMutation::kinds();
    assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared variant");
    for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
        assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
    }
    let manifest = include_str!("../../../../🔮️oracle/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
}
