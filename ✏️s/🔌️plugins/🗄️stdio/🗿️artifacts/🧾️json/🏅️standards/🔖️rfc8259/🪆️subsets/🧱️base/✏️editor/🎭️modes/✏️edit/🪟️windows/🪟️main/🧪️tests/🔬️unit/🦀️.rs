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
    assert_eq!(root.key.as_str(), JSON_ROOT_NODE_ID, "the root must carry a real key, never the positional `#0` fallback an empty id produces");
    let a = root.children.get(0).expect("child");
    assert_eq!(a.key.as_str(), "k=a");
    let item0 = a.children.get(0).expect("child");
    assert_eq!(item0.key.as_str(), "i=0", "a node is keyed by its SIBLING segment, never by its path from the root");
}

//#region 🪟️WindowLaws

/// 🪟️ A container's window identity is its PATH — the enclosing windowed containers' keys, outermost
/// first, then its own key — joined by `TREE_WINDOW_PATH_SEPARATOR`. Every node of a `TreeWindowKit`
/// body sits under the kit's one root section, so a host request names that section first.
fn window_path(keys: &[&str]) -> String {
    let mut path = format!("{}-root", TreeWindowKit::KIND_ID);
    for key in keys {
        path.push_str(semio_framework_plugin::TREE_WINDOW_PATH_SEPARATOR);
        path.push_str(key);
    }
    path
}
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
    assert!(json.matches("\"i=").count() <= MEASURED_VIEWPORT_ROWS as usize, "first paint materialises the measured viewport and stops: {json}");
}

/// 🪟️ Law (b): a node the host closed stamps its total and materialises nothing.
#[test]
fn closed_array_stamps_total_and_materialises_no_elements() {
    let json = window_body(&oversized_document(300), vec![TreeWindowRequest { body_key: BODY_KEY.into(), node_key: window_path(&[JSON_ROOT_NODE_ID, "k=a"]), open: Some(false), offset: 0, rows: 0 }]);
    assert!(json.contains("\"total\":300"), "a closed array still stamps its extent: {json}");
    assert!(!json.contains("\"i="), "a closed array materialises no elements: {json}");
}

/// 🪟️ Law (c): a host window materialises exactly `[offset, offset + rows)`, keyed by the raw path id.
#[test]
fn host_window_materialises_exactly_its_slice() {
    let json = window_body(&oversized_document(300), vec![TreeWindowRequest { body_key: BODY_KEY.into(), node_key: window_path(&[JSON_ROOT_NODE_ID, "k=a"]), open: Some(true), offset: 100, rows: 10 }]);
    assert!(json.contains("\"offset\":100"), "the array reports its offset: {json}");
    for index in 100..110 {
        assert!(json.contains(&format!("\"i={index}\"")), "element {index} is inside the window: {json}");
    }
    assert!(!json.contains("\"i=99\""), "the element before the window stays out: {json}");
    assert!(!json.contains("\"i=110\""), "the element after the window stays out: {json}");
}

/// 🪟️ Law (d) for a `TreeWindowKit` surface: it is a document structure, not a pick target — it binds no
/// interaction domain and stamps no granularity, so nodes keep exactly the `set-node` editing they had.
#[test]
fn json_tree_binds_no_interaction_domain() {
    let json = window_body(&oversized_document(4), Vec::new());
    assert!(!json.contains("interactionDomain"), "the json tree binds no domain: {json}");
    assert!(!json.contains("granularity"), "the json tree stamps no pick granularity: {json}");
}
/// 🪟️ A ten-level document whose deepest container is a long array — the shape that used to be
/// unreachable when a node keyed itself by its whole ancestry.
fn deep_document(depth: usize, items: usize) -> JsonSnapshot {
    let mut value = JsonValue::Array { items: (0..items).map(|index| JsonValue::Number { lexeme: index.to_string() }).collect() };
    for level in (0..depth).rev() {
        value = JsonValue::Object { members: vec![JsonMember { key: format!("configuration-level-{level}"), value }] };
    }
    JsonSnapshot { schema: "stdio.json".into(), value }
}

/// 🪟️ Like [`window_body`] with a viewport wide enough to walk ten nesting levels before it spends
/// anything on rows — the first-paint budget is shared across the WHOLE body, containers included.
fn deep_window_body(document: &JsonSnapshot, requests: Vec<TreeWindowRequest>) -> String {
    let view = ViewModel { tree_windows: requests, tree_viewport_rows: Some(32), ..Default::default() };
    let node = render(document, &TreeWindows::for_body(&view, BODY_KEY)).expect("render the json tree");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("project the json tree")
}

/// 🪟️ Law (e): a container's window path is a VIEW-CONTEXT IDENTIFIER — printable and at most 256
/// code points — and a deep container still streams the slice a host request addressed by that path.
/// Keying a node by its whole ancestry grew the path quadratically and put everything past about
/// seven levels beyond the host's reach; a sibling-segment key keeps it linear.
#[test]
fn a_deep_containers_window_path_stays_a_view_context_identifier_and_still_streams() {
    let document = deep_document(10, 300);
    let segments: Vec<String> = (0..10).map(|level| member_segment(&format!("configuration-level-{level}"))).collect();
    let path = encode_path_id(&segments);
    assert!(path.chars().count() <= 256, "a depth-10 container path stays inside the view-context bound, was {}: {path:?}", path.chars().count());
    assert!(!path.chars().any(|character| character.is_control()), "a container path carries no control code point: {path:?}");
    let json = deep_window_body(&document, vec![TreeWindowRequest { body_key: BODY_KEY.into(), node_key: path, open: Some(true), offset: 40, rows: 4 }]);
    assert!(json.contains("\"offset\":40"), "the deepest container honours a request addressed by its path: {json}");
    for index in 40..44 {
        assert!(json.contains(&format!("\"i={index}\"")), "element {index} is inside the deep window: {json}");
    }
    assert!(!json.contains("\"i=39\""), "the element before the deep window stays out: {json}");
}

/// 🪟️ Law (f): repeated member names are true siblings, so each gets its own key — the UI document and
/// the window ledger both refuse two siblings sharing one.
#[test]
fn repeated_member_names_get_distinct_sibling_keys() {
    let document = JsonSnapshot {
        schema: "stdio.json".into(),
        value: JsonValue::Object { members: vec![JsonMember { key: "a".into(), value: JsonValue::Array { items: vec![JsonValue::Null] } }, JsonMember { key: "a".into(), value: JsonValue::Array { items: vec![JsonValue::Null] } }] },
    };
    let json = window_body(&document, Vec::new());
    assert!(json.contains("\"k=a\""), "the first member keeps the plain key: {json}");
    assert!(json.contains("\"k=a#2\""), "the repeated member gets its own key: {json}");
}
//#endregion 🪟️WindowLaws
