//! 🔍️ Puzzle 3d play app panel — the field inspector for whatever is selected. 🕹️ ticket
//! 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `Puzzle3dPlayApp::render_with_request_context`
//! resolves the live `vortex` domain once per render and hands the selection here as a
//! [`Puzzle3dInteractionSnapshot`], so this panel switches on the granularity the user actually
//! picked at (object / vortex / attraction / target volume / reference) and renders THAT entity's real
//! fields. An empty selection falls back to the document summary.
//!
//! 🩹️ Every mutating row dispatches `patchInspector` with an explicit `{entity, ids, field, value}` —
//! the same arg shape `🎮️commands/🩹️patch-inspector/🦀️.rs` accepts — so the panel never relies on that
//! command's own selection fallback.

use crate::editor::puzzle3d::terminology::Puzzle3dLabels;
use std::collections::BTreeMap;
use crate::editor::puzzle3d::{
    object_scale_json, puzzle3d_vortex_full_id, target_volume_scale_json, ui_label, ui_node_list, Puzzle3dAttraction, Puzzle3dFixture, Puzzle3dInteractionSnapshot, Puzzle3dObject, Puzzle3dReference, Puzzle3dScene, Puzzle3dTargetVolume,
    Puzzle3dVortex, PUZZLE3D_GRANULARITY_ATTRACTION, PUZZLE3D_GRANULARITY_OBJECT, PUZZLE3D_GRANULARITY_REFERENCE, PUZZLE3D_GRANULARITY_TARGET_VOLUME, PUZZLE3D_GRANULARITY_VORTEX, PUZZLE3D_PLAY_CONTROLLER_ID,
};
use semio_framework_plugin::{
    tree_item_desc, tree_item_with_action, ActionFactory, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiAssemblyResult, UiFixedList, UiValue, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};

//#region 🔖️Constants
pub const BODY_KEY: &str = "puzzle.3d.play.inspector";
const ROOT: &str = "puzzle3d-play-inspector";
pub const IDS_SECTION: &str = "puzzle3d-play-inspector.ids";
pub const IDS_ROWS: usize = 16;
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
fn error(scope: &'static str) -> semio_framework_plugin::PluginAssemblyError {
    semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", scope)
}

fn push(fields: &mut UiFixedList<BuiltNode>, node: UiAssemblyResult<BuiltNode>) -> UiAssemblyResult<()> {
    fields.try_push(node?).map_err(|_| error("puzzle3d inspector field admission failed"))
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
    semio_framework_plugin::UiText::try_from_str(value).map(UiValue::Text).ok_or_else(|| error("puzzle3d inspector action text admission failed"))
}

fn id_list_value(ids: &[String]) -> UiAssemblyResult<UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new().ok_or_else(|| error("puzzle3d inspector action list admission failed"))?;
    for id in ids.iter().take(IDS_ROWS) {
        builder.push(text_value(id)?).map_err(|_| error("puzzle3d inspector action list entry admission failed"))?;
    }
    Ok(UiValue::List(builder.finish()))
}

fn ids_page<'a>(ids: &'a [String], pages: &BTreeMap<String, u32>) -> &'a [String] {
    if ids.is_empty() {
        return ids;
    }
    let max_page = (ids.len() - 1) / IDS_ROWS;
    let page = (pages.get(IDS_SECTION).copied().unwrap_or(0) as usize).min(max_page);
    &ids[(page * IDS_ROWS).min(ids.len())..]
}

fn push_ids(fields: &mut UiFixedList<BuiltNode>, ids: &[String], pages: &BTreeMap<String, u32>) -> UiAssemblyResult<()> {
    let rest = ids_page(ids, pages);
    let page = if ids.is_empty() { 0 } else { (pages.get(IDS_SECTION).copied().unwrap_or(0) as usize).min((ids.len() - 1) / IDS_ROWS) };
    for (index, id) in rest.iter().take(IDS_ROWS).enumerate() {
        read_only(fields, &format!("ids.{index}"), "id", id)?;
    }
    if rest.len() > IDS_ROWS {
        let omitted = rest.len() - IDS_ROWS;
        let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| error("puzzle3d inspector page map admission failed"))?;
        builder.push("page".to_owned(), UiValue::Number(f64::from(page as u32 + 1))).map_err(|_| error("puzzle3d inspector page map entry admission failed"))?;
        builder.push("section".to_owned(), text_value(IDS_SECTION)?).map_err(|_| error("puzzle3d inspector page map entry admission failed"))?;
        let action = ActionFactory::new(PUZZLE3D_PLAY_CONTROLLER_ID).action("setPanelPage", Some(UiValue::Map(builder.finish())))?;
        push(fields, tree_item_with_action(format!("{ROOT}.ids.more"), ui_label(&format!("+{omitted}"))?, None, action))?;
    }
    Ok(())
}

