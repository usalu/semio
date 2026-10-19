//! ↔️ snapshot's lower-left quadrant, framed by `🎭️modes/✏️edit`'s triptych camera.
//! 🎯️ Puzzle 2d play app — the Selection window: a non-interactive canvas pane framing the

use crate::editor::puzzle2d::engine::BoardHost;
use crate::editor::puzzle2d::modes::edit;
use crate::editor::puzzle2d::modes::edit::options;
use crate::editor::puzzle2d::terminology::Puzzle2dLabels;
use {crate::editor::puzzle2d::terminology::puzzle2d_localized, crate::editor::puzzle2d::snapshot::Puzzle2dScene, crate::editor::puzzle2d::config::PUZZLE2D_LOD_MODE_AUTOMATIC};
use semio_framework_plugin::SurfaceKind;
use semio_framework_plugin::WindowEngagement;
use semio_framework_plugin::WindowEngagementSlot;
use semio_framework_plugin::WindowKindDefinition;
use semio_framework_plugin::WindowMeasure;
use semio_framework_plugin::WindowOptions;

//#region 🔖️Constants
pub const BODY_KEY: &str = crate::editor::puzzle2d::config::PUZZLE2D_SELECTION_BODY_KEY;
pub const ZOOM_SCALE: f64 = crate::editor::puzzle2d::config::PUZZLE2D_SELECTION_ZOOM_SCALE;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle2d::create_puzzle2d_app`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: None,
        id: crate::editor::puzzle2d::config::PUZZLE2D_SELECTION_WINDOW_KIND_ID.into(),
        label: puzzle2d_localized(|l| l.window_selection),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::Board2d,
        icon_id: "crosshair".into(),
        options: WindowOptions { measures: Vec::new(), engagement: WindowEngagementSlot::None },
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: vec![semio_framework_plugin::InteractionRef::new(crate::editor::puzzle2d::snapshot::PUZZLE2D_INTERACTION_DOMAIN)],
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

/// 🎚️ The live chrome measures for this window, collected from the mode's `☑️options/*` components.
pub fn window_measures(envelope: &Puzzle2dScene, labels: &Puzzle2dLabels) -> Vec<WindowMeasure> {
    let mode = envelope.runtime.lod_mode_by_pane.get(crate::editor::puzzle2d::config::PUZZLE2D_SELECTION_WINDOW_KIND_ID).map_or(PUZZLE2D_LOD_MODE_AUTOMATIC, String::as_str);
    vec![options::lod::measure(crate::editor::puzzle2d::config::PUZZLE2D_SELECTION_WINDOW_KIND_ID, mode, labels), options::brush::measure(envelope, labels)]
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(document_json: &str, envelope: &Puzzle2dScene) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    edit::canvas::render_canvas(document_json, envelope, crate::editor::puzzle2d::config::PUZZLE2D_SELECTION_WINDOW_KIND_ID)
}

pub fn engagement(envelope: &Puzzle2dScene, host: &BoardHost, labels: &Puzzle2dLabels) -> WindowEngagement {
    edit::canvas::puzzle2d_engagement(envelope, host, crate::editor::puzzle2d::config::PUZZLE2D_SELECTION_WINDOW_KIND_ID, labels)
}
//#endregion 🔖️Render
