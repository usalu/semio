//! 🗂️ 🗂️ Note play app commands command — `set-selection`.

use crate::apps::note::config::{NoteConfig, NoteConfigMutation};
use crate::artifacts::note::schema::{block_id, flatten_blocks};
use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-selection")]
pub struct SetSelection {
    pub ids: Vec<String>,
}

pub fn handle(payload: &SetSelection, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    Ok(Emit::config(vec![NoteConfigMutation::SetSelection { block_ids: payload.ids.clone() }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::note::testkit::{dispatch, note_app, render};
    use crate::apps::note::{NoteCommand, NOTE_PLAY_BODY_PROPERTIES};

    #[test]
    fn properties_panel_reads_app_selection() {
        let mut app = note_app();
        dispatch(&mut app, NoteCommand::AddBlock(crate::apps::note::commands::add_block::AddBlock { kind: "text".into(), x: 0.0, y: 0.0 }));
        let id = block_id(&app.snapshot().expect("snapshot").blocks[0]).to_string();
        dispatch(&mut app, NoteCommand::SetSelection(SetSelection { ids: vec![id] }));
        let json = render(&mut app, NOTE_PLAY_BODY_PROPERTIES);
        assert!(json.contains("note-properties.block"), "selected block must render an inspector group: {json}");
    }
}
//#endregion 🧪️Tests
