//! 🔍️ Block 2D play app panel — the inspector: the node kind's identity fields plus a handle count.

use crate::artifacts::block2d::Block2dSnapshot;
use crate::editor::block2d::{block2d_action, ui_value_map, ui_value_text};
use crate::editor::block2d::terminology::Block2dLabels;
use semio_framework_plugin::{
    ui_inspector_groups_to_tree, ui_inspector_readonly_field, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiFieldNode, UiInputNode, UiInspectorFieldGroup, UiNode, UiPresence, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};

//#region 🔖️Constants
pub const BLOCK2D_BODY_INSPECTOR: &str = "block2d.play.inspector";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(BLOCK2D_BODY_INSPECTOR.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
async fn text_field(id: &str, label: impl Into<Label>, value: &str, field: &'static str) -> UiNode {
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
            on_change: block2d_action("patchNodeKind", Some(ui_value_map([("field", ui_value_text(field).expect("static field name fits ui text capacity"))]).expect("single-entry field map fits ui map capacity"))),
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

pub async fn render(definition: &Block2dSnapshot, labels: &Block2dLabels) -> UiNode {
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "block2d-play-inspector".into(),
        label: labels.summary.into(),
        default_open: Some(true),
        presence: UiPresence::default(),
        fields: vec![
            text_field("block2d-play-inspector.name", labels.name, &definition.node_kind.name, "name"),
            text_field("block2d-play-inspector.label", labels.label, &definition.node_kind.label, "label"),
            text_field("block2d-play-inspector.variant", labels.variant, definition.node_kind.variant.as_deref().unwrap_or(""), "variant"),
            text_field("block2d-play-inspector.description", labels.description, &definition.node_kind.description, "description"),
            ui_inspector_readonly_field("block2d-play-inspector.handle-count", labels.handles, definition.handles.len().to_string()),
        ],
    }])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::block2d::testkit::{new_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn renders_inspector_fields() {
        let mut app = new_app();
        let json = render_body(&mut app, BLOCK2D_BODY_INSPECTOR);
        assert!(json.contains("\"type\":\"tree\""), "inspection body must be a tree like document");
        assert!(json.contains("Name"));
        assert!(!json.contains("\"type\":\"stack\""), "inspection body must not be a free-form stack");
    }
}
//#endregion 🧪️Tests
