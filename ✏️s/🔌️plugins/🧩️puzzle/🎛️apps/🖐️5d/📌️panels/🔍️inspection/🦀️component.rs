//! 🔍️ Puzzle 5d play app panel — the inspector: the editable field group for whichever entity the
//! selection resolves to (grip wins over part wins over fastener), falling back to a read-only
//! document summary when nothing is selected.

use crate::apps::puzzle5d::terminology::Puzzle5dLabels;
use crate::apps::puzzle5d::{
    find_part_by_grip_full_id, puzzle5d_action, puzzle5d_grip_full_id, resolve_part_mesh_url, Puzzle5dDocument, Puzzle5dFastener, Puzzle5dGrip, Puzzle5dPart, Puzzle5dScene,
};
use semio_framework_plugin::{
    ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_mixed_select, ui_inspector_mixed_text, ui_inspector_readonly_field, ui_inspector_stepper_field,
    ui_inspector_vec3_group, ui_stack_vertical, ui_text, ActionDescriptor, Label, LabelText, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiFieldNode, UiInspectorFieldGroup,
    UiNode, UiPresence, UiSelectItem, UiSelectNode, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde_json::{json, Value};
use std::collections::BTreeMap;

//#region 🔖️Constants
pub const BODY_KEY: &str = "puzzle.5d.play.inspector";
const ANCHOR_FIXED: &str = "fixed";
const ANCHOR_DERIVED: &str = "derived";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(BODY_KEY.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️SerdeHelpers
fn value_field_f64(value: &Value, field: &str) -> f64 {
    value.get(field).and_then(Value::as_f64).unwrap_or(0.0)
}

fn part_anchor_token(part: &Puzzle5dPart) -> String {
    serde_json::to_value(part)
        .ok()
        .and_then(|value| value.get("anchor").and_then(Value::as_str).map(str::to_ascii_lowercase))
        .filter(|token| token == ANCHOR_FIXED || token == ANCHOR_DERIVED)
        .unwrap_or_else(|| ANCHOR_FIXED.into())
}

fn fastener_diagram_scalar(fastener: &Puzzle5dFastener, field: &str) -> f64 {
    serde_json::to_value(fastener).map(|value| value_field_f64(&value, field)).unwrap_or(0.0)
}

fn kind_catalog_entry<'a>(document: &'a Puzzle5dDocument, part_kind: &str) -> Option<&'a Value> {
    document.kind_catalogs.as_ref()?.get("parts")?.as_array()?.iter().find(|entry| entry.get("id").and_then(Value::as_str) == Some(part_kind))
}

fn representation_select_items(document: &Puzzle5dDocument, part_kinds: &[String]) -> Vec<UiSelectItem> {
    let mut by_url: BTreeMap<String, UiSelectItem> = BTreeMap::new();
    for part_kind in part_kinds {
        let Some(entry) = kind_catalog_entry(document, part_kind) else { continue };
        let Some(representations) = entry.get("representations").and_then(Value::as_array) else { continue };
        for representation in representations {
            let Some(url) = representation.get("url").and_then(Value::as_str).filter(|url| !url.is_empty()) else { continue };
            let name = representation
                .get("name")
                .or_else(|| representation.get("id"))
                .and_then(Value::as_str)
                .unwrap_or(url);
            let lod = representation.get("lod").and_then(Value::as_str).filter(|lod| !lod.is_empty());
            let label = lod.map(|lod| format!("{name} ({lod})")).unwrap_or_else(|| name.to_string());
            by_url.entry(url.to_string()).or_insert_with(|| UiSelectItem { value: url.to_string(), label: Label::data(label) });
        }
    }
    by_url.into_values().collect()
}
//#endregion 🔖️SerdeHelpers

//#region 🔖️Fields
fn inspector_text_field(id: &str, label: LabelText, mixed: semio_framework_plugin::UiInspectorMixedText, action: ActionDescriptor) -> UiNode {
    UiNode::Field(UiFieldNode {
        id: id.into(),
        label: label.into(),
        description: None,
        required: None,
        error: None,
        child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
            id: format!("{id}.input"),
            input_kind: "text".into(),
            value: mixed.value,
            placeholder: mixed.placeholder.map(Label::data),
            commit: None,
            min: None,
            max: None,
            step: None,
            accept: None,
            on_change: action,
            presence: UiPresence::default(),
            menu: None,
        })),
        presence: UiPresence::default(),
        menu: None,
    })
}

