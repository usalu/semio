use super::*;
use crate::editor::block3d::terminology::block3d_labels;
use crate::editor::block3d::unit_tests::context::{new_app, render as render_body};
use semio_framework_plugin::{TreeWindowRequest, TreeWindows, ViewModel};

#[semio_framework_async_macros::async_test]
async fn renders_document_tree() {
    let mut app = new_app().await;
    assert!(render_body(&mut app, BLOCK3D_BODY_ARTIFACT).await.contains("Representations"));
}

//#region 🪟️WindowLaws
/// 🔎️ `(total, offset, row keys)` of one container, read off a rendered/projected body.
fn container_of(json: &str, key: &str) -> (u64, u64, Vec<String>) {
    fn walk(node: &serde_json::Value, key: &str) -> Option<serde_json::Value> {
        if node["key"].as_str() == Some(key) {
            return Some(node.clone());
        }
        node["children"].as_array().and_then(|children| children.iter().find_map(|child| walk(child, key)))
    }
    let projection: serde_json::Value = serde_json::from_str(json).expect("document projection json");
    let node = walk(&projection, key).unwrap_or_else(|| panic!("{key} is not in the rendered document: {json}"));
    let window = node["component"]["window"].as_object().unwrap_or_else(|| panic!("{key} stamps no window: {json}"));
    (
        window.get("total").and_then(serde_json::Value::as_u64).unwrap_or_default(),
        window.get("offset").and_then(serde_json::Value::as_u64).unwrap_or_default(),
        node["children"].as_array().cloned().unwrap_or_default().iter().map(|row| row["key"].as_str().unwrap_or_default().to_string()).collect(),
    )
}

fn project(document: &Block3dSnapshot, windows: &TreeWindows<'_>) -> String {
    let labels = block3d_labels(&ViewModel::default());
    let node = render(document, labels, windows).expect("document tree renders");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("document projection")
}

fn window_view(node_key: &str, open: Option<bool>, offset: u32, rows: u32) -> ViewModel {
    ViewModel { tree_windows: vec![TreeWindowRequest { body_key: BLOCK3D_BODY_ARTIFACT.into(), node_key: node_key.into(), open, offset, rows }], ..Default::default() }
}

/// 🌾️ A document an order of magnitude past any viewport: 40 representations and 160 rim vortices.
fn oversized_document() -> Block3dSnapshot {
    let mut document = Block3dSnapshot::default();
    document.representations = (0..40)
        .map(|index| crate::BlockRepresentation {
            id: format!("rep-{index:03}"),
            name: format!("Representation {index}"),
            mesh_url: Some(format!("mesh://rep-{index:03}")),
            tags: Vec::new(),
            lod: None,
            description: String::new(),
            attributes: Vec::new(),
        })
        .collect();
    document.vortices = (0..160)
        .map(|index| crate::Block3dVortexTemplate {
            id: format!("vortex-{index:03}"),
            vortex_kind: format!("kind-{:03}", index % 40),
            position: [0.0, 0.0, 0.0],
            direction: [0.0, 0.0, 1.0],
            radius: 1.0,
            label: None,
        })
        .collect();
    document
}

/// ⚖️ LAW (a)/(d): an oversized document states each section's full extent, materialises at most that
/// slice, and never summarises a remainder as a `+n`.
#[test]
fn an_oversized_document_stamps_every_sections_total() {
    let document = oversized_document();
    let json = project(&document, &TreeWindows::unhosted());
    let (kinds_total, kinds_offset, kinds_rows) = container_of(&json, BLOCK3D_DOCUMENT_REPRESENTATIONS);
    assert_eq!(kinds_total as usize, 40, "the first section stamps its whole extent: {json}");
    assert_eq!(kinds_offset, 0, "a first paint starts at zero");
    assert!(kinds_rows.len() <= 40, "the first section materialises at most its slice");
    let (items_total, _, items_rows) = container_of(&json, BLOCK3D_DOCUMENT_VORTICES);
    assert_eq!(items_total as usize, 160, "the second section stamps its whole extent: {json}");
    assert!(items_rows.len() <= 160, "the second section materialises at most its slice");
    assert!(!json.contains(".more\""), "a windowed document has no continuation row");
    assert!(!json.contains(r#""label":"+"#), "a windowed document publishes no `+n` label");
}

/// ⚖️ LAW (b): a closed section states its extent and materialises nothing.
#[test]
fn a_closed_document_section_stamps_its_total_with_no_rows() {
    let document = oversized_document();
    let view = window_view(BLOCK3D_DOCUMENT_VORTICES, Some(false), 0, 0);
    let json = project(&document, &TreeWindows::for_body(&view, BLOCK3D_BODY_ARTIFACT));
    let (total, offset, rows) = container_of(&json, BLOCK3D_DOCUMENT_VORTICES);
    assert_eq!(total as usize, 160, "a closed section still states its extent: {json}");
    assert_eq!(offset, 0, "a closed section starts at zero");
    assert!(rows.is_empty(), "a closed section materialises nothing: {json}");
}

/// ⚖️ LAW (c): a host window request materialises exactly `[offset, offset + rows)`, keyed by the raw
/// canonical domain target id.
#[test]
fn a_document_window_request_materialises_exactly_its_slice() {
    let document = oversized_document();
    let view = window_view(BLOCK3D_DOCUMENT_VORTICES, Some(true), 90, 7);
    let json = project(&document, &TreeWindows::for_body(&view, BLOCK3D_BODY_ARTIFACT));
    let (total, offset, rows) = container_of(&json, BLOCK3D_DOCUMENT_VORTICES);
    assert_eq!(total as usize, 160);
    assert_eq!(offset, 90, "the stamped offset is the requested one: {json}");
    let expected: Vec<String> = (90..97).map(|index| format!("vortex:vortex-{index:03}")).collect();
    assert_eq!(rows, expected, "exactly entries [90, 97) are materialised: {json}");
}

/// ⚖️ LAW (d): every row is a domain pick target that costs no argument arena — it declares its
/// `granularity` and the TREE carries the one `interactionSelect` binding.
#[test]
fn document_rows_declare_their_granularity_and_carry_no_row_binding() {
    let document = oversized_document();
    let json = project(&document, &TreeWindows::unhosted());
    let projection: serde_json::Value = serde_json::from_str(&json).expect("document projection json");
    assert_eq!(projection["component"]["interactionDomain"].as_str(), Some(BLOCK3D_INTERACTION_VORTEX), "{json}");
    assert_eq!(projection["bindings"].as_array().cloned().unwrap_or_default().iter().filter(|binding| binding["trigger"] == "activate").count(), 1, "exactly one tree-level interactionSelect: {json}");
    for section in projection["children"].as_array().cloned().unwrap_or_default() {
        for row in section["children"].as_array().cloned().unwrap_or_default() {
            assert!(row["component"]["granularity"].as_str().is_some(), "{row}");
            assert!(row["bindings"].as_array().map(|bindings| bindings.is_empty()).unwrap_or(true), "a pick row carries no binding of its own: {row}");
        }
    }
}
//#endregion 🪟️WindowLaws
