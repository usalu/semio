use crate::editor::note::unit_tests::context::{note_app, render as render_body};
use crate::editor::note::NOTE_PLAY_BODY_PROPERTIES as BODY_PROPERTIES;

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    let mut app = note_app().await;
    assert!(render_body(&mut app, "note.play.nope").await.contains("Unknown body"));
}

/// 🔍️ The four document-wide summary rows. Each is a `tree_item_desc`, i.e. a row with a `label` and
/// a separate `description` — the old single `"Utility: …"` string it used to print is composed by the
/// shell now, so this reads the row's own label instead of a substring of the projected tree.
#[semio_framework_async_macros::async_test]
async fn renders_the_document_wide_summary() {
    let mut app = note_app().await;
    let json = render_body(&mut app, BODY_PROPERTIES).await;
    assert!(json.contains("\"type\":\"tree\""), "inspection body must be a tree: {json}");
    for label in ["Schema", "Blocks", "Utility", "Snap"] {
        assert!(json.contains(&format!("\"label\":\"{label}\"")), "inspection summary is missing its {label} row: {json}");
    }
}
