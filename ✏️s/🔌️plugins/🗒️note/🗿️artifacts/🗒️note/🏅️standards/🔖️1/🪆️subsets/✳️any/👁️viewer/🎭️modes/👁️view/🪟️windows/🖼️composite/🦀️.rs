//! 🖼️ Note viewer — the Composite window: a read-only render of the live `NoteSnapshot` projection,
//! built from the SAME `build_ink_canvas_scene`/`InkCanvasScene` framework helpers the editor's own
//! Composite window uses — this file itself imports nothing from the sibling editor module
//! (`policyViewerPurityBreaches` forbids it outright). No selection, no drawing utilities, no
//! engagement input: a viewer has none of those and emits no mutations by construction (`ViewEmit`).

use crate::NoteSnapshot;
use semio_framework_plugin::{InkCanvasScene, LocalizedLabel, SurfaceKind, BuiltNode, UiAssemblyResult, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "note-view-composite";
pub const BODY_KEY: &str = "note.view.composite";
pub const SURFACE_ID: &str = "note.view.composite";
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
/// 👁️ Pure `NoteSnapshot -> UiAssemblyResult<BuiltNode>` read: a hardcoded default camera (a viewer needs no persisted
/// per-session camera state — real block content renders exactly as the document stands, not through
/// any live pan/zoom the editor's own `NoteConfig.camera` carries; the same intentional
/// simplification the cad pilot's viewer documented for its own camera/environment defaults), no
/// active drawing utility (nothing is drawable), `InkCanvasScene.interactive: false`.
pub fn render(document: &NoteSnapshot) -> UiAssemblyResult<BuiltNode> {
    let camera = crate::NoteCamera::default();
    let document_json = crate::note_canvas_document_json(document, &camera);
    semio_framework_plugin::scene_surface(SURFACE_ID, semio_framework_ui_contract::SurfaceKind::InkCanvas, &InkCanvasScene::base(document_json, String::new(), "composite".into(), false))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_an_ink_canvas_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.surface_kind, SurfaceKind::InkCanvas);
    }

    #[semio_framework_async_macros::async_test]
    async fn render_produces_a_read_only_ink_canvas_scene_for_the_empty_document() {
        let document = crate::schema::empty_note_snapshot();
        let node = render(&document).expect("viewer canvas");
        let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("semantic canvas") };
        let scene: InkCanvasScene = semio_framework_ui_scene::decode(props).expect("packed viewer scene");
        assert!(!scene.interactive);
        assert!(scene.active_utility.is_empty());
        semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("retire viewer canvas");
    }
}
//#endregion 🧪️Tests
