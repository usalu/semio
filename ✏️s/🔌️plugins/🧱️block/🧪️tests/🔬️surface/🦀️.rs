
#[semio_framework_async_macros::async_test]
async fn block2d_viewer_never_mutates() {
    semio_framework_plugin::artifact_app_laws::assert_viewer_never_mutates::<semio_s_artifact_block_2d::viewer::block2d::Block2dViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn block2d_editor_and_viewer_share_dialect() {
    semio_framework_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<semio_s_artifact_block_2d::editor::block2d::Block2dPlayApp, semio_s_artifact_block_2d::viewer::block2d::Block2dViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn block3d_viewer_never_mutates() {
    semio_framework_plugin::artifact_app_laws::assert_viewer_never_mutates::<semio_s_artifact_block_3d::viewer::block3d::Block3dViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn block3d_editor_and_viewer_share_dialect() {
    semio_framework_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<semio_s_artifact_block_3d::editor::block3d::Block3dPlayApp, semio_s_artifact_block_3d::viewer::block3d::Block3dViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn block5d_viewer_never_mutates() {
    semio_framework_plugin::artifact_app_laws::assert_viewer_never_mutates::<semio_s_artifact_block_5d::viewer::block5d::Block5dViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn block5d_editor_and_viewer_share_dialect() {
    semio_framework_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<semio_s_artifact_block_5d::editor::block5d::Block5dPlayApp, semio_s_artifact_block_5d::viewer::block5d::Block5dViewer>().await;
}

/// 🗂️ The whole plugin has to ASSEMBLE: all three apps contribute the `kit.catalog` artifact kind on
/// their `"catalog:out"` port, and `PluginBuilder` (`🏗️builder/🦀️.rs:648`) refuses the plugin when two
/// contributions of one id differ. They did — each app spelled the spec out by hand with its own
/// `dimension` — so `plugin()` failed with `plugin-assembly.media-kind` and `describe` refused to
/// emit this plugin's `🔣️.json` descriptor at all.
#[semio_framework_async_macros::async_test]
async fn every_app_contributes_the_one_kit_catalog_spelling() {
    let canonical = semio_s_artifact_block_2d::kit_catalog_artifact_kind();
    let apps = [
        semio_s_artifact_block_2d::editor::block2d::create_block2d_app(),
        semio_s_artifact_block_3d::editor::block3d::create_block3d_app(),
        semio_s_artifact_block_5d::editor::block5d::create_block5d_app(),
    ];
    for app in &apps {
        let declared: Vec<_> = app.artifact_kinds.iter().filter(|kind| kind.id == canonical.id).collect();
        assert_eq!(declared.len(), 1, "each block app declares the kit catalog exactly once");
        assert_eq!(declared[0], &canonical, "every block app declares the identical kit catalog spec");
    }
    crate::plugin().expect("the block plugin assembles");
}