fn inspector_select_field(id: &str, label: impl Into<Label>, mixed: semio_framework_plugin::UiInspectorMixedText, items: Vec<UiSelectItem>, action: ActionDescriptor) -> UiNode {
    UiNode::Field(UiFieldNode {
        id: id.into(),
        label: label.into(),
        description: None,
        required: None,
        error: None,
        child: Box::new(UiNode::Select(UiSelectNode {
            id: format!("{id}.select"),
            value: mixed.value,
            items,
            placeholder: mixed.placeholder.map(Label::data),
            on_change: action,
            presence: UiPresence::default(),
            menu: None,
        })),
        presence: UiPresence::default(),
        menu: None,
    })
}

fn anchor_select_items() -> Vec<UiSelectItem> {
    vec![
        UiSelectItem { value: ANCHOR_FIXED.into(), label: Label::data("Fixed") },
        UiSelectItem { value: ANCHOR_DERIVED.into(), label: Label::data("Derived") },
    ]
}

fn build_part_inspector(document: &Puzzle5dDocument, parts: &[&Puzzle5dPart], labels: &Puzzle5dLabels) -> UiNode {
    let part_ids: Vec<String> = parts.iter().map(|part| part.id.clone()).collect();
    let patch_cmd = |field: &str| puzzle5d_action("patchPart", Some(json!({ "partIds": part_ids, "field": field })));
    let kinds: Vec<String> = parts.iter().map(|part| part.part_kind.clone()).collect();
    let labels_text: Vec<String> = parts.iter().map(|part| part.part_3d.label.clone().unwrap_or_default()).collect();
    let texts: Vec<String> = parts.iter().map(|part| part.part_2d.text.clone()).collect();
    let xs: Vec<f64> = parts.iter().map(|part| part.part_2d.x).collect();
    let ys: Vec<f64> = parts.iter().map(|part| part.part_2d.y).collect();
    let origins: Vec<[f64; 3]> = parts.iter().map(|part| part.part_3d.origin).collect();
    let anchors: Vec<String> = parts.iter().map(|part| part_anchor_token(part)).collect();
    let mesh_urls: Vec<String> = parts.iter().map(|part| resolve_part_mesh_url(part, document.kind_catalogs.as_ref()).unwrap_or_default()).collect();
    let mut fields = vec![
        ui_inspector_readonly_field(
            "puzzle5d-play-inspector.part.id",
            labels.id,
            if parts.len() == 1 { parts[0].id.clone() } else { format!("{} {}", parts.len(), labels.part.as_str()) },
        ),
        inspector_text_field("puzzle5d-play-inspector.part.kind", labels.kind, ui_inspector_mixed_text(&kinds), patch_cmd("partKind")),
        inspector_select_field("puzzle5d-play-inspector.part.anchor", Label::data("Anchor"), ui_inspector_mixed_select(&anchors), anchor_select_items(), patch_cmd("anchor")),
        inspector_text_field("puzzle5d-play-inspector.part.label", labels.label, ui_inspector_mixed_text(&labels_text), patch_cmd("label")),
        inspector_text_field("puzzle5d-play-inspector.part.text", labels.flat_text, ui_inspector_mixed_text(&texts), patch_cmd("text")),
        ui_inspector_stepper_field("puzzle5d-play-inspector.part.x", labels.flat_x, &xs, 0.1, patch_cmd("x")),
        ui_inspector_stepper_field("puzzle5d-play-inspector.part.y", labels.flat_y, &ys, 0.1, patch_cmd("y")),
        ui_inspector_vec3_group("puzzle5d-play-inspector.part.origin", labels.volume_origin, &origins, 0.1, |axis| patch_cmd(&format!("origin.{axis}"))),
    ];
    let representation_items = representation_select_items(document, &kinds);
    if !representation_items.is_empty() {
        fields.push(inspector_select_field(
            "puzzle5d-play-inspector.part.representation",
            labels.lod,
            ui_inspector_mixed_select(&mesh_urls),
            representation_items,
            patch_cmd("meshUrl"),
        ));
    }
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "puzzle5d-play-inspector.part".into(),
        label: labels.part.into(),
        default_open: None,
        presence: UiPresence::default(),
        fields,
    }])
}

