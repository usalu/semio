//! 🛍️ Draw play app panel — the layer-kind catalogue (constitutional: was `ui`'s `Panels` region,
//! catalogue half).

use crate::artifacts::draw::{DrawSnapshot, DRAW_BOOLEAN_OPERATIONS};
use crate::editor::draw::{draw_play_action, ui_node_list, ui_value_list, ui_value_map, ui_value_text};
use crate::editor::draw::terminology::DrawPlayLabels;
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase};
use semio_framework_plugin::{tree_item_with_action, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiFixedList, UiFixedMap, UiText, UiValue, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};
use semio_framework_ui_contract as ui;

pub const DRAW_PLAY_BODY_CATALOGUE: &str = "draw.play.catalogue";

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: semio_framework_plugin::LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(DRAW_PLAY_BODY_CATALOGUE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(_document: &DrawSnapshot, labels: &DrawPlayLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let catalogue_kinds = [
        ("path", labels.kind_path, "pen-tool"),
        ("shape:rect", labels.kind_rectangle, "square"),
        ("shape:ellipse", labels.kind_ellipse, "circle"),
        ("shape:line", labels.kind_line, "minus"),
        ("shape:polygon", labels.kind_polygon, "hexagon"),
        ("text", labels.kind_text, "type"),
        ("image", labels.kind_image, "image"),
        ("group", labels.kind_group, "folder"),
        ("boolean", labels.kind_boolean, "combine"),
        ("trace", labels.kind_trace, "scan-line"),
    ];
    let mut items = UiFixedList::default();
    for (kind, label, icon) in catalogue_kinds {
        let mut drag_data = UiFixedMap::default();
        let key = UiText::try_from_str(crate::editor::draw::panels::layers::DRAW_LAYER_KIND_DRAG_MIME).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "draw drag mime admission failed"))?;
        let value = UiText::try_from_string(dsl::json::to_json_string(&dsl::DslValue::object([("kind".to_string(), dsl::DslValue::String(kind.to_string()))]))).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "draw drag payload admission failed"))?;
        drag_data.try_push(key, value).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "draw drag map admission failed"))?;
        let label = ui::Label::try_from(label.as_str()).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "draw catalogue label admission failed"))?;
        let item = ui::tree_item(label)
            .try_id(format!("draw-play-catalogue.{kind}"))
            .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "draw catalogue id admission failed"))?
            .icon(UiText::try_from_str(icon).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "draw catalogue icon admission failed"))?)
            .draggable(true)
            .drag_data(drag_data)
            .try_build()
            .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "draw catalogue row admission failed"))?;
        items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "draw catalogue list admission failed"))?;
    }
    for operation in DRAW_BOOLEAN_OPERATIONS {
        let args = ui_value_map([
            ("ids", ui_value_list(std::iter::empty::<UiValue>())?),
            ("operation", ui_value_text(operation)?),
        ])?;
        let mut item = tree_item_with_action(
                format!("draw-play-catalogue.bool.{operation}"),
                format!("{} {operation}", labels.kind_boolean.as_str()),
                None,
                draw_play_action("combineBoolean", Some(args))?,
            )?;
        if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
            props.icon = Some(UiText::try_from_str("combine").ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "draw boolean icon admission failed"))?);
        }
        items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "draw boolean row admission failed"))?;
    }
    let section = ui::Label::try_from(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "draw catalogue section label admission failed"))?;
    PanelTreeBuilder::new("draw-play-catalogue")?.section("draw-play-catalogue", Some(section), true, items)?.build()
}
//#endregion 🔖️Render
