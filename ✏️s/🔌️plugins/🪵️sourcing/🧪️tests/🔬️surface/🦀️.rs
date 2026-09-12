
/// 👁️✏️ Editor and viewer must share the exact same `Dialect` — both surfaces address the same
/// artifact coordinate, only the role differs (contract §2.5).
#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_the_same_dialect() {
    semio_framework_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<crate::editor::sourcing::SourcingCurationApp, crate::viewer::sourcing::SourcingViewer>().await;
}

/// 👁️ Structural + runtime proof the viewer can never mutate the document or draft store
/// (contract §2.2/§2.5) — dispatches `SourcingViewCommand::default()` through the full
/// `VcsArtifactApp<ViewerApp<SourcingViewer>>` runtime path.
#[semio_framework_async_macros::async_test]
async fn viewer_never_mutates() {
    semio_framework_plugin::artifact_app_laws::assert_viewer_never_mutates::<crate::viewer::sourcing::SourcingViewer>().await;
}
