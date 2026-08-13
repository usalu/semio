//! 🐚️ 🐚️ Note play app commands command — `save-download`.

use crate::apps::note::config::{NoteConfig, NoteConfigMutation};
use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault, HostEffect};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "save-download")]
pub struct SaveDownload {}

pub fn handle(_payload: &SaveDownload, doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    let data = crate::artifacts::note::dsl::print_dsl(doc.snapshot);
    Ok(Emit::effect(HostEffect::DownloadMediaExport { filename: "🗒️semio.note.dsl".into(), mime_type: "text/plain".into(), data, encoding: None }))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::note::testkit::{dispatch, note_app};
    use crate::apps::note::NoteCommand;

    #[test]
    fn save_download_and_load_request_effects() {
        let mut app = note_app();
        let save = dispatch(&mut app, NoteCommand::SaveDownload(SaveDownload {}));
        assert!(save.mutations.is_empty());
        assert!(matches!(save.requested_effects.first(), Some(HostEffect::DownloadMediaExport { filename, .. }) if filename == "🗒️semio.note.dsl"), "saveDownload must request a media export: {:?}", save.requested_effects);

        let load = dispatch(&mut app, NoteCommand::LoadRequest(load_request::LoadRequest {}));
        assert!(matches!(load.requested_effects.first(), Some(HostEffect::RequestFileOpen { import_action, .. }) if import_action == "setFixtureJson"), "loadRequest must request a file open: {:?}", load.requested_effects);
    }
}
//#endregion 🧪️Tests
