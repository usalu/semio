
use super::*;
use crate::editor::note::NOTE_PLAY_BODY_COMPOSITE as BODY_COMPOSITE;
use crate::editor::note::testkit::{note_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn renders_composite_canvas() {
    let mut app = note_app().await;
    let json = render_body(&mut app, BODY_COMPOSITE).await;
    assert!(json.contains("ink-canvas"));
    assert!(json.contains("documentJson"));
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_ink_canvas_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, NOTE_PLAY_BODY_COMPOSITE);
    assert!(matches!(definition.surface_kind, SurfaceKind::InkCanvas));
    assert!(definition.options.measures.is_empty(), "measures are config-derived per frame, never frozen into the manifest");
}
