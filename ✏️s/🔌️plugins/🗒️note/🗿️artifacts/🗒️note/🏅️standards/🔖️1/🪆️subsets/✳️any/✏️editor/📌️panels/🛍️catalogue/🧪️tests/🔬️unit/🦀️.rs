
use crate::editor::note::NOTE_PLAY_BODY_CATALOGUE as BODY_CATALOGUE;
use crate::editor::note::testkit::{note_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn catalogue_lists_every_block_kind() {
    let mut app = note_app().await;
    let json = render_body(&mut app, BODY_CATALOGUE).await;
    assert!(json.contains("Block kinds"));
    assert!(json.contains("text — rich text block"));
}

#[semio_framework_async_macros::async_test]
async fn catalogue_resolves_german_locale() {
    use crate::editor::note::NoteCommand;
    use crate::editor::note::commands::set_locale::SetLocale;
    use crate::editor::note::testkit::dispatch;

    let mut app = note_app().await;
    dispatch(&mut app, NoteCommand::SetLocale(SetLocale { value: "de-DE".into() })).await;
    let json = render_body(&mut app, BODY_CATALOGUE).await;
    assert!(json.contains("Blockarten"));
    assert!(json.contains("Text — reicher Textblock"));
}
