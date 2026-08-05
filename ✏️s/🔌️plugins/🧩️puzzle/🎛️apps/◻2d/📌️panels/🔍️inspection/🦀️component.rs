//! 🔍️ Puzzle 2d play app panel — the inspector: per-selection id/kind/x/y fields (steppers patch the
//! whole multi-selection at once), falling back to a document summary when nothing is selected.

use crate::apps::puzzle2d::terminology::Puzzle2dLabels;
use crate::apps::puzzle2d::{fixture_edges, fixture_nodes, puzzle2d_action, puzzle_extension_id, Puzzle2dScene, PUZZLE2D_FIXTURE_SCHEMA};
use semio_framework_plugin::{
    ui_inspector_groups_to_tree, ui_inspector_mixed_text, ui_inspector_readonly_field, ui_inspector_stepper_field, ui_stack_vertical, ui_text, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiInspectorFieldGroup, UiNode,
    UiPresence, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const PUZZLE2D_PLAY_BODY_PROPERTIES: &str = "puzzle2d.play.properties";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(PUZZLE2D_PLAY_BODY_PROPERTIES.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(envelope: &Puzzle2dScene, labels: &Puzzle2dLabels) -> UiNode {
    let selected_nodes: Vec<&Value> = envelope.runtime.selected_ids.iter().filter_map(|id| fixture_nodes(&envelope.fixture).iter().find(|node| node.get("id").and_then(|value| value.as_str()) == Some(id.as_str()))).collect();
    if selected_nodes.is_empty() {
        return ui_stack_vertical(vec![
            ui_text(Label::data(format!("{}: {PUZZLE2D_FIXTURE_SCHEMA}", labels.schema.as_str()))),
            ui_text(Label::data(format!("{}: {}", labels.extension.as_str(), puzzle_extension_id()))),
            ui_text(Label::data(format!("{}: {}", labels.nodes.as_str(), fixture_nodes(&envelope.fixture).len()))),
            ui_text(Label::data(format!("{}: {}", labels.edges.as_str(), fixture_edges(&envelope.fixture).len()))),
        ]);
    }
    let ids: Vec<String> = selected_nodes.iter().filter_map(|node| node.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect();
    let ids_json = json!(ids);
    let patch_cmd = |field: &str| puzzle2d_action("patchInspectorNodes", Some(json!({ "ids": ids_json, "field": field })));
    let kinds: Vec<String> = selected_nodes.iter().map(|node| node.get("nodeKind").and_then(|value| value.as_str()).unwrap_or("").to_string()).collect();
    let xs: Vec<f64> = selected_nodes.iter().map(|node| node.get("x").and_then(|value| value.as_f64()).unwrap_or(0.0)).collect();
    let ys: Vec<f64> = selected_nodes.iter().map(|node| node.get("y").and_then(|value| value.as_f64()).unwrap_or(0.0)).collect();
    let id_text = if let [id] = ids.as_slice() { id.clone() } else { format!("{} nodes", ids.len()) };
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "puzzle2d-play-inspector".into(),
        label: labels.node_kind.into(),
        default_open: Some(true),
        presence: UiPresence::default(),
        fields: vec![
            ui_inspector_readonly_field("puzzle2d-play-inspector.id", labels.id, id_text),
            ui_inspector_readonly_field("puzzle2d-play-inspector.node-kind", labels.node_kind, ui_inspector_mixed_text(&kinds).value),
            ui_inspector_stepper_field("puzzle2d-play-inspector.x", labels.x, &xs, 1.0, patch_cmd("x")),
            ui_inspector_stepper_field("puzzle2d-play-inspector.y", labels.y, &ys, 1.0, patch_cmd("y")),
        ],
    }])
}
//#endregion 🔖️Render
