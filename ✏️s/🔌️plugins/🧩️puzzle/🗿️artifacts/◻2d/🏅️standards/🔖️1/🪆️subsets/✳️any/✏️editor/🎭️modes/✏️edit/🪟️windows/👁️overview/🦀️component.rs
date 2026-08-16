//! 👁️ Puzzle 2d play app — the Overview window: the wide, interactive canvas pane. The only pane
//! that accepts pointer input, and therefore the only one binding the select/brush utilities
//! (`🪛️utilities/*`).

use crate::editor::puzzle2d::modes::edit;
use crate::editor::puzzle2d::modes::edit::options;
use crate::editor::puzzle2d::terminology::Puzzle2dLabels;
use crate::editor::puzzle2d::{puzzle2d_localized, Puzzle2dScene, PUZZLE2D_LOD_MODE_AUTOMATIC};
use crate::editor::puzzle2d::engine::BoardHost;
use crate::editor::puzzle2d::modes::edit::windows::overview::utilities;
use semio_framework_plugin::{SurfaceKind, UiNode, WindowEngagement, WindowEngagementSlot, WindowKindDefinition, WindowMeasure, WindowOptions};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "2d-overview";
pub const BODY_KEY: &str = "puzzle2d.play.overview";
pub const ZOOM_SCALE: f64 = 0.68;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle2d::create_puzzle2d_app`. Unlike cad,
/// puzzle2d freezes the first `window_measures()` frame into `options.measures` so the shell has LOD
/// and brush chrome before the first `refreshUi` tick; every later frame comes from
/// `ArtifactApp::window_measures`.
pub fn definition(envelope: &Puzzle2dScene, host: &BoardHost, labels: &Puzzle2dLabels) -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: puzzle2d_localized(|l| l.window_overview),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "layout-grid".into(),
        options: WindowOptions { measures: window_measures(envelope, labels), engagement: WindowEngagementSlot::Some(engagement(envelope, host, labels)) },
        actions: Vec::new(),
        utilities: vec![utilities::select::UTILITY_ID.into(), utilities::brush::UTILITY_ID.into()],
        interactions: vec![semio_framework_plugin::InteractionRef::new(crate::editor::puzzle2d::PUZZLE2D_INTERACTION_DOMAIN)],
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

/// 🎚️ The live chrome measures for this window, collected from the mode's `🎚️options/*` components.
pub fn window_measures(envelope: &Puzzle2dScene, labels: &Puzzle2dLabels) -> Vec<WindowMeasure> {
    let mode = envelope.runtime.lod_mode_by_pane.get(WINDOW_KIND_ID).map_or(PUZZLE2D_LOD_MODE_AUTOMATIC, String::as_str);
    vec![options::lod::measure(WINDOW_KIND_ID, mode, labels), options::brush::measure(envelope, labels)]
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(document_json: &str, envelope: &Puzzle2dScene) -> UiNode {
    edit::render_canvas(document_json, envelope, WINDOW_KIND_ID)
}

pub fn engagement(envelope: &Puzzle2dScene, host: &BoardHost, labels: &Puzzle2dLabels) -> WindowEngagement {
    edit::puzzle2d_engagement(envelope, host, WINDOW_KIND_ID, labels)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::puzzle2d::testkit::*;

    #[test]
    fn renders_puzzle2d_board_scene() {
        let mut app = app();
        assert!(render_body(&mut app, BODY_KEY).contains("board-2d"));
    }
}
//#endregion 🧪️Tests
