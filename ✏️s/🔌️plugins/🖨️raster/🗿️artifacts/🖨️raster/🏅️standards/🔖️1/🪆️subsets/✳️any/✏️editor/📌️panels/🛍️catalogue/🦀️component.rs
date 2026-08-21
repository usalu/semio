//! 🛍️ Raster play app panel — the layer-kind catalogue.

use crate::editor::raster::terminology::RasterPlayLabels;
use semio_framework_plugin::{ui_declarative_sections_to_tree, ui_text, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiNode, UiPresence, UiSectionNode, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

//#region 🔖️Constants
pub const RASTER_PLAY_BODY_CATALOGUE: &str = "raster.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(RASTER_PLAY_BODY_CATALOGUE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub async fn render(labels: &RasterPlayLabels) -> UiNode {
    ui_declarative_sections_to_tree(&[UiSectionNode {
        id: "raster-catalogue".into(),
        label: Some(labels.layer_kinds.into()),
        default_open: Some(true),
        presence: UiPresence::default(),
        children: vec![ui_text(labels.catalogue_pixel), ui_text(labels.catalogue_group), ui_text(labels.catalogue_adjustment)],
        menu: None,
    }])
}
//#endregion 🔖️Render