fn build_grip_inspector(grips: &[(&Puzzle5dPart, &Puzzle5dGrip)], labels: &Puzzle5dLabels) -> UiNode {
    let grip_full_ids: Vec<String> = grips.iter().map(|(part, grip)| puzzle5d_grip_full_id(&part.id, &grip.id)).collect();
    let patch_cmd = |field: &str| puzzle5d_action("patchGrip", Some(json!({ "gripFullIds": grip_full_ids, "field": field })));
    let kinds: Vec<String> = grips.iter().map(|(_, grip)| grip.grip_kind.clone()).collect();
    let angles: Vec<f64> = grips.iter().map(|(_, grip)| grip.grip_2d.angle).collect();
    let radii: Vec<f64> = grips.iter().map(|(_, grip)| grip.grip_3d.radius).collect();
    let positions: Vec<[f64; 3]> = grips.iter().map(|(_, grip)| grip.grip_3d.position).collect();
    let directions: Vec<[f64; 3]> = grips.iter().map(|(_, grip)| grip.grip_3d.direction.unwrap_or([0.0, 0.0, -1.0])).collect();
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "puzzle5d-play-inspector.grip".into(),
        label: labels.grip.into(),
        default_open: None,
        presence: UiPresence::default(),
        fields: vec![
            ui_inspector_readonly_field(
                "puzzle5d-play-inspector.grip.id",
                labels.id,
                if grips.len() == 1 { grip_full_ids[0].clone() } else { format!("{} {}", grips.len(), labels.grip.as_str()) },
            ),
            inspector_text_field("puzzle5d-play-inspector.grip.kind", labels.kind, ui_inspector_mixed_text(&kinds), patch_cmd("gripKind")),
            ui_inspector_stepper_field("puzzle5d-play-inspector.grip.angle", labels.flat_angle, &angles, 1.0, patch_cmd("angle")),
            ui_inspector_stepper_field("puzzle5d-play-inspector.grip.radius", labels.radius, &radii, 0.05, patch_cmd("radius")),
            ui_inspector_vec3_group("puzzle5d-play-inspector.grip.position", labels.position, &positions, 0.1, |axis| patch_cmd(&format!("position.{axis}"))),
            ui_inspector_vec3_group("puzzle5d-play-inspector.grip.direction", labels.direction, &directions, 0.1, |axis| patch_cmd(&format!("direction.{axis}"))),
        ],
    }])
}

fn build_fastener_inspector(fasteners: &[&Puzzle5dFastener], labels: &Puzzle5dLabels) -> UiNode {
    let fastener_ids: Vec<String> = fasteners.iter().map(|fastener| fastener.id.clone()).collect();
    let patch_cmd = |field: &str| puzzle5d_action("patchFastener", Some(json!({ "fastenerIds": fastener_ids, "field": field })));
    let kinds: Vec<String> = fasteners.iter().map(|fastener| fastener.fastener_kind.clone().unwrap_or_default()).collect();
    let gaps: Vec<f64> = fasteners.iter().map(|fastener| fastener.gap).collect();
    let shifts: Vec<f64> = fasteners.iter().map(|fastener| fastener.shift).collect();
    let rises: Vec<f64> = fasteners.iter().map(|fastener| fastener.rise).collect();
    let rotations: Vec<f64> = fasteners.iter().map(|fastener| fastener.rotation).collect();
    let turns: Vec<f64> = fasteners.iter().map(|fastener| fastener.turn).collect();
    let tilts: Vec<f64> = fasteners.iter().map(|fastener| fastener.tilt).collect();
    let xs: Vec<f64> = fasteners.iter().map(|fastener| fastener_diagram_scalar(fastener, "x")).collect();
    let ys: Vec<f64> = fasteners.iter().map(|fastener| fastener_diagram_scalar(fastener, "y")).collect();
    let mut fields = vec![
        ui_inspector_readonly_field(
            "puzzle5d-play-inspector.fastener.id",
            labels.id,
            if fasteners.len() == 1 { fasteners[0].id.clone() } else { format!("{} {}", fasteners.len(), labels.fasteners.as_str()) },
        ),
    ];
    if fasteners.len() == 1 {
        let fastener = fasteners[0];
        fields.push(ui_inspector_readonly_field("puzzle5d-play-inspector.fastener.source", labels.source, &fastener.source));
        fields.push(ui_inspector_readonly_field("puzzle5d-play-inspector.fastener.target", labels.target, &fastener.target));
    }
    fields.extend([
        inspector_text_field("puzzle5d-play-inspector.fastener.kind", labels.kind, ui_inspector_mixed_text(&kinds), patch_cmd("fastenerKind")),
        ui_inspector_stepper_field("puzzle5d-play-inspector.fastener.gap", labels.gap, &gaps, 0.05, patch_cmd("gap")),
        ui_inspector_stepper_field("puzzle5d-play-inspector.fastener.shift", labels.shift, &shifts, 0.05, patch_cmd("shift")),
        ui_inspector_stepper_field("puzzle5d-play-inspector.fastener.rise", labels.rise, &rises, 0.05, patch_cmd("rise")),
        ui_inspector_stepper_field("puzzle5d-play-inspector.fastener.rotation", labels.rotation, &rotations, 1.0, patch_cmd("rotation")),
        ui_inspector_stepper_field("puzzle5d-play-inspector.fastener.turn", labels.turn, &turns, 1.0, patch_cmd("turn")),
        ui_inspector_stepper_field("puzzle5d-play-inspector.fastener.tilt", labels.tilt, &tilts, 1.0, patch_cmd("tilt")),
        ui_inspector_stepper_field("puzzle5d-play-inspector.fastener.x", labels.flat_x, &xs, 0.1, patch_cmd("x")),
        ui_inspector_stepper_field("puzzle5d-play-inspector.fastener.y", labels.flat_y, &ys, 0.1, patch_cmd("y")),
    ]);
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "puzzle5d-play-inspector.fastener".into(),
        label: labels.fasteners.into(),
        default_open: None,
        presence: UiPresence::default(),
        fields,
    }])
}
//#endregion 🔖️Fields

