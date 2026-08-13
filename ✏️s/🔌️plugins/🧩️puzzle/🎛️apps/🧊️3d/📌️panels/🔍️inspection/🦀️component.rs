//! 🔍️ Puzzle 3d play app panel — the inspector: one field group per selected entity kind (object,
//! vortex, attraction, reference, target volume), every stepper/toggle patching the whole
//! multi-selection at once through `patchInspector`, falling back to a document summary when nothing
//! is selected.

use crate::apps::puzzle3d::config::Puzzle3dSelection;
use crate::apps::puzzle3d::terminology::Puzzle3dLabels;
use crate::apps::puzzle3d::{object_scale_json, puzzle3d_action, puzzle3d_vortex_full_id, target_volume_scale_json, Puzzle3dAttraction, Puzzle3dObject, Puzzle3dReference, Puzzle3dScene, Puzzle3dTargetVolume, Puzzle3dVortex};
use semio_framework_plugin::{
    ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_readonly_field, ui_inspector_stepper_field, ui_inspector_toggle_field, ui_inspector_vec3_group, ui_text, ActionDescriptor, Label, LabelText, LocalizedLabel, PanelGroup,
    PanelTabDefinition, PanelTabKind, UiFieldNode, UiGroupNode, UiInspectorFieldGroup, UiNode, UiPresence, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde_json::json;

//#region 🔖️Constants
pub const BODY_KEY: &str = "puzzle.3d.play.inspector";
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

/// 🎯️ The ids one inspector `entity` group patches when an action carries no explicit `ids` — the
/// live selection bag for that entity kind. Shared with `🎮️commands/🧊️add-object-kind`'s `patch_inspector`.
pub fn target_ids(entity: &str, selection: &Puzzle3dSelection) -> Vec<String> {
    match entity {
        "object" => selection.object_ids.to_vec(),
        "vortex" => selection.vortex_ids.to_vec(),
        "attraction" => selection.attraction_ids.to_vec(),
        "reference" => selection.reference_ids.to_vec(),
        "targetVolume" => selection.target_volume_ids.to_vec(),
        _ => Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Fields
fn text_field(id: impl Into<String>, label: impl Into<Label>, mixed_text: semio_framework_plugin::UiInspectorMixedText, action: ActionDescriptor) -> UiNode {
    let id = id.into();
    UiNode::Field(UiFieldNode {
        id: id.clone(),
        label: label.into(),
        child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
            id: format!("{id}.input"),
            input_kind: "text".into(),
            value: mixed_text.value,
            placeholder: mixed_text.placeholder.map(Label::data),
            commit: None,
            on_change: action,
            min: None,
            max: None,
            step: None,
            accept: None,
            presence: UiPresence::default(),
            menu: None,
        })),
        description: None,
        required: None,
        error: None,
        presence: UiPresence::default(),
        menu: None,
    })
}

/// @emoji 🌀️ Builds an editable 4-component quaternion group (`X`/`Y`/`Z`/`W` steppers) — puzzle3d's
/// `orientation: Option<[f64; 4]>` fields have no shared helper (quaternions aren't `ui_inspector_vec3_group`'s
/// 3-wide shape), so this mirrors that helper's structure one component wider. `axis_action(component)`
/// builds the per-component action; the patch handler renormalizes after any component edit so the
/// result stays a valid unit quaternion.
fn quat_group(id: &str, label: impl Into<Label>, values: &[[f64; 4]], step: f64, axis_action: impl Fn(&str) -> ActionDescriptor) -> UiNode {
    let component = |index: usize, name: &str, label: &str| {
        let values: Vec<f64> = values.iter().map(|q| q[index]).collect();
        // 🔤️ Axis symbols (X/Y/Z/W) are mathematical notation, not translatable UI chrome.
        ui_inspector_stepper_field(format!("{id}.{name}"), Label::data(label), &values, step, axis_action(name))
    };
    UiNode::Group(UiGroupNode {
        id: id.into(),
        label: label.into(),
        default_open: Some(true),
        presence: UiPresence::default(),
        children: vec![component(0, "x", "X"), component(1, "y", "Y"), component(2, "z", "Z"), component(3, "w", "W")],
        menu: None,
    })
}

