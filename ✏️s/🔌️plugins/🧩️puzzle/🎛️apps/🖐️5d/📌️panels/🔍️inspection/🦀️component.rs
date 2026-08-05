//! 🔍️ Puzzle 5d play app panel — the inspector: the editable field group for whichever entity the
//! selection resolves to (grip wins over part wins over fastener), falling back to a read-only
//! document summary when nothing is selected.

use crate::apps::puzzle5d::terminology::Puzzle5dLabels;
use crate::apps::puzzle5d::{find_part_by_grip_full_id, puzzle5d_action, puzzle5d_grip_full_id, Puzzle5dFastener, Puzzle5dGrip, Puzzle5dPart, Puzzle5dScene};
use semio_framework_plugin::{
    ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_readonly_field, ui_inspector_stepper_field, ui_inspector_vec3_group, ui_stack_vertical, ui_text, ActionDescriptor, Label, LabelText, LocalizedLabel, PanelGroup,
    PanelTabDefinition, PanelTabKind, UiFieldNode, UiInspectorFieldGroup, UiNode, UiPresence, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde_json::json;

//#region 🔖️Constants
pub const BODY_KEY: &str = "puzzle.5d.play.inspector";
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

//#region 🔖️Fields
fn inspector_text_field(id: &str, label: LabelText, value: String, action: ActionDescriptor) -> UiNode {
    UiNode::Field(UiFieldNode {
        id: id.into(),
        label: label.into(),
        description: None,
        required: None,
        error: None,
        child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
            id: format!("{id}.input"),
            input_kind: "text".into(),
            value,
            placeholder: None,
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

fn build_part_inspector(part: &Puzzle5dPart, labels: &Puzzle5dLabels) -> UiNode {
    let origin = part.part_3d.origin;
    let patch_cmd = |field: &str| puzzle5d_action("patchPart", Some(json!({ "partId": part.id, "field": field })));
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "puzzle5d-play-inspector.part".into(),
        label: labels.part.into(),
        default_open: None,
        presence: UiPresence::default(),
        fields: vec![
            ui_inspector_readonly_field("puzzle5d-play-inspector.part.id", labels.id, &part.id),
            inspector_text_field("puzzle5d-play-inspector.part.kind", labels.kind, part.part_kind.clone(), patch_cmd("partKind")),
            inspector_text_field("puzzle5d-play-inspector.part.label", labels.label, part.part_3d.label.clone().unwrap_or_default(), patch_cmd("label")),
            inspector_text_field("puzzle5d-play-inspector.part.text", labels.flat_text, part.part_2d.text.clone(), patch_cmd("text")),
            ui_inspector_stepper_field("puzzle5d-play-inspector.part.x", labels.flat_x, &[part.part_2d.x], 0.1, patch_cmd("x")),
            ui_inspector_stepper_field("puzzle5d-play-inspector.part.y", labels.flat_y, &[part.part_2d.y], 0.1, patch_cmd("y")),
            ui_inspector_vec3_group("puzzle5d-play-inspector.part.origin", labels.volume_origin, &[origin], 0.1, |axis| patch_cmd(&format!("origin.{axis}"))),
        ],
    }])
}

fn build_grip_inspector(part: &Puzzle5dPart, grip: &Puzzle5dGrip, labels: &Puzzle5dLabels) -> UiNode {
    let full_id = puzzle5d_grip_full_id(&part.id, &grip.id);
    let position = grip.grip_3d.position;
    let direction = grip.grip_3d.direction.unwrap_or([0.0, 0.0, -1.0]);
    let patch_cmd = |field: &str| puzzle5d_action("patchGrip", Some(json!({ "gripFullId": full_id, "field": field })));
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "puzzle5d-play-inspector.grip".into(),
        label: labels.grip.into(),
        default_open: None,
        presence: UiPresence::default(),
        fields: vec![
            ui_inspector_readonly_field("puzzle5d-play-inspector.grip.id", labels.id, &full_id),
            inspector_text_field("puzzle5d-play-inspector.grip.kind", labels.kind, grip.grip_kind.clone(), patch_cmd("gripKind")),
            ui_inspector_stepper_field("puzzle5d-play-inspector.grip.angle", labels.flat_angle, &[grip.grip_2d.angle], 1.0, patch_cmd("angle")),
            ui_inspector_stepper_field("puzzle5d-play-inspector.grip.radius", labels.radius, &[grip.grip_3d.radius], 0.05, patch_cmd("radius")),
            ui_inspector_vec3_group("puzzle5d-play-inspector.grip.position", labels.position, &[position], 0.1, |axis| patch_cmd(&format!("position.{axis}"))),
            ui_inspector_vec3_group("puzzle5d-play-inspector.grip.direction", labels.direction, &[direction], 0.1, |axis| patch_cmd(&format!("direction.{axis}"))),
        ],
    }])
}

