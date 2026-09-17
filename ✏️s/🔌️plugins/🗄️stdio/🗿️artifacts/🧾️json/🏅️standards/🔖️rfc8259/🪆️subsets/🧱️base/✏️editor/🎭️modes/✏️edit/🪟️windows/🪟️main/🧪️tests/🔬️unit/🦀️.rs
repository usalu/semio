use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_tree_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
}

#[semio_framework_async_macros::async_test]
async fn render_walks_object_and_array_members() {
    let document = JsonSnapshot { schema: "stdio.json".into(), value: JsonValue::Object { members: vec![JsonMember { key: "a".into(), value: JsonValue::Array { items: vec![JsonValue::Bool { value: true }] } }] } };
    let node = render(&document, &semio_framework_plugin::TreeWindows::unhosted()).expect("render");
    let section = node.children.get(0).expect("tree section");
    let root = section.children.get(0).expect("tree root");
    assert_eq!(root.key.as_str(), "");
    let a = root.children.get(0).expect("child");
    assert_eq!(a.key.as_str(), "k=a");
    let item0 = a.children.get(0).expect("child");
    assert_eq!(item0.key.as_str(), "k=a/i=0");
}

//#region 🪟️WindowLaws
use semio_framework_plugin::{TreeWindowRequest, TreeWindows, ViewModel};

/// 🪟 The viewport the host reports for these laws. Deliberately small: a first paint is priced in
/// real `UiValue` argument-arena credit, shared process-wide, and a law that materialised a full
/// 48-row viewport starved the sibling panel tests running beside it.
const MEASURED_VIEWPORT_ROWS: u32 = 4;

/// 🪟️ An everyday document: one member holding an array far past the 32 siblings this window used to
/// refuse outright.
fn oversized_document(items: usize) -> JsonSnapshot {
    JsonSnapshot {
        schema: "stdio.json".into(),
        value: JsonValue::Object { members: vec![JsonMember { key: "a".into(), value: JsonValue::Array { items: (0..items).map(|index| JsonValue::Number { lexeme: index.to_string() }).collect() } }] },
    }
}

/// 🪟️ The window body exactly as the host reads it, for the host-known windows in `requests`.
fn window_body(document: &JsonSnapshot, requests: Vec<TreeWindowRequest>) -> String {
    let view = ViewModel { tree_windows: requests, tree_viewport_rows: Some(MEASURED_VIEWPORT_ROWS), ..Default::default() };
    let node = render(document, &TreeWindows::for_body(&view, BODY_KEY)).expect("render the json tree");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("project the json tree")
}

/// 🪟️ Law (a): a 300-element array renders — the array node stamps its full extent and no `+N` appears.
#[test]
fn oversized_array_stamps_totals_and_never_a_continuation_row() {
    let json = window_body(&oversized_document(300), Vec::new());
    assert!(json.contains("\"total\":300"), "the array node stamps its full extent: {json}");
    assert!(!json.contains(".more"), "no continuation row survives: {json}");
    assert!(!json.contains("\"+"), "no `+N` label survives: {json}");
    assert!(json.matches("\"k=a/i=").count() <= MEASURED_VIEWPORT_ROWS as usize, "first paint materialises the measured viewport and stops: {json}");
}

/// 🪟️ Law (b): a node the host closed stamps its total and materialises nothing.
#[test]
fn closed_array_stamps_total_and_materialises_no_elements() {
    let json = window_body(&oversized_document(300), vec![TreeWindowRequest { body_key: BODY_KEY.into(), node_key: "k=a".into(), open: Some(false), offset: 0, rows: 0 }]);
    assert!(json.contains("\"total\":300"), "a closed array still stamps its extent: {json}");
    assert!(!json.contains("\"k=a/i="), "a closed array materialises no elements: {json}");
}

/// 🪟️ Law (c): a host window materialises exactly `[offset, offset + rows)`, keyed by the raw path id.
#[test]
fn host_window_materialises_exactly_its_slice() {
    let json = window_body(&oversized_document(300), vec![TreeWindowRequest { body_key: BODY_KEY.into(), node_key: "k=a".into(), open: Some(true), offset: 100, rows: 10 }]);
    assert!(json.contains("\"offset\":100"), "the array reports its offset: {json}");
    for index in 100..110 {
        assert!(json.contains(&format!("\"k=a/i={index}\"")), "element {index} is inside the window: {json}");
    }
    assert!(!json.contains("\"k=a/i=99\""), "the element before the window stays out: {json}");
    assert!(!json.contains("\"k=a/i=110\""), "the element after the window stays out: {json}");
}

/// 🪟️ Law (d) for a `TreeWindowKit` surface: it is a document structure, not a pick target — it binds no
/// interaction domain and stamps no granularity, so nodes keep exactly the `set-node` editing they had.
#[test]
fn json_tree_binds_no_interaction_domain() {
    let json = window_body(&oversized_document(4), Vec::new());
    assert!(!json.contains("interactionDomain"), "the json tree binds no domain: {json}");
    assert!(!json.contains("granularity"), "the json tree stamps no pick granularity: {json}");
}
//#endregion 🪟️WindowLaws