/// 🩹️ One `patchInspector` toggle row — flips a boolean `field` (`hidden`/`locked`) on every id in
/// `ids`, the two fields that command accepts for any entity.
fn flag_row(fields: &mut UiFixedList<BuiltNode>, id: &str, label: &str, entity: &str, ids: &[String], field: &str, pressed: bool) -> UiAssemblyResult<()> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| error("puzzle3d inspector action map admission failed"))?;
    // 🔑️ `UiMapBuilder::push` admits keys in STRICTLY ASCENDING order only (`🎬️action.rs`: a key that
    // does not exceed the last one is refused, allocation returned) — `entity`, `ids`, `field`, `value`
    // regressed at `field` and killed every flag row on a live selection.
    for (key, value) in [("entity", text_value(entity)?), ("field", text_value(field)?), ("ids", id_list_value(ids)?), ("value", UiValue::Bool(!pressed))] {
        builder.push(key.to_owned(), value).map_err(|_| error("puzzle3d inspector action map entry admission failed"))?;
    }
    let action = ActionFactory::new(PUZZLE3D_PLAY_CONTROLLER_ID).action("patchInspector", Some(UiValue::Map(builder.finish())))?;
    push(fields, tree_item_with_action(format!("{ROOT}.{id}"), ui_label(label)?, Some(pressed.to_string()), action))
}
//#endregion 🔖️Rows

//#region 🔖️Sections
fn object_fields(object: &Puzzle3dObject, ids: &[String], pages: &BTreeMap<String, u32>, labels: &Puzzle3dLabels) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut fields = UiFixedList::default();
    push_ids(&mut fields, ids, pages)?;
    read_only(&mut fields, "object.id", labels.id.as_str(), &object.id)?;
    read_only(&mut fields, "object.label", labels.label.as_str(), object.label.as_deref().unwrap_or_default())?;
    read_only(&mut fields, "object.kind", labels.kind.as_str(), object.object_kind.as_deref().unwrap_or_default())?;
    read_only(&mut fields, "object.origin", labels.origin.as_str(), vec3(object.origin))?;
    read_only(&mut fields, "object.orientation", labels.orientation.as_str(), vec4(object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0])))?;
    read_only(&mut fields, "object.scale", labels.scale.as_str(), vec3(object_scale_json(object)))?;
    read_only(&mut fields, "object.mesh-url", labels.mesh_url.as_str(), object.mesh_url.as_deref().unwrap_or_default())?;
    read_only(&mut fields, "object.vortices", labels.vortices.as_str(), object.vortices.len())?;
    flag_row(&mut fields, "object.hidden", labels.hidden.as_str(), PUZZLE3D_GRANULARITY_OBJECT, ids_page(ids, pages), "hidden", object.hidden)?;
    flag_row(&mut fields, "object.locked", labels.locked.as_str(), PUZZLE3D_GRANULARITY_OBJECT, ids_page(ids, pages), "locked", object.locked)?;
    Ok(fields)
}

fn vortex_fields(object: &Puzzle3dObject, vortex: &Puzzle3dVortex, labels: &Puzzle3dLabels) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut fields = UiFixedList::default();
    read_only(&mut fields, "vortex.full-id", labels.full_id.as_str(), puzzle3d_vortex_full_id(&object.id, &vortex.id))?;
    read_only(&mut fields, "vortex.object", labels.object.as_str(), &object.id)?;
    read_only(&mut fields, "vortex.kind", labels.vortex_kind.as_str(), vortex.vortex_kind.as_deref().unwrap_or_default())?;
    read_only(&mut fields, "vortex.position", labels.position.as_str(), vec3(vortex.position))?;
    read_only(&mut fields, "vortex.direction", labels.direction.as_str(), vec3(vortex.direction.unwrap_or([0.0, 0.0, -1.0])))?;
    read_only(&mut fields, "vortex.radius", labels.radius.as_str(), vortex.radius.unwrap_or(0.36))?;
    Ok(fields)
}

