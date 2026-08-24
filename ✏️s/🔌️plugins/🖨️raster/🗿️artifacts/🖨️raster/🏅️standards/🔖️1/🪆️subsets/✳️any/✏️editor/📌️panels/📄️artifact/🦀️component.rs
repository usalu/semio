//! 📄️ Raster play app panel — the layer tree.

use crate::artifacts::raster::schema::layer_name;
use crate::artifacts::raster::schema::layer_visible;
use crate::artifacts::raster::{RasterLayerNode, RasterSnapshot as RasterDocument};
use crate::editor::raster::config::RasterConfig;
use crate::editor::raster::terminology::RasterPlayLabels;
use crate::editor::raster::{layer_row_id, raster_action, ui_node_list, ui_value_map, ui_value_text, RASTER_TREE_PREFIX};
use semio_framework_plugin::{tree_item_desc, tree_item_with_action, BuiltNode, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiFixedList, UiText, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

//#region 🔖️Constants
pub const RASTER_PLAY_BODY_LAYERS: &str = "raster.play.layers";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(RASTER_PLAY_BODY_LAYERS.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn layer_tree_item(layer: &RasterLayerNode) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let nested = match layer {
        RasterLayerNode::Group { children, .. } => ui_node_list(children.iter().map(layer_tree_item))?,
        _ => UiFixedList::default(),
    };
    let description = match layer {
        RasterLayerNode::Pixel { .. } => "pixel",
        RasterLayerNode::Group { .. } => "group",
        RasterLayerNode::Adjustment { .. } => "adjustment",
    };
    let icon_id = match layer {
        RasterLayerNode::Pixel { .. } => "image",
        RasterLayerNode::Group { .. } => "folder",
        RasterLayerNode::Adjustment { .. } => "sliders-horizontal",
    };
    let mut node = tree_item_desc(layer_row_id(layer), Label::data(layer_name(layer)), Some(description.into()))?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut node.component {
        props.icon = Some(UiText::try_from_str(icon_id).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "raster layer icon admission failed"))?);
        props.default_open = Some(matches!(layer, RasterLayerNode::Group { .. }));
        props.draggable = Some(true);
        props.dimmed = Some(!layer_visible(layer));
    }
    node.base.children = nested;
    Ok(node)
}

/// 🕹️ `runtime` is unused now — layer selection/hover moved into the framework-owned `"layers"`
/// interaction domain (granularity `"layer"`, ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-
/// MECHANISM); `.interaction_domain("layers")?` below has the framework's renderer translate row
/// clicks into injected `interactionSelect` and stamp presence from `InteractionState`, replacing the
/// deleted `.selected()?`/`.highlighted()?`/`.selection_change()` calls (row ids ARE the domain's ids —
/// this tree is the sole consumer of the `"layers"` domain today).
pub async fn render(document: &RasterDocument, _runtime: &RasterConfig, labels: &RasterPlayLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut items = UiFixedList::default();
    for (kind, label, icon) in [("pixel", labels.add_pixel, "image"), ("group", labels.add_group, "folder-plus")] {
        let args = ui_value_map([("kind", ui_value_text(kind)?)])?;
        let mut item = tree_item_with_action(format!("{RASTER_TREE_PREFIX}.add.{kind}"), label, None, raster_action("addLayer", Some(args))?)?;
        if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
            props.icon = Some(UiText::try_from_str(icon).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "raster add-layer icon admission failed"))?);
        }
        items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "raster action row admission failed"))?;
    }
    for layer in &document.layers {
        let item = layer_tree_item(layer)?;
        items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "raster layer list admission failed"))?;
    }
    PanelTreeBuilder::new(RASTER_TREE_PREFIX)?.section(RASTER_TREE_PREFIX, Some(Label::data(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL)), true, items)?.interaction_domain("layers")?.build()
}
//#endregion 🔖️Render
