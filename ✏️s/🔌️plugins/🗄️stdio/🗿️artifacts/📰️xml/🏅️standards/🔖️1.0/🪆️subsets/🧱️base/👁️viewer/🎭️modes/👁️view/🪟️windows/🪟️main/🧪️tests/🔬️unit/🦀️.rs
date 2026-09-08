
use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_tree_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
}

#[semio_framework_async_macros::async_test]
async fn render_walks_element_children() {
    let document =
        XmlSnapshot { schema: "stdio.xml".into(), doc: crate::schema::snapshot::XmlDocument { root: Some(XmlNode::Element { name: "root".into(), attrs: Vec::new(), children: Vec::new() }), doctype: None, declaration: None, prolog: Vec::new() } };
    let node = render(&document).expect("render");
    let section = node.children.get(0).expect("tree section");
    let root = section.children.get(0).expect("tree root");
    assert_eq!(root.key.as_str(), "");
}