fn attraction_fields(attraction: &Puzzle3dAttraction, labels: &Puzzle3dLabels) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut fields = UiFixedList::default();
    read_only(&mut fields, "attraction.id", labels.id.as_str(), &attraction.id)?;
    read_only(&mut fields, "attraction.attracting", labels.attracting.as_str(), &attraction.attracting)?;
    read_only(&mut fields, "attraction.attracted", labels.attracted.as_str(), &attraction.attracted)?;
    read_only(&mut fields, "attraction.gap", labels.gap.as_str(), attraction.gap)?;
    read_only(&mut fields, "attraction.shift", labels.shift.as_str(), attraction.shift)?;
    read_only(&mut fields, "attraction.rise", labels.rise.as_str(), attraction.rise)?;
    read_only(&mut fields, "attraction.rotation", labels.rotation_deg.as_str(), attraction.rotation)?;
    read_only(&mut fields, "attraction.turn", labels.turn_deg.as_str(), attraction.turn)?;
    read_only(&mut fields, "attraction.tilt", labels.tilt_deg.as_str(), attraction.tilt)?;
    Ok(fields)
}

fn target_volume_fields(volume: &Puzzle3dTargetVolume, ids: &[String], pages: &BTreeMap<String, u32>, labels: &Puzzle3dLabels) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut fields = UiFixedList::default();
    push_ids(&mut fields, ids, pages)?;
    read_only(&mut fields, "target-volume.id", labels.id.as_str(), &volume.id)?;
    read_only(&mut fields, "target-volume.origin", labels.origin.as_str(), vec3(volume.origin))?;
    read_only(&mut fields, "target-volume.orientation", labels.orientation.as_str(), vec4(volume.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0])))?;
    read_only(&mut fields, "target-volume.scale", labels.scale.as_str(), vec3(target_volume_scale_json(volume)))?;
    flag_row(&mut fields, "target-volume.hidden", labels.hidden.as_str(), PUZZLE3D_GRANULARITY_TARGET_VOLUME, ids_page(ids, pages), "hidden", volume.hidden)?;
    flag_row(&mut fields, "target-volume.locked", labels.locked.as_str(), PUZZLE3D_GRANULARITY_TARGET_VOLUME, ids_page(ids, pages), "locked", volume.locked)?;
    Ok(fields)
}

fn reference_fields(reference: &Puzzle3dReference, ids: &[String], pages: &BTreeMap<String, u32>, labels: &Puzzle3dLabels) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut fields = UiFixedList::default();
    push_ids(&mut fields, ids, pages)?;
    read_only(&mut fields, "reference.id", labels.id.as_str(), &reference.id)?;
    read_only(&mut fields, "reference.source-url", labels.source_url.as_str(), &reference.source.url)?;
    read_only(&mut fields, "reference.media-kind", labels.media_kind.as_str(), reference.source.media_kind.as_deref().unwrap_or_default())?;
    read_only(&mut fields, "reference.origin", labels.origin.as_str(), vec3(reference.origin))?;
    read_only(&mut fields, "reference.width-world", labels.width.as_str(), reference.width_world)?;
    flag_row(&mut fields, "reference.hidden", labels.hidden.as_str(), PUZZLE3D_GRANULARITY_REFERENCE, ids_page(ids, pages), "hidden", reference.hidden)?;
    flag_row(&mut fields, "reference.locked", labels.locked.as_str(), PUZZLE3D_GRANULARITY_REFERENCE, ids_page(ids, pages), "locked", reference.locked)?;
    Ok(fields)
}

