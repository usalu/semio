//! 🔍️ Trinity Rewrite app — Inspection panel (selected before-fixture node geometry/identity fields).

use crate::apps::rewrite::config::RewriteConfig;
use crate::apps::rewrite::terminology::TrinityRewriteLabels;
use crate::artifacts::jack::{Node, PropertyValue};
use crate::artifacts::rewrite::RewriteSnapshot;
use semio_framework_plugin::{
    ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_mixed_text, ui_inspector_readonly_field, ui_text, Label, UiFieldNode, UiInspectorFieldGroup, UiNode, UiPresence, UiSectionNode, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
    UI_INSPECTOR_MIXED_PLACEHOLDER,
};
use serde_json::json;

fn flat_position_uv(node: &Node) -> (String, String) {
    let Some(flat) = node.properties.get("flatPosition").and_then(PropertyValue::as_object) else {
        return (String::new(), String::new());
    };
    let format_axis = |axis: &str| flat.get(axis).and_then(PropertyValue::as_f64).map(|value| format!("{value:.2}")).unwrap_or_default();
    (format_axis("u"), format_axis("v"))
}

fn fixture_with_derived(fixture_json: &str) -> Option<crate::artifacts::jack::JackSnapshot> {
    let mut graph = crate::artifacts::jack::Graph::load_json(fixture_json).ok()?;
    graph.recompute_derived();
    Some(graph.to_fixture())
}

pub(crate) fn render(state: &RewriteSnapshot, cfg: &RewriteConfig, term_labels: &TrinityRewriteLabels) -> UiNode {
    let jack_action = crate::apps::rewrite::rewrite_action;
    let Some(fixture) = crate::apps::rewrite::parse_fixture_json(&state.before_fixture_json) else {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "trinity-inspector.empty".into(),
            label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
            default_open: Some(true),
            children: vec![ui_text(Label::data("Invalid trinity fixture"))],
            presence: UiPresence::default(),
            menu: None,
        }]);
    };
    if cfg.selected_node_ids.is_empty() {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "trinity-inspector.empty".into(),
            label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
            default_open: Some(true),
            presence: UiPresence::default(),
            children: vec![ui_text(Label::data("Select one or more pieces"))],
            menu: None,
        }]);
    }
    let nodes: Vec<&Node> = cfg.selected_node_ids.iter().filter_map(|id| fixture.nodes.iter().find(|node| &node.id == id)).collect();
    if nodes.is_empty() {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "trinity-inspector.empty".into(),
            label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
            default_open: Some(true),
            children: vec![ui_text(Label::data("Piece not found"))],
            presence: UiPresence::default(),
            menu: None,
        }]);
    }
    let node_ids: Vec<String> = nodes.iter().map(|node| node.id.clone()).collect();
    let name_mixed = ui_inspector_mixed_text(&nodes.iter().map(|node| node.name.clone()).collect::<Vec<_>>());
    let kind_mixed = ui_inspector_mixed_text(&nodes.iter().map(|node| node.kind.clone()).collect::<Vec<_>>());
    let derived_fixture = fixture_with_derived(&state.before_fixture_json);
    let derived_uv = |id: &str| -> (String, String) { derived_fixture.as_ref().and_then(|fixture| fixture.nodes.iter().find(|node| node.id == id)).map(flat_position_uv).unwrap_or_default() };
    let u_values: Vec<String> = node_ids.iter().map(|id| derived_uv(id).0).collect();
    let v_values: Vec<String> = node_ids.iter().map(|id| derived_uv(id).1).collect();
    let u_mixed = ui_inspector_mixed_text(&u_values);
    let v_mixed = ui_inspector_mixed_text(&v_values);
    ui_inspector_groups_to_tree(&[
        UiInspectorFieldGroup {
            presence: UiPresence::default(),
            id: "trinity-inspector.geometry".into(),
            label: term_labels.geometry.into(),
            default_open: None,
            fields: vec![
                ui_inspector_readonly_field(
                    "trinity-inspector.flat-u",
                    Label::data("Flat U"),
                    if u_mixed.placeholder.is_none() { u_values.first().cloned().unwrap_or_default() } else { u_mixed.placeholder.unwrap_or_else(|| UI_INSPECTOR_MIXED_PLACEHOLDER.into()) },
                ),
                ui_inspector_readonly_field(
                    "trinity-inspector.flat-v",
                    Label::data("Flat V"),
                    if v_mixed.placeholder.is_none() { v_values.first().cloned().unwrap_or_default() } else { v_mixed.placeholder.unwrap_or_else(|| UI_INSPECTOR_MIXED_PLACEHOLDER.into()) },
                ),
            ],
        },
        UiInspectorFieldGroup {
            presence: UiPresence::default(),
            id: "trinity-inspector.identity".into(),
            label: term_labels.identity.into(),
            default_open: None,
            fields: vec![
                UiNode::Field(UiFieldNode {
                    presence: UiPresence::default(),
                    id: "trinity-inspector.name".into(),
                    label: Label::data("Name"),
                    child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
                        presence: UiPresence::default(),
                        id: "trinity-inspector.name.input".into(),
                        input_kind: "text".into(),
                        value: name_mixed.value,
                        placeholder: name_mixed.placeholder.map(Label::data),
                        commit: None,
                        on_change: jack_action("patchNodes", Some(json!({ "nodeIds": node_ids, "field": "name" }))),
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
                ui_inspector_readonly_field(
                    "trinity-inspector.kind",
                    Label::data("Kind"),
                    if kind_mixed.placeholder.is_none() { nodes.first().map(|node| node.kind.clone()).unwrap_or_default() } else { kind_mixed.placeholder.unwrap_or_else(|| UI_INSPECTOR_MIXED_PLACEHOLDER.into()) },
                ),
            ],
        },
    ])
}
