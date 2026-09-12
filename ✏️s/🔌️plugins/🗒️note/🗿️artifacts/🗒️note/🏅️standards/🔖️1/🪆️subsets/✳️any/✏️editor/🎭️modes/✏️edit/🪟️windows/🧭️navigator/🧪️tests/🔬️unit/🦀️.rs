use super::*;
use crate::editor::note::unit_tests::context::{note_app, render as render_body};
use crate::editor::note::NOTE_PLAY_BODY_NAVIGATOR as BODY_NAVIGATOR;

#[semio_framework_async_macros::async_test]
async fn renders_navigator_canvas() {
    let mut app = note_app().await;
    let json = render_body(&mut app, BODY_NAVIGATOR).await;
    assert!(json.contains("ink-canvas"));
    assert!(json.contains("\"viewMode\":\"navigator\""));
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_ink_canvas_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, NOTE_PLAY_BODY_NAVIGATOR);
    assert!(matches!(definition.surface_kind, SurfaceKind::InkCanvas));
    assert!(definition.options.measures.is_empty(), "measures are config-derived per frame, never frozen into the manifest");
}
