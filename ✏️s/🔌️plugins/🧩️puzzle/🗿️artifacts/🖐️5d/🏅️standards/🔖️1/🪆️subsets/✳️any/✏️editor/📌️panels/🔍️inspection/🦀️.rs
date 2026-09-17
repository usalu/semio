//! 🔍️ Puzzle 5d play app panel — the field inspector for whatever is selected. 🕹️ ticket
//! 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the live `vortex` domain is read once per
//! render into a [`Puzzle5dInteractionSnapshot`] and carried on the scene, so this panel switches on
//! the granularity the user actually picked at (grip wins over part wins over fastener) and renders
//! THAT entity's real fields. An empty or unresolvable selection falls back to the document summary.
//!
//! 🪟️ The selected-id list is a windowed section of its own ([`IDS_SECTION`]): it stamps the full
//! selection size and materialises only the host's requested row window, so a wide selection scrolls.
//!
//! 🩹️ Every mutating row dispatches the entity's own patch verb (`patchPart`/`patchGrip`/
//! `patchFastener`) with the explicit id list that verb accepts, and the two part flags dispatch
//! `setSelectionFlag {entity, flag, ids, value}` — `value` is always the INVERSE of the row's current
//! state, never a hardcoded `true`.

use crate::editor::puzzle5d::terminology::Puzzle5dLabels;
use crate::editor::puzzle5d::{
    engine_grip_kind, part_scale_json, puzzle5d_grip_full_id, puzzle5d_part_display_label, target_volume_flat_rect, target_volume_scale_json, ui_label, Puzzle5dDocument, Puzzle5dFastener, Puzzle5dGrip,
    Puzzle5dInteractionSnapshot, Puzzle5dPart, Puzzle5dScene, Puzzle5dTargetVolume, PUZZLE5D_GRANULARITY_FASTENER, PUZZLE5D_GRANULARITY_GRIP, PUZZLE5D_GRANULARITY_PART, PUZZLE5D_GRANULARITY_TARGET_VOLUME,
    PUZZLE5D_PLAY_CONTROLLER_ID,
};
use semio_framework_plugin::{
    tree_item_desc, tree_item_with_action, ui_node_list, ActionFactory, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, UiAssemblyResult, UiFixedList, UiValue,
    FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};

//#region 🔖️Constants
pub const BODY_KEY: &str = "puzzle.5d.play.inspector";
const ROOT: &str = "puzzle5d-play-inspector";
/// 🪟️ The windowed section the selected ids are listed in — the node key the host reports its
/// open/scroll window for.
pub const IDS_SECTION: &str = "puzzle5d-play-inspector.ids";
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

//#region 🔖️Rows
fn error(scope: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new("ui.inspection.fields", scope)
}

fn push(fields: &mut UiFixedList<BuiltNode>, node: UiAssemblyResult<BuiltNode>) -> UiAssemblyResult<()> {
    fields.try_push(node?).map_err(|_| error("puzzle5d inspector field admission failed"))
}

/// 🧾️ One read-only `label` / `value` row.
fn read_only(fields: &mut UiFixedList<BuiltNode>, id: &str, label: &str, value: impl std::fmt::Display) -> UiAssemblyResult<()> {
    push(fields, tree_item_desc(format!("{ROOT}.{id}"), ui_label(label)?, Some(value.to_string())))
}

fn vec3(value: [f64; 3]) -> String {
    format!("{}, {}, {}", value[0], value[1], value[2])
}

fn vec4(value: [f64; 4]) -> String {
    format!("{}, {}, {}, {}", value[0], value[1], value[2], value[3])
}

fn text_value(value: &str) -> UiAssemblyResult<UiValue> {
    semio_framework_plugin::UiText::try_from_str(value).map(UiValue::Text).ok_or_else(|| error("puzzle5d inspector action text admission failed"))
}

