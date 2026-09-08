
#[semio_framework_async_macros::async_test]
async fn block2d_viewer_never_mutates() {
    semio_framework_plugin::testkit::assert_viewer_never_mutates::<semio_s_artifact_block_2d::viewer::block2d::Block2dViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn block2d_editor_and_viewer_share_dialect() {
    semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<semio_s_artifact_block_2d::editor::block2d::Block2dPlayApp, semio_s_artifact_block_2d::viewer::block2d::Block2dViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn block3d_viewer_never_mutates() {
    semio_framework_plugin::testkit::assert_viewer_never_mutates::<semio_s_artifact_block_3d::viewer::block3d::Block3dViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn block3d_editor_and_viewer_share_dialect() {
    semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<semio_s_artifact_block_3d::editor::block3d::Block3dPlayApp, semio_s_artifact_block_3d::viewer::block3d::Block3dViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn block5d_viewer_never_mutates() {
    semio_framework_plugin::testkit::assert_viewer_never_mutates::<semio_s_artifact_block_5d::viewer::block5d::Block5dViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn block5d_editor_and_viewer_share_dialect() {
    semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<semio_s_artifact_block_5d::editor::block5d::Block5dPlayApp, semio_s_artifact_block_5d::viewer::block5d::Block5dViewer>().await;
}
