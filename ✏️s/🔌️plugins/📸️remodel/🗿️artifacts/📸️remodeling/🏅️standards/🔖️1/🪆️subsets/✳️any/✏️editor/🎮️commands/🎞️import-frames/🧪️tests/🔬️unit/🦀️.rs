use semio_s_artifact_remodel_remodeling_cmd_ingest::editor::remodeling::commands::import_frames::*;
use semio_s_artifact_remodel_remodeling_cmd_ingest::editor::remodeling::commands::import_video;
use crate::editor::remodeling::unit_tests::context::{app, dispatch};
use semio_s_artifact_remodel_remodeling_editor::editor::remodeling::RemodelingCommand;

#[semio_framework_async_macros::async_test]
async fn import_pickers_emit_a_host_effect_and_no_operations() {
    let mut app = app().await;
    for command in [RemodelingCommand::ImportFrames(ImportFrames {}), RemodelingCommand::ImportVideo(import_video::ImportVideo {})] {
        let result = dispatch(&mut app, command).await;
        assert!(!result.edited_document(), "a shell picker never mutates the document: {:?}", result.lanes);
        assert_eq!(result.requested_effects.len(), 1, "{:?}", result.requested_effects);
    }
}