/// 🪙️ The mutating row's id-argument list. The process-wide argument arena, not a row quota, is the
/// ceiling: ids are admitted until the fixed list refuses one, and a shorter list still edits every id
/// it names.
fn id_list_value(ids: &[String]) -> UiAssemblyResult<UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new().ok_or_else(|| error("puzzle5d inspector action list admission failed"))?;
    for id in ids {
        if builder.push(text_value(id)?).is_err() {
            break;
        }
    }
    Ok(UiValue::List(builder.finish()))
}

fn action_map(entries: [(&'static str, UiValue); 3]) -> UiAssemblyResult<UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| error("puzzle5d inspector action map admission failed"))?;
    for (key, value) in entries {
        builder.push(key.to_owned(), value).map_err(|_| error("puzzle5d inspector action map entry admission failed"))?;
    }
    Ok(UiValue::Map(builder.finish()))
}

/// 🩹️ One editable numeric row: `patch<Entity> {<idsKey>, field, value}`, the exact arg shape
/// `🎮️commands/{🩹️patch-part,✊️patch-grip,🪛️patch-fastener}` accept. The host supplies the typed
/// value (or a `delta` for a stepper nudge) on top of these staged args.
///
/// 🔑️ `UiMapBuilder::push` admits keys in STRICTLY ASCENDING order only — `field` precedes the ids
/// key for `partIds`/`gripFullIds` and follows it for nothing here, so each verb states its own pair
/// in sorted order.
fn patch_row(fields: &mut UiFixedList<BuiltNode>, id: &str, label: &str, verb: &str, ids_key: &'static str, ids: &[String], field: &'static str, value: impl std::fmt::Display) -> UiAssemblyResult<()> {
    let mut entries = [("field", text_value(field)?), (ids_key, id_list_value(ids)?), ("value", text_value(&value.to_string())?)];
    entries.sort_by(|left, right| left.0.cmp(right.0));
    let action = ActionFactory::new(PUZZLE5D_PLAY_CONTROLLER_ID).action(verb, Some(action_map(entries)?))?;
    push(fields, tree_item_with_action(format!("{ROOT}.{id}"), ui_label(label)?, Some(value.to_string()), action))
}

/// 🙈️ One `setSelectionFlag` toggle row — flips `hidden`/`locked` on every id in `ids`. `value` is the
/// state the click ASKS FOR, always the inverse of the row's current one.
fn flag_row(fields: &mut UiFixedList<BuiltNode>, id: &str, label: &str, ids: &[String], field: &'static str, pressed: bool) -> UiAssemblyResult<()> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| error("puzzle5d inspector flag map admission failed"))?;
    for (key, value) in [("entity", text_value(PUZZLE5D_GRANULARITY_PART)?), ("flag", text_value(field)?), ("ids", id_list_value(ids)?), ("value", UiValue::Bool(!pressed))] {
        builder.push(key.to_owned(), value).map_err(|_| error("puzzle5d inspector flag entry admission failed"))?;
    }
    let action = ActionFactory::new(PUZZLE5D_PLAY_CONTROLLER_ID).action("setSelectionFlag", Some(UiValue::Map(builder.finish())))?;
    push(fields, tree_item_with_action(format!("{ROOT}.{id}"), ui_label(label)?, Some(pressed.to_string()), action))
}
/// 🧊️ A target volume's own flag toggle — `setTargetVolumeFlag {flag, id, value}`, 5G's verb, because
/// `setSelectionFlag` only ever writes the part slice. Keys ascend: `flag`, `id`, `value`.
fn volume_flag_row(fields: &mut UiFixedList<BuiltNode>, id: &str, label: &str, volume_id: &str, field: &'static str, pressed: bool) -> UiAssemblyResult<()> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| error("puzzle5d inspector volume flag map admission failed"))?;
    for (key, value) in [("flag", text_value(field)?), ("id", text_value(volume_id)?), ("value", UiValue::Bool(!pressed))] {
        builder.push(key.to_owned(), value).map_err(|_| error("puzzle5d inspector volume flag entry admission failed"))?;
    }
    let action = ActionFactory::new(PUZZLE5D_PLAY_CONTROLLER_ID).action("setTargetVolumeFlag", Some(UiValue::Map(builder.finish())))?;
    push(fields, tree_item_with_action(format!("{ROOT}.{id}"), ui_label(label)?, Some(pressed.to_string()), action))
}
//#endregion 🔖️Rows

//#region 🔖️Sections
fn part_fields(part: &Puzzle5dPart, document: &Puzzle5dDocument, ids: &[String], labels: &Puzzle5dLabels) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut fields = UiFixedList::default();
    read_only(&mut fields, "part.id", labels.id.as_str(), &part.id)?;
    patch_row(&mut fields, "part.kind", labels.kind.as_str(), "patchPart", "partIds", ids, "partKind", &part.part_kind)?;
    patch_row(&mut fields, "part.label", labels.label.as_str(), "patchPart", "partIds", ids, "label", part.part_3d.label.as_deref().unwrap_or_default())?;
    read_only(&mut fields, "part.display-label", labels.part.as_str(), puzzle5d_part_display_label(part, document))?;
    patch_row(&mut fields, "part.text", labels.flat_text.as_str(), "patchPart", "partIds", ids, "text", &part.part_2d.text)?;
    patch_row(&mut fields, "part.x", labels.flat_x.as_str(), "patchPart", "partIds", ids, "x", part.part_2d.x)?;
    patch_row(&mut fields, "part.y", labels.flat_y.as_str(), "patchPart", "partIds", ids, "y", part.part_2d.y)?;
    read_only(&mut fields, "part.radius", labels.radius.as_str(), part.part_2d.radius)?;
    read_only(&mut fields, "part.origin", labels.volume_origin.as_str(), vec3(part.part_3d.origin))?;
    read_only(&mut fields, "part.orientation", labels.orientation.as_str(), vec4(part.part_3d.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0])))?;
    read_only(&mut fields, "part.scale", labels.scale.as_str(), vec3(part_scale_json(part)))?;
    read_only(&mut fields, "part.grips", labels.grips.as_str(), part.grips.len())?;
    flag_row(&mut fields, "part.hidden", labels.hidden.as_str(), ids, "hidden", part.part_2d.hidden.unwrap_or(false))?;
    flag_row(&mut fields, "part.locked", labels.locked.as_str(), ids, "locked", part.part_2d.locked.unwrap_or(false))?;
    Ok(fields)
}

