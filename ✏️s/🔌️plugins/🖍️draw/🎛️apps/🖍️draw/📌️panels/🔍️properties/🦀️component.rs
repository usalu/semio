//! 🔍️ Draw play app panel — the selected-layer inspector (constitutional: was `ui`'s `Panels`
//! region, properties/inspector half).

use crate::apps::draw::config::DrawConfig;
use crate::apps::draw::draw_play_action;
use crate::apps::draw::terminology::DrawPlayLabels;
use crate::artifacts::draw::engine::{find_draw_layer, flatten_draw_layers, layer_base, rgba_to_hex};
use crate::artifacts::draw::{DrawSnapshot, DrawLayerNode, FillStyle, DRAW_BLEND_MODES, DRAW_DOCUMENT_SCHEMA};
use semio_framework_plugin::{
    ui_inspector_groups_to_tree, ui_inspector_mixed_number, ui_inspector_mixed_select, ui_inspector_mixed_slider, ui_inspector_mixed_text, ui_inspector_mixed_toggle, ui_inspector_readonly_field, ui_stack_vertical, ui_text, ActionDescriptor, Label,
    PanelGroup, PanelTabDefinition, PanelTabKind, UiFieldNode, UiInputNode, UiInspectorFieldGroup, UiNode, UiPresence, UiSelectItem, UiSelectNode, UiSliderNode, UiToggleNode, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
    UI_INSPECTOR_MIXED_PLACEHOLDER,
};
use serde_json::json;

pub const DRAW_PLAY_BODY_PROPERTIES: &str = "draw.play.properties";

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: semio_framework_plugin::LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(DRAW_PLAY_BODY_PROPERTIES.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn inspector_patch(layer_ids: &[String], field: &str) -> ActionDescriptor {
    draw_play_action("patchLayers", Some(json!({ "layerIds": layer_ids, "field": field })))
}

fn inspector_number_field(layer_ids: &[String], field_id: &str, label: impl Into<Label>, values: &[f64], field: &str) -> UiNode {
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
            on_change: inspector_patch(layer_ids, field),
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

fn inspector_text_field(layer_ids: &[String], field_id: &str, label: impl Into<Label>, values: &[String], field: &str) -> UiNode {
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
            commit: None,
            on_change: inspector_patch(layer_ids, field),
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

fn uniform_layers<'a>(layers: &[&'a DrawLayerNode]) -> Option<Vec<&'a DrawLayerNode>> {
    if layers.is_empty() {
        return None;
    }
    let kind = crate::artifacts::draw::engine::layer_kind_label(layers[0]);
    if layers.iter().all(|layer| crate::artifacts::draw::engine::layer_kind_label(layer) == kind) {
        Some(layers.to_vec())
    } else {
        None
    }
}

