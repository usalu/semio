//! 🛍️ Puzzle 3d play app panel — the kind catalogue: the object kinds available to place (draggable
//! into the viewport, with their rim-vortex templates nested) plus the vortex/cable/attraction kind
//! rows the fixture's `meta.kindCatalogs` declares.
//!
//! 🪟️ Virtualised on the same window contract as the outliner: each section stamps the catalog's full
//! `total` and materialises only the host's requested row window, and each object kind's nested vortex
//! templates are a window of their own. A catalog declaring up to `DOCUMENT_KIND_SLOTS` (256) rows no
//! longer truncates with a `+N` that never advanced — the panel's page cursor was dead
//! (`📓️audit-app-panels-a.md` §4a), and the scrollbar now spans the whole catalog instead.

use crate::editor::puzzle3d::terminology::Puzzle3dLabels;
use crate::editor::puzzle3d::{ui_label, Puzzle3dScene, PUZZLE3D_INTERACTION_DOMAIN, PUZZLE3D_PLAY_CONTROLLER_ID};
use dsl::json;
use dsl::os_pack::json::Value;
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, BuiltNode, HasBase, Trigger};
use semio_framework_plugin::{
    tree_window_item, ActionFactory, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, TreeWindows, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
};
use semio_framework_ui_contract as ui;

//#region 🔖️Constants
pub const BODY_KEY: &str = "puzzle.3d.play.kinds";
const ROOT: &str = "puzzle3d-play-kinds";
/// 🖱️ MIME key `DeclarativeTreePanel` (framework/renderer/react/ui-interpreter.tsx) reads to auto-wire catalogue drag sources.
pub const PUZZLE3D_CATALOGUE_DRAG_MIME: &str = "application/x-semio-catalogue-item";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(BODY_KEY.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Rows
fn ui_text_value(value: &str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value).map(semio_framework_plugin::UiValue::Text).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.action.text", "fixed action text admission failed"))
}

