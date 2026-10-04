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

/// 🧸️ A view without the composed `content` child refuses by name instead of rendering an empty graph (design §20.15).
#[semio_framework_async_macros::async_test]
async fn the_main_body_refuses_a_view_without_its_content_child() {
    let snapshot = default_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg_snapshot = NoConfig::default();
    let cfg = ConfigView { snapshot: &cfg_snapshot, window: None };
    assert!(<DagViewer as ArtifactViewer>::render(main::BODY_KEY, &doc, &cfg, &semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).is_err());
}
