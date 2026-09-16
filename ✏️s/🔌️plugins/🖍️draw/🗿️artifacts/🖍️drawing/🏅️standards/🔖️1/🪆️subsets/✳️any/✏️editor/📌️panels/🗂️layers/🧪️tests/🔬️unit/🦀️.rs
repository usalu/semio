use super::*;

//#region 🪟️WindowLaws
use crate::schema::default_layer_base;
use crate::{DrawingGroupBody, DrawingLayerBase, DrawingShapeBody};
use semio_framework_plugin::{TreeWindowRequest, ViewModel, TREE_WINDOW_DEFAULT_ROWS};

fn shape_layer(id: &str, name: &str) -> DrawingLayerNode {
    DrawingLayerNode::Shape(DrawingShapeBody { base: DrawingLayerBase { id: id.to_string(), name: name.to_string(), ..default_layer_base(name) }, shape_kind: "rect".into(), rect: None, ellipse: None, circle: None, line: None, polygon: None })
}

/// 🪟️ A document an order of magnitude past one viewport, whose FIRST layer is a group holding
/// `nested` children — so every law below covers the nested container too, not only the section.
fn oversized_document(top: usize, nested: usize) -> DrawingSnapshot {
    let children = (0..nested).map(|index| shape_layer(&format!("nested-{index}"), &format!("Nested {index}"))).collect();
    let group = DrawingLayerNode::Group(DrawingGroupBody { base: DrawingLayerBase { id: "group-0".into(), name: "Group 0".into(), ..default_layer_base("Group 0") }, children });
    let mut layers = vec![group];
    layers.extend((1..top).map(|index| shape_layer(&format!("shape-{index}"), &format!("Shape {index}"))));
    DrawingSnapshot { layers, ..Default::default() }
}

/// 🪟️ The panel body exactly as the host reads it, for the host-known windows in `requests`.
fn window_body(document: &DrawingSnapshot, requests: Vec<TreeWindowRequest>) -> String {
    let view = ViewModel { tree_windows: requests, ..Default::default() };
    let node = render(document, &DrawingPlayLabels::NATIVE_EN, &TreeWindows::for_body(&view, DRAWING_PLAY_BODY_LAYERS)).expect("render the drawing layer tree");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("project the drawing layer tree")
}

fn open(node_key: &str, offset: u32, rows: u32) -> TreeWindowRequest {
    TreeWindowRequest { body_key: DRAWING_PLAY_BODY_LAYERS.into(), node_key: node_key.into(), open: Some(true), offset, rows }
}

/// 🪟️ Law (a): the section AND the nested group stamp their FULL extent, materialise at most one
/// viewport between them, and never grow a `+N` continuation row.
#[test]
fn oversized_document_stamps_totals_and_never_a_continuation_row() {
    let json = window_body(&oversized_document(300, 40), Vec::new());
    assert!(json.contains("\"total\":305"), "the section stamps five add rows plus every layer: {json}");
    assert!(json.contains("\"total\":40"), "the nested group stamps its own full extent: {json}");
    assert!(!json.contains(".more"), "no continuation row survives: {json}");
    assert!(!json.contains("\"+"), "no `+N` label survives: {json}");
    assert!(json.matches("drawing-play-layers.shape.").count() <= TREE_WINDOW_DEFAULT_ROWS as usize, "first paint materialises about one viewport: {json}");
}

/// 🪟️ Law (b): a container the host closed stamps its total and materialises nothing — at both levels.
#[test]
fn closed_containers_stamp_totals_and_materialise_no_children() {
    let document = oversized_document(300, 40);
    let json = window_body(&document, vec![TreeWindowRequest { body_key: DRAWING_PLAY_BODY_LAYERS.into(), node_key: "drawing-play-layers".into(), open: Some(false), offset: 0, rows: 0 }]);
    assert!(json.contains("\"total\":305"), "a closed section still stamps its extent: {json}");
    assert!(!json.contains("drawing-play-layers.add.path"), "a closed section materialises no rows: {json}");

    let nested = window_body(&document, vec![open("drawing-play-layers", 0, 8), TreeWindowRequest { body_key: DRAWING_PLAY_BODY_LAYERS.into(), node_key: "drawing-play-layers.group.group-0".into(), open: Some(false), offset: 0, rows: 0 }]);
    assert!(nested.contains("\"total\":40"), "a closed group still stamps its extent: {nested}");
    assert!(!nested.contains("drawing-play-layers.shape.nested-0"), "a closed group materialises no children: {nested}");
}

/// 🪟️ Law (c): a host window materialises exactly `[offset, offset + rows)`, keyed by the raw row id —
/// asserted on the section and, independently, on the nested group container.
#[test]
fn host_windows_materialise_exactly_their_slice() {
    let document = oversized_document(300, 40);
    let json = window_body(&document, vec![open("drawing-play-layers", 105, 10)]);
    assert!(json.contains("\"offset\":105"), "the section reports its offset: {json}");
    for index in 100..110 {
        assert!(json.contains(&format!("drawing-play-layers.shape.shape-{index}\"")), "row {index} is inside the window: {json}");
    }
    assert!(!json.contains("drawing-play-layers.shape.shape-99\""), "the row before the window stays out: {json}");
    assert!(!json.contains("drawing-play-layers.shape.shape-110\""), "the row after the window stays out: {json}");

    let nested = window_body(&document, vec![open("drawing-play-layers", 5, 1), open("drawing-play-layers.group.group-0", 12, 4)]);
    assert!(nested.contains("\"offset\":12"), "the nested group reports its offset: {nested}");
    for index in 12..16 {
        assert!(nested.contains(&format!("drawing-play-layers.shape.nested-{index}\"")), "nested child {index} is inside the window: {nested}");
    }
    assert!(!nested.contains("drawing-play-layers.shape.nested-11\""), "the nested child before the window stays out: {nested}");
    assert!(!nested.contains("drawing-play-layers.shape.nested-16\""), "the nested child after the window stays out: {nested}");
}

/// 🪟️ Law (d): the tree carries exactly ONE `interactionSelect` binding for the `"strokes"` domain and
/// every layer row — nested rows included — is a pick target through its `granularity` alone, while the
/// fixed "add" rows keep their own `addLayer` action.
#[test]
fn domain_bound_rows_carry_granularity_and_one_tree_binding() {
    let json = window_body(&oversized_document(3, 2), Vec::new());
    assert!(json.contains("\"interactionDomain\":\"strokes\""), "the tree binds the strokes domain: {json}");
    assert_eq!(json.matches("interactionSelect").count(), 1, "exactly one tree-level pick binding exists: {json}");
    assert_eq!(json.matches("\"granularity\":\"stroke\"").count(), 5, "every layer row, nested ones included, is a pick target: {json}");
    assert!(json.contains("addLayer"), "the add rows keep their own action: {json}");
}
//#endregion 🪟️WindowLaws
