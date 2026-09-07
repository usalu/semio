//! 📄️ Raster play app panel — the layer tree.

use crate::artifacts::raster::schema::layer_name;
use crate::artifacts::raster::schema::layer_visible;
use crate::artifacts::raster::{RasterLayerNode, RasterSnapshot as RasterDocument};
use crate::editor::raster::config::RasterConfig;
use crate::editor::raster::terminology::RasterPlayLabels;
use crate::editor::raster::{layer_row_id, raster_action, ui_label, ui_value_map, ui_value_text, RASTER_TREE_PREFIX};
use semio_framework_plugin::{tree_item_desc, tree_item_with_action, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiFixedList, UiText, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

//#region 🔖️Constants
pub const RASTER_PLAY_BODY_LAYERS: &str = "raster.play.layers";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
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
/// 🖼️ Admits one fixed-capacity UI text for this panel's row chrome (icon keys, descriptions).
fn row_text(value: &str, stage: &'static str) -> semio_framework_plugin::UiAssemblyResult<UiText> {
    UiText::try_from_str(value).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", stage))
}

/// 🌳️ Nested group rows are admitted one at a time into the finished row's retained `BuiltChildren`
/// (`BuiltNode.children`) — the old whole-list assignment onto a `base` field is gone with the record's
/// flattening, and `BuiltChildren` is page-backed storage that only accepts `try_push`.
fn layer_tree_item(layer: &RasterLayerNode) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let (description, icon_id) = match layer {
        RasterLayerNode::Pixel { .. } => ("pixel", "image"),
        RasterLayerNode::Group { .. } => ("group", "folder"),
        RasterLayerNode::Adjustment { .. } => ("adjustment", "sliders-horizontal"),
    };
    let mut node = tree_item_desc(layer_row_id(layer), ui_label(layer_name(layer))?, Some(description.into()))?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut node.component {
        props.icon = Some(row_text(icon_id, "raster layer icon admission failed")?);
        props.default_open = Some(matches!(layer, RasterLayerNode::Group { .. }));
        props.draggable = Some(true);
        props.dimmed = Some(!layer_visible(layer));
    }
    if let RasterLayerNode::Group { children, .. } = layer {
        for child in children {
            node.children.try_push(layer_tree_item(child)?).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "raster nested layer admission failed"))?;
        }
    }
    Ok(node)
}

/// 🕹️ `runtime` is unused now — layer selection/hover moved into the framework-owned `"layers"`
/// interaction domain (granularity `"layer"`, ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-
/// MECHANISM); `.interaction_domain("layers")?` below has the framework's renderer translate row
/// clicks into injected `interactionSelect` and stamp presence from `InteractionState`, replacing the
/// deleted `.selected()?`/`.highlighted()?`/`.selection_change()` calls (row ids ARE the domain's ids —
/// this tree is the sole consumer of the `"layers"` domain today).
pub fn render(document: &RasterDocument, _runtime: &RasterConfig, labels: &RasterPlayLabels) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let mut items = UiFixedList::default();
    for (kind, label, icon) in [("pixel", labels.add_pixel, "image"), ("group", labels.add_group, "folder-plus")] {
        let args = ui_value_map([("kind", ui_value_text(kind)?)])?;
        let mut item = tree_item_with_action(format!("{RASTER_TREE_PREFIX}.add.{kind}"), ui_label(label.as_str())?, None, raster_action("addLayer", Some(args))?)?;
        if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
            props.icon = Some(row_text(icon, "raster add-layer icon admission failed")?);
        }
        items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "raster action row admission failed"))?;
    }
    for layer in &document.layers {
        let item = layer_tree_item(layer)?;
        items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "raster layer list admission failed"))?;
    }
    PanelTreeBuilder::new(RASTER_TREE_PREFIX)?.section(RASTER_TREE_PREFIX, Some(ui_label(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL)?), true, items)?.interaction_domain("layers")?.build()
}
//#endregion 🔖️Render
