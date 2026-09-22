use super::*;
use crate::editor::note::commands::set_camera_zoom;
use crate::editor::note::unit_tests::context::{dispatch, note_app, render};
use crate::editor::note::{NoteCommand, NOTE_PLAY_BODY_COMPOSITE};
use crate::NoteCamera;
use semio_framework_plugin::artifact_app_laws;

/// 🎥️ The camera the composite surface actually carries. A projected tree no longer spells its scene
/// as JSON text: since the retained wire went binary (ticket 26/09/15) `SurfaceProps.doc` is a byte
/// page, so `render(..).contains("\"zoom\":2.0")` looks for a substring that only exists INSIDE that
/// page. `decode_fixture_scene` is the current reader (the same one this editor's own
/// `…_isolate_and_reload_two_windows` law uses).
async fn composite_camera(app: &mut crate::editor::note::unit_tests::context::NoteApp) -> NoteCamera {
    let projection = render(app, NOTE_PLAY_BODY_COMPOSITE).await;
    let scene = artifact_app_laws::decode_fixture_scene::<semio_framework_plugin::InkCanvasScene>(&projection).expect("composite ink-canvas scene");
    let document: serde_json::Value = serde_json::from_str(&scene.document_json).expect("composite scene documentJson");
    let camera = document.get("camera").cloned().unwrap_or(serde_json::Value::Null);
    NoteCamera {
        x: camera.get("x").and_then(serde_json::Value::as_f64).unwrap_or(f64::NAN),
        y: camera.get("y").and_then(serde_json::Value::as_f64).unwrap_or(f64::NAN),
        zoom: camera.get("zoom").and_then(serde_json::Value::as_f64).unwrap_or(f64::NAN),
    }
}

/// 🎥️ `setCamera`/`setCameraZoom` are config-only — they must never emit a `NoteMutation` (no VCS
/// edit, no undo entry on the document store) and instead write into `cfg.camera`, which the
/// composite scene's `documentJson.camera` then reflects.
#[semio_framework_async_macros::async_test]
async fn set_camera_writes_config_and_emits_no_artifact_mutations() {
    let mut app = note_app().await;
    let before = app.snapshot().expect("snapshot");
    let result = dispatch(&mut app, NoteCommand::SetCamera(SetCamera { camera: NoteCamera { x: 4.0, y: 5.0, zoom: 2.0 } })).await;
    assert!(result.mutations.is_empty(), "camera is config-only and emits no document operations");
    assert_eq!(app.snapshot().expect("snapshot"), before, "camera never mutates the document");
    let camera = composite_camera(&mut app).await;
    assert_eq!(camera.zoom, 2.0, "composite scene camera reflects config state: {camera:?}");
    assert_eq!(camera.x, 4.0, "composite scene camera reflects config state: {camera:?}");
}

#[semio_framework_async_macros::async_test]
async fn set_camera_zoom_updates_zoom_and_keeps_pan_via_config() {
    let mut app = note_app().await;
    dispatch(&mut app, NoteCommand::SetCamera(SetCamera { camera: NoteCamera { x: 4.0, y: 5.0, zoom: 1.0 } })).await;
    let result = dispatch(&mut app, NoteCommand::SetCameraZoom(set_camera_zoom::SetCameraZoom { value: 3.0 })).await;
    assert!(result.mutations.is_empty(), "camera zoom is config-only and emits no document operations");
    let camera = composite_camera(&mut app).await;
    assert_eq!(camera.zoom, 3.0, "zoom updated: {camera:?}");
    assert_eq!(camera.x, 4.0, "pan preserved across zoom-only update: {camera:?}");
}

/// 🎥️ Dragging the viewport camera through several ticks must never create a VCS edit/undo step on
/// the DOCUMENT store at all.
#[semio_framework_async_macros::async_test]
async fn camera_drag_never_creates_a_document_undo_step() {
    use semio_framework_plugin::PluginApp;

    let mut app = note_app().await;
    for x in [1.0, 2.0, 3.0] {
        dispatch(&mut app, NoteCommand::SetCamera(SetCamera { camera: NoteCamera { x, y: 0.0, zoom: 1.0 } })).await;
    }
    assert_eq!(composite_camera(&mut app).await.x, 3.0);
    app.handle_action("undo", None, &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("undo (no-op: nothing on the document store to undo)");
    assert_eq!(composite_camera(&mut app).await.x, 3.0, "document undo has nothing to revert — the drag never touched the document");
}
