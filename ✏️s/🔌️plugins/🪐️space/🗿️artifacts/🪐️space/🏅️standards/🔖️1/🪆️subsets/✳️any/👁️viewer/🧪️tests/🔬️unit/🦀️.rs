
use super::*;

#[semio_framework_async_macros::async_test]
async fn create_space_index_viewer_builds_a_definition_for_this_dialect() {
    let def = create_space_index_viewer();
    assert_eq!(def.dialect, SPACE_INDEX_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<SpaceIndexViewer as ArtifactViewer>::DIALECT, SPACE_INDEX_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_falls_back_to_a_text_node() {
    let snapshot = SSpaceSnapshot::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let tree = <SpaceIndexViewer as ArtifactViewer>::render("nope", &doc, &ConfigView { snapshot: &NoConfig::default(), window: None }, &semio_framework_plugin::ViewModel::default()).expect("unknown Space viewer body tree");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("unknown Space viewer body projection");
    assert!(json.contains("Unknown body"));
}
