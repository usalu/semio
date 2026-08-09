//! 🔍️ DAG play app panel — the per-node inspector (name/kind/id plus slider-specific fields).

use crate::apps::dag::terminology::DagPlayLabels;
use crate::apps::dag::dag_action;
use crate::artifacts::dag::DagSnapshot;
use infinite_board_port_directed_dag::{dag_node_kind_tag, DagNodeKind, DagNodeSpec};
use semio_framework_plugin::{
    ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_mixed_number, ui_inspector_mixed_text, ui_inspector_readonly_field, ui_text, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiFieldNode, UiInputNode,
    UiInspectorFieldGroup, UiNode, UiPresence, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, UI_INSPECTOR_MIXED_PLACEHOLDER,
};
use serde_json::json;

//#region 🔖️Constants
pub const DAG_PLAY_BODY_INSPECTOR: &str = "dag.play.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()), label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"), group: PanelGroup::Details, body_key: Some(DAG_PLAY_BODY_INSPECTOR.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Fields
fn inspector_number_field(node_ids: &[String], field_id: &str, label: impl Into<Label>, values: &[f64], field: &str) -> UiNode {
    let mixed = ui_inspector_mixed_number(values);
    UiNode::Field(UiFieldNode {
        presence: UiPresence::default(),
        id: field_id.into(),
        label: label.into(),
        child: Box::new(UiNode::Input(UiInputNode {
            presence: UiPresence::default(),
            id: format!("{field_id}.input"),
            input_kind: "number".into(),
            value: if mixed.uniform { mixed.value.to_string() } else { String::new() },
            placeholder: if mixed.uniform { None } else { Some(Label::data(UI_INSPECTOR_MIXED_PLACEHOLDER)) },
            commit: None,
            on_change: dag_action("patchDagNodes", Some(json!({ "nodeIds": node_ids, "field": field }))),
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

fn inspector_text_field(node_ids: &[String], field_id: &str, label: impl Into<Label>, values: &[String], field: &str) -> UiNode {
    let mixed = ui_inspector_mixed_text(values);
    UiNode::Field(UiFieldNode {
        presence: UiPresence::default(),
        id: field_id.into(),
        label: label.into(),
        child: Box::new(UiNode::Input(UiInputNode {
            presence: UiPresence::default(),
            id: format!("{field_id}.input"),
            input_kind: "text".into(),
            value: mixed.value,
            placeholder: mixed.placeholder.map(Label::data),
            commit: Some("blur".into()),
            on_change: dag_action("patchDagNodes", Some(json!({ "nodeIds": node_ids, "field": field }))),
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
//#endregion 🔖️Fields

//#region 🔖️Render
pub fn render(document: &DagSnapshot, selected: &[String], labels: &DagPlayLabels) -> UiNode {
    if selected.is_empty() {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "dag-play-inspector.empty".into(),
            label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
            default_open: Some(true),
            presence: UiPresence::default(),
            children: vec![ui_text(labels.select_a_node)],
            menu: None,
        }]);
    }
    let nodes: Vec<&DagNodeSpec> = selected.iter().filter_map(|id| document.nodes.iter().find(|node| &node.id == id)).collect();
    if nodes.is_empty() {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "dag-play-inspector.missing".into(),
            label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
            default_open: Some(true),
            presence: UiPresence::default(),
            children: vec![ui_text(labels.node_not_found)],
            menu: None,
        }]);
    }
    let node_ids: Vec<String> = nodes.iter().map(|node| node.id.clone()).collect();
    let mut groups: Vec<UiInspectorFieldGroup> = Vec::new();
    if nodes.iter().all(|node| matches!(node.kind, DagNodeKind::Slider { .. })) {
        groups.push(UiInspectorFieldGroup {
            presence: UiPresence::default(),
            id: "dag-play-inspector.kind.slider".into(),
            label: labels.slider_group.into(),
            default_open: None,
            fields: vec![
                inspector_number_field(
                    &node_ids,
                    "dag-play-inspector.slider-value",
                    labels.field_value,
                    &nodes
                        .iter()
                        .map(|node| match &node.kind {
                            DagNodeKind::Slider { value, .. } => *value,
                            _ => 0.0,
                        })
                        .collect::<Vec<_>>(),
                    "value",
                ),
                inspector_number_field(
                    &node_ids,
                    "dag-play-inspector.slider-min",
                    labels.field_min,
                    &nodes
                        .iter()
                        .map(|node| match &node.kind {
                            DagNodeKind::Slider { min, .. } => *min,
                            _ => 0.0,
                        })
                        .collect::<Vec<_>>(),
                    "min",
                ),
                inspector_number_field(
                    &node_ids,
                    "dag-play-inspector.slider-max",
                    labels.field_max,
                    &nodes
                        .iter()
                        .map(|node| match &node.kind {
                            DagNodeKind::Slider { max, .. } => *max,
                            _ => 0.0,
                        })
                        .collect::<Vec<_>>(),
                    "max",
                ),
            ],
        });
    }
    let mut base_fields = vec![
        inspector_text_field(&node_ids, "dag-play-inspector.name", labels.field_name, &nodes.iter().map(|node| node.name.clone()).collect::<Vec<_>>(), "name"),
        ui_inspector_readonly_field(
            "dag-play-inspector.kind",
            labels.field_kind,
            if nodes.iter().map(|node| dag_node_kind_tag(&node.kind)).collect::<std::collections::HashSet<_>>().len() == 1 { dag_node_kind_tag(&nodes[0].kind).to_string() } else { "—".into() },
        ),
    ];
    if node_ids.len() == 1 {
        base_fields.insert(
            0,
            UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: "dag-play-inspector.id".into(),
                label: labels.field_id.into(),
                child: Box::new(UiNode::Input(UiInputNode {
                    presence: UiPresence::default(),
                    id: "dag-play-inspector.id.input".into(),
                    input_kind: "text".into(),
                    value: node_ids[0].clone(),
                    placeholder: None,
                    commit: Some("blur".into()),
                    on_change: dag_action("renameDagNode", Some(json!({ "oldId": node_ids[0] }))),
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
            }),
        );
    } else {
        base_fields.insert(0, ui_inspector_readonly_field("dag-play-inspector.id", labels.field_id, format!("{} {}", node_ids.len(), labels.selected_suffix.as_str())));
    }
    groups.push(UiInspectorFieldGroup { presence: UiPresence::default(), id: "dag-play-inspector.base".into(), label: labels.node_group.into(), default_open: None, fields: base_fields });
    ui_inspector_groups_to_tree(&groups)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::dag::commands::selection::set_selection;
    use crate::apps::dag::testkit::{dispatch, new_app, render as render_body};
    use crate::apps::dag::DagCommand;

    #[test]
    fn definition_binds_the_framework_inspection_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_INSPECTION_ID);
        assert_eq!(definition.body_key.as_deref(), Some(DAG_PLAY_BODY_INSPECTOR));
    }

    #[test]
    fn renders_the_select_a_node_placeholder_when_nothing_is_selected() {
        let mut app = new_app();
        assert!(render_body(&mut app, DAG_PLAY_BODY_INSPECTOR).contains("Select a node"));
    }

    #[test]
    fn renders_id_name_and_kind_fields_for_a_single_selected_node() {
        let mut app = new_app();
        let node_id = app.snapshot().expect("projection").nodes.first().map(|node| node.id.clone()).expect("node");
        dispatch(&mut app, DagCommand::SetSelection(set_selection::SetSelection { ids: vec![node_id.clone()] }));
        let json = render_body(&mut app, DAG_PLAY_BODY_INSPECTOR);
        assert!(json.contains(&node_id));
        assert!(json.contains("Name") || json.contains("Kind"));
    }
}
//#endregion 🧪️Tests
