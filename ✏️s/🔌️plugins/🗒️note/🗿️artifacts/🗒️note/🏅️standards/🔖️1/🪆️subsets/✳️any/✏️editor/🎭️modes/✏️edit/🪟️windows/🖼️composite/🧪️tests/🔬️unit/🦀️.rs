use super::*;
use crate::editor::note::unit_tests::context::{note_app, render as render_body};
use crate::editor::note::NOTE_PLAY_BODY_COMPOSITE as BODY_COMPOSITE;

/// 🖼️ The composite body renders an ink-canvas SURFACE carrying this document. `documentJson` is a
/// field of the packed scene, not a substring of the projected tree: since the retained wire went
/// binary (ticket 26/09/15) the surface's `doc` is a byte page, so the scene is read through
/// `decode_fixture_scene` (the reader this editor's window-config laws already use).
#[semio_framework_async_macros::async_test]
async fn renders_composite_canvas() {
    let mut app = note_app().await;
    let json = render_body(&mut app, BODY_COMPOSITE).await;
    assert!(json.contains("ink-canvas"));
    let scene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene::<semio_framework_plugin::InkCanvasScene>(&json).expect("composite ink-canvas scene");
    assert_eq!(scene.view_mode, "composite");
    let document: serde_json::Value = serde_json::from_str(&scene.document_json).expect("composite scene documentJson");
    assert!(document.get("blocks").is_some(), "the composite scene carries the note document: {}", scene.document_json);
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_ink_canvas_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, NOTE_PLAY_BODY_COMPOSITE);
    assert!(matches!(definition.surface_kind, SurfaceKind::InkCanvas));
    assert!(definition.options.measures.is_empty(), "measures are config-derived per frame, never frozen into the manifest");
}
