
use super::*;
use crate::editor::block3d::terminology::block3d_labels;
use crate::editor::block3d::unit_tests::context::{new_app, render as render_body};
use semio_framework_plugin::{TreeWindowRequest, TreeWindows, ViewModel};

#[semio_framework_async_macros::async_test]
async fn renders_inspector_fields() {
    let mut app = new_app().await;
    let json = render_body(&mut app, BLOCK3D_BODY_INSPECTOR).await;
    assert!(json.contains("\"type\":\"tree\""), "inspection body must be a tree like document");
    assert!(json.contains("Name"));
    assert!(json.contains("Vortices"));
    assert!(!json.contains("\"type\":\"stack\""), "inspection body must not be a free-form stack");
}

//#region 🪟️WindowLaws
/// 🔎️ `(total, offset, row keys)` of one container, read off a projected body.
fn container_of(json: &str, key: &str) -> (u64, u64, Vec<String>) {
    fn walk(node: &serde_json::Value, key: &str) -> Option<serde_json::Value> {
        if node["key"].as_str() == Some(key) {
            return Some(node.clone());
        }
        node["children"].as_array().and_then(|children| children.iter().find_map(|child| walk(child, key)))
    }
    let projection: serde_json::Value = serde_json::from_str(json).expect("inspector projection json");
    let node = walk(&projection, key).unwrap_or_else(|| panic!("{key} is not in the rendered inspector: {json}"));
    let window = node["component"]["window"].as_object().unwrap_or_else(|| panic!("{key} stamps no window: {json}"));
    (
        window.get("total").and_then(serde_json::Value::as_u64).unwrap_or_default(),
        window.get("offset").and_then(serde_json::Value::as_u64).unwrap_or_default(),
        node["children"].as_array().cloned().unwrap_or_default().iter().map(|row| row["key"].as_str().unwrap_or_default().to_string()).collect(),
    )
}

/// 🚚️ Reads through the retiring PROJECTION, never `serde_json::to_string` on a `BuiltNode`: a built
/// node's `BuiltChildren` only serialises through the retained page transport.
fn project(document: &Block3dSnapshot, windows: &TreeWindows<'_>) -> String {
    let labels = block3d_labels(&ViewModel::default());
    let node = render(document, None, labels, windows).expect("inspector renders");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("inspector projection")
}

fn window_view(open: Option<bool>, offset: u32, rows: u32) -> ViewModel {
    ViewModel { tree_windows: vec![TreeWindowRequest { body_key: BLOCK3D_BODY_INSPECTOR.into(), node_key: BLOCK3D_INSPECTOR_SUMMARY.into(), open, offset, rows }], ..Default::default() }
}

/// ⚖️ LAW (a): the summary section stamps its full field count, materialises at most that slice, and
/// never summarises a remainder as a `+n` / `…more` continuation row.
#[test]
fn the_inspector_summary_stamps_its_total_and_publishes_no_continuation_row() {
    let json = project(&Block3dSnapshot::default(), &TreeWindows::unhosted());
    let (total, offset, rows) = container_of(&json, BLOCK3D_INSPECTOR_SUMMARY);
    assert_eq!(total, 4, "name, label, representation picker and the vortex count: {json}");
    assert_eq!(offset, 0, "a first paint starts at zero");
    assert_eq!(rows.len(), 4, "a four-row section fits one viewport: {json}");
    assert!(!json.contains(".more\""), "a windowed inspector has no continuation row: {json}");
    assert!(!json.contains(r#""label":"+"#), "a windowed inspector publishes no `+n` label: {json}");
}

/// ⚖️ LAW (b): a closed section states its extent and materialises nothing.
#[test]
fn a_closed_inspector_summary_stamps_its_total_with_no_rows() {
    let view = window_view(Some(false), 0, 0);
    let json = project(&Block3dSnapshot::default(), &TreeWindows::for_body(&view, BLOCK3D_BODY_INSPECTOR));
    let (total, offset, rows) = container_of(&json, BLOCK3D_INSPECTOR_SUMMARY);
    assert_eq!(total, 4, "a closed section still states its extent: {json}");
    assert_eq!(offset, 0);
    assert!(rows.is_empty(), "a closed section materialises nothing: {json}");
    assert!(!json.contains("Vortices"), "…not even its read-only count: {json}");
}

/// ⚖️ LAW (c): a host window request materialises exactly `[offset, offset + rows)`.
#[test]
fn an_inspector_window_request_materialises_exactly_its_slice() {
    let view = window_view(Some(true), 2, 2);
    let json = project(&Block3dSnapshot::default(), &TreeWindows::for_body(&view, BLOCK3D_BODY_INSPECTOR));
    let (total, offset, rows) = container_of(&json, BLOCK3D_INSPECTOR_SUMMARY);
    assert_eq!(total, 4);
    assert_eq!(offset, 2, "the stamped offset is the requested one: {json}");
    assert_eq!(rows, vec!["block3d-play-inspector.representation-field".to_string(), "block3d-play-inspector.vortex-count".to_string()], "exactly fields [2, 4): {json}");
}

/// ⚖️ LAW (d), as it applies to an inspector: this tree is a FORM, not a domain pick surface — it
/// declares no `interactionDomain` and no tree-level `interactionSelect`, so its rows legitimately
/// keep the control bindings that commit their own edits (the domain-pick form of law (d) is pinned
/// on `📌️panels/🗿️artifact`, the panel that does bind a domain).
#[test]
fn the_inspector_tree_binds_no_interaction_domain_and_its_rows_keep_their_controls() {
    let json = project(&Block3dSnapshot::default(), &TreeWindows::unhosted());
    let projection: serde_json::Value = serde_json::from_str(&json).expect("inspector projection json");
    assert!(projection["component"]["interactionDomain"].as_str().is_none(), "an inspector declares no interaction domain: {json}");
    assert!(projection["bindings"].as_array().map(|bindings| bindings.is_empty()).unwrap_or(true), "an inspector carries no tree-level interactionSelect: {json}");
    let summary = projection["children"].as_array().cloned().unwrap_or_default();
    let fields = summary.first().expect("the summary section")["children"].as_array().cloned().unwrap_or_default();
    assert!(fields.iter().all(|field| !field["children"].as_array().cloned().unwrap_or_default().is_empty()), "every field nests its own control: {json}");
}
//#endregion 🪟️WindowLaws
