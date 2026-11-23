//! 🖼️ Note play app — the composite (editable) canvas window: the full infinite-canvas surface.

use crate::editor::note::modes::edit::windows::composite::options;
use crate::editor::note::support::{NOTE_INTERACTION_BLOCKS, NOTE_INTERACTION_GRANULARITY};
use crate::editor::note::terminology::NotePlayLabels;
use crate::{NoteCamera, NoteSnapshot};
use semio_plugin::BuiltNode;
use semio_plugin::InkCanvasInteractionDomain;
use semio_plugin::InkCanvasScene;
use semio_plugin::InteractionView;
use semio_ui_locale::LocalizedLabel;
use semio_plugin::SurfaceKind;
use semio_plugin::UiAssemblyResult;
use semio_plugin::WindowEngagement;
use semio_plugin::WindowEngagementInput;
use semio_plugin::WindowEngagementStatus;
use semio_plugin::WindowKindDefinition;
use semio_plugin::WindowMeasure;
use semio_plugin::WindowOptions;

//#region 🔖️Constants
pub const NOTE_PLAY_WINDOW_COMPOSITE: &str = "note-composite";
pub const NOTE_PLAY_BODY_COMPOSITE: &str = "note.play.composite";
const NOTE_PLAY_SURFACE_COMPOSITE: &str = "note.play.composite";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::note::create_note_app`. `options.measures` stays
/// empty here on purpose: note's measures are config-derived and rebuilt per frame by
/// [`window_measures`], not frozen into the manifest.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: None,
        id: NOTE_PLAY_WINDOW_COMPOSITE.into(),
        label: LocalizedLabel::native("Canvas", "Zeichenfläche"),
        body_key: NOTE_PLAY_BODY_COMPOSITE.into(),
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

/// 🎚️ The live chrome measures for this window, collected from its `☑️options/*` components.
pub fn window_measures(document: &NoteSnapshot, camera: &NoteCamera, labels: &NotePlayLabels) -> Vec<WindowMeasure> {
    vec![
        options::camera::measure(camera, labels),
        options::grid::measure(document, labels),
        options::snap::measure(document, labels),
        options::pencil::measure(document, labels),
        options::eraser_stroke::measure(document, labels),
        options::eraser_point::measure(document, labels),
    ]
}

// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `ArtifactEditor::window_engagements` is
// not threaded an `InteractionView` (only `handle`/`copy_fragment`/`cut_operations` are), so this can
// no longer tell which blocks are selected — the status line drops the selection count and the
// engagement input is always enabled now (its own `engagementSubmit` handler still correctly no-ops
// unless exactly one block is selected, read via `NoteDispatchCtx`).
pub fn engagement(document: &NoteSnapshot, camera: &NoteCamera, engagement_input: &str) -> WindowEngagement {
    let block_count = crate::schema::flatten_blocks(&document.blocks).len();
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
            disabled: None,
            on_change: Some(crate::editor::note::support::note_action("engagementInput", None)),
            on_submit: Some(crate::editor::note::support::note_action("engagementSubmit", None)),
            on_repeat_last: None,
            on_abort: None,
        }),
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus { id: "note-status.counts".into(), text: format!("{block_count} blocks · zoom {zoom:.2}") }, WindowEngagementStatus { id: "note-status.grid".into(), text: format!("{grid_status} · {snap_status}") }]),
        possible_engagements: None,
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🖱️ Builds the ink-canvas scene payload shared by both the composite and navigator windows —
/// `view_mode` picks which one. Camera is session-only runtime state, never part of `NoteSnapshot` —
/// merged into the wire payload here so the ink-canvas host still gets a `camera` key to render/pan/zoom
/// against.
pub fn render_canvas_scene(document: &NoteSnapshot, camera: &NoteCamera, active_utility: &str, surface_id: &str, view_mode: &str, name: &str) -> UiAssemblyResult<BuiltNode> {
    semio_plugin::scene_surface(surface_id, semio_ui_contract::SurfaceKind::InkCanvas, &canvas_scene(document, camera, active_utility, view_mode), name, None)
}

fn canvas_scene(document: &NoteSnapshot, camera: &NoteCamera, active_utility: &str, view_mode: &str) -> InkCanvasScene {
    let interactive = view_mode == "composite";
    let mut scene = InkCanvasScene::base(crate::note_canvas_document_json(document, camera), active_utility.into(), view_mode.into(), interactive);
    scene.interaction_domain = interactive.then(|| InkCanvasInteractionDomain { id: NOTE_INTERACTION_BLOCKS.into(), granularity_id: NOTE_INTERACTION_GRANULARITY.into() });
    scene
}

pub fn render_with_interaction(document: &NoteSnapshot, camera: &NoteCamera, active_utility: &str, interaction: &InteractionView<'_>, locale: semio_plugin::Locale) -> UiAssemblyResult<BuiltNode> {
    let mut scene = canvas_scene(document, camera, active_utility, "composite");
    let selection: Vec<String> = interaction.selection(NOTE_INTERACTION_BLOCKS).ids.iter().filter_map(|id| crate::schema::block_id_from_tree_row_id(id)).collect();
    scene.selection_json = serde_json::to_string(&selection).expect("note interaction selection is JSON-safe");
    scene.hovered_id = interaction.hover(NOTE_INTERACTION_BLOCKS, "pointer").ids.first().and_then(|id| crate::schema::block_id_from_tree_row_id(id));
    semio_plugin::scene_surface(NOTE_PLAY_SURFACE_COMPOSITE, semio_ui_contract::SurfaceKind::InkCanvas, &scene, semio_plugin::surface_name(locale, "Note canvas", "Notizfläche"), None)
}

pub fn render(document: &NoteSnapshot, camera: &NoteCamera, active_utility: &str, locale: semio_plugin::Locale) -> UiAssemblyResult<BuiltNode> {
    render_canvas_scene(document, camera, active_utility, NOTE_PLAY_SURFACE_COMPOSITE, "composite", semio_plugin::surface_name(locale, "Note canvas", "Notizfläche"))
}
//#endregion 🔖️Render

//#region 🧪️Tests

//#endregion 🧪️Tests
