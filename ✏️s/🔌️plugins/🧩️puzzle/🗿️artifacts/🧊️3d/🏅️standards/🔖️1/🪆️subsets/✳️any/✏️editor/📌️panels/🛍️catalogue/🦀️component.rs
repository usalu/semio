//! 🛍️ Puzzle 3d play app panel — the kind catalogue: the object kinds available to place (draggable
//! into the viewport, with their rim-vortex templates nested) plus the vortex/cable/attraction kind
//! rows the fixture's `meta.kindCatalogs` declares.

use crate::editor::puzzle3d::terminology::Puzzle3dLabels;
use crate::editor::puzzle3d::{Puzzle3dScene, PUZZLE3D_PLAY_CONTROLLER_ID};
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, BuiltNode, HasBase, HasChildren, Trigger};
use semio_framework_plugin::{ActionFactory, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};
use semio_framework_ui_contract as ui;
use serde_json::{json, Value};
use std::collections::HashMap;

//#region 🔖️Constants
pub const BODY_KEY: &str = "puzzle.3d.play.kinds";
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
    semio_framework_plugin::UiText::try_from_str(value)
        .map(semio_framework_plugin::UiValue::Text)
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.action.text", "fixed action text admission failed"))
}

fn ui_map_value(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new()
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.action.map", "fixed action map admission failed"))?;
    for (key, value) in values {
        builder
            .push(key.to_owned(), value)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.action.map.entry", "fixed action map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

fn fixed_nodes(values: impl IntoIterator<Item = semio_framework_plugin::UiAssemblyResult<BuiltNode>>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<BuiltNode>> {
    let mut nodes = semio_framework_plugin::UiFixedList::default();
    for value in values {
        nodes
            .try_push(value?)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.items", "fixed catalogue admission failed"))?;
    }
    Ok(nodes)
}

fn catalog_entry_label(entry: &Value) -> String {
    entry.get("label").and_then(|value| value.as_str()).or_else(|| entry.get("name").and_then(|value| value.as_str())).or_else(|| entry.get("id").and_then(|value| value.as_str())).unwrap_or("kind").into()
}

fn object_kind_vortex_items(entry: &Value) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode>> {
    entry
        .get("vortices")
        .and_then(|value| value.as_array())
        .map(|templates| {
            templates
                .iter()
                .enumerate()
                .map(|(index, template)| {
                    let vortex_kind = template.get("vortexKind").and_then(|value| value.as_str()).unwrap_or("vortex");
                    let position = template.get("position").cloned().unwrap_or(json!([0.0, 0.0, 0.0]));
                    ui::tree_item(vortex_kind)?.id(format!("puzzle3d-kind-vortex.{index}.{vortex_kind}")).description(position.to_string()).icon("circle-dot").build()
                })
                .collect()
        })
        .unwrap_or_default()
}

fn object_kind_item(entry: &Value) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let kind_id = entry.get("id").and_then(|value| value.as_str()).unwrap_or("kind").to_string();
    let mesh_url = entry
        .get("meshUrl")
        .and_then(|value| value.as_str())
        .filter(|url| !url.is_empty())
        .map(str::to_string)
        .or_else(|| entry.get("representations").and_then(Value::as_array).into_iter().flatten().filter_map(|rep| rep.get("url").and_then(Value::as_str)).find(|url| !url.is_empty()).map(str::to_string));
    let draggable = mesh_url.is_some();
    let action_args = ui_map_value([("objectKind", ui_text_value(&kind_id)?)])?;
    let (action, args) = ActionFactory::new(PUZZLE3D_PLAY_CONTROLLER_ID).action("addObjectKind", Some(action_args))?;
    let mut builder = ui::tree_item(catalog_entry_label(entry))?.id(kind_id.clone()).description(kind_id.clone()).icon("box").default_open(false).children(object_kind_vortex_items(entry));
    builder = match args {
        Some(args) => builder.on_with(Trigger::Activate, action, args),
        None => builder.on(Trigger::Activate, action),
    };
    if draggable {
        let mut payload = json!({ "objectKind": kind_id });
        if let Some(url) = mesh_url {
            payload["meshUrl"] = json!(url);
        }
        builder = builder.draggable(true).drag_data(HashMap::from([(PUZZLE3D_CATALOGUE_DRAG_MIME.to_string(), payload.to_string())]));
    }
    builder.build()
}

fn catalog_kind_item(entry: &Value, icon_id: &str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let kind_id = entry.get("id").and_then(|value| value.as_str()).unwrap_or("kind").to_string();
    ui::tree_item(catalog_entry_label(entry))?.id(format!("puzzle3d-kind-entry:{kind_id}")).description(kind_id).icon(icon_id).build()
}
//#endregion 🔖️Rows

//#region 🔖️Render
pub fn render(envelope: &Puzzle3dScene, labels: &Puzzle3dLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let entries = |section: &str| crate::editor::puzzle3d::puzzle3d_catalog_entries(&envelope.fixture, section);
    let object_entries = entries("objects");
    let vortex_entries = entries("vortices");
    let cable_entries = entries("cables");
    let attraction_entries = entries("attractions");
    PanelTreeBuilder::new("puzzle3d-play-kinds")?
        .section("puzzle3d-play-kinds.objects", Some(labels.objects.as_str().into()), false, fixed_nodes(object_entries.iter().map(object_kind_item))?)?
        .section("puzzle3d-play-kinds.vortices", Some(labels.vortices.as_str().into()), false, fixed_nodes(vortex_entries.iter().map(|entry| catalog_kind_item(entry, "circle-dot")))?)?
        .section("puzzle3d-play-kinds.cables", Some(labels.cables.as_str().into()), false, fixed_nodes(cable_entries.iter().map(|entry| catalog_kind_item(entry, "plug")))?)?
        .section("puzzle3d-play-kinds.attractions", Some(labels.attractions.as_str().into()), false, fixed_nodes(attraction_entries.iter().map(|entry| catalog_kind_item(entry, "link")))?)?
        .interaction_domain(crate::editor::puzzle3d::PUZZLE3D_INTERACTION_DOMAIN)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::puzzle3d::config::{Puzzle3dConfig, Puzzle3dRuntime};
    use crate::editor::puzzle3d::terminology::puzzle3d_labels;
    use crate::editor::puzzle3d::{nakagin_fixture, Puzzle3dScene, PUZZLE3D_DEFAULT_UTILITY};

    #[test]
    fn kinds_tree_object_drag_data_carries_object_kind_and_mesh_url() {
        let envelope = Puzzle3dScene { fixture: nakagin_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: PUZZLE3D_DEFAULT_UTILITY.into() };
        let labels = puzzle3d_labels(&Puzzle3dConfig::default());
        let node = render(&envelope, labels);
        assert!(matches!(node.component, semio_framework_ui_contract::Component::Tree(_)));
        let objects = node.children.iter().find(|section| section.key == "puzzle3d-play-kinds.objects").expect("objects section");
        let draggable = objects
            .children
            .iter()
            .find_map(|item| match &item.component {
                semio_framework_ui_contract::Component::TreeItem(props) if props.draggable == Some(true) => Some(props),
                _ => None,
            })
            .expect("draggable object kind");
        let drag_data = draggable.drag_data.as_ref().expect("drag data");
        let encoded = drag_data.get(PUZZLE3D_CATALOGUE_DRAG_MIME).expect("catalogue mime");
        let payload: Value = serde_json::from_str(encoded).expect("drag payload json");
        assert!(payload.get("objectKind").and_then(Value::as_str).is_some(), "drag payload must carry objectKind");
        assert!(payload.get("meshUrl").and_then(Value::as_str).filter(|url| !url.is_empty()).is_some(), "drag payload must carry meshUrl for preview");
    }
}
//#endregion 🧪️Tests
