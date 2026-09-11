//! 🛍️ Puzzle 3d play app panel — the kind catalogue: the object kinds available to place (draggable
//! into the viewport, with their rim-vortex templates nested) plus the vortex/cable/attraction kind
//! rows the fixture's `meta.kindCatalogs` declares.
//!
//! 🧾️ Paged on the same page contract as the outliner — a catalog declares up to `DOCUMENT_KIND_SLOTS`
//! (256) rows per section while a built node admits `UI_BUILT_CHILDREN_MAX` (32) children, so both the
//! kind sections and each object kind's nested vortex templates truncate with a `+N` continuation row
//! instead of failing admission. The paging primitives live with the outliner
//! (`📌️panels/🗿️artifact/🦀️.rs`), the panel that derives them, rather than being restated here.

use crate::editor::puzzle3d::panels::document::{page_rows, paged_section, RowBudget, SECTIONS};
use crate::editor::puzzle3d::terminology::Puzzle3dLabels;
use crate::editor::puzzle3d::{ui_label, Puzzle3dScene, PUZZLE3D_PLAY_CONTROLLER_ID};
use dsl::json;
use dsl::os_pack::json::Value;
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, BuiltNode, HasBase, HasChildren, Trigger};
use semio_framework_plugin::{ActionFactory, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};
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

fn object_kind_vortex_items(entry: &dsl::DslValue, budget: &mut RowBudget) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<BuiltNode>> {
    let kind_id = entry.get("id").and_then(|value| value.as_str()).unwrap_or("kind");
    let templates = entry.get("vortices").and_then(dsl::DslValue::as_array).unwrap_or(&[]);
    let mut index = 0;
    paged_section(&format!("puzzle3d-kind-vortex.{kind_id}"), templates, budget, |template, _| {
        let vortex_kind = template.get("vortexKind").and_then(dsl::DslValue::as_str).unwrap_or("vortex");
        let position_value = template.get("position").cloned().unwrap_or_else(|| dsl::ToValue::to_value(&[0.0, 0.0, 0.0]));
        let position = json::from_dsl_value(&position_value).to_string();
        let node = ui::tree_item(ui_label(vortex_kind)?)
            .try_id(format!("puzzle3d-kind-vortex.{index}.{vortex_kind}"))
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.vortex", "vortex id admission failed"))?
            .description(semio_framework_plugin::UiText::try_from_string(position).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.vortex", "vortex description admission failed"))?)
            .icon(semio_framework_plugin::UiText::try_from_str("circle-dot").ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.vortex", "vortex icon admission failed"))?)
            .try_build()
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.vortex", "vortex row admission failed"))?;
        index += 1;
        Ok(node)
    })
}

fn object_kind_item(entry: &dsl::DslValue, budget: &mut RowBudget) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
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
        .icon(semio_framework_plugin::UiText::try_from_str("box").ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.object", "object icon admission failed"))?)
        .default_open(false)
        .try_children(object_kind_vortex_items(entry, budget)?)
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.object", "object children admission failed"))?;
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
    builder.try_build().map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.object", "object row admission failed"))
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
/// than things a press places) stay folded — `📌️panels/🗿️artifact/🦀️.rs`'s `render_from` is the same
/// `true, false, false, false`.
///
/// Every section used to declare `default_open: false`, so opening the Catalogue showed four empty
/// headers and NO object-kind row had a layout box at all. A folded row still answers
/// `querySelectorAll`, so a press aimed at it measured `{0,0,0,0}` and landed at the viewport origin —
/// which is inside the navbar, and is why ticket 26/09/02 read this as "the navbar covers the catalogue"
/// for three waves (wave B26 §5, corrected in wave B27 §1).
pub fn render(envelope: &Puzzle3dScene, labels: &Puzzle3dLabels) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let entries = |section: &str| crate::editor::puzzle3d::puzzle3d_catalog_entries(&envelope.fixture, section);
    let budget = &mut RowBudget::new(page_rows());
    let objects = budget.nested(SECTIONS - 1, |share| paged_section(&format!("{ROOT}.objects"), entries("objects"), share, object_kind_item))?;
    let vortices = budget.nested(SECTIONS - 2, |share| paged_section(&format!("{ROOT}.vortices"), entries("vortices"), share, |entry, _| catalog_kind_item(entry, "circle-dot")))?;
    let cables = budget.nested(SECTIONS - 3, |share| paged_section(&format!("{ROOT}.cables"), entries("cables"), share, |entry, _| catalog_kind_item(entry, "plug")))?;
    let attractions = budget.nested(SECTIONS - 4, |share| paged_section(&format!("{ROOT}.attractions"), entries("attractions"), share, |entry, _| catalog_kind_item(entry, "link")))?;
    PanelTreeBuilder::new(ROOT)?
        .section(format!("{ROOT}.objects"), Some(ui_label(labels.objects.as_str())?), true, objects)?
        .section(format!("{ROOT}.vortices"), Some(ui_label(labels.vortices.as_str())?), false, vortices)?
        .section(format!("{ROOT}.cables"), Some(ui_label(labels.cables.as_str())?), false, cables)?
        .section(format!("{ROOT}.attractions"), Some(ui_label(labels.attractions.as_str())?), false, attractions)?
        .interaction_domain(crate::editor::puzzle3d::PUZZLE3D_INTERACTION_DOMAIN)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
