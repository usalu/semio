use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

#[semio_framework_async_macros::async_test]
async fn trinity_jack_viewer_never_mutates() {
    assert_viewer_never_mutates::<semio_s_artifact_trinity_jack::viewer::jack::TrinityJackViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn trinity_jack_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<semio_s_artifact_trinity_jack::editor::jack::TrinityJackPlayApp, semio_s_artifact_trinity_jack::viewer::jack::TrinityJackViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn trinity_rewriting_viewer_never_mutates() {
    assert_viewer_never_mutates::<semio_s_artifact_trinity_rewriting::viewer::rewriting::TrinityRewritingViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn trinity_rewriting_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<semio_s_artifact_trinity_rewriting::editor::rewriting::TrinityRewritingPlayApp, semio_s_artifact_trinity_rewriting::viewer::rewriting::TrinityRewritingViewer>().await;
}
