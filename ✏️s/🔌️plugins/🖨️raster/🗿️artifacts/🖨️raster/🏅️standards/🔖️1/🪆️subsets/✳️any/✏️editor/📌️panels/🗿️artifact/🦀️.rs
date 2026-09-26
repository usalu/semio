//! 📄️ Raster play app panel — the layer tree.
//!
//! 🪟️ The tree is VIRTUALISED, never paged: the section and every nested `Group` are window containers
//! that stamp the FULL `total` of their logical child list and materialise only the slice the host
//! asked for (`TreeWindows::for_body`, threaded in from the editor's `render`). There is no `+N` row
//! and no truncation — a closed group costs one stamped `total` and nothing else.

use crate::editor::raster::config::RasterConfig;
use crate::editor::raster::terminology::RasterPlayLabels;
use crate::editor::raster::{layer_row_id, raster_action, ui_label, ui_value_map, ui_value_text, RASTER_INTERACTION_DOMAIN, RASTER_INTERACTION_GRANULARITY, RASTER_PLAY_CONTROLLER_ID, RASTER_TREE_PREFIX};
use crate::standards::v1::subsets::any::schema::layer_name;
use crate::standards::v1::subsets::any::schema::layer_visible;
use crate::{RasterLayerNode, RasterSnapshot as RasterDocument};
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase};
use semio_framework_plugin::{
    tree_item_with_action, tree_window_item, BuiltNode, LabelText, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, UiAssemblyResult, UiText, FRAMEWORK_PANEL_TAB_ARTIFACT_ID,
    FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL,
};
use semio_framework_ui_contract as ui;

//#region 🔖️Constants
pub const RASTER_PLAY_BODY_LAYERS: &str = "raster.play.layers";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Artefakt"),
        group: PanelGroup::Workbench,
        body_key: Some(RASTER_PLAY_BODY_LAYERS.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🖼️ Admits one fixed-capacity UI text for this panel's row chrome (icon keys, descriptions,
/// interaction granularities).
fn row_text(value: &str, stage: &'static str) -> UiAssemblyResult<UiText> {
    UiText::try_from_str(value).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", stage))
}

/// 🌳️ One row of the layer tree's single logical roster: the two fixed "add" buttons first, then the
/// document's top-level layers. ONE windowed section over the whole roster.
enum LayersRow<'a> {
    Add(&'static str, LabelText, &'static str),
    Layer(&'a RasterLayerNode),
}

fn layers_rows<'a>(document: &'a RasterDocument, labels: &RasterPlayLabels) -> Vec<LayersRow<'a>> {
    let mut rows = vec![LayersRow::Add("pixel", labels.add_pixel, "image"), LayersRow::Add("group", labels.add_group, "folder-plus")];
    rows.extend(document.layers.iter().map(LayersRow::Layer));
    rows
}

fn add_row(kind: &str, label: LabelText, icon: &str) -> UiAssemblyResult<BuiltNode> {
    let args = ui_value_map([("kind", ui_value_text(kind)?)])?;
    let mut item = tree_item_with_action(format!("{RASTER_TREE_PREFIX}.add.{kind}"), ui_label(label.as_str())?, None, raster_action("addLayer", Some(args))?)?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
        props.icon = Some(row_text(icon, "raster add-layer icon admission failed")?);
    }
    Ok(item)
}

/// 🕹️ No per-row selection action and no argument map: the tree carries the ONE `interactionSelect`
/// binding `.interaction_domain(...)?` stamps and this row declares only its `granularity`
/// (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
///
/// 🪟️ Recursive: a `Group` is a windowed container of its own at every depth, so an expanded group
/// costs the first paint one slice instead of its whole subtree.
fn layer_tree_item(windows: &TreeWindows<'_>, layer: &RasterLayerNode, labels: &RasterPlayLabels) -> UiAssemblyResult<BuiltNode> {
    let (description, icon_id) = match layer {
        RasterLayerNode::Pixel { .. } => (labels.pixel_layer.as_str(), "image"),
        RasterLayerNode::Group { .. } => (labels.group_layer.as_str(), "folder"),
        RasterLayerNode::Adjustment { .. } => (labels.adjustment_layer.as_str(), "sliders-horizontal"),
    };
    let row_id = layer_row_id(layer);
    let item = ui::tree_item(ui_label(layer_name(layer))?)
        .try_id(&row_id)
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "raster layer row id admission failed"))?
        .description(row_text(description, "raster layer description admission failed")?)
        .icon(row_text(icon_id, "raster layer icon admission failed")?)
        .granularity(row_text(RASTER_INTERACTION_GRANULARITY, "raster layer granularity admission failed")?)
        .draggable(true)
        .dimmed(!layer_visible(layer));
    match layer {
        RasterLayerNode::Group { children, .. } => tree_window_item(windows, item, &row_id, true, children, |child| layer_tree_item(windows, child, labels)),
        _ => item.default_open(false).try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "raster layer row admission failed")),
    }
}

/// 🕹️ `runtime` is unused now — layer selection/hover moved into the framework-owned `"layers"`
/// interaction domain (granularity `"layer"`, ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-
/// MECHANISM); `.interaction_domain(...)?` below stamps the one tree-level `interactionSelect` every
/// pick row dispatches through and has the framework stamp presence from `InteractionState`, replacing
/// the deleted `.selected()?`/`.highlighted()?`/`.selection_change()` calls (row ids ARE the domain's
/// ids — this tree is the sole consumer of the `"layers"` domain today).
pub fn render(document: &RasterDocument, _runtime: &RasterConfig, labels: &RasterPlayLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let rows = layers_rows(document, labels);
    PanelTreeBuilder::new(RASTER_TREE_PREFIX)?
        .window_section(windows, RASTER_TREE_PREFIX, Some(ui_label(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL)?), true, &rows, |row| match row {
            LayersRow::Add(kind, label, icon) => add_row(kind, *label, icon),
            LayersRow::Layer(layer) => layer_tree_item(windows, layer, labels),
        })?
        .interaction_domain(RASTER_PLAY_CONTROLLER_ID, RASTER_INTERACTION_DOMAIN)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
