use super::*;

#[semio_framework_async_macros::async_test]
async fn create_dag_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_dag_viewer();
    assert_eq!(def.role, semio_framework::AppRole::Viewer);
    assert_eq!(def.dialect, DAG_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<DagViewer as ArtifactViewer>::DIALECT, DAG_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn renders_the_main_body_key_for_the_default_snapshot() {
    let snapshot = default_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg_snapshot = NoConfig::default();
    let cfg = ConfigView { snapshot: &cfg_snapshot, window: None };
    let _node = <DagViewer as ArtifactViewer>::render(main::BODY_KEY, &doc, &cfg, &semio_framework_plugin::ViewModel::default());
}
