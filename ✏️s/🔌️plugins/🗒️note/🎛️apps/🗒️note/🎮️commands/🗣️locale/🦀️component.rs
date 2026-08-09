//! 🗣️ Note play app command — the host-pushed locale switch. Config-only.

use crate::apps::note::config::{NoteConfig, NoteConfigMutation};
use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetLocale
pub mod set_locale {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "locale")]
    pub struct SetLocale {
        pub value: String,
    }

    pub fn handle(payload: &SetLocale, _doc: &DocumentView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
        Ok(Emit::config(vec![NoteConfigMutation::SetLocale { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetLocale

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::note::testkit::{dispatch, note_app, render};
    use crate::apps::note::{NoteCommand, NOTE_PLAY_BODY_DOCUMENT};

    #[test]
    fn note_labels_resolve_german_locale() {
        let mut app = note_app();
        dispatch(&mut app, NoteCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }));
        let document_json = render(&mut app, NOTE_PLAY_BODY_DOCUMENT);
        assert!(document_json.contains("Text hinzufügen"));
        assert!(document_json.contains("Tabelle hinzufügen"));
        assert!(document_json.contains("Mathe hinzufügen"));
        assert!(document_json.contains("Bild hinzufügen"));
        assert!(document_json.contains("Gruppe hinzufügen"));
    }
}
//#endregion 🧪️Tests
