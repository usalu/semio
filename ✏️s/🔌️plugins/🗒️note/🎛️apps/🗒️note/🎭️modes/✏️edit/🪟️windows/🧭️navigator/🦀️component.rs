//! 🧭️ Note play app — the navigator (overview/minimap) canvas window: a non-interactive scaled view.

use crate::apps::note::config::NoteConfig;
use crate::apps::note::modes::edit::windows::navigator::options;
use crate::apps::note::terminology::NotePlayLabels;
use crate::artifacts::note::{NoteCamera, NoteSnapshot};
use semio_framework_plugin::{LocalizedLabel, SurfaceKind, UiNode, WindowEngagement, WindowEngagementInput, WindowEngagementStatus, WindowKindDefinition, WindowMeasure, WindowOptions};

//#region 🔖️Constants
pub const NOTE_PLAY_WINDOW_NAVIGATOR: &str = "note-navigator";
pub const NOTE_PLAY_BODY_NAVIGATOR: &str = "note.play.navigator";
const NOTE_PLAY_SURFACE_NAVIGATOR: &str = "note.play.navigator";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::apps::note::create_note_app`. `options.measures` stays
/// empty here on purpose: note's measures are config-derived and rebuilt per frame by
/// [`window_measures`], not frozen into the manifest.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: NOTE_PLAY_WINDOW_NAVIGATOR.into(),
        label: LocalizedLabel::native("Navigator", "Navigator"),
        body_key: NOTE_PLAY_BODY_NAVIGATOR.into(),
        surface_kind: SurfaceKind::InkCanvas,
        icon_id: "focus".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        // 🕹️ Non-interactive overview — no `.window_kind_interactions(..)` reference for this window.
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

/// 🎚️ The live chrome measures for this window, collected from its `🎚️options/*` components.
pub fn window_measures(document: &NoteSnapshot, camera: &NoteCamera, labels: &NotePlayLabels) -> Vec<WindowMeasure> {
    vec![options::zoom::measure(camera, labels), options::grid_visible::measure(document, labels)]
}

pub fn engagement(active_utility: &str) -> WindowEngagement {
    WindowEngagement {
        session_active: Some(false),
        options: None,
        input: Some(WindowEngagementInput {
            id: Some("note-navigator-engagement".into()),
            value: None,
            placeholder: Some("Select all".into()),
            disabled: None,
            on_change: None,
            on_submit: Some(crate::apps::note::note_action("selectAll", None)),
            on_repeat_last: None,
            on_abort: None,
        }),
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus { id: "note-navigator-status.utility".into(), text: format!("utility: {active_utility}") }]),
        possible_engagements: None,
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(document: &NoteSnapshot, cfg: &NoteConfig) -> UiNode {
    crate::apps::note::modes::edit::windows::composite::render_canvas_scene(document, &cfg.camera, &cfg.active_utility_id, NOTE_PLAY_SURFACE_NAVIGATOR, "navigator")
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::note::testkit::{note_app, render as render_body};
    use crate::apps::note::NOTE_PLAY_BODY_NAVIGATOR as BODY_NAVIGATOR;

    #[test]
    fn renders_navigator_canvas() {
        let mut app = note_app();
        let json = render_body(&mut app, BODY_NAVIGATOR);
        assert!(json.contains("ink-canvas"));
        assert!(json.contains("\"viewMode\":\"navigator\""));
    }

    #[test]
    fn definition_declares_the_ink_canvas_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, NOTE_PLAY_BODY_NAVIGATOR);
        assert!(matches!(definition.surface_kind, SurfaceKind::InkCanvas));
        assert!(definition.options.measures.is_empty(), "measures are config-derived per frame, never frozen into the manifest");
    }
}
//#endregion 🧪️Tests