/// 🔧️ Editable fastener inspector: the six pose-solver offsets (gap/shift/rise/rotation/turn/tilt) as
/// steppers bound to `patchFastener`, plus a "Mixed" summary when more than one fastener is selected
/// (steppers edit the first selected fastener only; a real multi-edit broadcast is a follow-up).
fn build_fastener_inspector(fastener: &Puzzle5dFastener, selected_count: usize, labels: &Puzzle5dLabels) -> UiNode {
    let patch_cmd = |field: &str| puzzle5d_action("patchFastener", Some(json!({ "fastenerId": fastener.id, "field": field })));
    let mut fields = vec![
        ui_inspector_readonly_field("puzzle5d-play-inspector.fastener.id", labels.id, &fastener.id),
        ui_inspector_readonly_field("puzzle5d-play-inspector.fastener.source", labels.source, &fastener.source),
        ui_inspector_readonly_field("puzzle5d-play-inspector.fastener.target", labels.target, &fastener.target),
        inspector_text_field("puzzle5d-play-inspector.fastener.kind", labels.kind, fastener.fastener_kind.clone().unwrap_or_default(), patch_cmd("fastenerKind")),
        ui_inspector_stepper_field("puzzle5d-play-inspector.fastener.gap", labels.gap, &[fastener.gap], 0.05, patch_cmd("gap")),
        ui_inspector_stepper_field("puzzle5d-play-inspector.fastener.shift", labels.shift, &[fastener.shift], 0.05, patch_cmd("shift")),
        ui_inspector_stepper_field("puzzle5d-play-inspector.fastener.rise", labels.rise, &[fastener.rise], 0.05, patch_cmd("rise")),
        ui_inspector_stepper_field("puzzle5d-play-inspector.fastener.rotation", labels.rotation, &[fastener.rotation], 1.0, patch_cmd("rotation")),
        ui_inspector_stepper_field("puzzle5d-play-inspector.fastener.turn", labels.turn, &[fastener.turn], 1.0, patch_cmd("turn")),
        ui_inspector_stepper_field("puzzle5d-play-inspector.fastener.tilt", labels.tilt, &[fastener.tilt], 1.0, patch_cmd("tilt")),
    ];
    if selected_count > 1 {
        fields.push(ui_inspector_readonly_field("puzzle5d-play-inspector.fastener.mixed", labels.mixed, format!("{selected_count}")));
    }
    ui_stack_vertical(fields)
}
//#endregion 🔖️Fields

//#region 🔖️Render
pub fn render(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> UiNode {
    if let Some(grip_full_id) = envelope.runtime.selection.grip_ids.first() {
        if let Some((part, grip)) = find_part_by_grip_full_id(&envelope.document, grip_full_id) {
            return build_grip_inspector(part, grip, labels);
        }
    }
    if let Some(part_id) = envelope.runtime.selection.part_ids.first() {
        if let Some(part) = envelope.document.parts.iter().find(|entry| entry.id == part_id) {
            return build_part_inspector(part, labels);
        }
    }
    if let Some(fastener_id) = envelope.runtime.selection.fastener_ids.first() {
        if let Some(fastener) = envelope.document.fasteners.iter().find(|entry| entry.id == fastener_id) {
            return build_fastener_inspector(fastener, envelope.runtime.selection.fastener_ids.len(), labels);
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
        assert!(render_body(&mut app, BODY_KEY).contains("puzzle5d-play-inspector.part.origin"));
    }
}
//#endregion 🧪️Tests
