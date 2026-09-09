
use super::*;

#[semio_framework_async_macros::async_test]
async fn create_home_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_home_viewer().await;
    assert_eq!(def.role, semio_framework::AppRole::Viewer);
    assert_eq!(def.dialect, HOME_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<HomeViewer as ArtifactViewer>::DIALECT, HOME_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn renders_the_main_body_key_for_the_default_snapshot() {
    let snapshot = SHomeSnapshot::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg_snapshot = HomeConfig::default();
    let cfg = ConfigView { snapshot: &cfg_snapshot, window: None };
    let tree = <HomeViewer as ArtifactViewer>::render(main::S_HOME_VIEW_BODY, &doc, &cfg, &semio_framework_plugin::ViewModel::default()).expect("Home viewer main tree");
    let _ = semio_framework_plugin::testkit::project_and_retire_fixture_tree(tree).expect("Home viewer main projection");
}

#[semio_framework_async_macros::async_test]
async fn fold_directory_events_command_never_touches_the_document_store() {
    semio_framework_plugin::testkit::assert_viewer_never_mutates::<HomeViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_one_dialect() {
    semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<crate::editor::home::HomeApp, HomeViewer>().await;
}