fn ui_map_value(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.action.map", "fixed action map admission failed"))?;
    for (key, value) in values {
        builder.push(key.to_owned(), value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.action.map.entry", "fixed action map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

/// 🏷️ One catalog entry's display name — `label`, else `name`, else its `id`. Shared with the
/// manifest's `addObjectKind`/"Add Object" dialog option set (`✏️editor/🦀️.rs`), which must read
/// exactly the same catalog rows this panel renders.
pub(crate) fn catalog_entry_label(entry: &dsl::DslValue) -> String {
    entry.get("label").and_then(|value| value.as_str()).or_else(|| entry.get("name").and_then(|value| value.as_str())).or_else(|| entry.get("id").and_then(|value| value.as_str())).unwrap_or("kind").into()
}

/// 🌀️ One object kind's rim-vortex templates, paired with their ordinal so sibling templates that
/// declare the same `vortexKind` still key apart — a repeated key is a `DuplicateSiblingKey` refusal,
/// not a duplicate row.
fn vortex_templates(entry: &dsl::DslValue) -> Vec<(usize, &dsl::DslValue)> {
    entry.get("vortices").and_then(dsl::DslValue::as_array).unwrap_or(&[]).iter().enumerate().collect()
}

fn vortex_template_row(index: usize, template: &dsl::DslValue) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let vortex_kind = template.get("vortexKind").and_then(dsl::DslValue::as_str).unwrap_or("vortex");
    let position_value = template.get("point").or_else(|| template.get("position")).cloned().unwrap_or_else(|| dsl::ToValue::to_value(&[0.0, 0.0, 0.0]));
    let position = json::from_dsl_value(&position_value).to_string();
    ui::tree_item(ui_label(vortex_kind)?)
        .try_id(format!("puzzle3d-kind-vortex.{index}.{vortex_kind}"))
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.vortex", "vortex id admission failed"))?
        .description(semio_framework_plugin::UiText::try_from_string(position).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.vortex", "vortex description admission failed"))?)
        .icon(semio_framework_plugin::UiText::try_from_str("circle-dot").ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.vortex", "vortex icon admission failed"))?)
        .try_build()
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.vortex", "vortex row admission failed"))
}

fn object_kind_item(entry: &dsl::DslValue, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let kind_id = entry.get("id").and_then(|value| value.as_str()).unwrap_or("kind").to_string();
    let mesh_url = entry
        .get("meshUrl")
        .and_then(|value| value.as_str())
        .filter(|url| !url.is_empty())
        .map(str::to_string)
        .or_else(|| entry.get("representations").and_then(dsl::DslValue::as_array).into_iter().flatten().filter_map(|rep| rep.get("url").and_then(dsl::DslValue::as_str)).find(|url| !url.is_empty()).map(str::to_string));
    let draggable = mesh_url.is_some();
    let action_args = ui_map_value([("objectKind", ui_text_value(&kind_id)?)])?;
    let (action, args) = ActionFactory::new(PUZZLE3D_PLAY_CONTROLLER_ID).action("addObjectKind", Some(action_args))?;
    let mut builder = ui::tree_item(ui_label(catalog_entry_label(entry))?)
        .try_id(&kind_id)
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.object", "object id admission failed"))?
        .description(semio_framework_plugin::UiText::try_from_string(kind_id.clone()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.object", "object description admission failed"))?)
        .icon(semio_framework_plugin::UiText::try_from_str("box").ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.object", "object icon admission failed"))?);
    builder = match args {
        Some(args) => builder.try_on_with(Trigger::Activate, action, args),
        None => builder.try_on(Trigger::Activate, action),
    }
    .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.object", "object action admission failed"))?;
    if draggable {
        let mut payload = json!({ "objectKind": kind_id });
        if let Some(url) = mesh_url {
            if let Some(object) = payload.as_object_mut() {
                object.insert("meshUrl", Value::from(url));
            }
        }
        let mut drag_data = semio_framework_ui_contract::UiFixedMap::default();
        drag_data
            .try_push(
                semio_framework_plugin::UiText::try_from_str(PUZZLE3D_CATALOGUE_DRAG_MIME).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.drag", "drag mime admission failed"))?,
                semio_framework_plugin::UiText::try_from_string(payload.to_string()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.drag", "drag payload admission failed"))?,
            )
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.drag", "drag data admission failed"))?;
        builder = builder.draggable(true).drag_data(drag_data);
    }
    let templates = vortex_templates(entry);
    tree_window_item(windows, builder, &kind_id, false, &templates, |template| vortex_template_row(template.0, template.1))
}

fn catalog_kind_item(entry: &dsl::DslValue, icon_id: &str) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let kind_id = entry.get("id").and_then(|value| value.as_str()).unwrap_or("kind").to_string();
    ui::tree_item(ui_label(catalog_entry_label(entry))?)
        .try_id(format!("puzzle3d-kind-entry:{kind_id}"))
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.kind", "kind id admission failed"))?
        .description(semio_framework_plugin::UiText::try_from_string(kind_id).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.kind", "kind description admission failed"))?)
        .icon(semio_framework_plugin::UiText::try_from_str(icon_id).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.kind", "kind icon admission failed"))?)
        .try_build()
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.kind", "kind row admission failed"))
}
//#endregion 🔖️Rows

//#region 🔖️Render
/// 🛍️ The catalogue's placeable kinds, sectioned exactly like the outliner: the PRIMARY section opens
/// with the panel and the secondary catalogs (vortex/cable/attraction kinds, which are templates rather
/// than things a press places) stay folded — `📌️panels/🗿️artifact/🦀️.rs`'s `render` is the same
/// `true, false, false, false`.
///
/// Every section used to declare `default_open: false`, so opening the Catalogue showed four empty
/// headers and NO object-kind row had a layout box at all. A folded row still answers
/// `querySelectorAll`, so a press aimed at it measured `{0,0,0,0}` and landed at the viewport origin —
/// which is inside the navbar, and is why ticket 26/09/02 read this as "the navbar covers the catalogue"
/// for three waves (wave B26 §5, corrected in wave B27 §1).
pub fn render(envelope: &Puzzle3dScene, labels: &Puzzle3dLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let entries = |section: &str| crate::editor::puzzle3d::puzzle3d_catalog_entries(&envelope.fixture, section);
    PanelTreeBuilder::new(ROOT)?
        .window_section(windows, &format!("{ROOT}.objects"), Some(ui_label(labels.objects.as_str())?), true, entries("objects"), |entry| object_kind_item(entry, windows))?
        .window_section(windows, &format!("{ROOT}.vortices"), Some(ui_label(labels.vortices.as_str())?), false, entries("vortices"), |entry| catalog_kind_item(entry, "circle-dot"))?
        .window_section(windows, &format!("{ROOT}.cables"), Some(ui_label(labels.cables.as_str())?), false, entries("cables"), |entry| catalog_kind_item(entry, "plug"))?
        .window_section(windows, &format!("{ROOT}.attractions"), Some(ui_label(labels.attractions.as_str())?), false, entries("attractions"), |entry| catalog_kind_item(entry, "link"))?
        .interaction_domain(PUZZLE3D_PLAY_CONTROLLER_ID, PUZZLE3D_INTERACTION_DOMAIN)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
