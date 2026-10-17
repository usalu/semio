use semio_framework_plugin::Effect;
#[allow(unused_imports)]
use crate::op::NoteMutation;
#[allow(unused_imports)]
use crate::NoteSnapshot;
#[allow(unused_imports)]
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
#[allow(unused_imports)]
use semio_framework_value_derive::{FromValue, ToValue};
use crate::editor::note::commands::save_download::*;
use crate::editor::note::unit_tests::context::{dispatch, note_app};
use crate::editor::note::command_enum::NoteCommand;

#[semio_framework_async_macros::async_test]
async fn save_download_requests_a_media_export() {
    let mut app = note_app().await;
    let save = dispatch(&mut app, NoteCommand::SaveDownload(SaveDownload {})).await;
    assert!(save.mutations.is_empty());
    assert!(matches!(save.requested_effects.first(), Some(Effect::DownloadMediaExport { filename, .. }) if filename == "🗒️semio.note.dsl"), "saveDownload must request a media export: {:?}", save.requested_effects);

}
