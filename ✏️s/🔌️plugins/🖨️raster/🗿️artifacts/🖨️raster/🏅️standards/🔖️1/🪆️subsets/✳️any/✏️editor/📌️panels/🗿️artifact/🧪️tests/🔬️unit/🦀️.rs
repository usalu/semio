use super::*;

//#region 🪟️WindowLaws
use crate::RasterTransform;
use semio_framework_plugin::{TreeWindowRequest, ViewModel, TREE_WINDOW_DEFAULT_ROWS};

fn pixel_layer(id: &str, name: &str) -> RasterLayerNode {
    RasterLayerNode::Pixel { id: id.into(), name: name.into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, width: Some(64), height: Some(64), image_key: None }
}

/// 🪟️ A document an order of magnitude past one viewport, whose FIRST layer is a group holding
/// `nested` children — so every law below covers the nested container too, not only the section.
fn oversized_document(top: usize, nested: usize) -> RasterDocument {
    let children = (0..nested).map(|index| pixel_layer(&format!("nested-{index}"), &format!("Nested {index}"))).collect();
    let group = RasterLayerNode::Group { id: "group-0".into(), name: "Group 0".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, children };
    let mut layers = vec![group];
    layers.extend((1..top).map(|index| pixel_layer(&format!("pixel-{index}"), &format!("Pixel {index}"))));
    RasterDocument { layers, ..Default::default() }
}

/// 🪟️ The panel body exactly as the host reads it, for the host-known windows in `requests`.
fn window_body(document: &RasterDocument, requests: Vec<TreeWindowRequest>) -> String {
    let view = ViewModel { tree_windows: requests, ..Default::default() };
    let node = render(document, &RasterConfig::default(), &RasterPlayLabels::NATIVE_EN, &TreeWindows::for_body(&view, RASTER_PLAY_BODY_LAYERS)).expect("render the raster layer tree");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("project the raster layer tree")
}

fn open(node_key: &str, offset: u32, rows: u32) -> TreeWindowRequest {
    TreeWindowRequest { body_key: RASTER_PLAY_BODY_LAYERS.into(), node_key: node_key.into(), open: Some(true), offset, rows }
}

fn closed(node_key: &str) -> TreeWindowRequest {
    TreeWindowRequest { body_key: RASTER_PLAY_BODY_LAYERS.into(), node_key: node_key.into(), open: Some(false), offset: 0, rows: 0 }
}

/// 🔑️ A windowed container nested inside the section is addressed by its **window path** — the
/// enclosing windowed containers' keys, outermost first, then its own key, joined by
/// `TREE_WINDOW_PATH_SEPARATOR` (`TreeWindows::path_of`). A bare node key only ever matches a
/// top-level container, so the nested laws below file the path the host really sends.
fn nested_key(node_key: &str) -> String {
    format!("{RASTER_TREE_PREFIX}{}{node_key}", ui::TREE_WINDOW_PATH_SEPARATOR)
}

/// 🪟️ Law (a): the section AND the nested group stamp their FULL extent, materialise at most one
/// viewport between them, and never grow a `+N` continuation row.
#[test]
fn oversized_document_stamps_totals_and_never_a_continuation_row() {
    let json = window_body(&oversized_document(300, 40), Vec::new());
    assert!(json.contains("\"total\":302"), "the section stamps two add rows plus every layer: {json}");
    assert!(json.contains("\"total\":40"), "the nested group stamps its own full extent: {json}");
    assert!(!json.contains(".more"), "no continuation row survives: {json}");
    assert!(!json.contains("\"+"), "no `+N` label survives: {json}");
    assert!(json.matches("\"granularity\":\"layer\"").count() <= TREE_WINDOW_DEFAULT_ROWS as usize, "first paint materialises about one viewport: {json}");
}

/// 🪟️ Law (b): a container the host closed stamps its total and materialises nothing — at both levels.
#[test]
fn closed_containers_stamp_totals_and_materialise_no_children() {
    let document = oversized_document(300, 40);
    let json = window_body(&document, vec![closed(RASTER_TREE_PREFIX)]);
    assert!(json.contains("\"total\":302"), "a closed section still stamps its extent: {json}");
    assert!(!json.contains("raster-play-layers.add.pixel"), "a closed section materialises no rows: {json}");

    let nested = window_body(&document, vec![open(RASTER_TREE_PREFIX, 0, 4), closed(&nested_key("group-0"))]);
    assert!(nested.contains("\"total\":40"), "a closed group still stamps its extent: {nested}");
    assert!(!nested.contains("nested-0"), "a closed group materialises no children: {nested}");
}

/// 🪟️ Law (c): a host window materialises exactly `[offset, offset + rows)`, keyed by the raw row id —
/// asserted on the section and, independently, on the nested group container.
#[test]
fn host_windows_materialise_exactly_their_slice() {
    let document = oversized_document(300, 40);
    let json = window_body(&document, vec![open(RASTER_TREE_PREFIX, 102, 10)]);
    assert!(json.contains("\"offset\":102"), "the section reports its offset: {json}");
    for index in 100..110 {
        assert!(json.contains(&format!("pixel-{index}\"")), "row {index} is inside the window: {json}");
    }
    assert!(!json.contains("pixel-99\""), "the row before the window stays out: {json}");
    assert!(!json.contains("pixel-110\""), "the row after the window stays out: {json}");

    let nested = window_body(&document, vec![open(RASTER_TREE_PREFIX, 2, 1), open(&nested_key("group-0"), 12, 4)]);
    assert!(nested.contains("\"offset\":12"), "the nested group reports its offset: {nested}");
    for index in 12..16 {
        assert!(nested.contains(&format!("nested-{index}\"")), "nested child {index} is inside the window: {nested}");
    }
    assert!(!nested.contains("nested-11\""), "the nested child before the window stays out: {nested}");
    assert!(!nested.contains("nested-16\""), "the nested child after the window stays out: {nested}");
}

/// 🪟️ Law (d): the tree carries exactly ONE `interactionSelect` binding for the `"layers"` domain and
/// every layer row — nested rows included — is a pick target through its `granularity` alone, while the
/// fixed "add" rows keep their own `addLayer` action.
#[test]
fn domain_bound_rows_carry_granularity_and_one_tree_binding() {
    let json = window_body(&oversized_document(3, 2), Vec::new());
    assert!(json.contains("\"interactionDomain\":\"layers\""), "the tree binds the layers domain: {json}");
    assert_eq!(json.matches("interactionSelect").count(), 1, "exactly one tree-level pick binding exists: {json}");
    assert_eq!(json.matches("\"granularity\":\"layer\"").count(), 5, "every layer row, nested ones included, is a pick target: {json}");
    assert!(json.contains("addLayer"), "the add rows keep their own action: {json}");
}
//#endregion 🪟️WindowLaws
