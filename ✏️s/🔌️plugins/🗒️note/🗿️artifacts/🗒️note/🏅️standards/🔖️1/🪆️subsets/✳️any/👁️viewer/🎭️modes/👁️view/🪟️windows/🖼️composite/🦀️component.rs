//! 🖼️ Note viewer — the Composite window: a read-only render of the live `NoteSnapshot` projection,
//! built from the SAME `build_ink_canvas_scene`/`InkCanvasScene` framework helpers the editor's own
//! Composite window uses — this file itself imports nothing from the sibling editor module
//! (`policyViewerPurityBreaches` forbids it outright). No selection, no drawing utilities, no
//! engagement input: a viewer has none of those and emits no mutations by construction (`ViewEmit`).

use crate::artifacts::note::NoteSnapshot;
use semio_framework_plugin::{build_ink_canvas_scene, InkCanvasScene, LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "note-view-composite";
pub const BODY_KEY: &str = "note.view.composite";
pub const SURFACE_ID: &str = "note.view.composite";
/// 👁️ Read-only counterpart of the editor's `NOTE_PLAY_CONTROLLER_ID` — kept distinct so a viewer
/// session's ink-canvas controller can never be mistaken for an editor session's.
const NOTE_VIEW_CONTROLLER_ID: &str = "note-view";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::note::create_note_viewer`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Canvas", "Zeichenfläche"),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::InkCanvas,
        icon_id: "pen-tool".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `NoteSnapshot -> UiNode` read: a hardcoded default camera (a viewer needs no persisted
/// per-session camera state — real block content renders exactly as the document stands, not through
/// any live pan/zoom the editor's own `NoteConfig.camera` carries; the same intentional
/// simplification the cad pilot's viewer documented for its own camera/environment defaults), no
/// active drawing utility (nothing is drawable), `InkCanvasScene.interactive: false`.
pub fn render(document: &NoteSnapshot) -> UiNode {
    let camera = crate::artifacts::note::NoteCamera::default();
    let mut document_value = serde_json::to_value(document).unwrap_or_else(|_| serde_json::json!({}));
    if let Some(map) = document_value.as_object_mut() {
        map.insert("camera".into(), serde_json::to_value(&camera).unwrap_or_else(|_| serde_json::json!({ "x": 0.0, "y": 0.0, "zoom": 1.0 })));
    }
    let document_json = document_value.to_string();
    build_ink_canvas_scene(SURFACE_ID, NOTE_VIEW_CONTROLLER_ID, InkCanvasScene::base(document_json, String::new(), "composite".into(), false))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn definition_declares_an_ink_canvas_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.surface_kind, SurfaceKind::InkCanvas);
    }

    #[test]
    fn render_produces_a_read_only_ink_canvas_scene_for_the_empty_document() {
        let document = crate::artifacts::note::schema::empty_note_snapshot();
        let node = render(&document);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("ink-canvas"));
        assert!(json.contains("\"interactive\":false"));
    }
}
//#endregion 🧪️Tests
