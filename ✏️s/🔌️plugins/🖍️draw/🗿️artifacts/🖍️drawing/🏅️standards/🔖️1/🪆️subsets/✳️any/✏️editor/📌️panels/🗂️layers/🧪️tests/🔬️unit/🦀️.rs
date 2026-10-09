use super::*;

//#region 🪟️WindowLaws
use crate::schema::default_layer_base;
use crate::{DrawingGroupBody, DrawingLayerBase, DrawingShapeBody};
use semio_framework_plugin::{TreeWindowRequest, ViewModel, TREE_WINDOW_DEFAULT_ROWS};

fn shape_layer(id: &str, name: &str) -> DrawingLayerNode {
    DrawingLayerNode::Shape(DrawingShapeBody { base: DrawingLayerBase { id: id.into(), name: name.into(), ..default_layer_base(crate::schema::identity::DrawingIdentity::admit(((name)).to_string().into()).expect("nonempty authored identity"), name) }, shape_kind: "rect".into(), rect: None, ellipse: None, circle: None, line: None, polygon: None })
}

/// 🪟️ A document an order of magnitude past one viewport, whose FIRST layer is a group holding
/// `nested` children — so every law below covers the nested container too, not only the section.
fn oversized_document(top: usize, nested: usize) -> DrawingSnapshot {
    let children = (0..nested).map(|index| shape_layer(&format!("nested-{index}"), &format!("Nested {index}"))).collect();
    let group = DrawingLayerNode::Group(DrawingGroupBody { isolation:false, base: DrawingLayerBase { id: "group-0".into(), name: "Group 0".into(), ..default_layer_base(crate::schema::identity::DrawingIdentity::admit((("Group 0")).to_string().into()).expect("nonempty authored identity"), "Group 0") }, children });
    let mut layers = vec![group];
    layers.extend((1..top).map(|index| shape_layer(&format!("shape-{index}"), &format!("Shape {index}"))));
    DrawingSnapshot { layers:layers.into_iter().collect(), ..Default::default() }
}

/// 🪟️ The panel body exactly as the host reads it, for the host-known windows in `requests`.
fn window_body(document: &DrawingSnapshot, requests: Vec<TreeWindowRequest>) -> String {
    let view = ViewModel { tree_windows: requests, ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let node = render(document, &DrawingPlayLabels::NATIVE_EN, &TreeWindows::for_body(&view, DRAWING_PLAY_BODY_LAYERS)).expect("render the drawing layer tree");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("project the drawing layer tree")
}

fn open(node_key: &str, offset: u32, rows: u32) -> TreeWindowRequest {
    TreeWindowRequest { body_key: DRAWING_PLAY_BODY_LAYERS.into(), node_key: node_key.into(), open: Some(true), offset, rows }
}

/// 🪟️ A container's window identity is its PATH — the enclosing windowed containers' keys, outermost
/// first, then its own key — so a nested group is addressed under the section that builds it.
fn nested_path(keys: &[&str]) -> String {
    keys.join(semio_framework_plugin::TREE_WINDOW_PATH_SEPARATOR)
}

/// 🪟️ Law (a): the section AND the nested group stamp their FULL extent, materialise at most one
/// viewport between them, and never grow a `+N` continuation row.
#[test]
fn oversized_document_stamps_totals_and_never_a_continuation_row() {
    let json = window_body(&oversized_document(300, 40), Vec::new());
    assert!(json.contains("\"total\":310"), "the section stamps ten add rows plus every layer: {json}");
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
    assert!(json.contains("\"total\":310"), "a closed section still stamps its extent: {json}");
    assert!(!json.contains("drawing-play-layers.add.path"), "a closed section materialises no rows: {json}");

    let nested = window_body(&document, vec![open("drawing-play-layers", 0, 12), TreeWindowRequest { body_key: DRAWING_PLAY_BODY_LAYERS.into(), node_key: nested_path(&["drawing-play-layers", "drawing-play-layers.group.group-0"]), open: Some(false), offset: 0, rows: 0 }]);
    assert!(nested.contains("\"total\":40"), "a closed group still stamps its extent: {nested}");
    assert!(!nested.contains("drawing-play-layers.shape.nested-0"), "a closed group materialises no children: {nested}");
}

/// 🪟️ Law (c): a host window materialises exactly `[offset, offset + rows)`, keyed by the raw row id —
/// asserted on the section and, independently, on the nested group container.
#[test]
fn host_windows_materialise_exactly_their_slice() {
    let document = oversized_document(300, 40);
    let json = window_body(&document, vec![open("drawing-play-layers", 110, 10)]);
    assert!(json.contains("\"offset\":110"), "the section reports its offset: {json}");
    for index in 100..110 {
        assert!(json.contains(&format!("drawing-play-layers.shape.shape-{index}\"")), "row {index} is inside the window: {json}");
    }
    assert!(!json.contains("drawing-play-layers.shape.shape-99\""), "the row before the window stays out: {json}");
    assert!(!json.contains("drawing-play-layers.shape.shape-110\""), "the row after the window stays out: {json}");

    let nested = window_body(&document, vec![open("drawing-play-layers", 10, 1), open(&nested_path(&["drawing-play-layers", "drawing-play-layers.group.group-0"]), 12, 4)]);
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

#[test]
fn creation_choices_match_language_neutral_contract() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/➕️creation/🔣️.json")).unwrap();
    for labels in [&DrawingPlayLabels::NATIVE_EN,&DrawingPlayLabels::NATIVE_DE] {
        let document=DrawingSnapshot::default();
        let kinds=layers_rows(&document,labels).into_iter().filter_map(|row|match row {LayersRow::Add(_,_,_,kind)=>Some(kind),_=>None}).collect::<Vec<_>>();
        let expected=fixture["kinds"].as_array().unwrap().iter().map(|kind|kind.as_str().unwrap()).collect::<Vec<_>>();
        assert_eq!(kinds,expected);
        let view=ViewModel {tree_windows:vec![open("drawing-play-layers",0,11)],..ViewModel::new(semio_framework_ui_locale::Locale::En,semio_framework_ui_locale::Terminology::Native)};
        let tree=render(&document,labels,&TreeWindows::for_body(&view,DRAWING_PLAY_BODY_LAYERS)).unwrap();
        let json=semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(tree)).unwrap();
        for label in [labels.add_ellipse,labels.add_line,labels.add_polygon,labels.add_image,labels.add_trace] {assert!(json.contains(label.as_str()));}
        assert_eq!(json.matches("addLayer").count(),10);
    }
}
