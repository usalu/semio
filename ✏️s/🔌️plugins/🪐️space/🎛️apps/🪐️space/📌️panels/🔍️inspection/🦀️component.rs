//! 🔍️ S Studio app — inspector panel: selected node position (Transform-ish section) and
//! identity/parameter-binding facets (Properties-ish section), both driven off the `graph` interaction
//! domain's live selection (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — passed in by
//! the caller, since `ArtifactApp::render` carries no `InteractionView` (a discovered framework gap).

use crate::apps::space::config::SpaceConfig;
use crate::apps::space::engine::{os_parameter_types_compatible_shim, parameter_entity_id, workflow_parameter_to_os};
use crate::apps::space::terminology::SStudioLabels;
use crate::apps::space::{s_play_action, S_PLAY_INSPECTOR_BODY_KEY, S_PLAY_INSPECTOR_TAB_ID};
use semio_framework_os::{os_app_registration, os_parameter_value, WorkflowSnapshot, WorkflowNode, WorkflowParameter};
use semio_framework_plugin::{
    ui_declarative_sections_to_tree, ui_inspector_all_equal, ui_text, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiFieldNode, UiInputNode, UiNode, UiPresence, UiSectionNode, UiSelectItem, UiSelectNode,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde_json::json;

//#region 🔖️Manifest
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(S_PLAY_INSPECTOR_TAB_ID.into()), label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"), group: PanelGroup::Details, body_key: Some(S_PLAY_INSPECTOR_BODY_KEY.into()), children: Vec::new() }
}
//#endregion 🔖️Manifest

