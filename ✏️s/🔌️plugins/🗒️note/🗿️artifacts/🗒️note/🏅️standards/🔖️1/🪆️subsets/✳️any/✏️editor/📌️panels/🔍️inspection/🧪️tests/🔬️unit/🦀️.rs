use crate::editor::note::testkit::{note_app, render as render_body};
use crate::editor::note::NOTE_PLAY_BODY_PROPERTIES as BODY_PROPERTIES;

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    let mut app = note_app().await;
    assert!(render_body(&mut app, "note.play.nope").await.contains("Unknown body"));
}

#[semio_framework_async_macros::async_test]
async fn renders_the_document_wide_summary() {
    let mut app = note_app().await;
    let json = render_body(&mut app, BODY_PROPERTIES).await;
    assert!(json.contains("Utility:"));
}
