//! ⚙️ Puzzle 3d play app panel — the app-wide settings: the brush placement overlap budget, the
//! relocate proximity radius, the viewport chunk size and the grid spacing. Unlike the
//! window-instance chrome in `🎭️modes/✏️edit/🎚️options/*`, these are settings a user tunes once for
//! the whole session rather than per pane. 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM:
//! the default selection merge mode dropdown is gone — merge is a per-gesture `interactionSelect` arg
//! (modifier-driven) in the framework's model now, with no equivalent "default" setting.

use crate::editor::puzzle3d::puzzle3d_action;
use crate::editor::puzzle3d::terminology::Puzzle3dLabels;
use crate::editor::puzzle3d::Puzzle3dScene;
use semio_framework_plugin::{ui_inspector_groups_to_tree, ui_inspector_stepper_field, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiInspectorFieldGroup, UiNode, UiPresence};

//#region 🔖️Constants
pub const PANEL_TAB_ID: &str = "puzzle3d.panel.settings";
pub const BODY_KEY: &str = "puzzle.3d.play.settings";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(PANEL_TAB_ID.into()), label: LocalizedLabel::native("Settings", "Einstellungen"), group: PanelGroup::Settings, body_key: Some(BODY_KEY.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub async fn render(envelope: &Puzzle3dScene, labels: &Puzzle3dLabels) -> UiNode {
    let runtime = &envelope.runtime;
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "puzzle3d-play-settings".into(),
        label: labels.settings.into(),
        default_open: Some(true),
        presence: UiPresence::default(),
        fields: vec![
            ui_inspector_stepper_field("puzzle3d-play-settings.overlap-budget", labels.overlap_budget, &[runtime.overlap_budget], 0.05, puzzle3d_action("setBrushPlacementOverlapBudget", None)),
            ui_inspector_stepper_field("puzzle3d-play-settings.proximity-radius", labels.proximity_radius, &[runtime.proximity_radius], 0.1, puzzle3d_action("setProximityRadius", None)),
            ui_inspector_stepper_field("puzzle3d-play-settings.chunk-size", labels.chunk_size, &[runtime.chunk_size], 1.0, puzzle3d_action("setChunkSize", None)),
            ui_inspector_stepper_field("puzzle3d-play-settings.grid-spacing", labels.spacing, &[runtime.grid_spacing], 0.5, puzzle3d_action("setGridSpacing", None)),
        ],
    }])
}
//#endregion 🔖️Render