fn inspector_kind_group(doc: &DrawSnapshot, layers: &[&DrawLayerNode], labels: &DrawPlayLabels) -> Option<UiInspectorFieldGroup> {
    let uniform = uniform_layers(layers)?;
    let layer = uniform[0];
    let layer_ids: Vec<String> = uniform.iter().map(|entry| crate::artifacts::draw::engine::layer_id(entry).to_string()).collect();
    let mut fields: Vec<UiNode> = Vec::new();
    match layer {
        DrawLayerNode::Boolean(_boolean) => {
            let operations: Vec<String> = uniform
                .iter()
                .filter_map(|entry| match entry {
                    DrawLayerNode::Boolean(entry) => Some(entry.operation.clone()),
                    _ => None,
                })
                .collect();
            let op_mixed = ui_inspector_mixed_select(&operations);
            fields.push(UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: "draw-play-inspector.boolean-operation".into(),
                label: labels.boolean_operation.into(),
                child: Box::new(UiNode::Select(UiSelectNode {
                    presence: UiPresence::default(),
                    id: "draw-play-inspector.boolean-operation.select".into(),
                    value: op_mixed.value,
                    placeholder: op_mixed.placeholder.map(Label::data),
                    items: crate::artifacts::draw::DRAW_BOOLEAN_OPERATIONS.iter().map(|operation| UiSelectItem { value: (*operation).into(), label: Label::data(*operation) }).collect(),
                    on_change: inspector_patch(&layer_ids, "booleanOperation"),
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }));
            let DrawLayerNode::Boolean(boolean) = layer else { unreachable!() };
            let child_labels = boolean.children.iter().filter_map(|child_id| find_draw_layer(doc, child_id).map(|child| layer_base(child).name.clone())).collect::<Vec<_>>().join(", ");
            fields.push(ui_inspector_readonly_field("draw-play-inspector.boolean-children", labels.children, if child_labels.is_empty() { "—".into() } else { child_labels }));
            return Some(UiInspectorFieldGroup { presence: UiPresence::default(), id: "draw-play-inspector.kind.boolean".into(), label: labels.kind_boolean.into(), default_open: None, fields });
        }
        DrawLayerNode::Trace(trace) => {
            let thresholds: Vec<f64> = uniform
                .iter()
                .filter_map(|entry| match entry {
                    DrawLayerNode::Trace(entry) => Some(entry.params.threshold),
                    _ => None,
                })
                .collect();
            let simplifies: Vec<f64> = uniform
                .iter()
                .filter_map(|entry| match entry {
                    DrawLayerNode::Trace(entry) => Some(entry.params.simplify_epsilon),
                    _ => None,
                })
                .collect();
            let threshold_mixed = ui_inspector_mixed_slider(&thresholds);
            let simplify_mixed = ui_inspector_mixed_slider(&simplifies);
            fields.push(UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: "draw-play-inspector.trace-threshold".into(),
                label: labels.trace_threshold.into(),
                child: Box::new(UiNode::Slider(UiSliderNode {
                    presence: UiPresence::default(),
                    id: "draw-play-inspector.trace-threshold.slider".into(),
                    value: if threshold_mixed.uniform { threshold_mixed.value } else { 0.0 },
                    min: 0.0,
                    max: 1.0,
                    step: 0.01,
                    on_change: inspector_patch(&layer_ids, "traceThreshold"),
                    unit: None,
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }));
            fields.push(UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: "draw-play-inspector.trace-simplify".into(),
                label: labels.simplify.into(),
                child: Box::new(UiNode::Slider(UiSliderNode {
                    presence: UiPresence::default(),
                    id: "draw-play-inspector.trace-simplify.slider".into(),
                    value: if simplify_mixed.uniform { simplify_mixed.value } else { 0.0 },
                    min: 0.0,
                    max: 10.0,
                    step: 0.1,
                    on_change: inspector_patch(&layer_ids, "traceSimplify"),
                    unit: None,
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }));
            fields.push(ui_inspector_readonly_field("draw-play-inspector.trace-source", labels.source_key, trace.source_key.clone()));
            return Some(UiInspectorFieldGroup { presence: UiPresence::default(), id: "draw-play-inspector.kind.trace".into(), label: labels.kind_trace.into(), default_open: None, fields });
        }
        DrawLayerNode::Shape(shape) if shape.shape_kind == "rect" => {
            fields.push(inspector_number_field(
                &layer_ids,
                "draw-play-inspector.rect-width",
                labels.width,
                &uniform
                    .iter()
                    .filter_map(|entry| match entry {
                        DrawLayerNode::Shape(entry) => entry.rect.as_ref().map(|rect| rect.width),
                        _ => None,
                    })
                    .collect::<Vec<_>>(),
                "rectWidth",
            ));
            fields.push(inspector_number_field(
                &layer_ids,
                "draw-play-inspector.rect-height",
                labels.height,
                &uniform
                    .iter()
                    .filter_map(|entry| match entry {
                        DrawLayerNode::Shape(entry) => entry.rect.as_ref().map(|rect| rect.height),
                        _ => None,
                    })
                    .collect::<Vec<_>>(),
                "rectHeight",
            ));
            return Some(UiInspectorFieldGroup { presence: UiPresence::default(), id: "draw-play-inspector.kind.rect".into(), label: labels.kind_rectangle.into(), default_open: None, fields });
        }
        DrawLayerNode::Text(_) => {
            fields.push(inspector_text_field(
                &layer_ids,
                "draw-play-inspector.text-content",
                labels.content,
                &uniform
                    .iter()
                    .filter_map(|entry| match entry {
                        DrawLayerNode::Text(entry) => Some(entry.content.clone()),
                        _ => None,
                    })
                    .collect::<Vec<_>>(),
                "textContent",
            ));
            fields.push(inspector_number_field(
                &layer_ids,
                "draw-play-inspector.text-size",
                labels.size,
                &uniform
                    .iter()
                    .filter_map(|entry| match entry {
                        DrawLayerNode::Text(entry) => Some(entry.size),
                        _ => None,
                    })
                    .collect::<Vec<_>>(),
                "textSize",
            ));
            return Some(UiInspectorFieldGroup { presence: UiPresence::default(), id: "draw-play-inspector.kind.text".into(), label: labels.kind_text.into(), default_open: None, fields });
        }
        DrawLayerNode::Path(path) => {
            fields.push(ui_inspector_readonly_field("draw-play-inspector.path-segments", labels.segment_count, path.segments.len().to_string()));
            return Some(UiInspectorFieldGroup { presence: UiPresence::default(), id: "draw-play-inspector.kind.path".into(), label: labels.kind_path.into(), default_open: None, fields });
        }
        DrawLayerNode::Group(group) => {
            fields.push(ui_inspector_readonly_field("draw-play-inspector.group-children", labels.children_count, group.children.len().to_string()));
            return Some(UiInspectorFieldGroup { presence: UiPresence::default(), id: "draw-play-inspector.kind.group".into(), label: labels.kind_group.into(), default_open: None, fields });
        }
        _ => {}
    }
    None
}

fn inspector_appearance_group(layers: &[&DrawLayerNode], labels: &DrawPlayLabels) -> UiInspectorFieldGroup {
    let layer_ids: Vec<String> = layers.iter().map(|entry| crate::artifacts::draw::engine::layer_id(entry).to_string()).collect();
    let fill_colors: Vec<String> = layers
        .iter()
        .map(|entry| {
            layer_base(entry)
                .attributes
                .fill
                .as_ref()
                .map_or_else(
                    || "#000000".into(),
                    |fill| match fill {
                        FillStyle::Solid { color } => rgba_to_hex(*color),
                        FillStyle::LinearGradient { .. } | FillStyle::RadialGradient { .. } => "#000000".into(),
                    },
                )
        })
        .collect();
    let fill_alphas: Vec<f64> = layers
        .iter()
        .map(|entry| {
            layer_base(entry)
                .attributes
                .fill
                .as_ref()
                .map_or(1.0, |fill| match fill {
                    FillStyle::Solid { color } => color[3],
                    FillStyle::LinearGradient { .. } | FillStyle::RadialGradient { .. } => 1.0,
                })
        })
        .collect();
    let stroke_widths: Vec<f64> = layers.iter().map(|entry| layer_base(entry).attributes.stroke.as_ref().map_or(1.0, |stroke| stroke.width)).collect();
    let fill_alpha_mixed = ui_inspector_mixed_slider(&fill_alphas);
    UiInspectorFieldGroup {
        id: "draw-play-inspector.appearance".into(),
        label: labels.appearance.into(),
        default_open: None,
        presence: UiPresence::default(),
        fields: vec![
            inspector_text_field(&layer_ids, "draw-play-inspector.fill", labels.fill, &fill_colors, "fillColor"),
            UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: "draw-play-inspector.fill-alpha".into(),
                label: labels.fill_alpha.into(),
                child: Box::new(UiNode::Slider(UiSliderNode {
                    presence: UiPresence::default(),
                    id: "draw-play-inspector.fill-alpha.slider".into(),
                    value: if fill_alpha_mixed.uniform { fill_alpha_mixed.value } else { 0.0 },
                    min: 0.0,
                    max: 1.0,
                    step: 0.01,
                    on_change: inspector_patch(&layer_ids, "fillAlpha"),
                    unit: None,
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
            inspector_number_field(&layer_ids, "draw-play-inspector.stroke-width", labels.stroke_width, &stroke_widths, "strokeWidth"),
        ],
    }
}

fn inspector_layer_group(layers: &[&DrawLayerNode], labels: &DrawPlayLabels) -> UiInspectorFieldGroup {
    let layer_ids: Vec<String> = layers.iter().map(|entry| crate::artifacts::draw::engine::layer_id(entry).to_string()).collect();
    let names: Vec<String> = layers.iter().map(|entry| layer_base(entry).name.clone()).collect();
    let opacities: Vec<f64> = layers.iter().map(|entry| layer_base(entry).opacity).collect();
    let blend_modes: Vec<String> = layers.iter().map(|entry| layer_base(entry).blend_mode.clone()).collect();
    let visibles: Vec<bool> = layers.iter().map(|entry| layer_base(entry).visible).collect();
    let locked: Vec<bool> = layers.iter().map(|entry| layer_base(entry).locked).collect();
    let blend_mixed = ui_inspector_mixed_select(&blend_modes);
    let visible_mixed = ui_inspector_mixed_toggle(&visibles);
    let locked_mixed = ui_inspector_mixed_toggle(&locked);
    UiInspectorFieldGroup {
        id: "draw-play-inspector.layer".into(),
        label: labels.layer.into(),
        default_open: None,
        presence: UiPresence::default(),
        fields: vec![
            inspector_text_field(&layer_ids, "draw-play-inspector.name", labels.name, &names, "name"),
            inspector_number_field(&layer_ids, "draw-play-inspector.opacity", labels.opacity, &opacities, "opacity"),
            UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: "draw-play-inspector.blend-mode".into(),
                label: labels.blend_mode.into(),
                child: Box::new(UiNode::Select(UiSelectNode {
                    presence: UiPresence::default(),
                    id: "draw-play-inspector.blend-mode.select".into(),
                    value: blend_mixed.value,
                    placeholder: blend_mixed.placeholder.map(Label::data),
                    items: DRAW_BLEND_MODES.iter().map(|mode| UiSelectItem { value: (*mode).into(), label: Label::data(*mode) }).collect(),
                    on_change: inspector_patch(&layer_ids, "blendMode"),
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
            UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: "draw-play-inspector.visible".into(),
                label: labels.visible.into(),
                child: Box::new(UiNode::Toggle(UiToggleNode {
                    id: "draw-play-inspector.visible.toggle".into(),
                    icon_id: "eye".into(),
                    text: None,
                    on_change: inspector_patch(&layer_ids, "visible"),
                    presence: UiPresence::selected(visible_mixed.uniform && visible_mixed.pressed),
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
            UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: "draw-play-inspector.locked".into(),
                label: labels.locked.into(),
                child: Box::new(UiNode::Toggle(UiToggleNode {
                    id: "draw-play-inspector.locked.toggle".into(),
                    icon_id: "lock".into(),
                    text: None,
                    on_change: inspector_patch(&layer_ids, "locked"),
                    presence: UiPresence::selected(locked_mixed.uniform && locked_mixed.pressed),
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
        ],
    }
}

fn inspector_orientation_group(layers: &[&DrawLayerNode], labels: &DrawPlayLabels) -> UiInspectorFieldGroup {
    let layer_ids: Vec<String> = layers.iter().map(|entry| crate::artifacts::draw::engine::layer_id(entry).to_string()).collect();
    UiInspectorFieldGroup {
        id: "draw-play-inspector.orientation".into(),
        label: labels.orientation.into(),
        default_open: None,
        presence: UiPresence::default(),
        fields: vec![
            inspector_number_field(&layer_ids, "draw-play-inspector.transform-x", labels.position_x, &layers.iter().map(|entry| layer_base(entry).transform.x).collect::<Vec<_>>(), "transformX"),
            inspector_number_field(&layer_ids, "draw-play-inspector.transform-y", labels.position_y, &layers.iter().map(|entry| layer_base(entry).transform.y).collect::<Vec<_>>(), "transformY"),
            inspector_number_field(&layer_ids, "draw-play-inspector.transform-scale-x", labels.scale_x, &layers.iter().map(|entry| layer_base(entry).transform.scale_x).collect::<Vec<_>>(), "transformScaleX"),
            inspector_number_field(&layer_ids, "draw-play-inspector.transform-scale-y", labels.scale_y, &layers.iter().map(|entry| layer_base(entry).transform.scale_y).collect::<Vec<_>>(), "transformScaleY"),
            inspector_number_field(&layer_ids, "draw-play-inspector.transform-rotation", labels.rotation, &layers.iter().map(|entry| layer_base(entry).transform.rotation).collect::<Vec<_>>(), "transformRotation"),
        ],
    }
}

pub fn render(document: &DrawSnapshot, interaction: &DrawConfig, labels: &DrawPlayLabels, active_utility: &str) -> UiNode {
    let selected_layers: Vec<&DrawLayerNode> = interaction.selected_ids.iter().filter_map(|id| find_draw_layer(document, id)).collect();
    if selected_layers.is_empty() {
        return ui_stack_vertical(vec![
            ui_text(Label::data(format!("Schema: {}", DRAW_DOCUMENT_SCHEMA))),
            ui_text(Label::data(format!("Utility: {active_utility}"))),
            ui_text(Label::data(format!("Layers: {}", flatten_draw_layers(&document.layers).len()))),
        ]);
    }
    let mut groups = Vec::new();
    if let Some(kind_group) = inspector_kind_group(document, &selected_layers, labels) {
        groups.push(kind_group);
    }
    groups.push(inspector_orientation_group(&selected_layers, labels));
    groups.push(inspector_appearance_group(&selected_layers, labels));
    groups.push(inspector_layer_group(&selected_layers, labels));
    ui_inspector_groups_to_tree(&groups)
}
//#endregion 🔖️Render
