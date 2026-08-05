//! 🎥️ Note play app commands — the free/live canvas camera. Config-only.

use crate::apps::note::config::{NoteConfig, NoteConfigOperation};
use crate::artifacts::note::op::NoteOperation;
use crate::artifacts::note::NoteDocument;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetCamera
pub mod set_camera {
    use super::*;
    use crate::artifacts::note::NoteCamera;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "camera")]
    pub struct SetCamera {
        #[dsl(block)]
        pub camera: NoteCamera,
    }

    pub fn handle(payload: &SetCamera, _doc: &DocumentView<'_, NoteDocument>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteOperation, NoteConfigOperation>, Fault> {
        Ok(Emit::config(vec![NoteConfigOperation::SetCamera { camera: payload.camera.clone() }]))
    }
}
//#endregion 🔖️SetCamera

//#region 🔖️SetCameraZoom
pub mod set_camera_zoom {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "camera-zoom")]
    pub struct SetCameraZoom {
        pub value: f64,
    }

    pub fn handle(payload: &SetCameraZoom, _doc: &DocumentView<'_, NoteDocument>, cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteOperation, NoteConfigOperation>, Fault> {
        let mut camera = cfg.projection.camera.clone();
        camera.zoom = payload.value;
        Ok(Emit::config(vec![NoteConfigOperation::SetCamera { camera }]))
    }
}
//#endregion 🔖️SetCameraZoom

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::note::testkit::{dispatch, note_app, render};
    use crate::apps::note::{NoteCommand, NOTE_PLAY_BODY_COMPOSITE};
    use crate::artifacts::note::NoteCamera;

    /// 🎥️ `setCamera`/`setCameraZoom` are config-only — they must never emit a `NoteOperation` (no VCS
    /// edit, no undo entry on the document store) and instead write into `cfg.camera`, which the
    /// composite scene's `documentJson.camera` then reflects.
    #[test]
    fn set_camera_writes_config_and_emits_no_document_operations() {
        let mut app = note_app();
        let before = app.projection().expect("projection");
        let result = dispatch(&mut app, NoteCommand::SetCamera(set_camera::SetCamera { camera: NoteCamera { x: 4.0, y: 5.0, zoom: 2.0 } }));
        assert!(result.operations.is_empty(), "camera is config-only and emits no document operations");
        assert_eq!(app.projection().expect("projection"), before, "camera never mutates the document");
        let json = render(&mut app, NOTE_PLAY_BODY_COMPOSITE);
        assert!(json.contains(r#"\"zoom\":2.0"#), "composite scene camera reflects config state: {json}");
        assert!(json.contains(r#"\"x\":4.0"#), "composite scene camera reflects config state: {json}");
    }

    #[test]
    fn set_camera_zoom_updates_zoom_and_keeps_pan_via_config() {
        let mut app = note_app();
        dispatch(&mut app, NoteCommand::SetCamera(set_camera::SetCamera { camera: NoteCamera { x: 4.0, y: 5.0, zoom: 1.0 } }));
        let result = dispatch(&mut app, NoteCommand::SetCameraZoom(set_camera_zoom::SetCameraZoom { value: 3.0 }));
        assert!(result.operations.is_empty(), "camera zoom is config-only and emits no document operations");
        let json = render(&mut app, NOTE_PLAY_BODY_COMPOSITE);
        assert!(json.contains(r#"\"zoom\":3.0"#), "zoom updated: {json}");
        assert!(json.contains(r#"\"x\":4.0"#), "pan preserved across zoom-only update: {json}");
    }

    /// 🎥️ Dragging the viewport camera through several ticks must never create a VCS edit/undo step on
    /// the DOCUMENT store at all.
    #[test]
    fn camera_drag_never_creates_a_document_undo_step() {
        use semio_framework_plugin::PluginApp;

        let mut app = note_app();
        for x in [1.0, 2.0, 3.0] {
            dispatch(&mut app, NoteCommand::SetCamera(set_camera::SetCamera { camera: NoteCamera { x, y: 0.0, zoom: 1.0 } }));
        }
        assert!(render(&mut app, NOTE_PLAY_BODY_COMPOSITE).contains(r#"\"x\":3.0"#));
        app.handle_action("undo", None, &semio_framework_plugin::testkit::meta("local")).expect("undo (no-op: nothing on the document store to undo)");
        assert!(render(&mut app, NOTE_PLAY_BODY_COMPOSITE).contains(r#"\"x\":3.0"#), "document undo has nothing to revert — the drag never touched the document");
    }
}
//#endregion 🧪️Tests