fn grip_fields(part: &Puzzle5dPart, grip: &Puzzle5dGrip, ids: &[String], labels: &Puzzle5dLabels) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut fields = UiFixedList::default();
    read_only(&mut fields, "grip.full-id", labels.id.as_str(), puzzle5d_grip_full_id(&part.id, &grip.id))?;
    read_only(&mut fields, "grip.part", labels.part.as_str(), &part.id)?;
    patch_row(&mut fields, "grip.kind", labels.kind.as_str(), "patchGrip", "gripFullIds", ids, "gripKind", engine_grip_kind(grip))?;
    patch_row(&mut fields, "grip.angle", labels.flat_angle.as_str(), "patchGrip", "gripFullIds", ids, "angle", grip.grip_2d.angle)?;
    patch_row(&mut fields, "grip.radius", labels.radius.as_str(), "patchGrip", "gripFullIds", ids, "radius", grip.grip_3d.radius)?;
    read_only(&mut fields, "grip.position", labels.position.as_str(), vec3(grip.grip_3d.position))?;
    read_only(&mut fields, "grip.direction", labels.direction.as_str(), vec3(grip.grip_3d.direction.unwrap_or([0.0, 0.0, -1.0])))?;
    Ok(fields)
}

fn fastener_fields(fastener: &Puzzle5dFastener, ids: &[String], labels: &Puzzle5dLabels) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut fields = UiFixedList::default();
    read_only(&mut fields, "fastener.id", labels.id.as_str(), &fastener.id)?;
    patch_row(&mut fields, "fastener.kind", labels.kind.as_str(), "patchFastener", "fastenerIds", ids, "fastenerKind", fastener.fastener_kind.as_deref().unwrap_or_default())?;
    read_only(&mut fields, "fastener.source", labels.source.as_str(), &fastener.source)?;
    read_only(&mut fields, "fastener.target", labels.target.as_str(), &fastener.target)?;
    patch_row(&mut fields, "fastener.gap", labels.gap.as_str(), "patchFastener", "fastenerIds", ids, "gap", fastener.gap)?;
    patch_row(&mut fields, "fastener.shift", labels.shift.as_str(), "patchFastener", "fastenerIds", ids, "shift", fastener.shift)?;
    patch_row(&mut fields, "fastener.rise", labels.rise.as_str(), "patchFastener", "fastenerIds", ids, "rise", fastener.rise)?;
    patch_row(&mut fields, "fastener.rotation", labels.rotation.as_str(), "patchFastener", "fastenerIds", ids, "rotation", fastener.rotation)?;
    patch_row(&mut fields, "fastener.turn", labels.turn.as_str(), "patchFastener", "fastenerIds", ids, "turn", fastener.turn)?;
    patch_row(&mut fields, "fastener.tilt", labels.tilt.as_str(), "patchFastener", "fastenerIds", ids, "tilt", fastener.tilt)?;
    Ok(fields)
}

