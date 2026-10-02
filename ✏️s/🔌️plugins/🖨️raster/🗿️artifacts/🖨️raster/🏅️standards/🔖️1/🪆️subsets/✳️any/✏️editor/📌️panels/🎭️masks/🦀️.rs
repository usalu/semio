//! 🎭️ Raster play app panel — masked layers.
//!
//! 🪟️ The derived mask list is ONE windowed section: it stamps the full number of masked layers in the
//! document and materialises only the slice the host asked for, whatever the layer tree's depth.

use crate::editor::raster::config::RasterConfig;
use crate::editor::raster::terminology::RasterPlayLabels;
use crate::editor::raster::{mask_row_id, raster_action, ui_label, ui_value_list, ui_value_map, ui_value_text, RASTER_TREE_PREFIX};
use crate::{RasterLayerMask, RasterLayerNode, RasterSnapshot as RasterDocument};
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase, HasChildren, Trigger};
use semio_framework_ui_contract as ui;
use semio_framework_plugin::BuiltNode;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::PanelGroup;
use semio_framework_plugin::PanelTabDefinition;
use semio_framework_plugin::PanelTabKind;
use semio_framework_plugin::PanelTreeBuilder;
use semio_framework_plugin::PluginAssemblyError;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::UiAssemblyResult;
use semio_framework_plugin::UiText;

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
/// 🎭️ Flattens the layer tree into the identity and current settings of every layer carrying a mask — the ONE
/// logical list this panel's window container stamps its `total` over.
fn collect_masks<'a>(layer: &'a RasterLayerNode, masked: &mut Vec<(&'a str, &'a str, &'a RasterLayerMask)>) {
    if let RasterLayerNode::Pixel { id, name, mask, .. } | RasterLayerNode::Group { id, name, mask, .. } = layer {
        if let Some(mask) = mask {
            masked.push((id.as_str(), name.as_str(), mask));
        }
    }
    if let RasterLayerNode::Group { children, .. } = layer {
        for child in children {
            collect_masks(child, masked);
        }
    }
}

fn mask_row(id: &str, name: &str, mask: &RasterLayerMask, labels: &RasterPlayLabels) -> UiAssemblyResult<BuiltNode> {
    let capacity = || PluginAssemblyError::new("raster.masks.capacity", "Raster mask controls exceed UI capacity");
    let key = mask_row_id(id);
    let mut item = ui::tree_item(ui_label(format!("{name} {}", labels.mask_suffix.as_str()))?).icon(UiText::try_from_str("scan").ok_or_else(capacity)?).try_id(&key).map_err(|_| capacity())?.default_open(true);
    for (field, on, label) in [("maskEnabled", mask.enabled, labels.mask_enabled), ("maskInvert", mask.invert, labels.mask_invert)] {
        let args = ui_value_map([("field", ui_value_text(field)?), ("layerIds", ui_value_list(vec![ui_value_text(id)?])?)])?;
        let (action, args) = raster_action("patchLayers", Some(args))?;
        let control = ui::toggle(on).try_id(format!("{key}.{field}")).map_err(|_| capacity())?
            .try_label(format!("{name}: {}", label.as_str())).map_err(|_| capacity())?
            .try_on_with(Trigger::Change, action, args.ok_or_else(capacity)?).map_err(|_| capacity())?
            .try_build().map_err(|_| capacity())?;
        let row = ui::tree_item(ui_label(label.as_str())?).try_id(format!("{key}.{field}.row")).map_err(|_| capacity())?
            .try_child(control).map_err(|_| capacity())?.try_build().map_err(|_| capacity())?;
        item = item.try_child(row).map_err(|_| capacity())?;
    }
    item.try_build().map_err(|_| capacity())
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
        .window_section_or_placeholder(windows, "raster-play-masks", Some(ui_label(labels.masks.as_str())?), true, &masked, |entry| mask_row(entry.0, entry.1, entry.2, labels), ui_label(labels.no_masks.as_str())?)?
        .build()
}
//#endregion 🔖️Render

#[cfg(test)]
#[path = "🧪️tests/🎛️controls/🦀️.rs"]
mod tests;
