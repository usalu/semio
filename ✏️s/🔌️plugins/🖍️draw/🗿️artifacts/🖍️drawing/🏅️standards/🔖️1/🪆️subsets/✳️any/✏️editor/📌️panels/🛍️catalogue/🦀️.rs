//! 🛍️ Drawing play app panel — the layer-kind catalogue (constitutional: was `ui`'s `Panels` region,
//! catalogue half).
//!
//! 🪟️ The fixed roster (layer kinds, then boolean operations) is ONE windowed section over a single
//! `&[CatalogueRow]` slice, so the section stamps its full extent and the host materialises the slice
//! it can see — no truncation, no `+N` row.
//!
//! 🕹️ Deliberately UNBOUND to any interaction domain: catalogue rows are not document targets, they
//! carry their own drag payload (layer kinds) or their own `combineBoolean` action.

use crate::editor::drawing::terminology::DrawingPlayLabels;
use crate::editor::drawing::{drawing_play_action, ui_value_list, ui_value_map, ui_value_text};
use crate::{DrawingSnapshot, DRAWING_BOOLEAN_OPERATIONS};
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase};
use semio_framework_plugin::{
    tree_item_with_action, BuiltNode, LabelText, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, UiAssemblyResult, UiFixedMap, UiText, UiValue, FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
};
use semio_framework_ui_contract as ui;

pub const DRAWING_PLAY_BODY_CATALOGUE: &str = "drawing.play.catalogue";

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: semio_framework_plugin::LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(DRAWING_PLAY_BODY_CATALOGUE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🛍️ One row of the catalogue's single logical roster: a draggable layer kind, or a boolean operation
/// that dispatches its own `combineBoolean` action.
enum CatalogueRow {
    Kind(&'static str, LabelText, &'static str),
    Boolean(&'static str),
}

fn catalogue_rows(labels: &DrawingPlayLabels) -> Vec<CatalogueRow> {
    let mut rows = vec![
        CatalogueRow::Kind("path", labels.kind_path, "pen-tool"),
        CatalogueRow::Kind("shape:rect", labels.kind_rectangle, "square"),
        CatalogueRow::Kind("shape:ellipse", labels.kind_ellipse, "circle"),
        CatalogueRow::Kind("shape:line", labels.kind_line, "minus"),
        CatalogueRow::Kind("shape:polygon", labels.kind_polygon, "hexagon"),
        CatalogueRow::Kind("text", labels.kind_text, "type"),
        CatalogueRow::Kind("image", labels.kind_image, "image"),
        CatalogueRow::Kind("group", labels.kind_group, "folder"),
        CatalogueRow::Kind("boolean", labels.kind_boolean, "combine"),
        CatalogueRow::Kind("trace", labels.kind_trace, "scan-line"),
    ];
    rows.extend(DRAWING_BOOLEAN_OPERATIONS.iter().copied().map(CatalogueRow::Boolean));
    rows
}

fn kind_row(kind: &str, label: LabelText, icon: &str) -> UiAssemblyResult<BuiltNode> {
    let mut drag_data = UiFixedMap::default();
    let key = UiText::try_from_str(crate::editor::drawing::panels::layers::DRAWING_LAYER_KIND_DRAG_MIME).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "drawing drag mime admission failed"))?;
    let value = UiText::try_from_string(dsl::json::to_json_string(&dsl::DslValue::object([("kind".to_string(), dsl::DslValue::String(kind.to_string()))])))
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "drawing drag payload admission failed"))?;
    drag_data.try_push(key, value).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "drawing drag map admission failed"))?;
    let label = ui::Label::try_from(label.as_str()).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "drawing catalogue label admission failed"))?;
    ui::tree_item(label)
        .try_id(format!("drawing-play-catalogue.{kind}"))
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "drawing catalogue id admission failed"))?
        .icon(UiText::try_from_str(icon).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "drawing catalogue icon admission failed"))?)
        .draggable(true)
        .drag_data(drag_data)
        .try_build()
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "drawing catalogue row admission failed"))
}

fn boolean_row(operation: &str, labels: &DrawingPlayLabels) -> UiAssemblyResult<BuiltNode> {
    let args = ui_value_map([("ids", ui_value_list(std::iter::empty::<UiValue>())?), ("operation", ui_value_text(operation)?)])?;
    let mut item = tree_item_with_action(format!("drawing-play-catalogue.bool.{operation}"), format!("{} {operation}", labels.kind_boolean.as_str()), None, drawing_play_action("combineBoolean", Some(args))?)?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
        props.icon = Some(UiText::try_from_str("combine").ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "drawing boolean icon admission failed"))?);
    }
    Ok(item)
}

pub fn render(_document: &DrawingSnapshot, labels: &DrawingPlayLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let rows = catalogue_rows(labels);
    let section = ui::Label::try_from(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "drawing catalogue section label admission failed"))?;
    PanelTreeBuilder::new("drawing-play-catalogue")?
        .window_section(windows, "drawing-play-catalogue", Some(section), true, &rows, |row| match row {
            CatalogueRow::Kind(kind, label, icon) => kind_row(kind, *label, icon),
            CatalogueRow::Boolean(operation) => boolean_row(operation, labels),
        })?
        .build()
}
//#endregion 🔖️Render