fn target_volume_fields(volume: &Puzzle5dTargetVolume, labels: &Puzzle5dLabels) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut fields = UiFixedList::default();
    read_only(&mut fields, "target-volume.id", labels.id.as_str(), &volume.id)?;
    read_only(&mut fields, "target-volume.origin", labels.volume_origin.as_str(), vec3(volume.origin))?;
    read_only(&mut fields, "target-volume.orientation", labels.orientation.as_str(), vec4(volume.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0])))?;
    read_only(&mut fields, "target-volume.scale", labels.scale.as_str(), vec3(target_volume_scale_json(volume)))?;
    let rect = target_volume_flat_rect(volume);
    read_only(&mut fields, "target-volume.flat", labels.flat_x.as_str(), format!("{}, {}, {}, {}", rect[0], rect[1], rect[2], rect[3]))?;
    volume_flag_row(&mut fields, "target-volume.hidden", labels.hidden.as_str(), &volume.id, "hidden", volume.hidden)?;
    volume_flag_row(&mut fields, "target-volume.locked", labels.locked.as_str(), &volume.id, "locked", volume.locked)?;
    Ok(fields)
}

/// 🪟️ One entity's inspector body: the windowed selected-id section (only when the selection names
/// ids at all) followed by that entity's own field group.
fn entity_tree(id: &str, label: &str, ids: &[String], fields: UiAssemblyResult<UiFixedList<BuiltNode>>, windows: &TreeWindows<'_>, labels: &Puzzle5dLabels) -> UiAssemblyResult<BuiltNode> {
    let fields = fields?;
    let mut builder = PanelTreeBuilder::new(ROOT)?;
    if !ids.is_empty() {
        builder = builder.window_section(windows, IDS_SECTION, Some(ui_label(labels.id.as_str())?), true, ids, |id: &String| tree_item_desc(format!("{IDS_SECTION}.{id}"), ui_label(labels.id.as_str())?, Some(id.clone())))?;
    }
    builder.section(format!("{ROOT}.{id}"), Some(ui_label(label)?), true, fields)?.build()
}

