//! 🎭️ Raster play app panel — masked layers.

use crate::artifacts::raster::{RasterLayerNode, RasterSnapshot as RasterDocument};
use crate::editor::raster::config::RasterConfig;
use crate::editor::raster::terminology::RasterPlayLabels;
use crate::editor::raster::{mask_row_id, RASTER_TREE_PREFIX};
use semio_framework_plugin::{tree_item_desc, BuiltNode, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiFixedList, UiText};

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
fn collect_masks(layer: &RasterLayerNode, items: &mut UiFixedList<BuiltNode>, labels: &RasterPlayLabels) -> semio_framework_plugin::UiAssemblyResult<()> {
    if let RasterLayerNode::Pixel { id, name, mask, .. } | RasterLayerNode::Group { id, name, mask, .. } = layer {
        if mask.as_ref().is_some_and(|mask| mask.enabled) {
            let mut item = tree_item_desc(mask_row_id(id), Label::data(format!("{name} {}", labels.mask_suffix.as_str())), Some("mask".into()))?;
            if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
                props.icon = Some(UiText::try_from_str("scan").ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "raster mask icon admission failed"))?);
            }
            items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "raster mask row admission failed"))?;
        }
    }
    if let RasterLayerNode::Group { children, .. } = layer {
        for child in children {
            collect_masks(child, items, labels)?;
        }
    }
    Ok(())
}

/// 🕹️ `runtime` is unused now — the masked-layer highlight used to mirror `RasterConfig.selected_ids`
/// (deleted, ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM)?. This tree stays un-bound to
/// `.interaction_domain("layers")?`: its item ids (`mask_row_id`) are a different namespace than the
/// document/layers tree's (`layer_row_id`), so the two trees cannot both mirror the same domain
/// without id collisions — dropped rather than shown stale (matches the acceptance-bar precedent in
/// lowpoly's inspection panel).
pub fn render(document: &RasterDocument, _runtime: &RasterConfig, labels: &RasterPlayLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut items = UiFixedList::default();
    for layer in &document.layers {
        collect_masks(layer, &mut items, labels)?;
    }
    PanelTreeBuilder::new(RASTER_TREE_PREFIX)?.section_or_placeholder("raster-play-masks", Some(labels.masks.into()), true, items, labels.no_masks)?.build()
}
//#endregion 🔖️Render
