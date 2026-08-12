//! 🖼️ Note play app — the composite (editable) canvas window: the full infinite-canvas surface.

use crate::apps::note::config::NoteConfig;
use crate::apps::note::modes::edit::windows::composite::options;
use crate::apps::note::terminology::NotePlayLabels;
use crate::apps::note::NOTE_PLAY_CONTROLLER_ID;
use crate::artifacts::note::{NoteCamera, NoteSnapshot};
use semio_framework_plugin::{build_ink_canvas_scene, InkCanvasScene, LocalizedLabel, SurfaceKind, UiNode, WindowEngagement, WindowEngagementInput, WindowEngagementStatus, WindowKindDefinition, WindowMeasure, WindowOptions};

//#region 🔖️Constants
pub const NOTE_PLAY_WINDOW_COMPOSITE: &str = "note-composite";
pub const NOTE_PLAY_BODY_COMPOSITE: &str = "note.play.composite";
const NOTE_PLAY_SURFACE_COMPOSITE: &str = "note.play.composite";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::apps::note::create_note_app`. `options.measures` stays
/// empty here on purpose: note's measures are config-derived and rebuilt per frame by
/// [`window_measures`], not frozen into the manifest.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: NOTE_PLAY_WINDOW_COMPOSITE.into(),
        label: LocalizedLabel::native("Canvas", "Zeichenfläche"),
        body_key: NOTE_PLAY_BODY_COMPOSITE.into(),
        surface_kind: SurfaceKind::InkCanvas,
        icon_id: "pen-tool".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

/// 🎚️ The live chrome measures for this window, collected from its `🎚️options/*` components.
pub fn window_measures(document: &NoteSnapshot, camera: &NoteCamera, labels: &NotePlayLabels) -> Vec<WindowMeasure> {
    vec![options::camera::measure(camera, labels), options::grid::measure(document, labels), options::snap::measure(document, labels), options::pencil::measure(document, labels), options::eraser_stroke::measure(document, labels), options::eraser_point::measure(document, labels)]
}

pub fn engagement(document: &NoteSnapshot, camera: &NoteCamera, selected_ids: &[String], engagement_input: &str) -> WindowEngagement {
    let block_count = crate::artifacts::note::schema::flatten_blocks(&document.blocks).len();
    let selected_count = selected_ids.len();
    let zoom = camera.zoom;
    let snap_status = if document.snap_enabled.unwrap_or(false) { format!("snap {}px", document.snap_grid_spacing.unwrap_or(8.0)) } else { "snap off".into() };
    let grid_status = if document.grid_visible.unwrap_or(true) { format!("grid {}px", document.grid_spacing.unwrap_or(32.0)) } else { "grid off".into() };
    WindowEngagement {
        session_active: Some(false),
        options: None,
        input: Some(WindowEngagementInput {
            id: Some("note-engagement".into()),
            value: Some(engagement_input.to_string()),
            placeholder: Some("Block name".into()),
            disabled: Some(selected_ids.len() != 1),
            on_change: Some(crate::apps::note::note_action("engagementInput", None)),
            on_submit: Some(crate::apps::note::note_action("engagementSubmit", None)),
            on_repeat_last: None,
            on_abort: None,
        }),
        control: None,
        controls: None,
        status: Some(vec![
            WindowEngagementStatus { id: "note-status.counts".into(), text: format!("{block_count} blocks · {selected_count} selected · zoom {zoom:.2}") },
            WindowEngagementStatus { id: "note-status.grid".into(), text: format!("{grid_status} · {snap_status}") },
        ]),
        possible_engagements: None,
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🖱️ Builds the ink-canvas scene payload shared by both the composite and navigator windows —
/// `view_mode` picks which one. Camera is session-only runtime state, never part of `NoteSnapshot` —
/// merged into the wire payload here so the ink-canvas host still gets a `camera` key to render/pan/zoom
/// against.
pub fn render_canvas_scene(document: &NoteSnapshot, camera: &NoteCamera, selected_ids: &[String], hovered_id: Option<&str>, active_utility: &str, surface_id: &str, view_mode: &str) -> UiNode {
    let mut document_value = serde_json::to_value(document).unwrap_or_else(|_| serde_json::json!({}));
    if let Some(map) = document_value.as_object_mut() {
        map.insert("camera".into(), serde_json::to_value(camera).unwrap_or_else(|_| serde_json::json!({ "x": 0.0, "y": 0.0, "zoom": 1.0 })));
    }
    let document_json = document_value.to_string();
    let selection_json = serde_json::to_string(selected_ids).unwrap_or_else(|_| "[]".into());
    build_ink_canvas_scene(
        surface_id,
        NOTE_PLAY_CONTROLLER_ID,
        InkCanvasScene { document_json, selection_json, hovered_id: hovered_id.map(str::to_string), active_utility: active_utility.into(), view_mode: view_mode.into(), interactive: view_mode == "composite" },
    )
}

pub fn render(document: &NoteSnapshot, cfg: &NoteConfig) -> UiNode {
    render_canvas_scene(document, &cfg.camera, &cfg.selected_block_ids, cfg.hovered_block_id.as_deref(), &cfg.active_utility_id, NOTE_PLAY_SURFACE_COMPOSITE, "composite")
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::note::testkit::{note_app, render as render_body};
    use crate::apps::note::NOTE_PLAY_BODY_COMPOSITE as BODY_COMPOSITE;

    #[test]
    fn renders_composite_canvas() {
        let mut app = note_app();
        let json = render_body(&mut app, BODY_COMPOSITE);
        assert!(json.contains("ink-canvas"));
        assert!(json.contains("documentJson"));
    }

    #[test]
    fn definition_declares_the_ink_canvas_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, NOTE_PLAY_BODY_COMPOSITE);
        assert!(matches!(definition.surface_kind, SurfaceKind::InkCanvas));
        assert!(definition.options.measures.is_empty(), "measures are config-derived per frame, never frozen into the manifest");
    }
}
//#endregion 🧪️Tests
