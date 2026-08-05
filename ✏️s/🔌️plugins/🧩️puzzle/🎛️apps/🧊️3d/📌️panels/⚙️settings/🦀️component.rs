//! ⚙️ Puzzle 3d play app panel — the app-wide settings: the default selection merge mode, the brush
//! placement overlap budget, the relocate proximity radius, the viewport chunk size and the grid
//! spacing. Unlike the window-instance chrome in `🎭️modes/✏️edit/🎚️options/*`, these are settings a
//! user tunes once for the whole session rather than per pane.

use crate::apps::puzzle3d::puzzle3d_action;
use crate::apps::puzzle3d::terminology::Puzzle3dLabels;
use crate::apps::puzzle3d::Puzzle3dScene;
use semio_framework_plugin::{ui_inspector_groups_to_tree, ui_inspector_stepper_field, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiFieldNode, UiInspectorFieldGroup, UiNode, UiPresence};

//#region 🔖️Constants
pub const PANEL_TAB_ID: &str = "puzzle3d.panel.settings";
pub const BODY_KEY: &str = "puzzle.3d.play.settings";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(PANEL_TAB_ID.into()),
        label: LocalizedLabel::native("Settings", "Einstellungen"),
        group: PanelGroup::Settings,
        body_key: Some(BODY_KEY.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(envelope: &Puzzle3dScene, labels: &Puzzle3dLabels) -> UiNode {
    let runtime = &envelope.runtime;
    let selection_mode_field = UiNode::Field(UiFieldNode {
        id: "puzzle3d-play-settings.selection-mode".into(),
        label: labels.selection_mode.into(),
        child: Box::new(UiNode::Select(semio_framework_plugin::UiSelectNode {
            id: "puzzle3d-play-settings.selection-mode.input".into(),
            value: runtime.selection_mode_default.clone(),
            items: vec![
                semio_framework_plugin::UiSelectItem { value: "default".into(), label: labels.selective.into() },
                semio_framework_plugin::UiSelectItem { value: "additive".into(), label: labels.additive.into() },
                semio_framework_plugin::UiSelectItem { value: "subtractive".into(), label: labels.subtractive.into() },
                semio_framework_plugin::UiSelectItem { value: "invertive".into(), label: labels.invertive.into() },
            ],
            placeholder: None,
            on_change: puzzle3d_action("setSelectionModeDefault", None),
            presence: UiPresence::default(),
            menu: None,
        })),
        description: None,
        required: None,
        error: None,
        presence: UiPresence::default(),
        menu: None,
    });
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "puzzle3d-play-settings".into(),
        label: labels.settings.into(),
        default_open: Some(true),
        presence: UiPresence::default(),
        fields: vec![
            selection_mode_field,
            ui_inspector_stepper_field("puzzle3d-play-settings.overlap-budget", labels.overlap_budget, &[runtime.overlap_budget], 0.05, puzzle3d_action("setBrushPlacementOverlapBudget", None)),
            ui_inspector_stepper_field("puzzle3d-play-settings.proximity-radius", labels.proximity_radius, &[runtime.proximity_radius], 0.1, puzzle3d_action("setProximityRadius", None)),
            ui_inspector_stepper_field("puzzle3d-play-settings.chunk-size", labels.chunk_size, &[runtime.chunk_size], 1.0, puzzle3d_action("setChunkSize", None)),
            ui_inspector_stepper_field("puzzle3d-play-settings.grid-spacing", labels.spacing, &[runtime.grid_spacing], 0.5, puzzle3d_action("setGridSpacing", None)),
        ],
    }])
}
//#endregion 🔖️Render
