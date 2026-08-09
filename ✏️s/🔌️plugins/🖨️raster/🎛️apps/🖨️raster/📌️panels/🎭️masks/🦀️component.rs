//! 🎭️ Raster play app panel — masked layers.

use crate::apps::raster::config::RasterConfig;
use crate::apps::raster::terminology::RasterPlayLabels;
use crate::apps::raster::{mask_row_id, raster_action, RASTER_TREE_PREFIX};
use crate::artifacts::raster::{RasterLayerNode, RasterSnapshot as RasterDocument};
use semio_framework_plugin::{tree_item_with_action, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode};
use serde_json::json;

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
                ..tree_item_with_action(mask_row_id(id), Label::data(format!("{name} {}", labels.mask_suffix.as_str())), Some("mask".into()), raster_action("setSelection", Some(json!({ "ids": [id] }))))
            });
        }
    }
    if let RasterLayerNode::Group { children, .. } = layer {
        for child in children {
            collect_masks(child, items, labels);
        }
    }
}

pub fn render(document: &RasterDocument, runtime: &RasterConfig, labels: &RasterPlayLabels) -> UiNode {
    let mut items = Vec::new();
    for layer in &document.layers {
        collect_masks(layer, &mut items, labels);
    }
    PanelTreeBuilder::new(RASTER_TREE_PREFIX).section_or_placeholder("raster-play-masks", Some(labels.masks.into()), true, items, labels.no_masks).selected(runtime.selected_ids.iter().map(|id| mask_row_id(id)).collect()).build()
}
//#endregion 🔖️Render