/// 🈳️ The document summary — what an empty (or unresolvable) selection shows.
fn summary(fixture: &Puzzle3dFixture, labels: &Puzzle3dLabels) -> UiAssemblyResult<BuiltNode> {
    let rows = ui_node_list([
        tree_item_desc(format!("{ROOT}.schema"), ui_label(labels.schema.as_str())?, Some(fixture.schema.clone())),
        tree_item_desc(format!("{ROOT}.domain"), ui_label(labels.domain.as_str())?, Some(fixture.domain.clone())),
        tree_item_desc(format!("{ROOT}.objects"), ui_label(labels.objects.as_str())?, Some(fixture.objects.len().to_string())),
    ])?;
    PanelTreeBuilder::new(ROOT)?.section(format!("{ROOT}.empty"), Some(ui_label(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)?), true, rows)?.build()
}

/// 🔍️ The selected entity's own field group, or `None` when the selection resolves to nothing in this
/// document (a just-deleted id, or a granularity with no inspectable body such as `kind`).
fn selected_section(fixture: &Puzzle3dFixture, interaction: &Puzzle3dInteractionSnapshot, pages: &BTreeMap<String, u32>, labels: &Puzzle3dLabels) -> Option<UiAssemblyResult<BuiltNode>> {
    let section = |label: &str, id: &str, fields: UiAssemblyResult<UiFixedList<BuiltNode>>| -> UiAssemblyResult<BuiltNode> { PanelTreeBuilder::new(ROOT)?.section(format!("{ROOT}.{id}"), Some(ui_label(label)?), true, fields?)?.build() };
    match interaction.granularity.as_str() {
        PUZZLE3D_GRANULARITY_OBJECT => {
            let ids = interaction.selected_object_ids();
            fixture.objects.iter().find(|object| Some(&object.id) == ids.first()).map(|object| section(labels.object.as_str(), "object", object_fields(object, ids, pages, labels)))
        }
        PUZZLE3D_GRANULARITY_VORTEX => interaction.selected_vortex_ids().first().and_then(|full_id| {
            fixture
                .objects
                .iter()
                .find_map(|object| object.vortices.iter().find(|vortex| &puzzle3d_vortex_full_id(&object.id, &vortex.id) == full_id).map(|vortex| (object, vortex)))
                .map(|(object, vortex)| section(labels.vortex.as_str(), "vortex", vortex_fields(object, vortex, labels)))
        }),
        PUZZLE3D_GRANULARITY_ATTRACTION => {
            let id = interaction.selected_attraction_ids().first();
            id.and_then(|id| fixture.attractions.iter().find(|attraction| &attraction.id == id)).map(|attraction| section(labels.attraction.as_str(), "attraction", attraction_fields(attraction, labels)))
        }
        PUZZLE3D_GRANULARITY_TARGET_VOLUME => {
            let ids = interaction.selected_target_volume_ids();
            fixture.target_volumes.iter().find(|volume| Some(&volume.id) == ids.first()).map(|volume| section(labels.target_volume.as_str(), "target-volume", target_volume_fields(volume, ids, pages, labels)))
        }
        PUZZLE3D_GRANULARITY_REFERENCE => {
            let ids = interaction.selected_reference_ids();
            fixture.references.iter().find(|reference| Some(&reference.id) == ids.first()).map(|reference| section(labels.reference.as_str(), "reference", reference_fields(reference, ids, pages, labels)))
        }
        _ => None,
    }
    .or_else(|| {
        let ids = &interaction.selected;
        fixture.objects.iter().find(|object| ids.iter().any(|id| id == &object.id)).map(|object| section(labels.object.as_str(), "object", object_fields(object, ids, pages, labels)))
    })
    .or_else(|| {
        let ids = &interaction.selected;
        fixture.objects.iter().find(|object| {
            object.vortices.iter().any(|vortex| ids.iter().any(|id| id == &vortex.id || id == &puzzle3d_vortex_full_id(&object.id, &vortex.id)))
        }).map(|object| section(labels.object.as_str(), "object", object_fields(object, std::slice::from_ref(&object.id), pages, labels)))
    })
}
//#endregion 🔖️Sections

//#region 🔖️Render
pub fn render(envelope: &Puzzle3dScene, interaction: &Puzzle3dInteractionSnapshot, term_labels: &Puzzle3dLabels) -> UiAssemblyResult<BuiltNode> {
    match selected_section(&envelope.fixture, interaction, &envelope.runtime.panel_pages, term_labels) {
        Some(section) => section,
        None => summary(&envelope.fixture, term_labels),
    }
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