//#region 🔖️Render
pub fn render(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> UiNode {
    let selection = &envelope.runtime.selection;
    if !selection.grip_ids.is_empty() {
        let grips: Vec<(&Puzzle5dPart, &Puzzle5dGrip)> = selection
            .grip_ids
            .iter()
            .filter_map(|grip_full_id| find_part_by_grip_full_id(&envelope.document, grip_full_id))
            .collect();
        if !grips.is_empty() {
            return build_grip_inspector(&grips, labels);
        }
    }
    if !selection.part_ids.is_empty() {
        let parts: Vec<&Puzzle5dPart> = envelope.document.parts.iter().filter(|part| selection.part_ids.contains(&part.id)).collect();
        if !parts.is_empty() {
            return build_part_inspector(&envelope.document, &parts, labels);
        }
    }
    if !selection.fastener_ids.is_empty() {
        let fasteners: Vec<&Puzzle5dFastener> = envelope.document.fasteners.iter().filter(|fastener| selection.fastener_ids.contains(&fastener.id)).collect();
        if !fasteners.is_empty() {
            return build_fastener_inspector(&fasteners, labels);
        }
    }
    ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
        id: "puzzle5d-play-inspector.empty".into(),
        label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
        default_open: Some(true),
        children: vec![
            ui_text(Label::data(format!("{}: {}", labels.schema.as_str(), envelope.document.schema))),
            ui_text(Label::data(format!("{}: {}", labels.parts.as_str(), envelope.document.parts.len()))),
            ui_text(Label::data(format!("{}: {}", labels.fasteners.as_str(), envelope.document.fasteners.len()))),
            ui_text(Label::data(format!("{}: {}", labels.utility.as_str(), envelope.active_utility))),
        ],
        presence: UiPresence::default(),
        menu: None,
    }])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::puzzle5d::testkit::*;
    use serde_json::json as json_macro;

    #[test]
    fn empty_selection_renders_the_document_summary() {
        let mut app = app();
        assert!(render_body(&mut app, BODY_KEY).contains("puzzle5d-play-inspector.empty"));
    }

    #[test]
    fn selecting_a_part_renders_its_editable_field_group() {
        let mut app = app();
        let part_id = first_part_id(&app);
        dispatch(&mut app, "setSelection", Some(&json_macro!({ "partIds": [part_id] })), None).expect("select part");
        let rendered = render_body(&mut app, BODY_KEY);
        assert!(rendered.contains("puzzle5d-play-inspector.part.origin"));
        assert!(rendered.contains("puzzle5d-play-inspector.part.anchor"));
        assert!(rendered.contains("puzzle5d-play-inspector.part.x"));
    }

    #[test]
    fn selecting_a_fastener_renders_diagram_offset_steppers() {
        let mut app = app();
        dispatch(&mut app, "setActiveExample", Some(&json_macro!({ "exampleId": crate::apps::puzzle5d::PUZZLE5D_EXAMPLE_NAKAGIN })), None).expect("load nakagin");
        let projection = projection_of(&app);
        let fastener_id = projection["fasteners"][0]["id"].as_str().expect("seeded fastener").to_string();
        dispatch(&mut app, "setSelection", Some(&json_macro!({ "fastenerIds": [fastener_id] })), None).expect("select fastener");
        let rendered = render_body(&mut app, BODY_KEY);
        assert!(rendered.contains("puzzle5d-play-inspector.fastener.x"));
        assert!(rendered.contains("puzzle5d-play-inspector.fastener.y"));
    }
}
//#endregion 🧪️Tests
