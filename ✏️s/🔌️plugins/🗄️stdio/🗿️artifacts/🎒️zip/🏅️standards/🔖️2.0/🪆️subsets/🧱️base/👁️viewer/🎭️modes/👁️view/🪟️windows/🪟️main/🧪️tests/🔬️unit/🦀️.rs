use super::*;
use crate::schema::snapshot::ZipEntry;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_read_only_tree_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
    assert!(def.actions.is_empty(), "a viewer window kind declares no mutation-shaped actions");
}

#[semio_framework_async_macros::async_test]
async fn render_lists_the_comment_root_and_one_leaf_per_entry() {
    let document = ZipSnapshot { entries: vec![ZipEntry { name: "a.txt".into(), data: b"hi".to_vec() }], comment: "an archive".into(), ..ZipSnapshot::default() };
    let node = render(&document).expect("render");
    let section = node.children.get(0).expect("tree section");
    let root = section.children.get(0).expect("tree root");
    assert_eq!(root.key.as_str(), COMMENT_NODE_ID);
    let children = &root.children;
    assert_eq!(children.len(), 1);
}
