
use semio_framework_plugin::artifact_app_laws::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

#[semio_framework_async_macros::async_test]
async fn puzzle2d_viewer_never_mutates() {
    assert_viewer_never_mutates::<semio_s_artifact_puzzle_2d::viewer::puzzle2d::Puzzle2dViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn puzzle2d_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<semio_s_artifact_puzzle_2d::editor::puzzle2d::Puzzle2dPlayApp, semio_s_artifact_puzzle_2d::viewer::puzzle2d::Puzzle2dViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn puzzle3d_viewer_never_mutates() {
    assert_viewer_never_mutates::<semio_s_artifact_puzzle_3d::viewer::puzzle3d::Puzzle3dViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn puzzle3d_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<semio_s_artifact_puzzle_3d::editor::puzzle3d::Puzzle3dPlayApp, semio_s_artifact_puzzle_3d::viewer::puzzle3d::Puzzle3dViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn puzzle5d_viewer_never_mutates() {
    assert_viewer_never_mutates::<semio_s_artifact_puzzle_5d::viewer::puzzle5d::Puzzle5dViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn puzzle5d_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<semio_s_artifact_puzzle_5d::editor::puzzle5d::Puzzle5dPlayApp, semio_s_artifact_puzzle_5d::viewer::puzzle5d::Puzzle5dViewer>().await;
}
