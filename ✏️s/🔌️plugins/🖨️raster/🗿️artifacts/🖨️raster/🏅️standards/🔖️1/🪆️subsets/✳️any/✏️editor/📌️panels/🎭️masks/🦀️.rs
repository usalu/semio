//! 🎭️ Raster play app panel — masked layers.
//!
//! 🪟️ The derived mask list is ONE windowed section: it stamps the full number of masked layers in the
//! document and materialises only the slice the host asked for, whatever the layer tree's depth.

use crate::editor::raster::config::RasterConfig;
use crate::editor::raster::terminology::RasterPlayLabels;
use crate::editor::raster::{mask_row_id, ui_label, RASTER_TREE_PREFIX};
use crate::{RasterLayerNode, RasterSnapshot as RasterDocument};
use semio_framework_plugin::{tree_item_desc, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, UiAssemblyResult, UiText};

//#region 🔖️Constants
pub const RASTER_PLAY_BODY_MASKS: &str = "raster.play.masks";
pub const RASTER_PLAY_MASKS_TAB_ID: &str = "raster.panel.masks";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(RASTER_PLAY_MASKS_TAB_ID.into()), label: LocalizedLabel::native("Masks", "Masken"), group: PanelGroup::Workbench, body_key: Some(RASTER_PLAY_BODY_MASKS.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🎭️ Flattens the layer tree into the `(id, name)` of every layer carrying an enabled mask — the ONE
/// logical list this panel's window container stamps its `total` over.
fn collect_masks<'a>(layer: &'a RasterLayerNode, masked: &mut Vec<(&'a str, &'a str)>) {
    if let RasterLayerNode::Pixel { id, name, mask, .. } | RasterLayerNode::Group { id, name, mask, .. } = layer {
        if mask.as_ref().is_some_and(|mask| mask.enabled) {
            masked.push((id.as_str(), name.as_str()));
        }
    }
    if let RasterLayerNode::Group { children, .. } = layer {
        for child in children {
            collect_masks(child, masked);
        }
    }
}

fn mask_row(id: &str, name: &str, labels: &RasterPlayLabels) -> UiAssemblyResult<BuiltNode> {
    let mut item = tree_item_desc(mask_row_id(id), ui_label(format!("{name} {}", labels.mask_suffix.as_str()))?, Some("mask".into()))?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
        props.icon = Some(UiText::try_from_str("scan").ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "raster mask icon admission failed"))?);
    }
    Ok(item)
}

/// 🕹️ `runtime` is unused now — the masked-layer highlight used to mirror `RasterConfig.selected_ids`
/// (deleted, ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM)?. This tree stays un-bound to
/// the `"layers"` interaction domain: its item ids (`mask_row_id`) are a different namespace than the
/// document/layers tree's (`layer_row_id`), so the two trees cannot both mirror the same domain
/// without id collisions — dropped rather than shown stale (matches the acceptance-bar precedent in
/// lowpoly's inspection panel). Unbound means unbound: no `granularity` is stamped here either.
pub fn render(document: &RasterDocument, _runtime: &RasterConfig, labels: &RasterPlayLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let mut masked = Vec::new();
    for layer in &document.layers {
        collect_masks(layer, &mut masked);
    }
    PanelTreeBuilder::new(RASTER_TREE_PREFIX)?
        .window_section_or_placeholder(windows, "raster-play-masks", Some(ui_label(labels.masks.as_str())?), true, &masked, |entry| mask_row(entry.0, entry.1, labels), ui_label(labels.no_masks.as_str())?)?
        .build()
}
//#endregion 🔖️Render
