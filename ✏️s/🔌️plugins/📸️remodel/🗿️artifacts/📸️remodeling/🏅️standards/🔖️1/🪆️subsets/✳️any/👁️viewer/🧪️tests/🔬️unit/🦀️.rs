use super::*;

#[semio_framework_async_macros::async_test]
async fn create_remodeling_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_remodeling_viewer();
    assert_eq!(def.role, semio_framework::AppRole::Viewer);
    assert_eq!(def.dialect, REMODELING_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<RemodelingViewer as ArtifactViewer>::DIALECT, REMODELING_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn render_renders_the_model_window_body_and_falls_back_by_name_otherwise() {
    let scene = default_remodeling_scene();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&scene, &history);
    let cfg = ConfigView { snapshot: &NoConfig::default(), window: None };
    let _rendered = <RemodelingViewer as ArtifactViewer>::render(model::BODY_KEY, &doc, &cfg, &semio_framework_plugin::ViewModel::default());
    let fallback = <RemodelingViewer as ArtifactViewer>::render("nonsense", &doc, &cfg, &semio_framework_plugin::ViewModel::default());
    assert!(format!("{fallback:?}").contains("nonsense"));
}
