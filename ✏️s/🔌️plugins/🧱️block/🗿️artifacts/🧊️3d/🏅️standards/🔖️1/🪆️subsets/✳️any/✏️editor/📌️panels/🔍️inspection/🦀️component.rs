//! 🔍️ Block 3D play app panel — the inspector: the object kind's identity fields, active-representation
//! select, plus a vortex count.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::editor::block3d::block3d_action;
use crate::editor::block3d::terminology::Block3dLabels;
use semio_framework_plugin::{
    ui_inspector_groups_to_tree, ui_inspector_readonly_field, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiFieldNode, UiInputNode, UiInspectorFieldGroup, UiNode, UiPresence, UiSelectItem, UiSelectNode,
    FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde_json::json;

//#region 🔖️Constants
pub const BLOCK3D_BODY_INSPECTOR: &str = "block3d.play.inspector";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(BLOCK3D_BODY_INSPECTOR.into()),
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
            on_change: block3d_action("patchObjectKind", Some(json!({ "field": field }))),
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

pub async fn render(definition: &Block3dSnapshot, active_representation_id: Option<&str>, labels: &Block3dLabels) -> UiNode {
    let representation_select = UiNode::Select(UiSelectNode {
        id: "block3d-play-inspector.representation".into(),
        value: active_representation_id.unwrap_or_default().into(),
        items: definition.representations.iter().map(|representation| UiSelectItem { value: representation.id.clone(), label: Label::data(representation.name.clone()) }).collect(),
        placeholder: None,
        on_change: block3d_action("setActiveRepresentation", None),
        presence: UiPresence::default(),
        menu: None,
    });
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "block3d-play-inspector".into(),
        label: labels.summary.into(),
        default_open: Some(true),
        presence: UiPresence::default(),
        fields: vec![
            text_field("block3d-play-inspector.name", labels.name, &definition.object_kind.name, "name"),
            text_field("block3d-play-inspector.label", labels.label, &definition.object_kind.label, "label"),
            UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: "block3d-play-inspector.representation-field".into(),
                label: labels.representation.into(),
                child: Box::new(representation_select),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
            ui_inspector_readonly_field("block3d-play-inspector.vortex-count", labels.vortices, definition.vortices.len().to_string()),
        ],
    }])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::block3d::testkit::{new_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn renders_inspector_fields() {
        let mut app = new_app();
        let json = render_body(&mut app, BLOCK3D_BODY_INSPECTOR);
        assert!(json.contains("\"type\":\"tree\""), "inspection body must be a tree like document");
        assert!(json.contains("Name"));
        assert!(json.contains("Vortices"));
        assert!(!json.contains("\"type\":\"stack\""), "inspection body must not be a free-form stack");
    }
}
//#endregion 🧪️Tests
