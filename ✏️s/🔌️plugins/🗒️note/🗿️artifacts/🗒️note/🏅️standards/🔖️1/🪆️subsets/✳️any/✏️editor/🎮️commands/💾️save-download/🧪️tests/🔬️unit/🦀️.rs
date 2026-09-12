use super::*;
use crate::editor::note::commands::load_request;
use crate::editor::note::unit_tests::context::{dispatch, note_app};
use crate::editor::note::NoteCommand;

#[semio_framework_async_macros::async_test]
async fn save_download_and_load_request_effects() {
    let mut app = note_app().await;
    let save = dispatch(&mut app, NoteCommand::SaveDownload(SaveDownload {})).await;
    assert!(save.mutations.is_empty());
    assert!(matches!(save.requested_effects.first(), Some(Effect::DownloadMediaExport { filename, .. }) if filename == "🗒️semio.note.dsl"), "saveDownload must request a media export: {:?}", save.requested_effects);

    let load = dispatch(&mut app, NoteCommand::LoadRequest(load_request::LoadRequest {})).await;
    assert!(matches!(load.requested_effects.first(), Some(Effect::RequestFileOpen { import_action, .. }) if import_action == "setFixtureJson"), "loadRequest must request a file open: {:?}", load.requested_effects);
}