/// 🈳️ The document summary — what an empty (or unresolvable) selection shows.
fn summary(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> UiAssemblyResult<BuiltNode> {
    let rows = ui_node_list([
        tree_item_desc(format!("{ROOT}.schema"), ui_label(labels.schema.as_str())?, Some(envelope.document.schema.clone())),
        tree_item_desc(format!("{ROOT}.parts"), ui_label(labels.parts.as_str())?, Some(envelope.document.parts.len().to_string())),
        tree_item_desc(format!("{ROOT}.fasteners"), ui_label(labels.fasteners.as_str())?, Some(envelope.document.fasteners.len().to_string())),
        tree_item_desc(format!("{ROOT}.utility"), ui_label(labels.utility.as_str())?, Some(envelope.active_utility.clone())),
    ])?;
    PanelTreeBuilder::new(ROOT)?.section(format!("{ROOT}.empty"), Some(ui_label(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)?), true, rows)?.build()
}

/// 🔍️ The selected entity's own field group, or `None` when the selection resolves to nothing in this
/// document (a just-deleted id, or a granularity with no inspectable body such as `kind`). Grip wins
/// over part wins over fastener, exactly the precedence the document tree picks at.
fn selected_section(document: &Puzzle5dDocument, interaction: &Puzzle5dInteractionSnapshot, windows: &TreeWindows<'_>, labels: &Puzzle5dLabels) -> Option<UiAssemblyResult<BuiltNode>> {
    match interaction.granularity.as_str() {
        PUZZLE5D_GRANULARITY_GRIP => {
            let ids = interaction.selected_grip_ids();
            ids.first().and_then(|full_id| {
                document
                    .parts
                    .iter()
                    .find_map(|part| part.grips.iter().find(|grip| &puzzle5d_grip_full_id(&part.id, &grip.id) == full_id).map(|grip| (part, grip)))
                    .map(|(part, grip)| entity_tree("grip", labels.grip.as_str(), ids, grip_fields(part, grip, ids, labels), windows, labels))
            })
        }
        PUZZLE5D_GRANULARITY_PART => {
            let ids = interaction.selected_part_ids();
            document.parts.iter().find(|part| Some(&part.id) == ids.first()).map(|part| entity_tree("part", labels.part.as_str(), ids, part_fields(part, document, ids, labels), windows, labels))
        }
        PUZZLE5D_GRANULARITY_FASTENER => {
            let ids = interaction.selected_fastener_ids();
            document.fasteners.iter().find(|fastener| Some(&fastener.id) == ids.first()).map(|fastener| entity_tree("fastener", labels.fastener.as_str(), ids, fastener_fields(fastener, ids, labels), windows, labels))
        }
        PUZZLE5D_GRANULARITY_TARGET_VOLUME => {
            let ids = interaction.selected_target_volume_ids();
            document
                .target_volumes
                .iter()
                .find(|volume| Some(&volume.id) == ids.first())
                .map(|volume| entity_tree("target-volume", labels.target_volume.as_str(), ids, target_volume_fields(volume, labels), windows, labels))
        }
        _ => None,
    }
    .or_else(|| {
        let ids = &interaction.selected;
        document
            .target_volumes
            .iter()
            .find(|volume| ids.iter().any(|id| id == &volume.id))
            .map(|volume| entity_tree("target-volume", labels.target_volume.as_str(), ids, target_volume_fields(volume, labels), windows, labels))
    })
    .or_else(|| {
        let ids = &interaction.selected;
        document.parts.iter().find(|part| ids.iter().any(|id| id == &part.id)).map(|part| entity_tree("part", labels.part.as_str(), ids, part_fields(part, document, ids, labels), windows, labels))
    })
    .or_else(|| {
        let ids = &interaction.selected;
        document.fasteners.iter().find(|fastener| ids.iter().any(|id| id == &fastener.id)).map(|fastener| entity_tree("fastener", labels.fastener.as_str(), ids, fastener_fields(fastener, ids, labels), windows, labels))
    })
    .or_else(|| {
        let ids = &interaction.selected;
        document
            .parts
            .iter()
            .find(|part| part.grips.iter().any(|grip| ids.iter().any(|id| id == &grip.id || id == &puzzle5d_grip_full_id(&part.id, &grip.id))))
            .map(|part| entity_tree("part", labels.part.as_str(), std::slice::from_ref(&part.id), part_fields(part, document, std::slice::from_ref(&part.id), labels), windows, labels))
    })
}
//#endregion 🔖️Sections

//#region 🔖️Render
pub fn render(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    match selected_section(&envelope.document, &envelope.interaction, windows, labels) {
        Some(section) => section,
        None => summary(envelope, labels),
    }
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
