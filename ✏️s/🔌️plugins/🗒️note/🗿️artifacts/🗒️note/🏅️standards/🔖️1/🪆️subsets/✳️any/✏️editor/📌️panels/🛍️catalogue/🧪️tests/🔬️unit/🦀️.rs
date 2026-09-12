use crate::editor::note::unit_tests::context::{note_app, render as render_body};
use crate::editor::note::NOTE_PLAY_BODY_CATALOGUE as BODY_CATALOGUE;

#[semio_framework_async_macros::async_test]
async fn catalogue_lists_every_block_kind() {
    let mut app = note_app().await;
    let json = render_body(&mut app, BODY_CATALOGUE).await;
    assert!(json.contains("Block kinds"));
    assert!(json.contains("text — rich text block"));
}

#[semio_framework_async_macros::async_test]
async fn catalogue_resolves_german_locale() {
    let mut app = note_app().await;
    let view_state = semio_framework_plugin::ViewModel { locale: semio_framework_plugin::Locale::De, ..Default::default() };
    let json = crate::editor::note::unit_tests::context::render_with_view(&mut app, BODY_CATALOGUE, &view_state).await;
    assert!(json.contains("Blockarten"));
    assert!(json.contains("Text — reicher Textblock"));
}
