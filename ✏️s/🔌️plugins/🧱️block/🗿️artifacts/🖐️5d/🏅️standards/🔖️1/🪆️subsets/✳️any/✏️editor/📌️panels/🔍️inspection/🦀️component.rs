//! 🔍️ Block 5D play app panel — the inspector: the part kind's identity fields plus a grip count.

use crate::editor::block5d::block5d_action;
use crate::editor::block5d::terminology::Block5dLabels;
use crate::artifacts::block5d::Block5dSnapshot;
use semio_framework_plugin::{
    ui_inspector_groups_to_tree, ui_inspector_readonly_field, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiFieldNode, UiInputNode, UiInspectorFieldGroup, UiNode, UiPresence, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde_json::json;

//#region 🔖️Constants
pub const BLOCK5D_BODY_INSPECTOR: &str = "block5d.play.inspector";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(BLOCK5D_BODY_INSPECTOR.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
async fn text_field(id: &str, label: impl Into<Label>, value: &str, field: &str) -> UiNode {
    UiNode::Field(UiFieldNode {
        presence: UiPresence::default(),
        id: id.into(),
        label: label.into(),
        child: Box::new(UiNode::Input(UiInputNode {
            presence: UiPresence::default(),
            id: format!("{id}.input"),
            input_kind: "text".into(),
            value: value.into(),
            placeholder: None,
            commit: Some("blur".into()),
            on_change: block5d_action("patchPartKind", Some(json!({ "field": field }))),
            min: None,
            max: None,
            step: None,
            accept: None,
            menu: None,
        })),
        description: None,
        required: None,
        error: None,
        menu: None,
    })
}

pub async fn render(definition: &Block5dSnapshot, labels: &Block5dLabels) -> UiNode {
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "block5d-play-inspector".into(),
        label: labels.summary.into(),
        default_open: Some(true),
        presence: UiPresence::default(),
        fields: vec![
            text_field("block5d-play-inspector.name", labels.name, &definition.part_kind.name, "name"),
            text_field("block5d-play-inspector.label", labels.label, &definition.part_kind.label, "label"),
            ui_inspector_readonly_field("block5d-play-inspector.grip-count", labels.grips, definition.grips.len().to_string()),
        ],
    }])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::block5d::testkit::{new_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn renders_inspector_fields() {
        let mut app = new_app();
        let json = render_body(&mut app, BLOCK5D_BODY_INSPECTOR);
        assert!(json.contains("\"type\":\"tree\""), "inspection body must be a tree like document");
        assert!(json.contains("Name"));
        assert!(!json.contains("\"type\":\"stack\""), "inspection body must not be a free-form stack");
    }
}
//#endregion 🧪️Tests
