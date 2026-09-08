
use super::*;
use crate::editor::note::testkit::{dispatch, note_app, render};
use crate::editor::note::{NOTE_PLAY_BODY_DOCUMENT, NoteCommand};

#[semio_framework_async_macros::async_test]
async fn note_labels_resolve_german_locale() {
    let mut app = note_app().await;
    dispatch(&mut app, NoteCommand::SetLocale(SetLocale { value: "de-DE".into() })).await;
    let document_json = render(&mut app, NOTE_PLAY_BODY_DOCUMENT).await;
    assert!(document_json.contains("Text hinzufügen"));
    assert!(document_json.contains("Tabelle hinzufügen"));
    assert!(document_json.contains("Mathe hinzufügen"));
    assert!(document_json.contains("Bild hinzufügen"));
    assert!(document_json.contains("Gruppe hinzufügen"));
}
