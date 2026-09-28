use super::*;
use crate::editor::note::unit_tests::context::{dispatch, note_app};
use crate::editor::note::NoteCommand;

#[semio_framework_async_macros::async_test]
async fn save_download_requests_a_media_export() {
    let mut app = note_app().await;
    let save = dispatch(&mut app, NoteCommand::SaveDownload(SaveDownload {})).await;
    assert!(save.mutations.is_empty());
    assert!(matches!(save.requested_effects.first(), Some(Effect::DownloadMediaExport { filename, .. }) if filename == "🗒️semio.note.dsl"), "saveDownload must request a media export: {:?}", save.requested_effects);

}
