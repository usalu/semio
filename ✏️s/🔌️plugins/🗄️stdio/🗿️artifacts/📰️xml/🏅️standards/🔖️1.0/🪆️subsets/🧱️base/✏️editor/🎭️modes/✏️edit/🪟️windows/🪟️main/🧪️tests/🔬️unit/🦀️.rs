use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_tree_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
}

#[semio_framework_async_macros::async_test]
async fn render_walks_element_children() {
    let document = XmlSnapshot {
        schema: "stdio.xml".into(),
        doc: crate::schema::snapshot::XmlDocument { root: Some(XmlNode::Element { name: "root".into(), attrs: Vec::new(), children: vec![XmlNode::Text { text: "hi".into() }] }), doctype: None, declaration: None, prolog: Vec::new() },
    };
    let node = render(&document, &semio_framework_plugin::TreeWindows::unhosted()).expect("render");
    let section = node.children.get(0).expect("tree section");
    let root = section.children.get(0).expect("tree root");
    assert_eq!(root.key.as_str(), "");
    let child = root.children.get(0).expect("child");
    assert_eq!(child.key.as_str(), "0");
}

//#region 🪟️WindowLaws
use semio_framework_plugin::{TreeWindowRequest, TreeWindows, ViewModel, TREE_WINDOW_DEFAULT_ROWS};

/// 🪟️ An everyday document: one element with far more than the 32 children this window used to refuse
/// outright.
fn oversized_document(children: usize) -> XmlSnapshot {
    XmlSnapshot {
        schema: "stdio.xml".into(),
        doc: crate::schema::snapshot::XmlDocument {
            root: Some(XmlNode::Element { name: "root".into(), attrs: Vec::new(), children: (0..children).map(|index| XmlNode::Element { name: format!("child-{index}"), attrs: Vec::new(), children: Vec::new() }).collect() }),
            doctype: None,
            declaration: None,
            prolog: Vec::new(),
        },
    }
}

/// 🪟️ The window body exactly as the host reads it, for the host-known windows in `requests`.
fn window_body(document: &XmlSnapshot, requests: Vec<TreeWindowRequest>) -> String {
    let view = ViewModel { tree_windows: requests, ..Default::default() };
    let node = render(document, &TreeWindows::for_body(&view, BODY_KEY)).expect("render the xml tree");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("project the xml tree")
}

/// 🪟️ Law (a): a 300-child element renders — the root stamps its full extent and no `+N` appears.
#[test]
fn oversized_element_stamps_totals_and_never_a_continuation_row() {
    let json = window_body(&oversized_document(300), Vec::new());
    assert!(json.contains("\"total\":300"), "the root element stamps its full extent: {json}");
    assert!(!json.contains(".more"), "no continuation row survives: {json}");
    assert!(!json.contains("\"+"), "no `+N` label survives: {json}");
    assert!(json.matches("child-").count() <= TREE_WINDOW_DEFAULT_ROWS as usize, "first paint materialises about one viewport: {json}");
}

/// 🪟️ Law (b): an element the host closed stamps its total and materialises nothing.
#[test]
fn closed_element_stamps_total_and_materialises_no_children() {
    let json = window_body(&oversized_document(300), vec![TreeWindowRequest { body_key: BODY_KEY.into(), node_key: String::new(), open: Some(false), offset: 0, rows: 0 }]);
    assert!(json.contains("\"total\":300"), "a closed element still stamps its extent: {json}");
    assert!(!json.contains("child-"), "a closed element materialises no children: {json}");
}

/// 🪟️ Law (c): a host window materialises exactly `[offset, offset + rows)`, keyed by the raw node id.
#[test]
fn host_window_materialises_exactly_its_slice() {
    let json = window_body(&oversized_document(300), vec![TreeWindowRequest { body_key: BODY_KEY.into(), node_key: String::new(), open: Some(true), offset: 100, rows: 10 }]);
    assert!(json.contains("\"offset\":100"), "the root reports its offset: {json}");
    for index in 100..110 {
        assert!(json.contains(&format!("\"{index}\"")), "child {index} is inside the window: {json}");
    }
    assert!(!json.contains("child-99"), "the child before the window stays out: {json}");
    assert!(!json.contains("child-110"), "the child after the window stays out: {json}");
}

/// 🪟️ Law (d) for a `TreeWindowKit` surface: it is a document structure, not a pick target — it binds no
/// interaction domain and stamps no granularity, so nodes keep exactly the `set-node` editing they had.
#[test]
fn xml_tree_binds_no_interaction_domain() {
    let json = window_body(&oversized_document(4), Vec::new());
    assert!(!json.contains("interactionDomain"), "the xml tree binds no domain: {json}");
    assert!(!json.contains("granularity"), "the xml tree stamps no pick granularity: {json}");
}
//#endregion 🪟️WindowLaws