fn header_and_delete(count: usize, noun: LabelText, labels: &Puzzle3dLabels) -> Vec<UiNode> {
    vec![
        ui_text(Label::data(format!("{count} {} {}", noun.as_str(), labels.selected_count.as_str()))),
        UiNode::Button(semio_framework_plugin::UiButtonNode {
            id: Some("puzzle3d-play-inspector.delete".into()),
            icon_id: "trash-2".into(),
            label: labels.delete.into(),
            action: puzzle3d_action("deleteSelection", None),
            style: None,
            presence: UiPresence::default(),
            menu: None,
        }),
    ]
}
//#endregion 🔖️Fields

//#region 🔖️Render
pub fn render(envelope: &Puzzle3dScene, term_labels: &Puzzle3dLabels) -> UiNode {
    let selection = &envelope.runtime.selection;
    if !selection.object_ids.is_empty() {
        let objects: Vec<&Puzzle3dObject> = envelope.fixture.objects.iter().filter(|object| selection.object_ids.contains(&object.id)).collect();
        if !objects.is_empty() {
            let patch_cmd = |field: &str| puzzle3d_action("patchInspector", Some(json!({ "entity": "object", "field": field })));
            let mut fields = header_and_delete(objects.len(), term_labels.object, term_labels);
            if let [object] = objects.as_slice() {
                fields.push(ui_inspector_readonly_field("puzzle3d-play-inspector.object.id", term_labels.id, &object.id));
            }
            let labels: Vec<String> = objects.iter().map(|object| object.label.clone().unwrap_or_default()).collect();
            let kinds: Vec<String> = objects.iter().map(|object| object.object_kind.clone().unwrap_or_default()).collect();
            let origins: Vec<[f64; 3]> = objects.iter().map(|object| object.origin).collect();
            let orientations: Vec<[f64; 4]> = objects.iter().map(|object| object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0])).collect();
            let scales: Vec<[f64; 3]> = objects.iter().map(|object| object_scale_json(object)).collect();
            let mesh_urls: Vec<String> = objects.iter().map(|object| object.mesh_url.clone().unwrap_or_default()).collect();
            let hidden: Vec<bool> = objects.iter().map(|object| object.hidden).collect();
            let locked: Vec<bool> = objects.iter().map(|object| object.locked).collect();
            fields.push(text_field("puzzle3d-play-inspector.object.label", term_labels.label, semio_framework_plugin::ui_inspector_mixed_text(&labels), patch_cmd("label")));
            fields.push(text_field("puzzle3d-play-inspector.object.kind", term_labels.kind, semio_framework_plugin::ui_inspector_mixed_text(&kinds), patch_cmd("objectKind")));
            fields.push(ui_inspector_vec3_group("puzzle3d-play-inspector.object.origin", term_labels.origin, &origins, 0.1, |axis| patch_cmd(&format!("origin.{axis}"))));
            fields.push(quat_group("puzzle3d-play-inspector.object.orientation", term_labels.orientation, &orientations, 0.01, |axis| patch_cmd(&format!("orientation.{axis}"))));
            fields.push(ui_inspector_vec3_group("puzzle3d-play-inspector.object.scale", term_labels.scale, &scales, 0.1, |axis| patch_cmd(&format!("scale.{axis}"))));
            fields.push(text_field("puzzle3d-play-inspector.object.mesh-url", term_labels.mesh_url, semio_framework_plugin::ui_inspector_mixed_text(&mesh_urls), patch_cmd("meshUrl")));
            fields.push(ui_inspector_toggle_field("puzzle3d-play-inspector.object.hidden", term_labels.hidden, "eye-off", &hidden, patch_cmd("hidden")));
            fields.push(ui_inspector_toggle_field("puzzle3d-play-inspector.object.locked", term_labels.locked, "lock", &locked, patch_cmd("locked")));
            return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "puzzle3d-play-inspector.object".into(), label: term_labels.object.into(), default_open: None, presence: UiPresence::default(), fields }]);
        }
    }
    if !selection.vortex_ids.is_empty() {
        let vortices: Vec<(&Puzzle3dObject, &Puzzle3dVortex)> =
            envelope.fixture.objects.iter().flat_map(|object| object.vortices.iter().map(move |vortex| (object, vortex))).filter(|(object, vortex)| selection.vortex_ids.contains(&puzzle3d_vortex_full_id(&object.id, &vortex.id))).collect();
        if !vortices.is_empty() {
            let full_ids: Vec<String> = vortices.iter().map(|(object, vortex)| puzzle3d_vortex_full_id(&object.id, &vortex.id)).collect();
            let patch_cmd = |field: &str| puzzle3d_action("patchInspector", Some(json!({ "entity": "vortex", "field": field })));
            let mut fields = header_and_delete(vortices.len(), term_labels.vortex, term_labels);
            if let [(_, vortex)] = vortices.as_slice() {
                fields.push(ui_inspector_readonly_field("puzzle3d-play-inspector.vortex.id", term_labels.full_id, &full_ids[0]));
                let _ = vortex;
            }
            let kinds: Vec<String> = vortices.iter().map(|(_, vortex)| vortex.vortex_kind.clone().unwrap_or_default()).collect();
            let positions: Vec<[f64; 3]> = vortices.iter().map(|(_, vortex)| vortex.position).collect();
            let directions: Vec<[f64; 3]> = vortices.iter().map(|(_, vortex)| vortex.direction.unwrap_or([0.0, 0.0, 1.0])).collect();
            let radii: Vec<f64> = vortices.iter().map(|(_, vortex)| vortex.radius.unwrap_or(0.35)).collect();
            let hidden: Vec<bool> = vortices.iter().map(|(_, vortex)| vortex.hidden).collect();
            let locked: Vec<bool> = vortices.iter().map(|(_, vortex)| vortex.locked).collect();
            fields.push(text_field("puzzle3d-play-inspector.vortex.kind", term_labels.vortex_kind, semio_framework_plugin::ui_inspector_mixed_text(&kinds), patch_cmd("vortexKind")));
            fields.push(ui_inspector_vec3_group("puzzle3d-play-inspector.vortex.position", term_labels.position, &positions, 0.1, |axis| patch_cmd(&format!("position.{axis}"))));
            fields.push(ui_inspector_vec3_group("puzzle3d-play-inspector.vortex.direction", term_labels.direction, &directions, 0.1, |axis| patch_cmd(&format!("direction.{axis}"))));
            fields.push(ui_inspector_stepper_field("puzzle3d-play-inspector.vortex.radius", term_labels.radius, &radii, 0.05, patch_cmd("radius")));
            fields.push(ui_inspector_toggle_field("puzzle3d-play-inspector.vortex.hidden", term_labels.hidden, "eye-off", &hidden, patch_cmd("hidden")));
            fields.push(ui_inspector_toggle_field("puzzle3d-play-inspector.vortex.locked", term_labels.locked, "lock", &locked, patch_cmd("locked")));
            return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "puzzle3d-play-inspector.vortex".into(), label: term_labels.vortex.into(), default_open: None, presence: UiPresence::default(), fields }]);
        }
    }
    if !selection.attraction_ids.is_empty() {
        let attractions: Vec<&Puzzle3dAttraction> = envelope.fixture.attractions.iter().filter(|attraction| selection.attraction_ids.contains(&attraction.id)).collect();
        if !attractions.is_empty() {
            let patch_cmd = |field: &str| puzzle3d_action("patchInspector", Some(json!({ "entity": "attraction", "field": field })));
            let mut fields = header_and_delete(attractions.len(), term_labels.attraction, term_labels);
            let attracting: Vec<String> = attractions.iter().map(|attraction| attraction.attracting.clone()).collect();
            let attracted: Vec<String> = attractions.iter().map(|attraction| attraction.attracted.clone()).collect();
            fields.push(text_field("puzzle3d-play-inspector.attraction.attracting", term_labels.attracting, semio_framework_plugin::ui_inspector_mixed_text(&attracting), patch_cmd("attracting")));
            fields.push(text_field("puzzle3d-play-inspector.attraction.attracted", term_labels.attracted, semio_framework_plugin::ui_inspector_mixed_text(&attracted), patch_cmd("attracted")));
            let gaps: Vec<f64> = attractions.iter().map(|attraction| attraction.gap).collect();
            let shifts: Vec<f64> = attractions.iter().map(|attraction| attraction.shift).collect();
            let rises: Vec<f64> = attractions.iter().map(|attraction| attraction.rise).collect();
            let rotations: Vec<f64> = attractions.iter().map(|attraction| attraction.rotation).collect();
            let turns: Vec<f64> = attractions.iter().map(|attraction| attraction.turn).collect();
            let tilts: Vec<f64> = attractions.iter().map(|attraction| attraction.tilt).collect();
            fields.push(ui_inspector_stepper_field("puzzle3d-play-inspector.attraction.gap", term_labels.gap, &gaps, 0.1, patch_cmd("gap")));
            fields.push(ui_inspector_stepper_field("puzzle3d-play-inspector.attraction.shift", term_labels.shift, &shifts, 0.1, patch_cmd("shift")));
            fields.push(ui_inspector_stepper_field("puzzle3d-play-inspector.attraction.rise", term_labels.rise, &rises, 0.1, patch_cmd("rise")));
            fields.push(ui_inspector_stepper_field("puzzle3d-play-inspector.attraction.rotation", term_labels.rotation_deg, &rotations, 1.0, patch_cmd("rotation")));
            fields.push(ui_inspector_stepper_field("puzzle3d-play-inspector.attraction.turn", term_labels.turn_deg, &turns, 1.0, patch_cmd("turn")));
            fields.push(ui_inspector_stepper_field("puzzle3d-play-inspector.attraction.tilt", term_labels.tilt_deg, &tilts, 1.0, patch_cmd("tilt")));
            return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "puzzle3d-play-inspector.attraction".into(), label: term_labels.attraction.into(), default_open: None, presence: UiPresence::default(), fields }]);
        }
    }
    if !selection.reference_ids.is_empty() {
        let references: Vec<&Puzzle3dReference> = envelope.fixture.references.iter().filter(|reference| selection.reference_ids.contains(&reference.id)).collect();
        if !references.is_empty() {
            let patch_cmd = |field: &str| puzzle3d_action("patchInspector", Some(json!({ "entity": "reference", "field": field })));
            let mut fields = header_and_delete(references.len(), term_labels.reference, term_labels);
            let urls: Vec<String> = references.iter().map(|reference| reference.source.url.clone()).collect();
            let media_kinds: Vec<String> = references.iter().map(|reference| reference.source.media_kind.clone().unwrap_or_default()).collect();
            let origins: Vec<[f64; 3]> = references.iter().map(|reference| reference.origin).collect();
            let widths: Vec<f64> = references.iter().map(|reference| reference.width_world).collect();
            let hidden: Vec<bool> = references.iter().map(|reference| reference.hidden).collect();
            let locked: Vec<bool> = references.iter().map(|reference| reference.locked).collect();
            fields.push(text_field("puzzle3d-play-inspector.reference.url", term_labels.source_url, semio_framework_plugin::ui_inspector_mixed_text(&urls), patch_cmd("sourceUrl")));
            fields.push(text_field("puzzle3d-play-inspector.reference.media-kind", term_labels.media_kind, semio_framework_plugin::ui_inspector_mixed_text(&media_kinds), patch_cmd("mediaKind")));
            fields.push(ui_inspector_vec3_group("puzzle3d-play-inspector.reference.origin", term_labels.origin, &origins, 0.1, |axis| patch_cmd(&format!("origin.{axis}"))));
            fields.push(ui_inspector_stepper_field("puzzle3d-play-inspector.reference.width", term_labels.width, &widths, 0.1, patch_cmd("widthWorld")));
            fields.push(ui_inspector_toggle_field("puzzle3d-play-inspector.reference.hidden", term_labels.hidden, "eye-off", &hidden, patch_cmd("hidden")));
            fields.push(ui_inspector_toggle_field("puzzle3d-play-inspector.reference.locked", term_labels.locked, "lock", &locked, patch_cmd("locked")));
            return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "puzzle3d-play-inspector.reference".into(), label: term_labels.reference.into(), default_open: None, presence: UiPresence::default(), fields }]);
        }
    }
    if !selection.target_volume_ids.is_empty() {
        let volumes: Vec<&Puzzle3dTargetVolume> = envelope.fixture.target_volumes.iter().filter(|volume| selection.target_volume_ids.contains(&volume.id)).collect();
        if !volumes.is_empty() {
            let patch_cmd = |field: &str| puzzle3d_action("patchInspector", Some(json!({ "entity": "targetVolume", "field": field })));
            let mut fields = header_and_delete(volumes.len(), term_labels.target_volume, term_labels);
            let origins: Vec<[f64; 3]> = volumes.iter().map(|volume| volume.origin).collect();
            let orientations: Vec<[f64; 4]> = volumes.iter().map(|volume| volume.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0])).collect();
            let scales: Vec<[f64; 3]> = volumes.iter().map(|volume| target_volume_scale_json(volume)).collect();
            let hidden: Vec<bool> = volumes.iter().map(|volume| volume.hidden).collect();
            let locked: Vec<bool> = volumes.iter().map(|volume| volume.locked).collect();
            fields.push(ui_inspector_vec3_group("puzzle3d-play-inspector.target-volume.origin", term_labels.origin, &origins, 0.1, |axis| patch_cmd(&format!("origin.{axis}"))));
            fields.push(quat_group("puzzle3d-play-inspector.target-volume.orientation", term_labels.orientation, &orientations, 0.01, |axis| patch_cmd(&format!("orientation.{axis}"))));
            fields.push(ui_inspector_vec3_group("puzzle3d-play-inspector.target-volume.scale", term_labels.scale, &scales, 0.1, |axis| patch_cmd(&format!("scale.{axis}"))));
            fields.push(ui_inspector_toggle_field("puzzle3d-play-inspector.target-volume.hidden", term_labels.hidden, "eye-off", &hidden, patch_cmd("hidden")));
            fields.push(ui_inspector_toggle_field("puzzle3d-play-inspector.target-volume.locked", term_labels.locked, "lock", &locked, patch_cmd("locked")));
            return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "puzzle3d-play-inspector.target-volume".into(), label: term_labels.target_volume.into(), default_open: None, presence: UiPresence::default(), fields }]);
        }
    }
    ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
        id: "puzzle3d-play-inspector.empty".into(),
        label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
        default_open: Some(true),
        children: vec![
            ui_text(Label::data(format!("{}: {}", term_labels.schema.as_str(), envelope.fixture.schema))),
            ui_text(Label::data(format!("{}: {}", term_labels.domain.as_str(), envelope.fixture.domain))),
            ui_text(Label::data(format!("{}: {}", term_labels.objects.as_str(), envelope.fixture.objects.len()))),
        ],
        presence: UiPresence::default(),
        menu: None,
    }])
}
//#endregion 🔖️Render
