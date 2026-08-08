//! 🛍️ Draw play app panel — the layer-kind catalogue (constitutional: was `ui`'s `Panels` region,
//! catalogue half).

use crate::apps::draw::draw_play_action;
use crate::apps::draw::terminology::DrawPlayLabels;
use crate::artifacts::draw::{DrawSnapshot, DRAW_BOOLEAN_OPERATIONS};
use semio_framework_plugin::{tree_item, tree_item_with_action, Label, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};
use std::collections::HashMap;

pub const DRAW_PLAY_BODY_CATALOGUE: &str = "draw.play.catalogue";

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: semio_framework_plugin::LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(DRAW_PLAY_BODY_CATALOGUE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(_document: &DrawSnapshot, interaction: &crate::apps::draw::config::DrawConfig, labels: &DrawPlayLabels) -> UiNode {
    let catalogue_kinds = [
        ("path", labels.kind_path, "pen-tool"),
        ("shape:rect", labels.kind_rectangle, "square"),
        ("shape:ellipse", labels.kind_ellipse, "circle"),
        ("shape:line", labels.kind_line, "minus"),
        ("shape:polygon", labels.kind_polygon, "hexagon"),
        ("text", labels.kind_text, "type"),
        ("image", labels.kind_image, "image"),
        ("group", labels.kind_group, "folder"),
        ("boolean", labels.kind_boolean, "combine"),
        ("trace", labels.kind_trace, "scan-line"),
    ];
    let mut items: Vec<UiTreeItemNode> = catalogue_kinds
        .into_iter()
        .map(|(kind, label, icon)| {
            let mut drag_data = HashMap::new();
            drag_data.insert(crate::apps::draw::panels::layers::DRAW_LAYER_KIND_DRAG_MIME.into(), serde_json::json!({ "kind": kind }).to_string());
            UiTreeItemNode { icon_id: Some(icon.into()), draggable: Some(true), drag_data: Some(drag_data), ..tree_item(format!("draw-play-catalogue.{kind}"), label) }
        })
        .collect();
    for operation in DRAW_BOOLEAN_OPERATIONS {
        items.push(UiTreeItemNode {
            icon_id: Some("combine".into()),
            ..tree_item_with_action(
                format!("draw-play-catalogue.bool.{operation}"),
                Label::data(format!("{} {operation}", labels.kind_boolean.as_str())),
                None,
                draw_play_action("combineBoolean", Some(serde_json::json!({ "operation": operation, "ids": interaction.selected_ids }))),
            )
        });
    }
    PanelTreeBuilder::new("draw-play-catalogue").section("draw-play-catalogue", Some(Label::data(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL)), true, items).build()
}
//#endregion 🔖️Render
