//! 🗣️ 🗣️ Note play app command command — `set-locale`.

use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use crate::editor::note::config::{NoteConfig, NoteConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "locale")]
pub struct SetLocale {
    pub value: String,
}

pub async fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>, _ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    Ok(Emit::config(vec![NoteConfigMutation::SetLocale { value: payload.value.clone() }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::note::testkit::{dispatch, note_app, render};
    use crate::editor::note::{NoteCommand, NOTE_PLAY_BODY_DOCUMENT};

    #[semio_framework_async_macros::async_test]
    async fn note_labels_resolve_german_locale() {
        let mut app = note_app();
        dispatch(&mut app, NoteCommand::SetLocale(SetLocale { value: "de-DE".into() }));
        let document_json = render(&mut app, NOTE_PLAY_BODY_DOCUMENT);
        assert!(document_json.contains("Text hinzufügen"));
        assert!(document_json.contains("Tabelle hinzufügen"));
        assert!(document_json.contains("Mathe hinzufügen"));
        assert!(document_json.contains("Bild hinzufügen"));
        assert!(document_json.contains("Gruppe hinzufügen"));
    }
}
//#endregion 🧪️Tests
