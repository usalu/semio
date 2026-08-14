//! 🎭️ Raster play app panel — masked layers.

use crate::apps::raster::config::RasterConfig;
use crate::apps::raster::terminology::RasterPlayLabels;
use crate::apps::raster::{mask_row_id, RASTER_TREE_PREFIX};
use crate::artifacts::raster::{RasterLayerNode, RasterSnapshot as RasterDocument};
use semio_framework_plugin::{tree_item_desc, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode};

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
fn collect_masks(layer: &RasterLayerNode, items: &mut Vec<UiTreeItemNode>, labels: &RasterPlayLabels) {
    if let RasterLayerNode::Pixel { id, name, mask, .. } | RasterLayerNode::Group { id, name, mask, .. } = layer {
        if mask.as_ref().is_some_and(|mask| mask.enabled) {
            items.push(UiTreeItemNode {
                icon_id: Some("scan".into()),
                ..tree_item_desc(mask_row_id(id), Label::data(format!("{name} {}", labels.mask_suffix.as_str())), Some("mask".into()))
            });
        }
    }
    if let RasterLayerNode::Group { children, .. } = layer {
        for child in children {
            collect_masks(child, items, labels);
        }
    }
}

/// 🕹️ `runtime` is unused now — the masked-layer highlight used to mirror `RasterConfig.selected_ids`
/// (deleted, ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM). This tree stays un-bound to
/// `.interaction_domain("layers")`: its item ids (`mask_row_id`) are a different namespace than the
/// document/layers tree's (`layer_row_id`), so the two trees cannot both mirror the same domain
/// without id collisions — dropped rather than shown stale (matches the acceptance-bar precedent in
/// lowpoly's inspection panel).
pub fn render(document: &RasterDocument, _runtime: &RasterConfig, labels: &RasterPlayLabels) -> UiNode {
    let mut items = Vec::new();
    for layer in &document.layers {
        collect_masks(layer, &mut items, labels);
    }
    PanelTreeBuilder::new(RASTER_TREE_PREFIX).section_or_placeholder("raster-play-masks", Some(labels.masks.into()), true, items, labels.no_masks).build()
}
//#endregion 🔖️Render
