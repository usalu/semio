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