//#region 🔖️Render
pub fn render(projection: &WorkflowSnapshot, selected_node_ids: &[String], term_labels: &SStudioLabels) -> UiNode {
    let mut children = vec![UiSectionNode {
        id: "s-play-inspector.header".into(),
        label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
        default_open: Some(true),
        presence: UiPresence::default(),
        children: vec![ui_text(Label::data(format!("{} {}", selected_node_ids.len(), term_labels.media_node_count_label.as_str())))],
        menu: None,
    }];
    let nodes: Vec<&WorkflowNode> = selected_node_ids.iter().filter_map(|node_id| projection.graph.nodes.iter().find(|node| &node.id == node_id)).collect();
    if !nodes.is_empty() {
        let xs: Vec<_> = nodes.iter().map(|node| node.x).collect();
        let ys: Vec<_> = nodes.iter().map(|node| node.y).collect();
        let x_uniform = ui_inspector_all_equal(&xs.iter().map(|v| v.to_string()).collect::<Vec<_>>());
        let y_uniform = ui_inspector_all_equal(&ys.iter().map(|v| v.to_string()).collect::<Vec<_>>());
        let mut node_fields = Vec::new();
        if selected_node_ids.len() == 1 {
            node_fields.push(UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: "s-play-inspector.media-node.id".into(),
                label: term_labels.node_id.into(),
                child: Box::new(UiNode::Input(UiInputNode {
                    presence: UiPresence::default(),
                    id: "s-play-inspector.media-node.id.input".into(),
                    input_kind: "text".into(),
                    value: selected_node_ids[0].clone(),
                    placeholder: None,
                    commit: None,
                    on_change: s_play_action("noOperation", None),
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
            }));
        }
        node_fields.push(UiNode::Field(UiFieldNode {
            presence: UiPresence::default(),
            id: "s-play-inspector.media-node.x".into(),
            // 📊️ Coordinate-axis notation, identical in every locale — genuine runtime/technical data,
            // not translatable prose.
            label: Label::data("X"),
            child: Box::new(UiNode::Input(UiInputNode {
                presence: UiPresence::default(),
                id: "s-play-inspector.media-node.x.input".into(),
                input_kind: "number".into(),
                value: if x_uniform { xs.first().map(|v| v.to_string()).unwrap_or_default() } else { String::new() },
                placeholder: if x_uniform { None } else { Some(term_labels.mixed_placeholder.into()) },
                commit: None,
                on_change: s_play_action("patchMediaNodes", Some(json!({ "nodeIds": selected_node_ids, "field": "position", "axis": "x" }))),
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
        }));
        node_fields.push(UiNode::Field(UiFieldNode {
            presence: UiPresence::default(),
            id: "s-play-inspector.media-node.y".into(),
            label: Label::data("Y"),
            child: Box::new(UiNode::Input(UiInputNode {
                presence: UiPresence::default(),
                id: "s-play-inspector.media-node.y.input".into(),
                input_kind: "number".into(),
                value: if y_uniform { ys.first().map(|v| v.to_string()).unwrap_or_default() } else { String::new() },
                placeholder: if y_uniform { None } else { Some(term_labels.mixed_placeholder.into()) },
                commit: None,
                on_change: s_play_action("patchMediaNodes", Some(json!({ "nodeIds": selected_node_ids, "field": "position", "axis": "y" }))),
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
        }));
        children.push(UiSectionNode {
            id: "s-play-inspector.media-nodes".into(),
            label: Some(if selected_node_ids.len() == 1 { term_labels.workflow_node.into() } else { Label::data(format!("{} ({})", term_labels.workflow_nodes.as_str(), selected_node_ids.len())) }),
            default_open: Some(true),
            presence: UiPresence::default(),
            children: node_fields,
            menu: None,
        });

        let labels: Vec<_> = nodes.iter().map(|node| node.label.clone()).collect();
        let programs: Vec<_> = nodes.iter().map(|node| node.plugin_id.clone()).collect();
        let apps: Vec<_> = nodes.iter().map(|node| node.app_id.clone()).collect();
        let label_uniform = ui_inspector_all_equal(&labels);
        let program_uniform = ui_inspector_all_equal(&programs);
        let app_uniform = ui_inspector_all_equal(&apps);
        let mut instance_fields = vec![
            ui_text(Label::data(format!("{}: {}", term_labels.program_prefix.as_str(), if program_uniform { programs.first().cloned().unwrap_or_default() } else { term_labels.mixed_placeholder.as_str().to_string() }))),
            ui_text(Label::data(format!("{}: {}", term_labels.app_prefix.as_str(), if app_uniform { apps.first().cloned().unwrap_or_default() } else { term_labels.mixed_placeholder.as_str().to_string() }))),
            UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: "s-play-inspector.app-instance.label".into(),
                label: term_labels.label.into(),
                child: Box::new(UiNode::Input(UiInputNode {
                    presence: UiPresence::default(),
                    id: "s-play-inspector.app-instance.label.input".into(),
                    input_kind: "text".into(),
                    value: if label_uniform { labels.first().cloned().unwrap_or_default() } else { String::new() },
                    placeholder: if label_uniform { None } else { Some(term_labels.mixed_placeholder.into()) },
                    commit: None,
                    on_change: s_play_action("patchAppInstances", Some(json!({ "nodeIds": selected_node_ids, "field": "label" }))),
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
        ];
        if selected_node_ids.len() == 1 {
            instance_fields.insert(2, ui_text(Label::data(format!("{}: {}", term_labels.instance_id_prefix.as_str(), selected_node_ids[0]))));
        }
        if selected_node_ids.len() == 1 {
            if let Some(node) = nodes.first() {
                if let Some(registration) = os_app_registration(&node.plugin_id, &node.app_id) {
                    for field_spec in &registration.parameter_fields {
                        let binding = projection.parameter_bindings.iter().find(|entry| entry.node_id == node.id && entry.field_path == field_spec.field_path);
                        let compatible: Vec<_> = projection.parameters.iter().filter(|parameter| os_parameter_types_compatible_shim(parameter, &field_spec.parameter_type)).collect();
                        let mut items = vec![UiSelectItem { value: "__direct__".into(), label: term_labels.direct_value.into() }];
                        for parameter in compatible {
                            items.push(UiSelectItem {
                                value: parameter_entity_id(parameter).into(),
                                label: Label::data(match parameter {
                                    WorkflowParameter::Numeric { name, .. } | WorkflowParameter::Categorical { name, .. } | WorkflowParameter::Toggle { name, .. } | WorkflowParameter::Text { name, .. } => name.clone(),
                                }),
                            });
                        }
                        instance_fields.push(UiNode::Field(UiFieldNode {
                            presence: UiPresence::default(),
                            id: format!("s-play-inspector.app-parameter.{}", field_spec.field_path),
                            label: Label::data(field_spec.label.clone()),
                            child: Box::new(UiNode::Select(UiSelectNode {
                                presence: UiPresence::default(),
                                id: format!("s-play-inspector.app-parameter.{}.select", field_spec.field_path),
                                value: binding.map_or_else(|| "__direct__".into(), |entry| entry.parameter_id.clone()),
                                items,
                                placeholder: None,
                                on_change: s_play_action("bindParameterField", Some(json!({ "nodeId": node.id, "fieldPath": field_spec.field_path }))),
                                menu: None,
                            })),
                            description: None,
                            required: None,
                            error: None,
                            menu: None,
                        }));
                        if let Some(binding) = binding {
                            if let Some(parameter) = projection.parameters.iter().find(|entry| parameter_entity_id(entry) == binding.parameter_id) {
                                instance_fields.push(ui_text(Label::data(format!("{}: {}", term_labels.bound_value_prefix.as_str(), os_parameter_value(&workflow_parameter_to_os(parameter))))));
                            }
                        }
                    }
                }
            }
        }
        children.push(UiSectionNode {
            id: "s-play-inspector.app-instances".into(),
            label: Some(if selected_node_ids.len() == 1 { term_labels.app_instance.into() } else { Label::data(format!("{} ({})", term_labels.app_instances.as_str(), selected_node_ids.len())) }),
            default_open: Some(true),
            presence: UiPresence::default(),
            children: instance_fields,
            menu: None,
        });
    } else {
        children[0].children.push(ui_text(term_labels.select_hint));
    }
    ui_declarative_sections_to_tree(&children)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::demo_space_projection;
    use semio_framework_plugin::UiControlNode;

    #[test]
    fn inspector_tree_exposes_label_field() {
        let projection = demo_space_projection();
        let ids: Vec<String> = projection.graph.nodes.iter().take(2).map(|node| node.id.clone()).collect();
        let config = SpaceConfig::default();
        let tree = render(&projection, &ids, semio_framework_plugin::resolve_labels_for_locale::<SStudioLabels>(&config.locale));
        let UiNode::Tree(tree_node) = tree else {
            panic!("expected tree");
        };
        let section = tree_node.sections.iter().find(|section| section.id == "s-play-inspector.app-instances").expect("instances section");
        let label_field = section.items.iter().find(|item| item.id == "s-play-inspector.app-instance.label").expect("label field");
        let control = label_field.control.as_ref().expect("label control");
        let UiControlNode::Input(input) = control else {
            panic!("expected input control");
        };
        assert_eq!(input.on_change.action, "patchAppInstances");
    }
}
//#endregion 🧪️Tests
