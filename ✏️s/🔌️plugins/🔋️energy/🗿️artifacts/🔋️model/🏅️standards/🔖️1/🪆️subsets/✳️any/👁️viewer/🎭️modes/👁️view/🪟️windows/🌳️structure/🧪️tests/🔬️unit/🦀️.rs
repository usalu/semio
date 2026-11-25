use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_tree_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
}

#[semio_framework_async_macros::async_test]
async fn render_lists_name_version_and_every_collection_count() {
    let _arena = semio_ui_contract_values::testkit::UiArenaFixture::new();
    let document = EnergyModelSnapshot::empty().unwrap();
    let tree = render(&document, semio_plugin::Locale::En).expect("the tree window assembles");
    assert_eq!(tree.key.as_str(), WINDOW_KIND_ID);
    let root = &tree.children[0].children[0];
    assert!(root.children.iter().any(|item| item.key.as_str() == "name"));
    let geometry = root.children.iter().find(|item| item.key.as_str() == "geometry").expect("the geometry group is a root child");
    assert!(geometry.children.iter().any(|item| item.key.as_str() == "zones"));
    document.retire().expect("the document retires");
}
